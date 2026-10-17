"""Robustness tearsheet example.

Plain runnable script (not a marimo notebook, unlike its neighbors in this
directory) — its whole point is to produce an actual HTML file on disk so
the robustness tearsheet output can be sanity-checked visually, which is
easiest as a direct `python3` run rather than a marimo cell graph.

Pipeline: deterministic synthetic OHLCV (same generator style as
strategy_backtest.py) -> SuperTrend streaming signal -> exposure column ->
`.bt.backtest_with_report()` -> `quantwave.tearsheet.save_robustness_html()`,
which appends the KPI/perf-matrix/robustness-audit/drawdown/heatmap sections
computed by `quantwave.robustness` onto the native Rust `to_html()` output
(equity curve, drawdown plot, rolling Sharpe, trade blotter).

Usage:
    python3 docs/examples/notebooks/robustness_tearsheet_example.py [output.html]
"""

from __future__ import annotations

import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path

import numpy as np
import polars as pl

import quantwave as qw  # noqa: F401 — registers LazyFrame.bt
from quantwave import tearsheet


def generate_deterministic_ohlcv(n: int = 1500) -> pl.DataFrame:
    """Deterministic OHLCV (no RNG) — same shape as strategy_backtest.py's
    generator, extended with a slow secular trend + regime wobble so an
    IS/OOS split has visibly different character."""
    t = np.arange(n, dtype=np.float64)
    base = 150.0 + 12.0 * np.sin(t * 0.015) + 0.04 * t
    noise = 2.0 * np.sin(t * 0.17) + 0.8 * np.sin(t * 0.53)
    close = base + noise
    ts0 = datetime(2022, 1, 1, tzinfo=timezone.utc)
    timestamps = [int((ts0 + timedelta(days=int(i))).timestamp()) for i in range(n)]
    return pl.DataFrame(
        {
            "timestamp": timestamps,
            "open": close - 0.2,
            "high": close + 0.5,
            "low": close - 0.5,
            "close": close,
            "volume": 2000.0 + 500.0 * np.abs(np.sin(t * 0.11)),
        }
    )


def build_signal(data: pl.DataFrame) -> pl.DataFrame:
    """SuperTrend direction -> long-only exposure (streaming Next<T> API,
    same pattern as strategy_backtest.py)."""
    st = qw.streaming_class("supertrend")(period=10, multiplier=3.0)
    dirs = []
    for row in data.iter_rows(named=True):
        r = st.next(row["high"], row["low"], row["close"])
        dirs.append(float(r.direction))
    return data.with_columns(
        pl.Series("supertrend_dir", dirs),
        pl.when(pl.Series("supertrend_dir", dirs) > 0).then(1.0).otherwise(0.0).alias("signal"),
    )


def main() -> Path:
    out_path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("/tmp/quantwave_robustness_tearsheet_sample.html")

    data = generate_deterministic_ohlcv(1500)
    signal_df = build_signal(data)

    report = signal_df.lazy().bt.backtest_with_report(
        signal="signal",
        timestamp_col="timestamp",
        commission_bps=5.0,
        slippage_bps=2.0,
    )

    print(f"Trades: {report.metrics()['num_trades']}")
    print(f"Sharpe: {report.metrics()['sharpe_ratio']:.3f}")
    print(f"Max drawdown: {report.metrics()['max_drawdown_pct'] * 100:.2f}%")

    # IS/OOS split at the 70% mark of the synthetic series.
    split_date = datetime(2022, 1, 1, tzinfo=timezone.utc) + timedelta(days=int(1500 * 0.7))

    path = tearsheet.save_robustness_html(
        report,
        out_path,
        title="QuantWave Robustness Tearsheet — SuperTrend Demo",
        split_date=split_date,
        trials=5,  # e.g. "we tried 5 SuperTrend (period, multiplier) variants"
        mc_simulations=2000,
    )
    print(f"Wrote robustness tearsheet: {path.resolve()}")
    return path


if __name__ == "__main__":
    main()
