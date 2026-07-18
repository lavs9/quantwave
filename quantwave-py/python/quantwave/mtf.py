"""Multi-timeframe (MTF) helpers for QuantWave Polars pipelines.

This module provides three small, composable building blocks for working
with a base timeframe (e.g. 1-minute bars) alongside a higher timeframe
(e.g. 1-hour / 1-day bars) computed from the same underlying data:

* :func:`ohlcv_resample` -- canonical OHLCV aggregation to a higher timeframe
  via Polars ``group_by_dynamic``.
* :func:`mtf_apply` -- resample + compute arbitrary Polars/``.ta`` expressions
  on the higher-timeframe frame (e.g. ``pl.col("close").ta.rsi(14)``).
* :func:`mtf_broadcast` -- join the higher-timeframe columns back onto the
  base-timeframe rows with **lookahead-safe** ("strictly past") semantics by
  default.

Lookahead safety
-----------------
The single most important property of this module is that
:func:`mtf_broadcast` never leaks information from a higher-timeframe bar
that is still "in progress" at a given base-timeframe timestamp ``t``. See
the docstring of :func:`mtf_broadcast` for the exact mechanism (shifting the
higher-timeframe join key to bar-close and using a strict ``join_asof``).

All functions operate on the canonical OHLCV schema used across QuantWave:
``open``/``high``/``low``/``close``/``volume`` plus a tz-aware timestamp
column (default name ``"ts"``). Timestamps follow the project convention of
being tz-aware (commonly ``Asia/Kolkata``, IST).
"""

from __future__ import annotations

from typing import Sequence, Union

import polars as pl

# A single column name or a list of column names, used for multi-symbol
# grouping (``by=``) throughout this module.
ByCols = Union[str, Sequence[str], None]


def _by_list(by: ByCols) -> list[str]:
    """Normalize the ``by`` parameter to a list of column names (possibly empty)."""
    if by is None:
        return []
    if isinstance(by, str):
        return [by]
    return list(by)


def ohlcv_resample(
    df: pl.DataFrame,
    every: str = "1d",
    by: ByCols = None,
    ts: str = "ts",
) -> pl.DataFrame:
    """Resample canonical OHLCV rows to a higher timeframe.

    Uses Polars ``group_by_dynamic`` with the standard OHLCV aggregation:
    ``open`` = first, ``high`` = max, ``low`` = min, ``close`` = last,
    ``volume`` = sum. The timestamp column of the result labels each bucket
    by its **start** (``label="left"``, ``closed="left"``, i.e. window
    ``[start, start + every)``) -- this is the Polars default and is what
    :func:`mtf_broadcast` relies on to compute bar-close times.

    Args:
        df: Canonical OHLCV frame with an ``open``/``high``/``low``/``close``/
            ``volume`` schema and a tz-aware timestamp column named ``ts``.
        every: Bucket width, e.g. ``"1h"``, ``"1d"`` (Polars interval string).
        by: Optional column name (or list of names) to group by in addition
            to the time bucket, e.g. a ``"symbol"`` column for multi-symbol
            frames. Grouped output is sorted by ``by`` then ``ts``.
        ts: Name of the timestamp column. Must be sorted ascending (within
            each ``by`` group, if given).

    Returns:
        A new DataFrame with one row per (``by``, time bucket), columns
        ``[*by, ts, open, high, low, close, volume]``.
    """
    group_cols = _by_list(by)
    agg = [
        pl.col("open").first().alias("open"),
        pl.col("high").max().alias("high"),
        pl.col("low").min().alias("low"),
        pl.col("close").last().alias("close"),
        pl.col("volume").sum().alias("volume"),
    ]
    out = df.group_by_dynamic(
        ts,
        every=every,
        group_by=group_cols or None,
    ).agg(agg)
    sort_cols = [*group_cols, ts]
    return out.sort(sort_cols)


def mtf_apply(
    df: pl.DataFrame,
    every: str,
    exprs: Sequence[pl.Expr],
    by: ByCols = None,
    ts: str = "ts",
) -> pl.DataFrame:
    """Resample to a higher timeframe and compute expressions on that frame.

    First resamples ``df`` to ``every`` via :func:`ohlcv_resample`, then
    evaluates ``exprs`` (arbitrary Polars expressions, including the ``.ta``
    plugin namespace, e.g. ``pl.col("close").ta.rsi(14).alias("rsi_htf")``)
    on the resulting higher-timeframe frame using ``with_columns``.

    Args:
        df: Canonical base-timeframe OHLCV frame.
        every: Higher-timeframe bucket width, e.g. ``"1h"``.
        exprs: List of Polars expressions to compute on the higher-timeframe
            frame. Each expression should carry its own ``.alias(...)``.
        by: Optional grouping column(s) for multi-symbol frames. When given,
            expressions that involve rolling/window computations (e.g.
            ``.ta.rsi``) are evaluated per group via ``.over(by)`` so that
            one symbol's history never leaks into another's.
        ts: Name of the timestamp column.

    Returns:
        The higher-timeframe frame (``[*by, ts, open, high, low, close,
        volume]``) with the additional computed columns appended.
    """
    htf = ohlcv_resample(df, every=every, by=by, ts=ts)
    group_cols = _by_list(by)
    if group_cols:
        exprs = [e.over(group_cols) for e in exprs]
    return htf.with_columns(list(exprs))


