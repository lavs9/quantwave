# Average Price (AVGPRICE)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">price-transform</span> <span class="kw-badge">classic</span> <span class="kw-badge">smoothing</span></div>

The simple average of the Open, High, Low, and Close prices for a given period.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Average Price (AVGPRICE)** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only average_price_avgprice` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/price_transform.rs`.*

## Description

The simple average of the Open, High, Low, and Close prices for a given period.

Use as a smoothed price input for other indicators. It provides a more balanced view of the period's price action than the Close price alone.

Average Price is the arithmetic mean of the four key price points in a bar. In technical analysis, using Average Price instead of Close can help filter out erratic price spikes and provide a more stable foundation for trend-following algorithms. — TA-Lib Documentation

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/price_transform.rs`):

\[
AVGPRICE = \frac{Open + High + Low + Close}{4}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/avgprice.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::AVGPRICE;
use quantwave_core::traits::Next;

let mut ind = AVGPRICE::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import AVGPRICE

ind = AVGPRICE(14)
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
        pl.col("open").ta.avgprice("open", "high", "low", "close").alias("average_price_avgprice")
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

**Primary Source**: https://www.tradingview.com/support/solutions/43000502588-average-price-avgprice/

**Implementation**: `quantwave-core/src/indicators/price_transform.rs` (`AVGPRICE` / `AVGPRICE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/avgprice.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
