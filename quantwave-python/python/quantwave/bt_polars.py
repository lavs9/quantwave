"""Polars LazyFrame `.bt` namespace (quantwave-cr6v.5).

Registers `df.lazy().bt.backtest(...)` on import. Delegates to the native
`quantwave._backtest` engine (same semantics as `quantwave-polars` Rust `.bt()`).
"""

from __future__ import annotations

import polars as pl

from quantwave._backtest import BacktestConfig, BacktestEngine


def _config_from_kwargs(
    *,
    signal: str = "signal",
    timestamp_col: str = "timestamp",
    close_col: str = "close",
    symbol_col: str | None = None,
    entry_filter_col: str | None = None,
    size_multiplier_col: str | None = None,
    initial_cash: float = 100_000.0,
    commission_bps: float = 5.0,
    slippage_bps: float = 2.0,
    execution_delay: str = "same_bar",
    stop_loss_pct: float | None = None,
    take_profit_pct: float | None = None,
    trailing_stop_pct: float | None = None,
) -> BacktestConfig:
    return BacktestConfig(
        signal_col=signal,
        timestamp_col=timestamp_col,
        close_col=close_col,
        symbol_col=symbol_col,
        entry_filter_col=entry_filter_col,
        size_multiplier_col=size_multiplier_col,
        initial_cash=initial_cash,
        commission_bps=commission_bps,
        slippage_bps=slippage_bps,
        execution_delay=execution_delay,
        stop_loss_pct=stop_loss_pct,
        take_profit_pct=take_profit_pct,
        trailing_stop_pct=trailing_stop_pct,
    )


@pl.api.register_lazyframe_namespace("bt")
class BtLazyNamespace:
    """`df.lazy().bt.backtest(signal=...)` namespace."""

    def __init__(self, ldf: pl.LazyFrame) -> None:
        self._ldf = ldf

    def backtest(
        self,
        signal: str = "signal",
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        symbol_col: str | None = None,
        entry_filter_col: str | None = None,
        size_multiplier_col: str | None = None,
        initial_cash: float = 100_000.0,
        commission_bps: float = 5.0,
        slippage_bps: float = 2.0,
        execution_delay: str = "same_bar",
        stop_loss_pct: float | None = None,
        take_profit_pct: float | None = None,
        trailing_stop_pct: float | None = None,
    ):
        config = _config_from_kwargs(
            signal=signal,
            timestamp_col=timestamp_col,
            close_col=close_col,
            symbol_col=symbol_col,
            entry_filter_col=entry_filter_col,
            size_multiplier_col=size_multiplier_col,
            initial_cash=initial_cash,
            commission_bps=commission_bps,
            slippage_bps=slippage_bps,
            execution_delay=execution_delay,
            stop_loss_pct=stop_loss_pct,
            take_profit_pct=take_profit_pct,
            trailing_stop_pct=trailing_stop_pct,
        )
        return BacktestEngine(config).run(self._ldf.collect())

    def backtest_with_report(
        self,
        signal: str = "signal",
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        symbol_col: str | None = None,
        entry_filter_col: str | None = None,
        size_multiplier_col: str | None = None,
        initial_cash: float = 100_000.0,
        commission_bps: float = 5.0,
        slippage_bps: float = 2.0,
        execution_delay: str = "same_bar",
        stop_loss_pct: float | None = None,
        take_profit_pct: float | None = None,
        trailing_stop_pct: float | None = None,
    ):
        config = _config_from_kwargs(
            signal=signal,
            timestamp_col=timestamp_col,
            close_col=close_col,
            symbol_col=symbol_col,
            entry_filter_col=entry_filter_col,
            size_multiplier_col=size_multiplier_col,
            initial_cash=initial_cash,
            commission_bps=commission_bps,
            slippage_bps=slippage_bps,
            execution_delay=execution_delay,
            stop_loss_pct=stop_loss_pct,
            take_profit_pct=take_profit_pct,
            trailing_stop_pct=trailing_stop_pct,
        )
        return BacktestEngine(config).backtest_with_report(self._ldf.collect())

    def sweep(
        self,
        *,
        param_name: str,
        param_values: list[float],
        signal_cols: list[str],
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        symbol_col: str | None = None,
        entry_filter_col: str | None = None,
        size_multiplier_col: str | None = None,
        initial_cash: float = 100_000.0,
        commission_bps: float = 5.0,
        slippage_bps: float = 2.0,
        execution_delay: str = "same_bar",
        stop_loss_pct: float | None = None,
        take_profit_pct: float | None = None,
        trailing_stop_pct: float | None = None,
    ) -> pl.DataFrame:
        """Run one backtest per param value; return param × metrics DataFrame."""
        if len(param_values) != len(signal_cols):
            raise ValueError(
                f"param_values len {len(param_values)} != signal_cols len {len(signal_cols)}"
            )
        if not param_values:
            raise ValueError("sweep requires at least one variant")

        base_kwargs = dict(
            timestamp_col=timestamp_col,
            close_col=close_col,
            symbol_col=symbol_col,
            entry_filter_col=entry_filter_col,
            size_multiplier_col=size_multiplier_col,
            initial_cash=initial_cash,
            commission_bps=commission_bps,
            slippage_bps=slippage_bps,
            execution_delay=execution_delay,
            stop_loss_pct=stop_loss_pct,
            take_profit_pct=take_profit_pct,
            trailing_stop_pct=trailing_stop_pct,
        )

        rows: list[dict[str, float]] = []
        for value, signal_col in zip(param_values, signal_cols, strict=True):
            report = self.backtest_with_report(signal=signal_col, **base_kwargs)
            row = {param_name: value, **report.metrics()}
            rows.append(row)

        return pl.DataFrame(rows)