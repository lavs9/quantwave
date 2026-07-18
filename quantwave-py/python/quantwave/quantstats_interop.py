"""QuantStats-compatible tear-sheet interop.

This module derives a periodic returns series from a quantwave
``BacktestResult`` (or a raw equity-curve ``polars.DataFrame``) and, when the
optional ``pandas`` / ``quantstats`` packages are installed, hands that series
to `QuantStats <https://github.com/ranaroussi/quantstats>`_ for its reports
and metrics.

The native tear sheet in :mod:`quantwave.tearsheet` (backed by the Rust
engine) remains the default, zero-dependency way to inspect a backtest. This
module is purely an *interop* layer for teams that already have a QuantStats
workflow (benchmark comparisons, HTML tear sheets, etc.) and want to feed it
quantwave results — it does not replace anything.

Returns derivation
-------------------
``backtest_returns`` takes the ``equity_curve`` polars frame (columns
``ts``, ``equity``, ...), optionally resamples it to a fixed ``freq`` by
taking the last equity observed in each bucket (``polars`` ``dt.truncate``
semantics — e.g. ``freq="1d"`` keeps the last equity value on each calendar
day), then computes simple returns as a percent-change:

    return[i] = equity[i] / equity[i - 1] - 1

The first row of the (resampled) equity curve has no prior observation and
is dropped, so the returned series always has one fewer row than the number
of distinct ``freq`` buckets. This is the same convention pandas/QuantStats
use for ``prices.pct_change().dropna()``.

``freq`` accepts any `polars duration string
<https://docs.pola.rs/api/python/stable/reference/expressions/api/polars.Expr.dt.truncate.html>`_
(``"1d"`` for daily, ``"1h"``/``"5m"`` for intraday, etc.), or ``None`` /
``"raw"`` to skip resampling entirely and take per-bar returns as-is (only
sensible when every bar in ``equity_curve`` is already at the frequency you
want). The default ``"1d"`` is a no-op when the underlying backtest already
runs on daily bars.

Timestamps are converted to timezone-aware ``Asia/Kolkata`` (IST) datetimes,
matching this project's canonical timestamp convention (see
:mod:`quantwave.datasets`). The native ``equity_curve.ts`` column is Unix
seconds (UTC epoch); it is interpreted as UTC and converted to IST.

Sharpe / Max Drawdown mapping
------------------------------
``quantwave`` and QuantStats compute Sharpe and Max Drawdown independently,
and the definitions coincide only under matching conventions:

* **Sharpe ratio** — quantwave: ``sqrt(252) * mean(r) / std(r, ddof=1)`` on
  *per-bar* returns, risk-free rate 0 (see
  ``quantwave-backtest/src/metrics.rs::compute_sharpe``). QuantStats'
  ``quantstats.stats.sharpe(returns, rf=0.0, periods=252)`` uses the same
  formula (sample std, ``ddof=1``) on whatever return series it is given.
  These match to floating-point precision when ``returns`` is derived with
  ``freq`` equal to the backtest's native bar frequency (e.g. ``freq="1d"``
  for a daily-bar backtest) — QuantStats has no notion of the underlying bar
  size, so passing a mismatched ``freq`` (e.g. resampling an intraday
  backtest to daily) will annualize a different return series than the
  engine used and the two numbers will legitimately diverge.

* **Max drawdown** — quantwave's ``max_drawdown_pct`` is a **positive**
  fraction computed peak-to-trough directly on the (unresampled) per-bar
  equity curve. QuantStats' ``quantstats.stats.max_drawdown(returns)``
  returns a **negative** fraction computed from the cumulative product of
  whatever ``returns`` series it receives. For a daily-bar backtest with
  ``freq="1d"`` (a no-op resample), ``abs(qs.stats.max_drawdown(returns))``
  matches ``result.metrics()["max_drawdown_pct"]`` to floating-point
  precision, because both are then computed on the same per-bar equity
  path. For an intraday backtest resampled to a coarser ``freq``, QuantStats
  can only see drawdowns at bucket boundaries and may report a *smaller*
  magnitude than the engine's true intraday peak-to-trough drawdown.

Documented tolerance: for a backtest whose native bar frequency matches the
``freq`` passed to :func:`to_quantstats` (the common case — daily bars with
``freq="1d"``), Sharpe and ``abs(max_drawdown)`` are expected to agree within
``1e-6`` (float64 rounding only). For mismatched frequencies, no tolerance is
documented — the values encode genuinely different quantities and should not
be compared directly.
"""

