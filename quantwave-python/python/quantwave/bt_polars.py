"""Polars ``.bt`` namespace for the QuantWave backtest engine.

Importing ``quantwave`` (when Polars and ``quantwave._backtest`` are available)
registers ``LazyFrame.bt`` so you can run simulations without leaving Polars::

    import quantwave
    report = signal_df.lazy().bt.backtest_with_report(
        signal="signal", close_col="close", timestamp_col="bar"
    )

All methods delegate to :class:`quantwave.backtest.BacktestEngine` with a
:class:`quantwave._backtest.BacktestConfig` built from keyword arguments.
The Rust core matches ``quantwave-backtest`` used in native Polars Rust pipelines.
"""

from __future__ import annotations

import polars as pl

from quantwave._backtest import BacktestConfig
from quantwave.backtest import BacktestEngine


def _config_from_kwargs(
    *,
    signal: str = "signal",
    timestamp_col: str = "timestamp",
    close_col: str = "close",
    high_col: str | None = None,
    low_col: str | None = None,
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
    touched_exit: bool = False,
    portfolio_mode: str = "independent_books",
    portfolio_allocator: str = "equal_weight",
) -> BacktestConfig:
    return BacktestConfig(
        signal_col=signal,
        timestamp_col=timestamp_col,
        close_col=close_col,
        high_col=high_col,
        low_col=low_col,
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
        touched_exit=touched_exit,
        portfolio_mode=portfolio_mode,
        portfolio_allocator=portfolio_allocator,
    )


