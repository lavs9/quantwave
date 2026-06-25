# Choppiness Index

<div class="indicator-meta"><span class="category-badge">Modern</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend-strength</span> <span class="kw-badge">classic</span> <span class="kw-badge">range</span></div>

Determines if the market is trending (low values) or ranging/choppy (high values).

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Choppiness Index** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only choppiness_index` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/choppiness_index.rs`.*

## Description

Determines if the market is trending (low values) or ranging/choppy (high values).

Use to determine whether a market is trending or choppy before selecting a trading strategy. Values above 61.8 indicate chop; values below 38.2 indicate a strong trend.

The Choppiness Index, developed by E.W. Dreiss, measures how much of the total ATR-based range is consumed by the actual net price move over N bars. A value near 100 means price wandered back and forth using all available range without net progress (maximum chop); near 0 means a straight directional move with minimal retracement. — StockCharts ChartSchool

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/choppiness_index.rs`):

\[
CHOP = 100 \times \frac{\log_{10}(\sum_{i=1}^n ATR(1)_i / (\max(H, n) - \min(L, n)))}{\log_{10}(n)}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/choppiness_index.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Lookback period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::CHOPPINESS_INDEX;
use quantwave_core::traits::Next;

let mut ind = CHOPPINESS_INDEX::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import CHOPPINESS_INDEX

ind = CHOPPINESS_INDEX(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_choppiness_index(series: pl.Series) -> pl.Series:
    ind = qw.CHOPPINESS_INDEX(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_choppiness_index, return_dtype=pl.Float64).alias("choppiness_index")
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

**Primary Source**: https://www.tradingview.com/support/solutions/43000501980-choppiness-index-chop/

**Implementation**: `quantwave-core/src/indicators/choppiness_index.rs` (`CHOPPINESS_INDEX` / `CHOPPINESS_INDEX_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/choppiness_index.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
