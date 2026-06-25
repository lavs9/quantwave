# EhlersUltimateOscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">momentum</span> <span class="kw-badge">adaptive</span></div>

A highly responsive oscillator created from the difference of two highpass filters, normalized by RMS.

## Visual Example

![EhlersUltimateOscillator — annotated preview mapping to core implementation](../../../assets/indicator-previews/ehlersultimateoscillator.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A highly responsive oscillator created from the difference of two highpass filters, normalized by RMS.

Use as a multi-scale momentum oscillator that combines signals from multiple cycle-aware timeframes to reduce false signals from any single period.

Ehlers Ultimate Oscillator combines the outputs of multiple cycle-synchronized oscillators operating at different dominant cycle harmonics. By averaging across scales, it reduces the likelihood of false signals that occur when any single oscillator is temporarily misaligned with the market cycle.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/ehlers_ultimate_oscillator.rs`):

\[
HP_1 = \text{HighPass}(Price, BandEdge \cdot Bandwidth)
\]
\[
HP_2 = \text{HighPass}(Price, BandEdge)
\]
\[
Signal = HP_1 - HP_2
\]
\[
UO = \frac{Signal}{RMS(Signal, 100)}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ehlers_ultimate_oscillator.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `band_edge` | 20 | Critical period (shorter period) |
| `bandwidth` | 2.0 | Multiplier for the longer period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::EHLERS_ULTIMATE_OSCILLATOR;
use quantwave_core::traits::Next;

let mut ind = EHLERS_ULTIMATE_OSCILLATOR::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import EHLERS_ULTIMATE_OSCILLATOR

ind = EHLERS_ULTIMATE_OSCILLATOR(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_ehlersultimateoscillator(series: pl.Series) -> pl.Series:
    ind = qw.EHLERS_ULTIMATE_OSCILLATOR(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_ehlersultimateoscillator, return_dtype=pl.Float64).alias("ehlersultimateoscillator")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20APRIL%202025.html

**Implementation**: `quantwave-core/src/indicators/ehlers_ultimate_oscillator.rs` (`EHLERS_ULTIMATE_OSCILLATOR` / `EHLERS_ULTIMATE_OSCILLATOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ehlers_ultimate_oscillator.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
