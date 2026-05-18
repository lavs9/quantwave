import marimo

__generated_with = "0.1.0"
app = marimo.App()


@app.cell
def __(marimo):
    marimo.md("# Strategy Backtesting with QuantWave")
    return


@app.cell
def __():
    import polars as pl
    import quantwave as qw
    import numpy as np

    # Generate synthetic OHLCV data
    num_rows = 1000
    data = pl.DataFrame({
        "time": pl.date_range(start="2023-01-01", periods=num_rows, interval="1h", eager=True),
        "open": np.random.uniform(100, 200, num_rows),
        "high": np.random.uniform(100, 200, num_rows),
        "low": np.random.uniform(100, 200, num_rows),
        "close": np.random.uniform(100, 200, num_rows),
        "volume": np.random.uniform(1000, 5000, num_rows),
    })
    return data, np, pl, qw


@app.cell
def __(data, pl):
    # Apply SuperTrend indicator using QuantWave
    # (Assuming quantwave is installed and exposed)
    # df = data.lazy().with_columns([
    #     pl.col("close").ta.supertrend(period=10, multiplier=3.0).alias("supertrend")
    # ]).collect()
    
    # Placeholder for actual calculation
    df = data.with_columns(pl.lit(150.0).alias("supertrend"))
    df.head()
    return df,


@app.cell
def __(df, marimo):
    marimo.md(f"## Visualization\n\nGenerated {len(df)} rows of data.")
    return


if __name__ == "__main__":
    app.run()
