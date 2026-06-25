# Parabolic SAR

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">classic</span> <span class="kw-badge">stop-loss</span> <span class="kw-badge">wilder</span></div>

A trend-following indicator used to determine price direction and potential reversals.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Parabolic SAR** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only parabolic_sar` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/overlap.rs`.*

## Description

A trend-following indicator used to determine price direction and potential reversals.

Use for setting trailing stop losses and identifying trend reversals. Dots below price indicate an uptrend, while dots above price indicate a downtrend.

Developed by J. Welles Wilder, the Parabolic Stop and Reverse (SAR) uses an acceleration factor that increases as the trend persists. This 'parabolic' nature allows the indicator to stay close to price action and provide timely exit signals when a trend exhausts. — StockCharts ChartSchool

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/overlap.rs`):

\[
SAR_{t+1} = SAR_t + AF \times (EP - SAR_t)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/sar.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `acceleration` | 0.02 | Acceleration factor |
| `maximum` | 0.2 | Maximum acceleration |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::SAR;
use quantwave_core::traits::Next;

let mut ind = SAR::new(0.02);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import SAR

ind = SAR(0.02)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_parabolic_sar(series: pl.Series) -> pl.Series:
    ind = qw.SAR(0.02)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_parabolic_sar, return_dtype=pl.Float64).alias("parabolic_sar")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `0.02` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.investopedia.com/terms/p/parabolicindicator.asp

**Implementation**: `quantwave-core/src/indicators/overlap.rs` (`SAR` / `SAR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/sar.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
