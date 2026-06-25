# TriangleFilter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">triangle</span></div>

Triangle windowed FIR filter.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **TriangleFilter** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only trianglefilter` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/triangle.rs`.*

## Description

Triangle windowed FIR filter.

Use as a pre-smoother to reduce noise before applying cycle or momentum indicators when a symmetric low-ripple response is needed.

The Triangle (Bartlett) window is a linearly-tapered FIR filter equivalent to applying two rectangular windows in sequence. It provides moderate sidelobe suppression and is useful when computational simplicity is preferred over maximum spectral attenuation.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/triangle.rs`):

\[
Coef(n) = \begin{cases} n & n < L/2 \\ L/2 & n = L/2 \\ L + 1 - n & n > L/2 \end{cases}
\]
\[
Filt = \frac{\sum_{n=1}^L Coef(n) \cdot Price_{t-n+1}}{\sum Coef(n)}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/triangle_filter.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 20 | Filter length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::TRIANGLE_FILTER;
use quantwave_core::traits::Next;

let mut ind = TRIANGLE_FILTER::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import TRIANGLE_FILTER

ind = TRIANGLE_FILTER(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_trianglefilter(series: pl.Series) -> pl.Series:
    ind = qw.TRIANGLE_FILTER(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_trianglefilter, return_dtype=pl.Float64).alias("trianglefilter")
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

**Implementation**: `quantwave-core/src/indicators/triangle.rs` (`TRIANGLE_FILTER` / `TRIANGLE_FILTER_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/triangle_filter.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
