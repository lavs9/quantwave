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

    def walk_forward(
        self,
        *,
        train_bars: int,
        test_bars: int,
        step_bars: int | None = None,
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        signal: str = "signal",
        symbol_col: str | None = None,
        initial_cash: float = 100_000.0,
        commission_bps: float = 5.0,
        slippage_bps: float = 2.0,
        execution_delay: str = "same_bar",
    ) -> pl.DataFrame:
        """Rolling OOS walk-forward → fold × metrics DataFrame (cr6v.14)."""
        if train_bars <= 0 or test_bars <= 0:
            raise ValueError("train_bars and test_bars must be > 0")

        df = self._ldf.collect()
        timestamps = df[timestamp_col].unique().sort().to_list()
        step = step_bars or test_bars
        base_kwargs = dict(
            signal=signal,
            timestamp_col=timestamp_col,
            close_col=close_col,
            symbol_col=symbol_col,
            initial_cash=initial_cash,
            commission_bps=commission_bps,
            slippage_bps=slippage_bps,
            execution_delay=execution_delay,
        )

        rows: list[dict[str, float]] = []
        fold = 0
        start = 0
        while start + train_bars + test_bars <= len(timestamps):
            test_ts = timestamps[start + train_bars : start + train_bars + test_bars]
            oos = df.filter(pl.col(timestamp_col).is_in(test_ts))
            report = BacktestEngine(_config_from_kwargs(**base_kwargs)).backtest_with_report(
                oos
            )
            rows.append(
                {
                    "fold_id": float(fold),
                    "oos_start_ts": float(test_ts[0]),
                    "oos_end_ts": float(test_ts[-1]),
                    "train_bars": float(train_bars),
                    "test_bars": float(test_bars),
                    **report.metrics(),
                }
            )
            fold += 1
            start += step

        if not rows:
            raise ValueError(
                f"insufficient bars for walk-forward: need >= {train_bars + test_bars} "
                f"unique timestamps, got {len(timestamps)}"
            )
        return pl.DataFrame(rows)

    def cross_sectional_backtest(
        self,
        *,
        factor_col: str,
        top_frac: float = 0.2,
        bottom_frac: float = 0.2,
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        symbol_col: str = "symbol",
        initial_cash: float = 100_000.0,
        commission_bps: float = 5.0,
        slippage_bps: float = 2.0,
        execution_delay: str = "same_bar",
    ):
        """sigc-inspired rank → long/short panel backtest (cr6v.15)."""
        if top_frac <= 0 or bottom_frac <= 0 or top_frac + bottom_frac > 1.0:
            raise ValueError("invalid top_frac/bottom_frac")

        df = self._ldf.collect()
        ranked = (
            df.with_columns(
                pl.col(factor_col)
                .rank(method="min", descending=True)
                .over(timestamp_col)
                .alias("_rank"),
                pl.col(timestamp_col).count().over(timestamp_col).alias("_n"),
            )
            .with_columns(
                pl.when(
                    pl.col("_rank")
                    <= (pl.col("_n") * top_frac).ceil().clip(lower_bound=1)
                )
                .then(1.0 / (pl.col("_n") * top_frac).ceil().clip(lower_bound=1))
                .when(
                    pl.col("_rank")
                    >= pl.col("_n")
                    - (pl.col("_n") * bottom_frac).ceil().clip(lower_bound=1)
                    + 1
                )
                .then(
                    -1.0 / (pl.col("_n") * bottom_frac).ceil().clip(lower_bound=1)
                )
                .otherwise(0.0)
                .alias("cs_exposure")
            )
        )
        config = _config_from_kwargs(
            signal="cs_exposure",
            timestamp_col=timestamp_col,
            close_col=close_col,
            symbol_col=symbol_col,
            initial_cash=initial_cash,
            commission_bps=commission_bps,
            slippage_bps=slippage_bps,
            execution_delay=execution_delay,
        )
        return BacktestEngine(config).backtest_with_report(ranked)