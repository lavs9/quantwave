# UndersampledDoubleMA

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">internal</span> <span class="kw-badge">utility</span></div>

Undersampled price data smoothed by dual Hann filters to eliminate high frequency noise.

## Visual Example

![UndersampledDoubleMA — annotated preview mapping to core implementation](../../../assets/indicator-previews/undersampleddoublema.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Undersampled price data smoothed by dual Hann filters to eliminate high frequency noise.

Internal implementation module — not intended as a standalone trading indicator.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

This module contains internal utility functions used by other indicators in the library. It is not intended to be used directly as a standalone trading indicator.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `6`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/just_ignore_them.rs`):

\[
Sample = \begin{cases} Price & \text{if } t \pmod N = 0 \\ Sample_{t-1} & \text{otherwise} \end{cases}
\]
\[
Fast = Hann(Sample, FastLen)
\]
\[
Slow = Hann(Sample, SlowLen)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/undersampled_double_ma.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `fast_len` | 6 | Fast Hann filter length |
| `slow_len` | 12 | Slow Hann filter length |
| `sampling_period` | 5 | Undersampling rate (bars) |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::UNDERSAMPLED_DOUBLE_MA;
use quantwave_core::traits::Next;

let mut ind = UNDERSAMPLED_DOUBLE_MA::new(6);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import UNDERSAMPLED_DOUBLE_MA

ind = UNDERSAMPLED_DOUBLE_MA(6)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_undersampleddoublema(series: pl.Series) -> pl.Series:
    ind = qw.UNDERSAMPLED_DOUBLE_MA(6)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_undersampleddoublema, return_dtype=pl.Float64).alias("undersampleddoublema")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/JustIgnoreThem.pdf

**Implementation**: `quantwave-core/src/indicators/just_ignore_them.rs` (`UNDERSAMPLED_DOUBLE_MA` / `UNDERSAMPLED_DOUBLE_MA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/undersampled_double_ma.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
