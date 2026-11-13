import marimo as mo

__generated_with = "0.13.0"
app = mo.App()


@app.cell
def _():
    mo.md(
        """
        # Strategy Backtesting with QuantWave

        SuperTrend signal → exposure → Rust `.bt.backtest_with_report()` with trades,
        equity curve, and performance metrics.

        **Note:** Requires `pip install quantwave` (or `maturin develop` from source).
        When viewed on the documentation website, cells show fallback behavior.
        """
    )
    return


@app.cell
def _():
    import polars as pl
    import numpy as np
    import sys
    from datetime import datetime, timedelta

    RUNNING_IN_BROWSER = sys.platform == "emscripten"

    try:
        import quantwave as qw  # noqa: F401 — registers LazyFrame.bt
        from quantwave.backtest import BacktestEngine
        HAS_QUANTWAVE = True
    except ImportError:
        HAS_QUANTWAVE = False
        qw = None
        BacktestEngine = None

    def generate_deterministic_ohlcv(n: int = 1000):
        """Deterministic OHLCV (no RNG) — reproducible backtest results."""
        t = np.arange(n, dtype=np.float64)
        base = 150.0 + 12.0 * np.sin(t * 0.04) + 0.05 * t
        noise = 2.0 * np.sin(t * 0.17) + 0.8 * np.sin(t * 0.53)
        close = base + noise
        ts0 = datetime(2023, 1, 1)
        timestamps = [ts0 + timedelta(hours=int(i)) for i in range(n)]
        return pl.DataFrame(
            {
                "time": timestamps,
                "open": close - 0.2,
                "high": close + 0.5,
                "low": close - 0.5,
                "close": close,
                "volume": 2000.0 + 500.0 * np.abs(np.sin(t * 0.11)),
            }
        )

    data = generate_deterministic_ohlcv(1000)
    return (
        BacktestEngine,
        HAS_QUANTWAVE,
        RUNNING_IN_BROWSER,
        data,
        datetime,
        generate_deterministic_ohlcv,
        mo,
        np,
        pl,
        qw,
        sys,
        timedelta,
    )


@app.cell
def _(data, pl, HAS_QUANTWAVE, mo, qw, RUNNING_IN_BROWSER):
    if HAS_QUANTWAVE and not RUNNING_IN_BROWSER:
        st = qw.streaming_class("supertrend")(period=10, multiplier=3.0)
        st_vals = []
        dirs = []
        for row in data.iter_rows(named=True):
            r = st.next(row["high"], row["low"], row["close"])
            st_vals.append(r.value)
            dirs.append(float(r.direction))
        df = data.with_columns(
            [
                pl.Series("supertrend", st_vals),
                pl.Series("supertrend_dir", dirs),
            ]
        )
        mo.md("## SuperTrend computed with real QuantWave (streaming Next<T> API)")
        _ = df
    else:
        df = data.with_columns(
            [
                (pl.col("close") * 0.98).alias("supertrend"),
                pl.lit(1).alias("supertrend_dir"),
            ]
        )
        if RUNNING_IN_BROWSER:
            mo.md(
                """
                ## Running in Browser (Documentation Site)

                The `quantwave` package (Rust extension) cannot run here.

                **Best experience:** Clone the repo and run locally:
                ```bash
                pip install quantwave
                marimo edit docs/examples/notebooks/strategy_backtest.py
                ```
                """
            )
        else:
            mo.md(
                """
                ## Fallback Mode

                `quantwave` package not found. Install with `pip install quantwave`
                to see real SuperTrend + backtest output.
                """
            )
        _ = df  # satisfy marimo branch-expression rule

    return (df,)


@app.cell
def _(df, mo, pl, HAS_QUANTWAVE, RUNNING_IN_BROWSER):
    mo.md("### Exposure from SuperTrend direction (long-only: dir > 0 → 1.0, else 0.0)")

    if HAS_QUANTWAVE and not RUNNING_IN_BROWSER:
        signal_df = df.with_columns(
            pl.when(pl.col("supertrend_dir") > 0)
            .then(1.0)
            .otherwise(0.0)
            .alias("exposure")
        ).select(["time", "close", "exposure", "supertrend", "supertrend_dir"])
        mo.ui.table(signal_df.head(8))
        _ = signal_df
    else:
        signal_df = None
        mo.md("_Exposure column requires quantwave (skipped in fallback)._")
        _ = signal_df

    return (signal_df,)


@app.cell
def _(signal_df, mo, HAS_QUANTWAVE, RUNNING_IN_BROWSER):
    if HAS_QUANTWAVE and not RUNNING_IN_BROWSER and signal_df is not None:
        report = signal_df.lazy().bt.backtest_with_report(
            signal="exposure",
            timestamp_col="time",
            commission_bps=0.0,
            slippage_bps=0.0,
        )
        metrics = report.metrics()
        stats = report.result.stats()
        mo.md(
            f"""
            ## Backtest Results (Rust engine via `.bt`)

            | Metric | Value |
            |--------|-------|
            | Trades | {int(metrics['num_trades'])} |
            | Final equity | {metrics['final_equity']:,.2f} |
            | Total return | {metrics['total_return'] * 100:.2f}% |
            | Sharpe | {metrics['sharpe_ratio']:.3f} |
            | Max drawdown | {metrics['max_drawdown_pct'] * 100:.2f}% |
            | Win rate | {metrics['win_rate'] * 100:.1f}% |
            | Profit factor | {metrics['profit_factor']:.2f} |

            Net PnL: **{stats.get('net_pnl', 0.0):,.2f}**
            """
        )
        mo.md("### Closed trades (first 10)")
        mo.ui.table(report.result.trades.head(10))
        mo.md("### Equity curve (last 10 bars)")
        mo.ui.table(report.result.equity_curve.tail(10))
        _ = report
    else:
        report = None
        mo.md("_Backtest skipped in fallback / browser mode._")
        _ = report

    return (report,)


if __name__ == "__main__":
    app.run()