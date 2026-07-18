"""ML label helpers: forward returns and triple-barrier labeling.

Both helpers produce *targets* (labels) built from future price data — using
the future here is correct because these columns feed supervised-learning
targets, not point-in-time features. Callers must still be careful never to
join these columns back in as model *inputs* for the same timestamp.

Triple-barrier definition (Lopez de Prado, kept intentionally simple):

    From each bar ``t``, look forward over bars ``t+1 .. t+max_holding``
    (a closed/inclusive window of up to ``max_holding`` bars).

    ``upper = price[t] * (1 + pt)``
    ``lower = price[t] * (1 - sl)``

    The first bar in the window with ``price >= upper`` touches the profit
    barrier -> ``label = +1``, ``touch_kind = "pt"``.
    The first bar in the window with ``price <= lower`` touches the loss
    barrier -> ``label = -1``, ``touch_kind = "sl"``.
    First touch (by bar order) wins. If a single bar satisfies both
    conditions simultaneously (only possible with degenerate/non-positive
    ``pt``/``sl``), the profit barrier wins by convention.

    If neither barrier is touched anywhere in the window, the vertical
    (time) barrier applies -> ``label = 0``, ``touch_kind = "time"``,
    ``touch_idx = t + max_holding``.

    Tie policy: a touch landing exactly on bar ``t + max_holding`` still
    counts as a profit/loss touch, not a time-out — the window is closed on
    the right, so the last bar is eligible for both barrier and time checks,
    and barrier touches take priority.

    If fewer than ``max_holding`` future bars exist (end of the series or
    end of the symbol's own data when ``by`` is set) and no barrier was
    touched in the bars that *do* exist, the verdict is undetermined:
    ``label``, ``touch_idx``, and ``touch_kind`` are all null. This mirrors
    ``forward_returns``' convention of nulling rows with insufficient future
    data rather than fabricating a verdict.

    ``pt``/``sl`` are fractional thresholds (e.g. ``0.02`` == 2%) and may be
    passed as a column name instead of a scalar, in which case each row uses
    its own per-row threshold (e.g. an ATR-scaled column computed upstream).

    ``touch_idx`` is the bar index within the symbol's own series: 0-based,
    local to each group when ``by`` is supplied (not a global row offset).
"""

from __future__ import annotations

from typing import TYPE_CHECKING, List, Optional, Sequence, Union

if TYPE_CHECKING:
    import polars as pl


def _pl():
    """Late-bind polars so ``import quantwave`` works without the optional extra."""
    import polars as pl

    return pl


def forward_returns(
    df: "pl.DataFrame",
    horizons: Sequence[int] = (1, 5, 10),
    price: str = "close",
    by: Optional[str] = None,
) -> "pl.DataFrame":
    """Add forward-return label columns ``fwd_ret_{h}`` for each horizon.

    ``fwd_ret_{h}[t] = (price[t+h] - price[t]) / price[t]``.

    The last ``h`` rows of each horizon (per symbol, when ``by`` is given)
    are null since no future price exists to compute the return. When
    ``by`` is set, horizons never look past the end of a symbol's own rows
    into the next symbol's data.

    Args:
        df: Input DataFrame containing at least the ``price`` column (and
            ``by`` column, if given).
        horizons: Bar counts to look forward.
        price: Column to compute returns from.
        by: Optional symbol/group column; horizons are computed
            independently within each group and never cross group
            boundaries.

    Returns:
        ``df`` with one added column ``fwd_ret_{h}`` per horizon.
    """
    pl = _pl()
    if by is None:
        return _forward_returns_single(df, horizons, price)

    parts = df.partition_by(by, maintain_order=True)
    computed = [_forward_returns_single(part, horizons, price) for part in parts]
    return pl.concat(computed)


