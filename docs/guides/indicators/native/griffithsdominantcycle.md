# GriffithsDominantCycle

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">dominant-cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">spectral</span></div>

Dominant cycle estimation using Griffiths adaptive spectral analysis.

## Visual Example

![GriffithsDominantCycle — annotated preview mapping to core implementation](../../../assets/indicator-previews/griffithsdominantcycle.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Dominant cycle estimation using Griffiths adaptive spectral analysis.

Use as a robust dominant cycle estimator less sensitive to amplitude changes than DFT-based methods, making it reliable across different market volatility regimes.

The Griffiths method computes the dominant cycle by solving the real-roots of an autocorrelation polynomial. Adapted by Ehlers in Cycle Analytics for Traders, it remains stable even when market amplitude changes rapidly, unlike power-spectrum methods that can shift with volatility.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/griffiths_dominant_cycle.rs`):

\[
Pwr(Period) = \frac{0.1}{(1-Real)^2 + Imag^2}
\]
\[
Real = \sum coef_i \cos(2\pi i / Period)
\]
\[
Imag = \sum coef_i \sin(2\pi i / Period)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/griffiths_dominant_cycle.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `lower_bound` | 18 | Lower period bound |
| `upper_bound` | 40 | Upper period bound |
| `length` | 40 | LMS filter length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::GRIFFITHS_DOMINANT_CYCLE;
use quantwave_core::traits::Next;

let mut ind = GRIFFITHS_DOMINANT_CYCLE::new(18);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import GRIFFITHS_DOMINANT_CYCLE

ind = GRIFFITHS_DOMINANT_CYCLE(18)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_griffithsdominantcycle(series: pl.Series) -> pl.Series:
    ind = qw.GRIFFITHS_DOMINANT_CYCLE(18)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_griffithsdominantcycle, return_dtype=pl.Float64).alias("griffithsdominantcycle")
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

**Implementation**: `quantwave-core/src/indicators/griffiths_dominant_cycle.rs` (`GRIFFITHS_DOMINANT_CYCLE` / `GRIFFITHS_DOMINANT_CYCLE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/griffiths_dominant_cycle.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
