# BandPass

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">bandpass</span></div>

A bandpass filter that isolates cycle components around a center period.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **BandPass** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only bandpass` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/bandpass.rs`.*

## Description

A bandpass filter that isolates cycle components around a center period.

Apply to isolate a specific cycle period in price, filtering out both trend and noise. Use zero crossings of the filtered output as entry and exit signals.

Ehlers presents the BandPass filter in Cybernetic Analysis as a second-order IIR filter centred on a target cycle period with tunable bandwidth. It simultaneously attenuates lower and higher frequencies, leaving only the desired cycle band in the output.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/bandpass.rs`):

\[
\beta = \cos(360/P), \gamma = 1/\cos(720\delta/P), \alpha = \gamma - \sqrt{\gamma^2 - 1}
\]
\[
BP = 0.5(1 - \alpha)(Price - Price_{t-2}) + \beta(1 + \alpha)BP_{t-1} - \alpha BP_{t-2}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/bandpass.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 20 | Center period of the passband |
| `bandwidth` | 0.1 | Relative bandwidth (delta) |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::BANDPASS;
use quantwave_core::traits::Next;

let mut ind = BANDPASS::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import BANDPASS

ind = BANDPASS(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_bandpass(series: pl.Series) -> pl.Series:
    ind = qw.BANDPASS(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_bandpass, return_dtype=pl.Float64).alias("bandpass")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EmpiricalModeDecomposition.pdf

**Implementation**: `quantwave-core/src/indicators/bandpass.rs` (`BANDPASS` / `BANDPASS_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/bandpass.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
