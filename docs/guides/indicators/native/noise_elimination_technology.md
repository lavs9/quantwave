# Noise Elimination Technology

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">noise</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span></div>

Nonlinear noise removal using Kendall correlation against a straight line.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Noise Elimination Technology** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only noise_elimination_technology` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/noise_elimination.rs`.*

## Description

Nonlinear noise removal using Kendall correlation against a straight line.

Use as a pre-filter to remove spike noise from price or intermediate indicator data without introducing lag. Particularly useful when raw tick or 1-minute data is used.

Ehlers Noise Elimination Technology (NET) is a nonlinear filter that removes isolated noise spikes while leaving genuine price moves intact. It works by comparing each bar to its neighbors and replacing outliers with interpolated values, achieving noise reduction without the lag of conventional smoothers.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/noise_elimination.rs`):

\[
Num = \sum_{i=1}^{N-1} \sum_{j=0}^{i-1} -sgn(X_i - X_j)
\]
\[
Denom = \frac{N(N-1)}{2}
\]
\[
NET = \frac{Num}{Denom}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/noise_elimination.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 14 | Correlation length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::NOISE_ELIMINATION;
use quantwave_core::traits::Next;

let mut ind = NOISE_ELIMINATION::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import NOISE_ELIMINATION

ind = NOISE_ELIMINATION(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_noise_elimination_technology(series: pl.Series) -> pl.Series:
    ind = qw.NOISE_ELIMINATION(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_noise_elimination_technology, return_dtype=pl.Float64).alias("noise_elimination_technology")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Noise%20Elimination%20Technology.pdf

**Implementation**: `quantwave-core/src/indicators/noise_elimination.rs` (`NOISE_ELIMINATION` / `NOISE_ELIMINATION_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/noise_elimination.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
