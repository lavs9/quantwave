# Adaptive Exponential Moving Average

<div class="indicator-meta"><span class="category-badge">Moving Averages</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span></div>

An adaptive moving average that adjusts its smoothing factor based on volatility.

## Visual Example

![Adaptive Exponential Moving Average — annotated preview mapping to core implementation](../../../assets/indicator-previews/adaptive_exponential_moving_average.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

An adaptive moving average that adjusts its smoothing factor based on volatility.

Use to identify overall trends. AEMA reacts faster to large price movements by adapting the smoothing factor using the highest high and lowest low of a lookback period.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Introduced by Vitali Apirine in TASC April 2019, AEMA alters the EMA's alpha (smoothing factor) by comparing the distance of the close from the lowest low and highest high. This amplifies the smoothing factor during strong price moves while reducing it during sideways chop, yielding a moving average with less lag when it matters most.

**Typical applications:**

- Size stops and position risk from band width or ATR expansion
- Detect squeeze conditions (narrow bands) before breakout systems
- Warm-up: first `10` bars build rolling volatility state
- Combine with trend direction (SuperTrend, MACD) for breakout bias

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/adaptive_ema.rs`):

\[
Rate = \frac{2}{P+1} \times \left(1 + \frac{|(C - L_{min}) - (H_{max} - C)|}{H_{max} - L_{min}}\right) \\ AEMA_t = AEMA_{t-1} + Rate \times (C - AEMA_{t-1})
\]


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 10 | Smoothing period |
| `pds` | 10 | Lookback period for volatility |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ADAPTIVE_EMA;
use quantwave_core::traits::Next;

let mut ind = ADAPTIVE_EMA::new(10);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ADAPTIVE_EMA

ind = ADAPTIVE_EMA(10)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_adaptive_exponential_moving_average(series: pl.Series) -> pl.Series:
    ind = qw.ADAPTIVE_EMA(10)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_adaptive_exponential_moving_average, return_dtype=pl.Float64).alias("adaptive_exponential_moving_average")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `10` bars may return NaN or partial state per implementation.
- Parameter sensitivity: smaller periods increase noise; larger periods increase lag.
- Sudden gaps or bad ticks can distort rolling windows — consider pre-filtering.
- Single-series indicators ignore volume unless otherwise documented.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; streaming and Polars batch paths are bit-identical.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |
| period > len | When period exceeds series length, output is all NaN. |
| NaN inputs | NaN in input propagates to output (NaN out). |
| Invalid params | Non-positive period or missing required params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend/)

## Sources & References

**Primary Source**: Technical Analysis of Stocks & Commodities, April 2019

**Implementation**: `quantwave-core/src/indicators/adaptive_ema.rs` (`ADAPTIVE_EMA` / `ADAPTIVE_EMA_METADATA`).

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
