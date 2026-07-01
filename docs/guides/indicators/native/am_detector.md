# AM Detector

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">amplitude</span> <span class="kw-badge">frequency</span></div>

Recovers market volatility from the amplitude-modulated whitened price spectrum.

## Visual Example

![AM Detector — annotated preview mapping to core implementation](../../../assets/indicator-previews/am_detector.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Recovers market volatility from the amplitude-modulated whitened price spectrum.

Use to extract the instantaneous amplitude and frequency of market cycles. The AM output measures cycle energy for position sizing; the FM output tracks cycle period for adaptive indicator tuning.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

Ehlers adapts AM and FM demodulation techniques from radio engineering in Cycle Analytics for Traders to extract cycle amplitude and instantaneous frequency from market data. The amplitude envelope measures how energetic the current cycle is, while FM reveals whether the cycle period is expanding or contracting.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `4`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/amfm.rs`):

\[
Deriv = |Close - Open|, Envel = \max(Deriv, 4), Volatil = \text{Avg}(Envel, 8)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/am_detector.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `highest_len` | 4 | Envelope lookback length |
| `avg_len` | 8 | Smoothing length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::AM_DETECTOR;
use quantwave_core::traits::Next;

let mut ind = AM_DETECTOR::new(4);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import AM_DETECTOR

ind = AM_DETECTOR(4)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_am_detector(series: pl.Series) -> pl.Series:
    ind = qw.AM_DETECTOR(4)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_am_detector, return_dtype=pl.Float64).alias("am_detector")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/AMFM.pdf

**Implementation**: `quantwave-core/src/indicators/amfm.rs` (`AM_DETECTOR` / `AM_DETECTOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/am_detector.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
