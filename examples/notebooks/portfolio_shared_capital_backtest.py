import marimo

__generated_with = "0.1.75"
app = marimo.App()


@app.cell
def __():
    import marimo as mo
    import polars as pl
    import quantwave
    return mo, pl, quantwave


@app.cell
def __(mo):
    mo.md(
        """
        # Shared-Capital Portfolio Backtesting

        QuantWave v0.7 introduces the `shared_capital` portfolio mode. Unlike `independent_books` (which allocates a fixed initial cash amount to each symbol individually), `shared_capital` maintains a single global cash pool. When one symbol generates a profit, that capital becomes available for other symbols to use.
        """
    )
    return


@app.cell
def __(pl, quantwave):
    # 1. Generate multi-symbol synthetic data
    # In a real scenario, this would be a panel of hundreds of symbols.
    df = pl.DataFrame(
        {
            "timestamp": [1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4],
            "symbol": ["AAPL", "MSFT", "GOOG", "AAPL", "MSFT", "GOOG", "AAPL", "MSFT", "GOOG", "AAPL", "MSFT", "GOOG"],
            "close": [150.0, 300.0, 100.0, 155.0, 295.0, 102.0, 152.0, 310.0, 105.0, 160.0, 305.0, 104.0],
            "signal": [1.0, 0.0, -1.0, 1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        }
    ).lazy()

    # 2. Run shared-capital backtest
    # This automatically tracks one global cash pool (initial_cash=100,000)
    report = df.bt.portfolio_backtest(
        symbol_col="symbol",
        commission_bps=1.0,
        slippage_bps=1.0,
        portfolio_mode="shared_capital",
        initial_cash=100_000.0,
    )
    
    # 3. View global metrics
    metrics = report.metrics()
    return df, report, metrics


@app.cell
def __(metrics, mo):
    mo.ui.table([
        {"Metric": k, "Value": f"{v:.4f}" if isinstance(v, float) else v}
        for k, v in metrics.items()
    ])
    return


if __name__ == "__main__":
    app.run()
