from __future__ import annotations

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
    BacktestEngine as _BacktestEngine,
    BacktestReport as _BacktestReport,
    BacktestResult as _BacktestResult,
)
from quantwave.backtest_types import PerformanceMetrics, BacktestStats

class BacktestResult:
    """Raw backtest result with trades and stats."""
    def __init__(self, inner: _BacktestResult):
        self._inner = inner
        
    @property
    def trades(self):
        """Polars DataFrame of executed trades."""
        return self._inner.trades
        
    @property
    def equity_curve(self):
        """Polars DataFrame of the portfolio equity curve over time."""
        return self._inner.equity_curve
        
    def stats(self) -> BacktestStats:
        """Core summary statistics from the backtest."""
        return BacktestStats(**self._inner.stats())
        
    def metrics(self) -> PerformanceMetrics:
        """Detailed performance metrics."""
        return PerformanceMetrics(**self._inner.metrics())

class BacktestReport:
    """Full backtest report with detailed metrics."""
    def __init__(self, inner: _BacktestReport):
        self._inner = inner
        
    @property
    def trades(self):
        """Polars DataFrame of executed trades."""
        return self._inner.trades
        
    @property
    def equity_curve(self):
        """Polars DataFrame of the portfolio equity curve over time."""
        return self._inner.equity_curve
        
    @property
    def result(self) -> BacktestResult:
        """Raw backtest result object."""
        return BacktestResult(self._inner.result)
        
    def metrics(self) -> PerformanceMetrics:
        """Detailed performance metrics."""
        return PerformanceMetrics(**self._inner.metrics())
        
    def stats(self) -> BacktestStats:
        """Core summary statistics from the backtest."""
        return BacktestStats(**self._inner.stats())
        
    def to_html(self, title: str | None = None) -> str:
        return self._inner.to_html(title=title)
        
    def save_html(self, path: str, title: str | None = None):
        return self._inner.save_html(path, title=title)

class BacktestEngine:
    """Vectorized backtest engine."""
    def __init__(self, config: BacktestConfig):
        self._inner = _BacktestEngine(config)
        
    @classmethod
    def with_default_costs(cls) -> "BacktestEngine":
        config = BacktestConfig(
            signal_col="signal",
            close_col="close",
            commission_bps=5.0,
            slippage_bps=2.0,
        )
        return cls(config)
        
    def run(self, df) -> BacktestResult:
        return BacktestResult(self._inner.run(df))
        
    def backtest_with_report(self, df) -> BacktestReport:
        return BacktestReport(self._inner.backtest_with_report(df))
        
    def run_metrics_only(self, df) -> dict[str, float]:
        return self._inner.run_metrics_only(df)

__all__ = [
    "BacktestConfig",
    "BacktestEngine",
    "BacktestResult",
    "BacktestReport",
]