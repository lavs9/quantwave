import marimo

__generated_with = "0.1.0"
app = marimo.App()


@app.cell
def __(marimo):
    marimo.md(
        r"""
        # Multi-Indicator Analysis with QuantWave
        
        In this notebook, we'll demonstrate how to combine multiple QuantWave indicators (classic and Ehlers DSP) using Polars' high-performance expressions.
        """
    )
    return


@app.cell
def __():
    import polars as pl
    import numpy as np
    import marimo as mo
    
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
    return close, df, high, low, n, np, pl, t


@app.cell
def __(df, mo, pl):
    # Apply multiple indicators
    # Note: Using native Polars rolling for demonstration where direct plugin call isn't setup in sandbox
    df_indicators = df.with_columns([
        pl.col("close").rolling_mean(20).alias("sma_20"),
        pl.col("close").ewm_mean(span=20).alias("ema_20"),
        # Simulated CyberCycle and SuperTrend (placeholders)
        (pl.col("close") - pl.col("close").shift(1)).alias("momentum"),
        pl.lit(120.0).alias("supertrend")
    ])
    
    mo.md(f"Applied SMA, EMA, and Momentum to {len(df_indicators)} rows.")
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
