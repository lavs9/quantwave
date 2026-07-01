# Homodyne Discriminator

<div class="indicator-meta"><span class="category-badge">Rocket Science</span> <span class="kw-badge">cycle</span> <span class="kw-badge">dominant-cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">spectral</span></div>

Estimates the dominant cycle period using a homodyne approach.

## Visual Example

![Homodyne Discriminator — annotated preview mapping to core implementation](../../../assets/indicator-previews/homodyne_discriminator.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Estimates the dominant cycle period using a homodyne approach.

Use to measure the instantaneous dominant cycle period from price data. Feed its output into adaptive indicators as the dynamic period parameter.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

Described in Rocket Science for Traders (2001), the Homodyne Discriminator borrows from radio engineering to measure instantaneous frequency by multiplying the analytic signal by its one-bar-delayed conjugate, giving cycle period without DFT latency.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `N`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/homodyne_discriminator.rs`):

\[
\text{Period} = \frac{360}{\text{atan}(Im / Re)}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/homodyne_discriminator.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::HOMODYNE_DISCRIMINATOR;
use quantwave_core::traits::Next;

let mut ind = HOMODYNE_DISCRIMINATOR::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import HOMODYNE_DISCRIMINATOR

ind = HOMODYNE_DISCRIMINATOR(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_homodyne_discriminator(series: pl.Series) -> pl.Series:
    ind = qw.HOMODYNE_DISCRIMINATOR(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_homodyne_discriminator, return_dtype=pl.Float64).alias("homodyne_discriminator")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf

**Implementation**: `quantwave-core/src/indicators/homodyne_discriminator.rs` (`HOMODYNE_DISCRIMINATOR` / `HOMODYNE_DISCRIMINATOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/homodyne_discriminator.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