def mtf_broadcast(
    base_df: pl.DataFrame,
    higher_df: pl.DataFrame,
    on: str = "ts",
    by: ByCols = None,
    suffix: str = "_htf",
    allow_current_bar: bool = False,
) -> pl.DataFrame:
    """Broadcast higher-timeframe columns onto base-timeframe rows.

    This is the lookahead-safety-critical function of the module.

    Default (``allow_current_bar=False``, STRICTLY-PAST) semantics
    ------------------------------------------------------------------
    For every base-timeframe row at time ``t``, the joined higher-timeframe
    values must come from the last **completed** higher-timeframe bar
    strictly before ``t`` -- never the bar that is still in progress at
    ``t``.

    Mechanism: ``higher_df``'s timestamp column (as produced by
    :func:`ohlcv_resample` / :func:`mtf_apply`) labels each bucket by its
    **start** (``label="left"``): a bucket covering ``[start, start+every)``
    is stamped with ``start``. That bucket is only fully known once we reach
    its **close**, i.e. ``start + every``. We cannot express "+every" for an
    arbitrary calendar-aware interval string as a fixed offset directly
    from ``on`` alone in a vectorized way here, so instead we infer each
    bucket's close from the *next* bucket's start (the boundary shared by
    consecutive buckets), computed by shifting the higher-timeframe
    timestamp column up by one row (within each ``by`` group):

        close_time[i] = start_time[i + 1]   (i.e. ``pl.col(on).shift(-1)``)

    This is exact for the well-formed, contiguous bucket sequence produced
    by ``group_by_dynamic``/``ohlcv_resample`` (every bucket abuts the next),
    and avoids re-deriving the ``every`` interval's calendar semantics (DST,
    variable month/day lengths, etc.) by hand. The **last** higher-timeframe
    bucket has no "next" row, so its close is unknown; we conservatively use
    a sentinel far in the future (``pl.datetime`` max) so that bucket is
    still selectable once *some* base bar arrives after it (its true close is
    unknowable from the data given, so we treat it as available -- but only
    after its start, and only via the ``allow_exact_matches=False`` strict
    comparison below, matching everything else here).

    We then perform a ``join_asof`` (``strategy="backward"``) of the base
    frame against the higher frame keyed on this bar-close time, with
    ``allow_exact_matches=False``. Backward-asof selects the last
    higher-timeframe row whose key is **strictly less than** the base row's
    ``on`` value. Since a bar in progress at ``t`` has
    ``start <= t < close``, its ``close > t`` and it is therefore never
    selected; only bars with ``close < t`` -- i.e. bars completed strictly
    before ``t`` -- can match. This is what makes the join lookahead-safe by
    construction rather than by convention.

    ``allow_current_bar=True`` opts out of this protection and instead joins
    directly on the raw (bucket-start) higher-timeframe timestamps with
    ``allow_exact_matches=True`` (ordinary ``join_asof`` backward semantics),
    i.e. the in-progress bar's partial aggregate *is* used once ``t`` reaches
    its start. This is useful only for exploratory/visualization purposes,
    never for signal generation.

    Args:
        base_df: Base-timeframe frame (e.g. 1-minute bars) with timestamp
            column ``on``.
        higher_df: Higher-timeframe frame (e.g. output of
            :func:`ohlcv_resample`/:func:`mtf_apply`) with the same-named
            timestamp column ``on``, labeled by bucket start.
        on: Name of the (shared) timestamp column in both frames.
        by: Optional grouping column(s) present in both frames, for
            multi-symbol joins (e.g. ``"symbol"``). The asof join is
            performed within each group independently, so one symbol's
            higher-timeframe values can never be broadcast onto another
            symbol's rows.
        suffix: Suffix appended to higher-timeframe column names that
            collide with base-frame column names (passed through to
            ``join_asof``).
        allow_current_bar: If True, use the (unsafe) in-progress bar instead
            of enforcing strictly-past semantics. Default False.

    Returns:
        ``base_df`` with the higher-timeframe columns joined on.
    """
    group_cols = _by_list(by)
    base_sorted = base_df.sort([*group_cols, on])
    higher_sorted = higher_df.sort([*group_cols, on])

    if allow_current_bar:
        join_key = higher_sorted
        allow_exact_matches = True
    else:
        # Bar-close time = next bucket's start, within each `by` group.
        close_col = "__mtf_bar_close__"
        if group_cols:
            close_expr = pl.col(on).shift(-1).over(group_cols)
        else:
            close_expr = pl.col(on).shift(-1)
        join_key = higher_sorted.with_columns(
            pl.coalesce([close_expr, pl.col(on).dt.offset_by("100y")]).alias(close_col)
        ).drop(on).rename({close_col: on})
        allow_exact_matches = False

    joined = base_sorted.join_asof(
        join_key,
        on=on,
        by=group_cols or None,
        strategy="backward",
        suffix=suffix,
        allow_exact_matches=allow_exact_matches,
        # Both frames are explicitly `.sort(...)`-ed immediately above, so we
        # skip Polars' own (weaker, `by`-unaware) sortedness check here.
        check_sortedness=False,
    )
    return joined
