# Weighted Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span> <span class="kw-badge">ema</span></div>

The Weighted Moving Average assigns linearly decreasing weights.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Weighted Moving Average** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only weighted_moving_average` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/smoothing.rs`.*

## Description

The Weighted Moving Average assigns linearly decreasing weights.

Use as the foundational smoothing module providing SMA, EMA, WMA, and SMMA implementations that power higher-level indicators across the library.

The core smoothing algorithms — SMA, EMA, WMA — are the building blocks of nearly all technical indicators. EMA applies exponential decay weighting (alpha = 2/(n+1)), SMA applies uniform weighting over N bars, and WMA applies linearly increasing weights emphasizing more recent bars.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

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

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").ta.wma("close", 14).alias("weighted_moving_average")
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

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend.md)

## Sources & References

**Primary Source**: https://www.investopedia.com/articles/technical/060401.asp

**Implementation**: `quantwave-core/src/indicators/smoothing.rs` (`WMA` / `WMA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/wma.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
