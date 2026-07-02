"""Vectorized backtest engine exposed to Python.

The simulation core is implemented in Rust (``quantwave._backtest``) and
wrapped for Polars ``DataFrame`` / ``LazyFrame`` interop. For ergonomic
research pipelines, prefer the Polars namespace::

    import quantwave  # registers LazyFrame.bt
    report = df.lazy().bt.backtest_with_report(...)

This module re-exports the native types for direct use when you already
hold a collected ``DataFrame`` or need custom ``BacktestConfig`` wiring.

Requires ``pip install "quantwave[polars]"`` for Polars integration.
"""

from quantwave._backtest import (
    BacktestConfig,
    BacktestEngine,
    BacktestReport,
    BacktestResult,
)

__all__ = [
    "BacktestConfig",
    "BacktestEngine",
    "BacktestResult",
    "BacktestReport",
]