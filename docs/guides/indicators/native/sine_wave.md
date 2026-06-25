# Sine Wave

<div class="indicator-meta"><span class="category-badge">Rocket Science</span> <span class="kw-badge">cycle</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">phase</span></div>

Plots a sine wave and a lead-sine wave based on the cyclic phase of price movement.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Sine Wave** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only sine_wave` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/sine_wave.rs`.*

## Description

Plots a sine wave and a lead-sine wave based on the cyclic phase of price movement.

Use to confirm whether the market is in cycle or trend mode. When price follows the sine wave trade cycle reversals; when it diverges switch to trend-following.

Introduced in Rocket Science for Traders, the Sine Wave Indicator plots the sine and cosine of measured instantaneous phase. In cycling markets price tracks the sine wave; in trending markets price breaks through the lead line signaling a mode change.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/sine_wave.rs`):

\[
\text{Sine} = \sin(\text{Phase})
\]
\[
\text{LeadSine} = \sin(\text{Phase} + 45^\circ)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/sine_wave.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::SINE_WAVE;
use quantwave_core::traits::Next;

let mut ind = SINE_WAVE::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import SINE_WAVE

ind = SINE_WAVE(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_sine_wave(series: pl.Series) -> pl.Series:
    ind = qw.SINE_WAVE(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_sine_wave, return_dtype=pl.Float64).alias("sine_wave")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf

**Implementation**: `quantwave-core/src/indicators/sine_wave.rs` (`SINE_WAVE` / `SINE_WAVE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/sine_wave.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