def _forward_returns_single(
    df: "pl.DataFrame",
    horizons: Sequence[int],
    price: str,
) -> "pl.DataFrame":
    pl = _pl()
    prices = df[price].cast(pl.Float64).to_list()
    n = len(prices)

    new_cols = {}
    for h in horizons:
        col: List[Optional[float]] = [None] * n
        for t in range(n - h):
            p0 = prices[t]
            col[t] = (prices[t + h] - p0) / p0 if p0 else None
        new_cols[f"fwd_ret_{h}"] = col

    return df.with_columns(
        [pl.Series(name, values, dtype=pl.Float64) for name, values in new_cols.items()]
    )


def triple_barrier(
    df: "pl.DataFrame",
    price: str = "close",
    pt: Union[float, str] = 0.02,
    sl: Union[float, str] = 0.02,
    max_holding: int = 10,
    by: Optional[str] = None,
) -> "pl.DataFrame":
    """Label each bar with the López de Prado triple-barrier method.

    See the module docstring for the precise barrier math and tie policy.
    Adds three columns: ``label`` (``-1``/``0``/``+1``), ``touch_idx`` (bar
    index of the first touch, local to the symbol's own series), and
    ``touch_kind`` (``"pt"``/``"sl"``/``"time"``).

    Args:
        df: Input DataFrame containing at least the ``price`` column (and
            ``by``/``pt``/``sl`` columns, if given as column names).
        price: Column to evaluate barriers against.
        pt: Profit-taking threshold, fractional (e.g. ``0.02`` == 2% above
            entry price), either a scalar applied to every row or the name
            of a per-row threshold column.
        sl: Stop-loss threshold, fractional, same scalar-or-column rules
            as ``pt``.
        max_holding: Maximum number of bars to hold before the vertical
            (time) barrier applies.
        by: Optional symbol/group column; barrier windows are computed
            independently within each group and never look into the next
            symbol's rows.

    Returns:
        ``df`` with added ``label``, ``touch_idx``, ``touch_kind`` columns.
    """
    pl = _pl()
    if by is None:
        return _triple_barrier_single(df, price, pt, sl, max_holding)

    parts = df.partition_by(by, maintain_order=True)
    computed = [_triple_barrier_single(part, price, pt, sl, max_holding) for part in parts]
    return pl.concat(computed)


def _triple_barrier_single(
    df: "pl.DataFrame",
    price: str,
    pt: Union[float, str],
    sl: Union[float, str],
    max_holding: int,
) -> "pl.DataFrame":
    pl = _pl()
    prices = df[price].cast(pl.Float64).to_list()
    n = len(prices)

    pt_vals = (
        df[pt].cast(pl.Float64).to_list() if isinstance(pt, str) else [float(pt)] * n
    )
    sl_vals = (
        df[sl].cast(pl.Float64).to_list() if isinstance(sl, str) else [float(sl)] * n
    )

    labels: List[Optional[int]] = [None] * n
    touch_idx: List[Optional[int]] = [None] * n
    touch_kind: List[Optional[str]] = [None] * n

    for t in range(n):
        p0 = prices[t]
        upper = p0 * (1.0 + pt_vals[t])
        lower = p0 * (1.0 - sl_vals[t])

        window_end = min(t + max_holding, n - 1)
        found = False
        for k in range(t + 1, window_end + 1):
            pk = prices[k]
            hit_upper = pk >= upper
            hit_lower = pk <= lower
            if hit_upper:
                labels[t] = 1
                touch_idx[t] = k
                touch_kind[t] = "pt"
                found = True
                break
            if hit_lower:
                labels[t] = -1
                touch_idx[t] = k
                touch_kind[t] = "sl"
                found = True
                break

        if not found:
            if t + max_holding <= n - 1:
                labels[t] = 0
                touch_idx[t] = t + max_holding
                touch_kind[t] = "time"
            # else: insufficient future bars to reach a verdict -> leave null

    return df.with_columns(
        [
            pl.Series("label", labels, dtype=pl.Int64),
            pl.Series("touch_idx", touch_idx, dtype=pl.Int64),
            pl.Series("touch_kind", touch_kind, dtype=pl.Utf8),
        ]
    )
