# Hull Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">low-lag</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

The Hull Moving Average (HMA) aims to reduce lag while maintaining smoothness.

## Visual Example

![Hull Moving Average — annotated preview mapping to core implementation](../../../assets/indicator-previews/hull_moving_average.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Hull Moving Average (HMA) aims to reduce lag while maintaining smoothness.

Use as a near-zero-lag moving average for trend-following systems where entry timing is critical. The HMA substantially reduces the lag of a same-period WMA.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Alan Hull designed the Hull Moving Average to nearly eliminate lag while maintaining smoothness. It achieves this by computing a WMA of doubled period, subtracting a WMA of full period, then applying a final WMA to the difference over the square-root period, combining speed with noise reduction. — AlanHull.com

**Typical applications:**

- Trend filter or signal line for systematic entries
- Default lookback `14` — tune per asset volatility
- Cross with faster oscillator for entry timing
- Streaming and Polars paths are bit-identical for production parity

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

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
- [SuperTrend](../supertrend/)

## Sources & References

**Primary Source**: https://alanhull.com/hull-moving-average

**Implementation**: `quantwave-core/src/indicators/hma.rs` (`HMA` / `HMA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/hma.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
