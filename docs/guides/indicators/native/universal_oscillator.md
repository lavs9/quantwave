# Universal Oscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">universal</span> <span class="kw-badge">momentum</span></div>

An adaptive oscillator that normalizes price momentum using a SuperSmoother filter and AGC.

## Visual Example

![Universal Oscillator — annotated preview mapping to core implementation](../../../assets/indicator-previews/universal_oscillator.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

An adaptive oscillator that normalizes price momentum using a SuperSmoother filter and AGC.

Use as a generic oscillator framework that works on any pre-filtered input. Feed it the output of any smoother or filter to produce a normalized zero-centered oscillator.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

Ehlers Universal Oscillator is a generic momentum computation that can be applied to any filtered price input. It computes the rate of change of the filtered series normalized by its RMS amplitude, producing a consistently scaled oscillator that works regardless of the underlying filter or price instrument.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `20`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/universal_oscillator.rs`):

\[
WN = (Price - Price_{t-2}) / 2
\]
\[
AvgWN = (WN + WN_{t-1}) / 2
\]
\[
Filt = c_1 AvgWN + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]
\[
Peak = \max(0.991 \times Peak_{t-1}, |Filt|)
\]
\[
Universal = Filt / Peak
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/universal_oscillator.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `band_edge` | 20 | Critical period for the SuperSmoother filter |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::UNIVERSAL_OSCILLATOR;
use quantwave_core::traits::Next;

let mut ind = UNIVERSAL_OSCILLATOR::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import UNIVERSAL_OSCILLATOR

ind = UNIVERSAL_OSCILLATOR(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_universal_oscillator(series: pl.Series) -> pl.Series:
    ind = qw.UNIVERSAL_OSCILLATOR(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_universal_oscillator, return_dtype=pl.Float64).alias("universal_oscillator")
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

**Primary Source**: https://www.traders.com/Documentation/FEEDbk_docs/2015/01/TradersTips.html

**Implementation**: `quantwave-core/src/indicators/universal_oscillator.rs` (`UNIVERSAL_OSCILLATOR` / `UNIVERSAL_OSCILLATOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/universal_oscillator.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
