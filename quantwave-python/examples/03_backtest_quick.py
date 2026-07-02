#!/usr/bin/env python3
"""Minimal backtest — LazyFrame.bt.backtest_with_report (ships in quantwave wheel)."""

import polars as pl
import quantwave  # noqa: F401 — registers LazyFrame.bt

bars = pl.DataFrame({
    "timestamp": list(range(20)),
    "close": [100.0 + i * 0.5 for i in range(20)],
    "signal": [0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0] * 2,
})

report = (
    bars.lazy()
    .bt.backtest_with_report(
        signal="signal",
        close_col="close",
        timestamp_col="timestamp",
        initial_cash=100_000.0,
    )
)
metrics = report.metrics()
print("num_trades:", metrics.get("num_trades"))
print("sharpe_ratio:", metrics.get("sharpe_ratio"))
print("trade rows:", report.result.trades.height)