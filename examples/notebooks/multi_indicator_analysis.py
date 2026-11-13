import marimo as mo

__generated_with = "0.13.0"
app = mo.App()


@app.cell
def _():
    mo.md(
        r"""
        # Multi-Indicator Analysis with QuantWave
        
        In this notebook, we'll demonstrate how to combine multiple QuantWave indicators (classic and Ehlers DSP) using Polars' high-performance expressions.
        
        **Note:** This notebook requires `pip install quantwave`. When viewed on the documentation website it will show a fallback message.
        """
    )
    return


@app.cell
def _():
    import polars as pl
    import numpy as np

    try:
        import quantwave
        HAS_QUANTWAVE = True
    except ImportError:
        HAS_QUANTWAVE = False

    # Generate some realistic trending and cyclical data
    np.random.seed(42)
    n = 500
    t = np.arange(n)
    trend = 0.1 * t
    cycle = 10 * np.sin(2 * np.pi * t / 20)
    noise = np.random.normal(0, 2, n)
    close = 100 + trend + cycle + noise
    high = close + np.random.uniform(1, 5, n)
    low = close - np.random.uniform(1, 5, n)
    
    df = pl.DataFrame({
        "time": pl.datetime_range(start="2024-01-01", periods=n, interval="1h", eager=True),
        "high": high,
        "low": low,
        "close": close
    })
    return close, df, high, low, n, np, pl, HAS_QUANTWAVE, t


@app.cell
def _(df, mo, pl, HAS_QUANTWAVE):
    if HAS_QUANTWAVE:
        # Apply multiple indicators using real QuantWave Polars expressions
        df_indicators = (
            df.lazy()
            .ta.sma("close", 20)
            .ta.ema("close", 20)
            .ta.mom("close", 1)
            .ta.supertrend(period=10, multiplier=3.0)
            .collect()
        )
        
        # Unnest SuperTrend struct for clean columns
        df_indicators = df_indicators.with_columns([
            pl.col("supertrend_data").struct.field("supertrend").alias("supertrend"),
            pl.col("supertrend_data").struct.field("supertrend_direction").alias("supertrend_dir"),
        ]).drop("supertrend_data")
        
        mo.md(f"Applied real QuantWave indicators (SMA, EMA, Momentum, SuperTrend) to {len(df_indicators)} rows.")
    else:
        # Fallback when quantwave is not installed (e.g. on docs site)
        df_indicators = df.with_columns([
            pl.col("close").rolling_mean(20).alias("sma_20"),
            pl.col("close").rolling_mean(20).alias("ema_20"),  # simplified
            (pl.col("close") - pl.col("close").shift(1)).alias("mom_1"),
        ])
        mo.md(
            """
            **Fallback mode** — `quantwave` not found.

            In a real environment with `pip install quantwave`, this cell would compute  
            SMA, EMA, Momentum, and SuperTrend using the native Rust + Polars backend.
            """
        )
    return df_indicators,


@app.cell
def __(df_indicators, mo):
    mo.md("## Data Preview")
    return mo.ui.table(df_indicators.head(10))


@app.cell
def __(mo):
    mo.md(
        r"""
        ### Conclusion
        QuantWave's integration with Polars allows for extremely clean "method chaining" where multiple indicators can be calculated in a single vectorized pass.
        """
    )
    return


if __name__ == "__main__":
    app.run()
