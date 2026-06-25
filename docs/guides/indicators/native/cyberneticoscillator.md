# CyberneticOscillator

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">cycle</span> <span class="kw-badge">momentum</span></div>

Combined HighPass and SuperSmoother filters normalized by RMS.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **CyberneticOscillator** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only cyberneticoscillator` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/cybernetic_oscillator.rs`.*

## Description

Combined HighPass and SuperSmoother filters normalized by RMS.

Use as a generalized Ehlers cycle oscillator when you need a configurable bandpass response tuned to a specific dominant cycle period.

The Cybernetic Oscillator is derived from the bandpass filter framework in Ehlers Cybernetic Analysis for Stocks and Futures (2004). By tuning the filter center frequency to the measured dominant cycle period, it extracts only the cyclical component and presents it as an oscillator ranging above and below zero.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/cybernetic_oscillator.rs`):

\[
HP = HighPass(Price, HPLen)
\]
\[
LP = SuperSmoother(HP, LPLen)
\]
\[
RMS = \sqrt{\frac{1}{N} \sum_{i=0}^{N-1} LP_{t-i}^2}
\]
\[
CO = \frac{LP}{RMS}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/cybernetic_oscillator.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `hp_length` | 30 | HighPass filter length |
| `lp_length` | 20 | LowPass (SuperSmoother) length |
| `rms_len` | 100 | RMS normalization length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::CYBERNETIC_OSCILLATOR;
use quantwave_core::traits::Next;

let mut ind = CYBERNETIC_OSCILLATOR::new(30);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import CYBERNETIC_OSCILLATOR

ind = CYBERNETIC_OSCILLATOR(30)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_cyberneticoscillator(series: pl.Series) -> pl.Series:
    ind = qw.CYBERNETIC_OSCILLATOR(30)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_cyberneticoscillator, return_dtype=pl.Float64).alias("cyberneticoscillator")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JUNE%202025.html

**Implementation**: `quantwave-core/src/indicators/cybernetic_oscillator.rs` (`CYBERNETIC_OSCILLATOR` / `CYBERNETIC_OSCILLATOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/cybernetic_oscillator.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
