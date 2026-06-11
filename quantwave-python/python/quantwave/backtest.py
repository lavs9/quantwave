"""Vectorized backtest engine (Rust core via PyO3).

Requires ``pip install "quantwave[polars]"`` for Polars DataFrame interop.
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