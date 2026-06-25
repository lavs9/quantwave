# Standard Deviation

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">statistics</span> <span class="kw-badge">classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span></div>

Standard Deviation is a statistical measure of market volatility.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Standard Deviation** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only standard_deviation` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/statistics.rs`.*

## Description

Standard Deviation is a statistical measure of market volatility.

Use for statistical analysis of price series: linear regression, standard deviation, correlation coefficients, and other descriptive statistics used as indicator inputs.

Standard statistical measures provide the mathematical foundation for many technical indicators. Linear regression finds the best-fit line through price, standard deviation quantifies dispersion, and correlation coefficients measure how closely two series move together — all are essential for quantitative strategy construction.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/statistics.rs`):

\[
\sigma = \sqrt{ \frac{\sum (x_i - \mu)^2}{N} }
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/stddev.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::STDDEV;
use quantwave_core::traits::Next;

let mut ind = STDDEV::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import STDDEV

ind = STDDEV(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_standard_deviation(series: pl.Series) -> pl.Series:
    ind = qw.STDDEV(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_standard_deviation, return_dtype=pl.Float64).alias("standard_deviation")
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

**Primary Source**: https://www.investopedia.com/terms/s/standarddeviation.asp

**Implementation**: `quantwave-core/src/indicators/statistics.rs` (`STDDEV` / `STDDEV_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/stddev.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