from __future__ import annotations

from typing import Any, Optional, Union

import polars as pl

IST = "Asia/Kolkata"

#: Human-readable mapping from this engine's ``PerformanceMetrics`` fields to
#: the closest QuantStats ``quantstats.stats`` function / ``reports.metrics``
#: row label. Not exhaustive — only the fields cross-validated by this module.
METRIC_MAPPING: dict[str, dict[str, str]] = {
    "sharpe_ratio": {
        "quantstats_stat": "quantstats.stats.sharpe(returns, rf=0.0, periods=252)",
        "quantstats_report_row": "Sharpe",
        "sign": "same",
        "tolerance": "1e-6 (when freq matches the backtest's native bar size)",
    },
    "max_drawdown_pct": {
        "quantstats_stat": "abs(quantstats.stats.max_drawdown(returns))",
        "quantstats_report_row": "Max Drawdown",
        "sign": "engine is positive, quantstats is negative (compare via abs())",
        "tolerance": "1e-6 (when freq matches the backtest's native bar size)",
    },
}


def _equity_curve_frame(result_or_equity: Any) -> pl.DataFrame:
    """Extract a plain polars ``DataFrame`` with ``ts`` and ``equity`` columns."""
    equity_curve = getattr(result_or_equity, "equity_curve", result_or_equity)
    if isinstance(equity_curve, pl.LazyFrame):
        equity_curve = equity_curve.collect()
    if not isinstance(equity_curve, pl.DataFrame):
        raise TypeError(
            "backtest_returns expects a BacktestResult/BacktestReport (with an "
            "'equity_curve' attribute) or a polars DataFrame/LazyFrame with "
            f"'ts' and 'equity' columns; got {type(equity_curve)!r}"
        )
    missing = {"ts", "equity"} - set(equity_curve.columns)
    if missing:
        raise ValueError(
            f"equity curve is missing required column(s) {sorted(missing)}; "
            f"got columns {equity_curve.columns}"
        )
    return _normalize_ts(equity_curve.select(["ts", "equity"]).sort("ts"))


def _normalize_ts(frame: pl.DataFrame) -> pl.DataFrame:
    """Coerce the ``ts`` column to a tz-aware (IST) ``Datetime``."""
    dtype = frame.schema["ts"]
    if dtype in (pl.Int64, pl.Int32, pl.UInt64, pl.UInt32):
        # Native BacktestResult.equity_curve.ts is Unix seconds (UTC epoch).
        return frame.with_columns(
            pl.from_epoch(pl.col("ts"), time_unit="s")
            .dt.replace_time_zone("UTC")
            .dt.convert_time_zone(IST)
            .alias("ts")
        )
    if isinstance(dtype, pl.Datetime):
        if dtype.time_zone is None:
            return frame.with_columns(
                pl.col("ts").dt.replace_time_zone("UTC").dt.convert_time_zone(IST)
            )
        return frame.with_columns(pl.col("ts").dt.convert_time_zone(IST))
    raise TypeError(
        f"unsupported 'ts' dtype {dtype!r}; expected an integer epoch or a "
        "polars Datetime column"
    )


def _resample_equity(frame: pl.DataFrame, freq: Optional[str]) -> pl.DataFrame:
    if freq is None or freq == "raw":
        return frame
    return (
        frame.with_columns(pl.col("ts").dt.truncate(freq).alias("ts"))
        .group_by("ts")
        .agg(pl.col("equity").last())
        .sort("ts")
    )


