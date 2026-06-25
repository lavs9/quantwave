# GriffithsSpectrum

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">spectrum</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">periodogram</span></div>

Normalized power spectrum estimation using Griffiths adaptive filters.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **GriffithsSpectrum** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only griffithsspectrum` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/griffiths_spectrum.rs`.*

## Description

Normalized power spectrum estimation using Griffiths adaptive filters.

Use to generate a high-resolution periodogram for cycle analysis. Best visualized as a heatmap to identify and track multiple market cycles simultaneously.

The Griffiths Spectrum is an adaptive spectral estimation method that provides higher resolution than a standard DFT for short data segments. It fits an all-pole model to the signal using an LMS algorithm, allowing for instantaneous frequency measurement without the windowing artifacts of FFT-based methods.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/griffiths_spectrum.rs`):

\[
Pwr(P) = \frac{0.1}{(1 - \sum coef_i \cos(2\pi i/P))^2 + (\sum coef_i \sin(2\pi i/P))^2}
\]
\[
Pwr_{norm}(P) = \frac{Pwr(P)}{\max(Pwr)}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/griffiths_spectrum.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `lower_bound` | 18 | Lower period bound |
| `upper_bound` | 40 | Upper period bound |
| `length` | 40 | LMS filter length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::GRIFFITHS_SPECTRUM;
use quantwave_core::traits::Next;

let mut ind = GRIFFITHS_SPECTRUM::new(18);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import GRIFFITHS_SPECTRUM

ind = GRIFFITHS_SPECTRUM(18)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_griffithsspectrum(series: pl.Series) -> pl.Series:
    ind = qw.GRIFFITHS_SPECTRUM(18)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_griffithsspectrum, return_dtype=pl.Float64).alias("griffithsspectrum")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html

**Implementation**: `quantwave-core/src/indicators/griffiths_spectrum.rs` (`GRIFFITHS_SPECTRUM` / `GRIFFITHS_SPECTRUM_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/griffiths_spectrum.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST (DOCUMENTATION_STANDARDS.md).
