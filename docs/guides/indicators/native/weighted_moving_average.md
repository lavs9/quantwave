# Weighted Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span> <span class="kw-badge">ema</span></div>

The Weighted Moving Average assigns linearly decreasing weights.

## Visual Example

![Weighted Moving Average — annotated preview mapping to core implementation](../../../assets/indicator-previews/weighted_moving_average.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Weighted Moving Average assigns linearly decreasing weights.

Use as the foundational smoothing module providing SMA, EMA, WMA, and SMMA implementations that power higher-level indicators across the library.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

The core smoothing algorithms — SMA, EMA, WMA — are the building blocks of nearly all technical indicators. EMA applies exponential decay weighting (alpha = 2/(n+1)), SMA applies uniform weighting over N bars, and WMA applies linearly increasing weights emphasizing more recent bars.

**Typical applications:**

- Trend filter or signal line for systematic entries
- Default lookback `14` — tune per asset volatility
- Cross with faster oscillator for entry timing
- Streaming and Polars paths are bit-identical for production parity

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/smoothing.rs`):

\[
WMA = \frac{P_1 \times n + P_2 \times (n-1) + \dots}{n + (n-1) + \dots + 1}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/wma.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Smoothing period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::WMA;
use quantwave_core::traits::Next;

let mut ind = WMA::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import WMA

ind = WMA(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave  # registers pl.col().ta

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").ta.wma(14).alias("weighted_moving_average")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `14` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.investopedia.com/articles/technical/060401.asp

**Implementation**: `quantwave-core/src/indicators/smoothing.rs` (`WMA` / `WMA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/wma.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
