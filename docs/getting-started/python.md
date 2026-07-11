# Getting Started with Python

QuantWave is designed to feel like a natural extension of Polars.

!!! tip "New here?"
    Follow the [Getting Started funnel](../index.md) for install → first indicator → pick your path.  
    Migrating from TA-Lib or pandas-ta? See [QuantWave vs alternatives](../comparison.md).

## Installation

```bash
pip install quantwave
# Polars batch/backtest examples also need:
pip install "quantwave[polars]"
```

The PyPI wheel bundles the core extension, Polars expression plugins (`pl.col().ta`), and the backtest engine. The `[polars]` extra installs the Polars Python package.

Verify your install:

```bash
quantwave doctor
quantwave list --category "Classic"
quantwave info rsi
```

## Quick Start

### Polars batch (recommended)

```python
import polars as pl
import quantwave  # registers pl.col().ta and LazyFrame.bt

df = pl.read_parquet("ohlcv.parquet")

df = df.lazy().with_columns(
    pl.col("close").ta.rsi(timeperiod=14).alias("rsi"),
    pl.col("close").ta.ema(period=20).alias("ema"),
).collect()

print(df.head())
```

### List-based batch API

```python
import quantwave as qw

closes = [float(x) for x in range(1, 100)]
rsi = qw.ta.rsi(14, closes)
```

## Batch vs Streaming

Polars batch and streaming share the same math. For live or tick-by-tick use:

```python
import quantwave as qw

cls = qw.streaming_class("supertrend")
st = qw.wrap_streaming(cls(period=10, multiplier=3.0), name="supertrend")

for high, low, close in price_data:
    signal = st.next((high, low, close))
    if st.is_ready:
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

## Discovery, categories & boundaries

```python
import quantwave as qw

qw.indicators()              # sorted list of 221 names
qw.categories()              # e.g. "Classic", "Ehlers DSP", "Momentum", ...
qw.category("Ehlers DSP")    # indicators in one category
qw.indicators_by_category()  # full map for UIs / autocomplete

meta = qw.metadata("rsi")
info = qw.boundary_info("rsi")  # warmup, NaN, invalid-param semantics
```

## TA-Lib migration

```python
from quantwave import talib as ta

print(ta.list_functions())   # uppercase TA-Lib names in this build
rsi = ta.RSI(closes, timeperiod=14)
```

## Exception contract

```python
import quantwave as qw

try:
    qw.assert_parity("rsi", {"period": 14}, closes)
except qw.ParityError:
    ...  # batch vs streaming mismatch
except qw.IndicatorNotFoundError:
    ...  # unknown name
except qw.QuantwaveError:
    ...  # any library-specific error
```

`qw.__version__` is exposed via `importlib.metadata` (e.g. `"0.6.0"`).

## ML features & backtesting

- [ML Feature Engineering guide](../guides/ml_features.md)
- [Backtest Quickstart](../guides/backtest/quickstart.md)
- [ML Features → Backtest (E2E)](../examples/notebooks/ml_feature_backtest_parity.md)

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

## Where to go next

| Goal | Next step |
|------|-----------|
| Compare stacks | [QuantWave vs TA-Lib & pandas-ta](../comparison.md) |
| Browse indicators | [Indicators overview](../guides/indicators/index.md) · [Gallery](../guides/indicators/gallery.md) |
| Batch ↔ streaming deep dive | [Examples guide](../examples/batch-streaming.md) |
| Plugin vs `.ta` | [When to use which](../guides/plugin_vs_ta.md) |
| Backtest | [Quickstart](../guides/backtest/quickstart.md) · [Strategy notebook](../examples/notebooks/strategy_backtest.md) |
| Full funnel | [Getting Started hub](../index.md) |
