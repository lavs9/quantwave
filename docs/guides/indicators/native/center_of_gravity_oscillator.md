# Center of Gravity Oscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">momentum</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">zero-lag</span></div>

The CG Oscillator identifies price turning points with essentially zero lag by calculating the balance point of prices.

## Visual Example

![Center of Gravity Oscillator — annotated preview mapping to core implementation](../../../assets/indicator-previews/center_of_gravity_oscillator.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Center of Gravity Oscillator indicator is a technical analysis tool that the cg oscillator identifies price turning points with essentially zero lag by calculating the balance point of prices.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

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

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |
| period > len | When period exceeds series length, output is all NaN. |
| NaN inputs | NaN in input propagates to output (NaN out). |
| Invalid params | Non-positive period or missing required params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

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
