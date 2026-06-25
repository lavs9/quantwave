# Hurst Exponent

<div class="indicator-meta"><span class="category-badge">ML Features</span> <span class="kw-badge">statistics</span> <span class="kw-badge">regime-detection</span> <span class="kw-badge">hurst</span> <span class="kw-badge">ml</span> <span class="kw-badge">trending</span> <span class="kw-badge">mean-reversion</span></div>

Measures the persistence or anti-persistence of a time series using R/S analysis.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Hurst Exponent** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only hurst_exponent` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/hurst.rs`.*

## Description

Measures the persistence or anti-persistence of a time series using R/S analysis.

Use to classify the current market regime. H > 0.5 suggests a trending market (persistent); H < 0.5 suggests a mean-reverting market (anti-persistent). Useful as a filter for trend-following or mean-reversion strategies.

The Hurst Exponent, pioneered by Harold Edwin Hurst in 1951, quantifies the 'memory' of a time series. In technical analysis, it distinguishes between trending, mean-reverting, and random walk price action. It is a critical feature for machine learning models to adapt their logic to the underlying market structure.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/hurst.rs`):

\[
H = \frac{\ln(R/S)}{\ln(N)}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/hurst_exponent.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 100 | Lookback period for R/S analysis |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::HURST_EXPONENT;
use quantwave_core::traits::Next;

let mut ind = HURST_EXPONENT::new(100);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import HURST_EXPONENT

ind = HURST_EXPONENT(100)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_hurst_exponent(series: pl.Series) -> pl.Series:
    ind = qw.HURST_EXPONENT(100)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_hurst_exponent, return_dtype=pl.Float64).alias("hurst_exponent")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `100` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://en.wikipedia.org/wiki/Hurst_exponent

**Implementation**: `quantwave-core/src/indicators/hurst.rs` (`HURST_EXPONENT` / `HURST_EXPONENT_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/hurst_exponent.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
