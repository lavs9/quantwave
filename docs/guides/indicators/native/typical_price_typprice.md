# Typical Price (TYPPRICE)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">price-transform</span> <span class="kw-badge">classic</span></div>

An average of the High, Low, and Close prices.

## Visual Example

![Typical Price (TYPPRICE) — annotated preview mapping to core implementation](../../../assets/indicator-previews/typical_price_typprice.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

An average of the High, Low, and Close prices.

Use as the primary price input for the Money Flow Index (MFI) and Commodity Channel Index (CCI). It provides a representative price level for the entire bar.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Typical Price is a simple average of the High, Low, and Close. It is widely used in indicators that measure the relationship between price and volume, as it offers a more comprehensive view of the day's activity than the Close price alone. — StockCharts ChartSchool

**Typical applications:**

- See Parameters — default period/length `N`
- Validated via proptests and gold-standard vectors where available
- Use Polars `.ta` plugins for batch; `streaming_class()` for live

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/price_transform.rs`):

\[
TYPPRICE = \frac{High + Low + Close}{3}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/typprice.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::TYPPRICE;
use quantwave_core::traits::Next;

let mut ind = TYPPRICE::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import TYPPRICE

ind = TYPPRICE(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_typical_price_typprice(series: pl.Series) -> pl.Series:
    ind = qw.TYPPRICE(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_typical_price_typprice, return_dtype=pl.Float64).alias("typical_price_typprice")
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

**Primary Source**: https://www.investopedia.com/terms/t/typicalprice.asp

**Implementation**: `quantwave-core/src/indicators/price_transform.rs` (`TYPPRICE` / `TYPPRICE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/typprice.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
