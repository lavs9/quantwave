# ChannelCycle

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">dominant-cycle</span></div>

Extracts cyclic components and a leading function using channel-normalized bandpass filtering.

## Visual Example

![ChannelCycle — annotated preview mapping to core implementation](../../../assets/indicator-previews/channelcycle.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Extracts cyclic components and a leading function using channel-normalized bandpass filtering.

Use to estimate the dominant cycle period from the width of price channels. Useful as a simpler alternative to Hilbert Transform cycle measurement when computational resources are limited.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

Ehlers estimates the dominant cycle period by tracking successive peaks and troughs of price. The distance between turning points approximates half the cycle period, and smoothing this measurement across recent bars gives a stable period estimate for use in adaptive indicators.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `20`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/channel_cycle.rs`):

\[
Detrended = \frac{Price - Low}{High - Low} - 0.5
\]
\[
BP = \text{Bandpass}(Detrended, Period)
\]
\[
Leading = \frac{BP - BP_{t-1}}{2\pi/Period}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/channel_cycle.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 20 | Channel and Bandpass period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::CHANNEL_CYCLE;
use quantwave_core::traits::Next;

let mut ind = CHANNEL_CYCLE::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import CHANNEL_CYCLE

ind = CHANNEL_CYCLE(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_channelcycle(series: pl.Series) -> pl.Series:
    ind = qw.CHANNEL_CYCLE(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_channelcycle, return_dtype=pl.Float64).alias("channelcycle")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/InferringTradingStrategies.pdf

**Implementation**: `quantwave-core/src/indicators/channel_cycle.rs` (`CHANNEL_CYCLE` / `CHANNEL_CYCLE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/channel_cycle.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
