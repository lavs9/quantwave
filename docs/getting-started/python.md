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

## Warmup and NaN Semantics

Most indicators need a **warmup period** before their output is meaningful. During warmup, batch columns typically contain `NaN` and streaming `next()` may return `NaN` until enough history is accumulated.

```python
import quantwave as qw

# How many leading bars to skip before trusting the signal?
n = qw.warmup_bars("rsi", {"period": 14})  # -> 14

meta = qw.metadata("macd")
print(meta.warmup_bars)  # curated default when available

# Streaming readiness (uses warmup_bars when you pass name=)
cls = qw.streaming_class("rsi")
wrapped = qw.wrap_streaming(cls(14), name="rsi")
for price in closes:
    val = wrapped.next(price)
    if wrapped.is_ready:
        ...  # safe to use val in a live strategy
```

**Conventions:**

| Style | Behavior | Examples |
|-------|----------|----------|
| NaN until ready | Output is `NaN` for the first `warmup_bars` bars | RSI, EMA, MACD, ATR |
| Cumulative from bar 1 | Value exists immediately but is not period-stable | OBV, NVI |
| Event / struct | Empty events or default structs early on | Market Structure, S/R monitor |

Use `qw.assert_parity()` for batch vs streaming checks — it compares warmup bars for agreement, then enforces equality on post-warmup values.

## Options (India)

Options chain analytics and Black–Scholes helpers live under `quantwave.options` (not the top-level indicator namespace):

```python
from quantwave import options

options.bs_call_price(spot=100, k=100, r=0.07, t=0.1, sigma=0.2)
options.nse_lot_size("NIFTY")
```

Legacy `import quantwave; quantwave.bs_call_price(...)` still works but emits a `DeprecationWarning`.

## Backtesting

QuantWave includes a Polars-native, high-performance backtest engine. You can run backtests, param sweeps, and walk-forward optimizations directly on your dataframes using the `.bt` namespace. For a 5-minute introduction, see the [Backtest Quickstart](../guides/backtest/quickstart.md).
