# Generalized Laguerre

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">laguerre</span></div>

A generalized Laguerre filter of arbitrary order using an UltimateSmoother as the primary component.

## Visual Example

![Generalized Laguerre — annotated preview mapping to core implementation](../../../assets/indicator-previews/generalized_laguerre.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A generalized Laguerre filter of arbitrary order using an UltimateSmoother as the primary component.

Use when the standard 4-element Laguerre filter needs further customization. The additional gamma2 parameter allows independent control of the pole spacing for more flexible frequency response shaping.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

The Generalized Laguerre Filter extends the classic 4-element Laguerre design with an additional parameter that controls the distribution of poles across the frequency spectrum. This gives finer control over the transition band slope and passband flatness, useful for specialized spectral analysis applications.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `40`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/generalized_laguerre.rs`):

\[
LG_1 = UltimateSmoother(Price, Length)
\]
\[
LG_i = -\gamma LG_{i-1,t-1} + LG_{i-1,t-1} + \gamma LG_{i,t-1} \text{ for } i=2 \dots Order
\]
\[
Filter = \frac{1}{Order} \sum_{i=1}^{Order} LG_i
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/generalized_laguerre.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 40 | UltimateSmoother period |
| `gamma` | 0.8 | Smoothing factor (0.0 to 1.0) |
| `order` | 8 | Filter order (1 to 10) |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::GENERALIZED_LAGUERRE;
use quantwave_core::traits::Next;

let mut ind = GENERALIZED_LAGUERRE::new(40);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import GENERALIZED_LAGUERRE

ind = GENERALIZED_LAGUERRE(40)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_generalized_laguerre(series: pl.Series) -> pl.Series:
    ind = qw.GENERALIZED_LAGUERRE(40)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_generalized_laguerre, return_dtype=pl.Float64).alias("generalized_laguerre")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202025.html

**Implementation**: `quantwave-core/src/indicators/generalized_laguerre.rs` (`GENERALIZED_LAGUERRE` / `GENERALIZED_LAGUERRE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/generalized_laguerre.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
