# Median Price (MEDPRICE)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">price-transform</span> <span class="kw-badge">classic</span> <span class="kw-badge">midpoint</span></div>

The midpoint between the High and Low prices for a given period.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Median Price (MEDPRICE)** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only median_price_medprice` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/price_transform.rs`.*

## Description

The midpoint between the High and Low prices for a given period.

Use to identify the central tendency of a bar's range. It is the basis for many oscillators and trend-following indicators like the Bill Williams Alligator.

Median Price represents the 50% retracement level of the current period's range. By focusing on the High-Low midpoint, it removes the 'bias' of the closing price, which can often be manipulated by end-of-day positioning. — TA-Lib Documentation

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/price_transform.rs`):

\[
MEDPRICE = \frac{High + Low}{2}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/medprice.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::MEDPRICE;
use quantwave_core::traits::Next;

let mut ind = MEDPRICE::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import MEDPRICE

ind = MEDPRICE(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_median_price_medprice(series: pl.Series) -> pl.Series:
    ind = qw.MEDPRICE(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_median_price_medprice, return_dtype=pl.Float64).alias("median_price_medprice")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `N` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.tradingview.com/support/solutions/43000502589-median-price-medprice/

**Implementation**: `quantwave-core/src/indicators/price_transform.rs` (`MEDPRICE` / `MEDPRICE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/medprice.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
