# Butterworth2

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">low-pass</span></div>

2-pole Butterworth low-pass filter.

## Visual Example

![Butterworth2 — annotated preview mapping to core implementation](../../../assets/indicator-previews/butterworth2.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

2-pole Butterworth low-pass filter.

Use to smooth price or intermediate indicator values with a flat passband and sharp rolloff. The 3-pole version provides steeper attenuation at the cost of marginally more lag.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

Butterworth filters are maximally flat in the passband, introducing no ripple. Ehlers implements 2-pole and 3-pole Butterworth IIR designs in Cycle Analytics for Traders, noting that the SuperSmoother is actually a critically-damped 2-pole Butterworth variant.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `14`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/butterworth.rs`):

\[
a = \exp(-1.414\pi/P)
\]
\[
b = 2a \cos(1.414\pi/P)
\]
\[
f = bf_{t-1} - a^2f_{t-2} + \frac{1-b+a^2}{4}(g + 2g_{t-1} + g_{t-2})
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/butterworth2.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Critical period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::BUTTERWORTH2;
use quantwave_core::traits::Next;

let mut ind = BUTTERWORTH2::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import BUTTERWORTH2

ind = BUTTERWORTH2(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_butterworth2(series: pl.Series) -> pl.Series:
    ind = qw.BUTTERWORTH2(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_butterworth2, return_dtype=pl.Float64).alias("butterworth2")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Poles.pdf

**Implementation**: `quantwave-core/src/indicators/butterworth.rs` (`BUTTERWORTH2` / `BUTTERWORTH2_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/butterworth2.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
