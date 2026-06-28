# RecursiveMedianOscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">median</span> <span class="kw-badge">cycle</span> <span class="kw-badge">highpass</span></div>

Oscillator derived from the Recursive Median filter using a 2nd-order Highpass filter.

## Visual Example

![RecursiveMedianOscillator — annotated preview mapping to core implementation](../../../assets/indicator-previews/recursivemedianoscillator.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The RecursiveMedianOscillator indicator is a technical analysis tool that oscillator derived from the recursive median filter using a 2nd-order highpass filter.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Identify cyclic turning points with reduced lag and noise. The high-pass component removes the trend, leaving the cycle.

By applying a 2nd-order Highpass filter to the Recursive Median output, we create an oscillator that is specifically tuned to the dominant cycle while remaining immune to the outlier spikes that would otherwise create false signals.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/recursive_median.rs`):

\[
\alpha_2 = \frac{\cos(0.707 \cdot 360/HP) + \sin(0.707 \cdot 360/HP) - 1}{\cos(0.707 \cdot 360/HP)}
\]
\[
RMO_t = (1-\alpha_2/2)^2(RM_t - 2RM_{t-1} + RM_{t-2}) + 2(1-\alpha_2)RMO_{t-1} - (1-\alpha_2)^2RMO_{t-2}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/recursive_median_oscillator.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `lp_period` | 12 | Low-pass smoothing period |
| `hp_period` | 30 | High-pass cutoff period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::RECURSIVE_MEDIAN_OSCILLATOR;
use quantwave_core::traits::Next;

let mut ind = RECURSIVE_MEDIAN_OSCILLATOR::new(12);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import RECURSIVE_MEDIAN_OSCILLATOR

ind = RECURSIVE_MEDIAN_OSCILLATOR(12)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_recursivemedianoscillator(series: pl.Series) -> pl.Series:
    ind = qw.RECURSIVE_MEDIAN_OSCILLATOR(12)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_recursivemedianoscillator, return_dtype=pl.Float64).alias("recursivemedianoscillator")
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

**Primary Source**: https://www.traders.com/Documentation/FEEDbk_docs/2018/03/TradersTips.html

**Implementation**: `quantwave-core/src/indicators/recursive_median.rs` (`RECURSIVE_MEDIAN_OSCILLATOR` / `RECURSIVE_MEDIAN_OSCILLATOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/recursive_median_oscillator.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
