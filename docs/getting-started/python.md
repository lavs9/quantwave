# Getting Started with Python

QuantWave is designed to feel like a natural extension of Polars.

## Installation

```bash
pip install quantwave
# For Polars integration (used in many examples): pip install "quantwave[polars]"
```

## Quick Start

```python
import polars as pl
from quantwave import ta

# Load your data
df = pl.read_parquet("ohlcv.parquet")

# Add indicators using the .ta namespace
df = df.with_columns(
    ta.rsi("close", 14).alias("rsi"),
    ta.mama("close").alias("mama"),
)

print(df.head())
```

## Batch vs Streaming

While the above example shows batch processing with Polars, QuantWave also supports streaming:

```python
from quantwave import SuperTrend

# Initialize the indicator
st = SuperTrend(10, 3.0)

# Process ticks
for high, low, close in price_data:
    signal = st.next(high, low, close)
    print(signal)
```

The streaming API is powered by the universal `Next<T>` trait. Every indicator implements this single trait, which is the same mathematical core used by the Polars expressions. This design guarantees that batch results (via the `ta` namespace or `.ta` on LazyFrame) and streaming results are **bit-identical**.

## Backtesting

QuantWave includes a Polars-native, high-performance backtest engine. You can run backtests, param sweeps, and walk-forward optimizations directly on your dataframes using the `.bt` namespace. For a 5-minute introduction, see the [Backtest Quickstart](../guides/backtest/quickstart.md).
