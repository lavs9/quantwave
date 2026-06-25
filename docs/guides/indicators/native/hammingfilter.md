# HammingFilter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">windowing</span> <span class="kw-badge">spectral</span></div>

Hamming windowed FIR filter with pedestal.

## Visual Example

![HammingFilter — annotated preview mapping to core implementation](../../../assets/indicator-previews/hammingfilter.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Hamming windowed FIR filter with pedestal.

Apply as a windowing function before DFT-based cycle detection to reduce sidelobe leakage and obtain cleaner dominant cycle estimates.

The Hamming window is a raised-cosine weighting function that reduces spectral leakage by tapering the edges of a data block. Ehlers uses it in DFT-based cycle measurement tools to prevent energy in one frequency bin from contaminating adjacent bins, improving cycle period resolution.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/hamming.rs`):

\[
Deg(n) = Pedestal + (180 - 2 \times Pedestal) \times \frac{n}{L-1}
\]
\[
Coef(n) = \sin\left(\frac{Deg(n) \times \pi}{180}\right)
\]
\[
Filt = \frac{\sum_{n=0}^{L-1} Coef(n) \cdot Price_{t-n}}{\sum Coef(n)}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/hamming_filter.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 20 | Filter length |
| `pedestal` | 10.0 | Pedestal in degrees |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::HAMMING_FILTER;
use quantwave_core::traits::Next;

let mut ind = HAMMING_FILTER::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import HAMMING_FILTER

ind = HAMMING_FILTER(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_hammingfilter(series: pl.Series) -> pl.Series:
    ind = qw.HAMMING_FILTER(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_hammingfilter, return_dtype=pl.Float64).alias("hammingfilter")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - SEPTEMBER 2021.html

**Implementation**: `quantwave-core/src/indicators/hamming.rs` (`HAMMING_FILTER` / `HAMMING_FILTER_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/hamming_filter.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
