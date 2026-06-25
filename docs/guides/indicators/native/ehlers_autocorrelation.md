# Ehlers Autocorrelation

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">spectral</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">dominant-cycle</span></div>

Computes Pearson correlation of smoothed price with its lags to identify market structure.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Ehlers Autocorrelation** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only ehlers_autocorrelation` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/ehlers_autocorrelation.rs`.*

## Description

Computes Pearson correlation of smoothed price with its lags to identify market structure.

Use to generate an autocorrelation periodogram showing which cycle periods are currently dominant. Visualise as a heatmap to track cycle period shifts over time.

Ehlers introduces autocorrelation-based cycle measurement in Cycle Analytics for Traders (2013) as a more robust alternative to DFT. By computing autocorrelation of Roofing-filtered price at each lag, then applying a spectral DFT to the lag series, he obtains a periodogram insensitive to amplitude variations.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/ehlers_autocorrelation.rs`):

\[
\rho(lag) = \frac{N \sum X Y - \sum X \sum Y}{\sqrt{(N \sum X^2 - (\sum X)^2)(N \sum Y^2 - (\sum Y)^2)}}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ehlers_autocorrelation.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 20 | Correlation window length |
| `num_lags` | 100 | Number of lags to compute |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::EHLERS_AUTOCORRELATION;
use quantwave_core::traits::Next;

let mut ind = EHLERS_AUTOCORRELATION::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import EHLERS_AUTOCORRELATION

ind = EHLERS_AUTOCORRELATION(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_ehlers_autocorrelation(series: pl.Series) -> pl.Series:
    ind = qw.EHLERS_AUTOCORRELATION(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_ehlers_autocorrelation, return_dtype=pl.Float64).alias("ehlers_autocorrelation")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Recursive DSP filters require a warm-up period; first N bars may be unstable or raw-pass-through.
- Designed for cyclic/mean-reverting regimes; trending markets can produce lag or drift.
- Parameter `period` (or equivalent) controls cutoff — too small adds noise, too large adds lag.
- Prefer chaining with other Ehlers tools (Roofing Filter, SuperSmoother) on noisy inputs.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; suitable for live streaming and batch feature pipelines.

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Ehlers DSP guide](../ehlers/index.md)
- [Cyber Cycle](cyber_cycle.md)
- [SuperSmoother](supersmoother.md)

## Sources & References

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - FEBRUARY 2025.html

**Implementation**: `quantwave-core/src/indicators/ehlers_autocorrelation.rs` (`EHLERS_AUTOCORRELATION` / `EHLERS_AUTOCORRELATION_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ehlers_autocorrelation.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