def backtest_returns(result_or_equity: Any, freq: str = "1d") -> pl.DataFrame:
    """Derive a periodic simple-returns series from a backtest equity curve.

    Parameters
    ----------
    result_or_equity:
        A ``BacktestResult`` / ``BacktestReport`` (anything exposing an
        ``equity_curve`` polars frame), or the equity-curve
        ``DataFrame``/``LazyFrame`` itself. The frame must contain ``ts``
        (Unix-seconds integer or a polars ``Datetime``) and ``equity``
        columns — the native shape produced by ``BacktestResult.equity_curve``.
    freq:
        A polars duration string (``"1d"``, ``"1h"``, ``"5m"``, ...) used to
        resample the equity curve before differencing (last-observation per
        bucket). Pass ``None`` or ``"raw"`` to skip resampling and use every
        bar as-is. Defaults to ``"1d"`` (daily), a no-op for already-daily
        backtests.

    Returns
    -------
    polars.DataFrame
        Two columns: ``ts`` (tz-aware ``Asia/Kolkata`` datetime, the end of
        each return period) and ``return`` (simple return over the prior
        period). The first bucket is dropped (no prior value to diff
        against), so this has one fewer row than the number of resampled
        buckets.
    """
    equity = _equity_curve_frame(result_or_equity)
    resampled = _resample_equity(equity, freq)
    returns = resampled.with_columns(
        (pl.col("equity") / pl.col("equity").shift(1) - 1.0).alias("return")
    )
    return returns.drop_nulls("return").select(["ts", "return"])


def to_quantstats(result_or_equity: Any, freq: str = "1d") -> "Any":
    """Return backtest returns as a tz-aware ``pandas.Series``.

    Shape matches what ``quantstats.reports.metrics`` / ``quantstats.stats.*``
    expect: a ``pandas.Series`` of simple returns indexed by a
    ``DatetimeIndex`` (tz-aware, ``Asia/Kolkata``), sorted ascending.

    Requires the optional ``pandas`` package; raises ``ImportError`` with an
    install hint if it is not available. See :func:`backtest_returns` for the
    returns-derivation and resampling rules, both applied here first.
    """
    try:
        import pandas as pd  # type: ignore  # lazy import — optional dependency
    except ImportError as exc:  # pragma: no cover - exercised via missing-dep test
        raise ImportError(
            "quantwave.quantstats_interop.to_quantstats requires the optional "
            "'pandas' package, which is not installed. Install it with:\n"
            "    pip install pandas\n"
            "(pandas is intentionally NOT a core quantwave dependency.)"
        ) from exc

    returns = backtest_returns(result_or_equity, freq=freq)
    index = pd.DatetimeIndex(returns["ts"].to_list(), name="ts")
    series = pd.Series(returns["return"].to_list(), index=index, name="returns")
    return series.sort_index()


def quantstats_metrics(result_or_equity: Any, freq: str = "1d", **kwargs: Any) -> "Any":
    """Run ``quantstats.reports.metrics`` on the backtest's derived returns.

    ``**kwargs`` are forwarded verbatim to ``quantstats.reports.metrics``
    (e.g. ``mode="full"``, ``benchmark=...``). Requires the optional
    ``quantstats`` (and transitively ``pandas``) packages; raises
    ``ImportError`` with an install hint if either is unavailable.
    """
    try:
        import quantstats as qs  # type: ignore  # lazy import — optional dependency
    except ImportError as exc:  # pragma: no cover - exercised via missing-dep test
        raise ImportError(
            "quantwave.quantstats_interop.quantstats_metrics requires the "
            "optional 'quantstats' package, which is not installed. Install "
            "it with:\n"
            "    pip install quantstats\n"
            "(quantstats is intentionally NOT a core quantwave dependency.)"
        ) from exc

    returns = to_quantstats(result_or_equity, freq=freq)
    kwargs.setdefault("display", False)
    return qs.reports.metrics(returns, **kwargs)


__all__ = [
    "METRIC_MAPPING",
    "backtest_returns",
    "to_quantstats",
    "quantstats_metrics",
]
