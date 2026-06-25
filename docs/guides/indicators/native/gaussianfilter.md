# GaussianFilter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">low-pass</span></div>

Multi-pole Gaussian low-pass filter for reduced lag.

## Visual Example

![GaussianFilter — annotated preview mapping to core implementation](../../../assets/indicator-previews/gaussianfilter.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Multi-pole Gaussian low-pass filter for reduced lag.

Use when smooth symmetric price averaging with near-zero phase shift is needed. Works well as a preprocessing step for spectral analysis indicators.

Gaussian filters are the theoretically optimal lowpass filter for minimizing the product of time-domain duration and frequency-domain bandwidth. Ehlers implements them as cascaded pole filters with Gaussian-function-derived coefficients, achieving very smooth output with excellent stopband attenuation.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/gaussian.rs`):

\[
\alpha = -\beta + \sqrt{\beta^2 + 2\beta}
\]
\[
\beta = \frac{1 - \cos(2\pi/P)}{2^{1/(2N)} - 1}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/gaussian_filter.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Critical period |
| `poles` | 4 | Number of poles (1-4) |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::GAUSSIAN_FILTER;
use quantwave_core::traits::Next;

let mut ind = GAUSSIAN_FILTER::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import GAUSSIAN_FILTER

ind = GAUSSIAN_FILTER(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_gaussianfilter(series: pl.Series) -> pl.Series:
    ind = qw.GAUSSIAN_FILTER(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_gaussianfilter, return_dtype=pl.Float64).alias("gaussianfilter")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/GaussianFilters.pdf

**Implementation**: `quantwave-core/src/indicators/gaussian.rs` (`GAUSSIAN_FILTER` / `GAUSSIAN_FILTER_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/gaussian_filter.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
