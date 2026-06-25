# Hull Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">low-lag</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

The Hull Moving Average (HMA) aims to reduce lag while maintaining smoothness.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Hull Moving Average** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only hull_moving_average` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/hma.rs`.*

## Description

The Hull Moving Average (HMA) aims to reduce lag while maintaining smoothness.

Use as a near-zero-lag moving average for trend-following systems where entry timing is critical. The HMA substantially reduces the lag of a same-period WMA.

Alan Hull designed the Hull Moving Average to nearly eliminate lag while maintaining smoothness. It achieves this by computing a WMA of doubled period, subtracting a WMA of full period, then applying a final WMA to the difference over the square-root period, combining speed with noise reduction. — AlanHull.com

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/hma.rs`):

\[
HMA = WMA(2 \times WMA(\frac{n}{2}) - WMA(n), \sqrt{n})
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/hma.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Smoothing period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::HMA;
use quantwave_core::traits::Next;

let mut ind = HMA::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import HMA

ind = HMA(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_hull_moving_average(series: pl.Series) -> pl.Series:
    ind = qw.HMA(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_hull_moving_average, return_dtype=pl.Float64).alias("hull_moving_average")
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

**Primary Source**: https://alanhull.com/hull-moving-average

**Implementation**: `quantwave-core/src/indicators/hma.rs` (`HMA` / `HMA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/hma.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
