# Center of Gravity Oscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">momentum</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">zero-lag</span></div>

The CG Oscillator identifies price turning points with essentially zero lag by calculating the balance point of prices.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Center of Gravity Oscillator** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only center_of_gravity_oscillator` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/cg.rs`.*

## Description

The CG Oscillator identifies price turning points with essentially zero lag by calculating the balance point of prices.

Use as a zero-lag momentum oscillator to detect cycle turning points. Crossovers of the trigger line provide high-accuracy entry and exit signals.

Ehlers introduces the Center of Gravity oscillator in Cybernetic Analysis (2004) as a near-zero-lag indicator. It computes the center of mass of a price series over a lookback window, producing an oscillator whose turning points lead price turns — a reversal of the usual indicator lag relationship.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/cg.rs`):

\[
CG = -\frac{\sum_{i=0}^{N-1} (i+1) \times Price_i}{\sum_{i=0}^{N-1} Price_i}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/cg.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 10 | Observation window length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::CG;
use quantwave_core::traits::Next;

let mut ind = CG::new(10);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import CG

ind = CG(10)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_center_of_gravity_oscillator(series: pl.Series) -> pl.Series:
    ind = qw.CG(10)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_center_of_gravity_oscillator, return_dtype=pl.Float64).alias("center_of_gravity_oscillator")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TheCGOscillator.pdf

**Implementation**: `quantwave-core/src/indicators/cg.rs` (`CG` / `CG_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/cg.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
