# Laguerre Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">laguerre</span></div>

A trend-following filter that excels at smoothing long-wavelength components using Laguerre polynomials and an UltimateSmoother base.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Laguerre Filter** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only laguerre_filter` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/laguerre_filter.rs`.*

## Description

A trend-following filter that excels at smoothing long-wavelength components using Laguerre polynomials and an UltimateSmoother base.

Use as a low-lag smoothing filter with only 4 elements of state. Ideal when memory-efficiency matters or when a highly responsive smoother for real-time streaming is needed.

Ehlers introduces Laguerre filters in Cybernetic Analysis (2004), noting they achieve the response of much longer conventional filters using only four coefficients. The single gamma parameter controls the trade-off between lag and smoothness.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/laguerre_filter.rs`):

\[
L_0 = UltimateSmoother(Close, Length)
\]
\[
L_1 = -\gamma L_{0,t-1} + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
...
\]
\[
Laguerre = (L_0 + 4L_1 + 6L_2 + 4L_3 + L_5) / 16
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/laguerre_filter.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 40 | UltimateSmoother period |
| `gamma` | 0.8 | Smoothing factor (0.0 to 1.0) |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::LAGUERRE_FILTER;
use quantwave_core::traits::Next;

let mut ind = LAGUERRE_FILTER::new(40);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import LAGUERRE_FILTER

ind = LAGUERRE_FILTER(40)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_laguerre_filter(series: pl.Series) -> pl.Series:
    ind = qw.LAGUERRE_FILTER(40)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_laguerre_filter, return_dtype=pl.Float64).alias("laguerre_filter")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JULY%202025.html

**Implementation**: `quantwave-core/src/indicators/laguerre_filter.rs` (`LAGUERRE_FILTER` / `LAGUERRE_FILTER_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/laguerre_filter.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
