import marimo as mo

__generated_with = "0.13.0"
app = mo.App()


@app.cell
def _():
    mo.md(
        """
        # Strategy Backtesting with QuantWave

        **Note:** This notebook requires `pip install quantwave` to run the real Polars
        extensions. When viewed on the documentation website, some cells will show
        fallback behavior.
        """
    )
    return


@app.cell
def _():
    import polars as pl
    import numpy as np
    import sys

    # Detect if we're running in the browser (Pyodide) environment used by the docs site
    RUNNING_IN_BROWSER = sys.platform == "emscripten"

    try:
        import quantwave as qw
        HAS_QUANTWAVE = True
    except ImportError:
        HAS_QUANTWAVE = False
        qw = None

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
    return data, HAS_QUANTWAVE, np, pl, qw, RUNNING_IN_BROWSER


@app.cell
def _(data, pl, HAS_QUANTWAVE, mo, RUNNING_IN_BROWSER):
    if HAS_QUANTWAVE and not RUNNING_IN_BROWSER:
        # Apply SuperTrend using the real QuantWave Polars extension
        df = (
            data.lazy()
            .ta.supertrend(period=10, multiplier=3.0)
            .collect()
        )

        # Unnest the struct so we have clean columns
        df = df.with_columns([
            pl.col("supertrend_data").struct.field("supertrend").alias("supertrend"),
            pl.col("supertrend_data").struct.field("supertrend_direction").alias("supertrend_dir"),
        ]).drop("supertrend_data")

        mo.md("## SuperTrend computed with real QuantWave (native Rust + Polars)")
    else:
        # Fallback for docs site or when package is missing
        df = data.with_columns([
            (pl.col("close") * 0.98).alias("supertrend"),
            pl.lit(1).alias("supertrend_dir"),
        ])

        if RUNNING_IN_BROWSER:
            mo.md(
                """
                ## Running in Browser (Documentation Site)

                This notebook is being viewed inside the documentation site's embedded
                marimo environment (Pyodide/WASM).

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

                `quantwave` package not found.

                Install with `pip install quantwave` (or develop from source) to see
                real SuperTrend output.
                """
            )

    return df,


@app.cell
def _(df, mo):
    mo.md(f"Generated **{len(df)}** rows of data.")
    return


if __name__ == "__main__":
    app.run()