@pl.api.register_lazyframe_namespace("bt")
class BtLazyNamespace:
    """Backtest methods on ``LazyFrame`` via ``df.lazy().bt.*``.

    Typical columns: ``timestamp``, ``close``, ``signal`` (long/short/flat encoding).
    See https://lavs9.github.io/quantwave/guides/backtest/quickstart/ for signal
    conventions and portfolio modes.
    See https://lavs9.github.io/quantwave/guides/backtest/output-contract/ for output schema.

    Primary entry points:

    * :meth:`backtest_with_report` — metrics + trades + equity (recommended)
    * :meth:`backtest` — raw :class:`~quantwave._backtest.BacktestResult`
    * :meth:`portfolio_backtest` — shared-capital multi-symbol book
    * :meth:`sweep` / :meth:`walk_forward_optimize` — research workflows
    """

    def __init__(self, ldf: pl.LazyFrame) -> None:
        """Attach the namespace to a lazy frame (called by Polars)."""
        self._ldf = ldf

    def backtest(
        self,
        signal: str = "signal",
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        high_col: str | None = None,
        low_col: str | None = None,
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
        touched_exit: bool = False,
    ):
        config = _config_from_kwargs(
            signal=signal,
            timestamp_col=timestamp_col,
            close_col=close_col,
            high_col=high_col,
            low_col=low_col,
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
            touched_exit=touched_exit,
        )
        return BacktestEngine(config).run(self._ldf.collect())

    def backtest_with_report(
        self,
        signal: str = "signal",
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        high_col: str | None = None,
        low_col: str | None = None,
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
        touched_exit: bool = False,
    ):
        """Run a backtest and return a full report (metrics, trades, equity).

        This is the recommended high-level API for strategy evaluation.

        Args:
            signal: Column name with position signal (-1, 0, 1 or strategy-specific).
            timestamp_col: Monotonic bar index or datetime column.
            close_col: Price column for mark-to-market and fills.
            symbol_col: Optional symbol column for per-symbol books.
            entry_filter_col: Optional boolean column to gate entries.
            size_multiplier_col: Optional sizing multiplier per bar.
            initial_cash: Starting capital.
            commission_bps: Commission in basis points per trade leg.
            slippage_bps: Slippage in basis points.
            execution_delay: ``"same_bar"`` or ``"next_bar"`` fill timing.
            stop_loss_pct: Optional stop-loss fraction (e.g. ``0.02`` for 2%).
            take_profit_pct: Optional take-profit fraction.
            trailing_stop_pct: Optional trailing stop fraction.
            high_col: Bar high column (required when ``touched_exit=True``).
            low_col: Bar low column (required when ``touched_exit=True``).
            touched_exit: Use OHLC intrabar stop/target detection (polars-backtest style).

        Returns:
            BacktestReport with ``metrics``, ``trades``, and equity series accessors.

        Example:
            >>> import polars as pl
            >>> import quantwave  # registers .bt
            >>> df = pl.DataFrame({
            ...     "timestamp": [0, 1, 2],
            ...     "close": [100.0, 101.0, 102.0],
            ...     "signal": [0, 1, 0],
            ... })
            >>> report = df.lazy().bt.backtest_with_report(
            ...     signal="signal", close_col="close", timestamp_col="timestamp"
            ... )
        """
        config = _config_from_kwargs(
            signal=signal,
            timestamp_col=timestamp_col,
            close_col=close_col,
            high_col=high_col,
            low_col=low_col,
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
            touched_exit=touched_exit,
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

    def portfolio_backtest(
        self,
        *,
        signal: str = "signal",
        timestamp_col: str = "timestamp",
        close_col: str = "close",
        symbol_col: str = "symbol",
        entry_filter_col: str | None = None,
        size_multiplier_col: str | None = None,
        initial_cash: float = 100_000.0,
        commission_bps: float = 5.0,
        slippage_bps: float = 2.0,
        execution_delay: str = "same_bar",
        stop_loss_pct: float | None = None,
        take_profit_pct: float | None = None,
        trailing_stop_pct: float | None = None,
        portfolio_mode: str = "shared_capital",
        portfolio_allocator: str = "equal_weight",
    ):
        """Shared-capital multi-symbol backtest (quantwave-qzpi.9)."""
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
            portfolio_mode=portfolio_mode,
            portfolio_allocator=portfolio_allocator,
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
        """Rolling OOS walk-forward → fold × metrics DataFrame (delegates to Rust)."""
        if train_bars <= 0 or test_bars <= 0:
            raise ValueError("train_bars and test_bars must be > 0")

        from quantwave._backtest import run_walk_forward_py

        config = _config_from_kwargs(
            signal=signal,
            timestamp_col=timestamp_col,
            close_col=close_col,
            symbol_col=symbol_col,
            initial_cash=initial_cash,
            commission_bps=commission_bps,
            slippage_bps=slippage_bps,
            execution_delay=execution_delay,
        )
        return run_walk_forward_py(
            self._ldf.collect(),
            config,
            train_bars,
            test_bars,
            step_bars,
        )

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
        """Walk-forward with train-window parameter optimization (Rust core)."""
        import itertools

        if not param_grid:
            raise ValueError("param_grid cannot be empty")
        if not callable(build_fn):
            raise ValueError("build_fn must be callable")
        if train_bars <= 0 or test_bars <= 0:
            raise ValueError("train_bars and test_bars must be > 0")

        from quantwave._backtest import run_walk_forward_optimize_py

        df = self._ldf.collect()
        keys = list(param_grid.keys())
        values_lists = [param_grid[k] for k in keys]

        enriched = df.clone()
        variants: list[tuple[dict[str, float], str]] = []
        for combo in itertools.product(*values_lists):
            params = dict(zip(keys, combo, strict=True))
            sig_col = "__qw_wfo_" + "_".join(f"{k}{v:g}" for k, v in params.items())
            built = build_fn(self._ldf, params).collect()
            if signal not in built.columns:
                raise ValueError(f"build_fn must return a frame with signal column '{signal}'")
            if built.height != enriched.height:
                raise ValueError("build_fn must preserve row count of input frame")
            enriched = enriched.with_columns(built[signal].alias(sig_col))
            variants.append((params, sig_col))

        config = _config_from_kwargs(
            signal=variants[0][1],
            timestamp_col=timestamp_col,
            close_col=close_col,
            symbol_col=symbol_col,
            initial_cash=initial_cash,
            commission_bps=commission_bps,
            slippage_bps=slippage_bps,
            execution_delay=execution_delay,
        )
        return run_walk_forward_optimize_py(
            enriched,
            config,
            train_bars,
            test_bars,
            variants,
            objective,
            step_bars,
            overfit_threshold,
        )

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
        elif transform == "winsorize":
            lower = pl.col(factor_col).quantile(0.05).over(timestamp_col)
            upper = pl.col(factor_col).quantile(0.95).over(timestamp_col)
            df = df.with_columns(
                pl.col(factor_col).clip(lower_bound=lower, upper_bound=upper).alias(factor_col)
            )
            
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

    def monte_carlo(
        self,
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
        n_simulations: int = 1000,
        seed: int = 42,
        mode: str = "trade_bootstrap",
        n_bars_forward: int = 252,
    ) -> dict:
        """Run backtest then Monte Carlo robustness (quantwave-fsg3).

        Args:
            mode: ``trade_bootstrap`` (resample closed-trade PnLs) or
                ``return_paths`` (return-path VaR/CVaR).
        """
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
        from quantwave._backtest import (
            monte_carlo_return_paths_py,
            monte_carlo_trade_bootstrap_py,
        )

        result = BacktestEngine(config).run(self._ldf.collect())
        if mode == "return_paths":
            return monte_carlo_return_paths_py(
                result._inner,
                n_simulations=n_simulations,
                seed=seed,
                n_bars_forward=n_bars_forward,
            )
        if mode != "trade_bootstrap":
            raise ValueError("mode must be 'trade_bootstrap' or 'return_paths'")
        return monte_carlo_trade_bootstrap_py(
            result._inner,
            initial_cash=initial_cash,
            n_simulations=n_simulations,
            seed=seed,
        )