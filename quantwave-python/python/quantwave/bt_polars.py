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

    def backtest_metrics(
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
    ) -> dict[str, float]:
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
        return BacktestEngine(config).run_metrics_only(self._ldf.collect())

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

    def sweep_callback(
        self,
        *,
        param_grid: dict[str, list[float]],
        build_fn,
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
    ) -> pl.DataFrame:
        import itertools
        if not param_grid:
            raise ValueError("param_grid cannot be empty")
        if not callable(build_fn):
            raise ValueError("build_fn must be callable")
            
        base_kwargs = dict(
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
        keys = list(param_grid.keys())
        values_lists = [param_grid[k] for k in keys]
        rows = []
        for combo in itertools.product(*values_lists):
            params = dict(zip(keys, combo))
            built_lf = build_fn(self._ldf, params)
            if signal not in built_lf.collect_schema().names():
                raise ValueError(f"build_fn must return a frame with signal column '{signal}'")
            config = _config_from_kwargs(**base_kwargs)
            report = BacktestEngine(config).backtest_with_report(built_lf.collect())
            row = {**params, **report.metrics()}
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

    def walk_forward_optimize(
        self,
        *,
        param_grid: dict[str, list[float]],
        build_fn,
        objective: str = "sharpe_ratio",
        train_bars: int,
        test_bars: int,
        step_bars: int | None = None,
        overfit_threshold: float = 1.0,
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        signal: str = "signal",
        symbol_col: str | None = None,
        initial_cash: float = 100_000.0,
        commission_bps: float = 5.0,
        slippage_bps: float = 2.0,
        execution_delay: str = "same_bar",
    ) -> pl.DataFrame:
        """Walk-forward with train-window parameter optimization."""
        import itertools
        if not param_grid:
            raise ValueError("param_grid cannot be empty")
        if not callable(build_fn):
            raise ValueError("build_fn must be callable")
            
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
        
        keys = list(param_grid.keys())
        values_lists = [param_grid[k] for k in keys]
        
        rows: list[dict[str, float | bool]] = []
        fold = 0
        start = 0
        while start + train_bars + test_bars <= len(timestamps):
            train_ts = timestamps[start : start + train_bars]
            test_ts = timestamps[start + train_bars : start + train_bars + test_bars]
            
            train_df = df.filter(pl.col(timestamp_col).is_in(train_ts))
            
            # Sweep on train window
            best_val = -float("inf")
            best_params = None
            best_signal_col = None
            
            for combo in itertools.product(*values_lists):
                params = dict(zip(keys, combo))
                built_lf = build_fn(train_df.lazy(), params)
                config = _config_from_kwargs(**base_kwargs)
                report = BacktestEngine(config).backtest_with_report(built_lf.collect())
                
                val = report.metrics().get(objective, -float("inf"))
                if val > best_val or (best_val == -float("inf") and val != -float("inf")):
                    best_val = val
                    best_params = params
                    
            if best_params is None:
                raise ValueError("No valid param found during train sweep")
                
            # OOS evaluation using best params
            oos_df = df.filter(pl.col(timestamp_col).is_in(test_ts))
            built_oos = build_fn(oos_df.lazy(), best_params)
            config = _config_from_kwargs(**base_kwargs)
            report = BacktestEngine(config).backtest_with_report(built_oos.collect())
            
            oos_val = report.metrics().get(objective, 0.0)
            overfit = (best_val - oos_val) > overfit_threshold
            
            row = {
                "fold_id": float(fold),
                "oos_start_ts": float(test_ts[0]),
                "oos_end_ts": float(test_ts[-1]),
                "train_metric": float(best_val),
                "oos_metric": float(oos_val),
                "overfit_flag": bool(overfit),
                **{f"best_{k}": float(v) for k, v in best_params.items()},
                **report.metrics(),
            }
            rows.append(row)
            fold += 1
            start += step

        if not rows:
            raise ValueError("insufficient bars for wfo")
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
        transform: str | None = None,
    ):
        """sigc-inspired rank → long/short panel backtest (cr6v.15)."""
        if top_frac <= 0 or bottom_frac <= 0 or top_frac + bottom_frac > 1.0:
            raise ValueError("invalid top_frac/bottom_frac")

        df = self._ldf.collect()
        
        if transform == "zscore":
            mean = pl.col(factor_col).mean().over(timestamp_col)
            std = pl.col(factor_col).std(ddof=1).over(timestamp_col)
            df = df.with_columns(((pl.col(factor_col) - mean) / std).alias(factor_col))
        elif transform == "neutralize":
            mean = pl.col(factor_col).mean().over(timestamp_col)
            df = df.with_columns((pl.col(factor_col) - mean).alias(factor_col))
            
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