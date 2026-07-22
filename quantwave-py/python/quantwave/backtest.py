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

    def extended_metrics(self) -> dict:
        """Additive metrics beyond the stable 10-key contract: adds
        ``calmar_ratio``, ``var_95``, ``cvar_95``, and a ``benchmark`` key
        (``None`` unless a benchmark series was supplied via
        :meth:`metrics_with_benchmark`).
        """
        return self._inner.extended_metrics()

    def metrics_with_benchmark(self, benchmark_returns: list[float]) -> dict:
        """Extended metrics with benchmark-relative analytics (alpha, beta,
        cumulative return vs. benchmark) computed against ``benchmark_returns``
        (per-bar simple returns, aligned by index to the strategy's per-bar
        returns).
        """
        return self._inner.metrics_with_benchmark(benchmark_returns)

    def stats(self) -> BacktestStats:
        """Core summary statistics from the backtest."""
        return BacktestStats(**self._inner.stats())

    def to_html(
        self,
        title: str | None = None,
        seed: int | None = None,
        run_metadata: list[tuple[str, str]] | dict | None = None,
        benchmark_returns: list[float] | None = None,
        rolling_window: int | None = None,
    ) -> str:
        """Self-contained HTML tear sheet.

        Beyond equity/drawdown/metrics/trades, the report packet includes a
        monthly-returns heatmap, rolling Sharpe/volatility charts, a full trade
        blotter, and a reproducible run-metadata section (title, generated-at
        timestamp, optional ``seed``, and any ``run_metadata`` key/value pairs
        e.g. serialized config). Passing ``benchmark_returns`` adds a
        Benchmark-Relative section (alpha/beta/cumulative return).
        """
        if isinstance(run_metadata, dict):
            run_metadata = list(run_metadata.items())
        return self._inner.to_html(
            title=title,
            seed=seed,
            run_metadata=run_metadata,
            benchmark_returns=benchmark_returns,
            rolling_window=rolling_window,
        )

    def save_html(
        self,
        path: str,
        title: str | None = None,
        seed: int | None = None,
        run_metadata: list[tuple[str, str]] | dict | None = None,
        benchmark_returns: list[float] | None = None,
        rolling_window: int | None = None,
    ):
        if isinstance(run_metadata, dict):
            run_metadata = list(run_metadata.items())
        return self._inner.save_html(
            path,
            title=title,
            seed=seed,
            run_metadata=run_metadata,
            benchmark_returns=benchmark_returns,
            rolling_window=rolling_window,
        )

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