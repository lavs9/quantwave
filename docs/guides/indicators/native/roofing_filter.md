# Roofing Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">cycle</span> <span class="kw-badge">high-pass</span> <span class="kw-badge">low-pass</span></div>

Combines a 2-pole HighPass filter and a SuperSmoother to isolate specific cyclic components in the tradable frequency band while removing both low-frequency trend drift and high-frequency noise.

## Visual Example

![Roofing Filter: raw price (gray) containing strong trend, high-frequency noise, and a clear ~28-bar cycle. The Roofing output (purple) is a clean, zero-centered oscillation containing only the desired cycle band. Shaded region and annotations mark the HP trend removal and SS noise removal stages.](../../../assets/indicator-previews/roofing_filter.png)

*Synthetic ideal: low-frequency trend + high-frequency noise + tradable cycle (28/41 bars). After 2-pole HP (hp_period=48) + SuperSmoother (ss_period=10) only the cycle band remains. Matches the exact coefficient derivation and state in `quantwave-core/src/indicators/roofing_filter.rs`. Generated 2026-05-31 IST via `docs/gen_indicator_previews.py`.*

## Description

The Roofing Filter is a bandpass preprocessor designed by Ehlers to solve the "spectral dilation" problem — the fact that raw price contains energy across many frequencies, causing oscillators to produce unreliable signals. By first applying a 2-pole high-pass filter tuned to remove periods longer than `hp_period` (default 48), the dominant trend is eliminated. A subsequent SuperSmoother then removes periods shorter than `ss_period` (default 10). The result is a clean, stationary signal containing only the 10–48 bar cycle band that most discretionary and systematic traders actually trade.

Feed the Roofing output directly into Cyber Cycle, Stochastic, RSI, or any other oscillator for dramatically cleaner crossovers and divergences.

## Formula / Specification

**Coefficient derivation and recurrence (exact)**:

High-pass (hp_period):
```
angle = 0.707 * 2π / hp_period
α1 = (cos(angle) + sin(angle) - 1) / cos(angle)
hp_c1 = (1 - α1/2)²
hp_c2 = 2(1 - α1)
hp_c3 = -(1 - α1)²
HP = hp_c1*(Price - 2*Price[t-1] + Price[t-2]) + hp_c2*HP[t-1] + hp_c3*HP[t-2]
```

SuperSmoother stage on HP (ss_period):
```
a1 = exp(-1.414π / ss_period)
ss_c2 = 2*a1*cos(1.414π / ss_period)
ss_c3 = -a1²
ss_c1 = 1 - ss_c2 - ss_c3
Filt = ss_c1*(HP + HP[t-1])/2 + ss_c2*Filt[t-1] + ss_c3*Filt[t-2]
```

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `hp_period` | 48 | High-pass critical period. Larger values remove more of the long-term trend (wider bandpass from below). |
| `ss_period` | 10 | SuperSmoother critical period. Smaller values allow shorter cycles through (wider bandpass from above). Typical tradable band is ~10–48 bars. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::RoofingFilter;
use quantwave_core::traits::Next;

let mut rf = RoofingFilter::new(48, 10);
for price in prices {
    let cycle = rf.next(price);
    // cycle is now band-limited; feed to oscillator or use for cycle timing
}
```

**Streaming (Python)**

```python
from quantwave import RoofingFilter

rf = RoofingFilter(48, 10)
for price in prices:
    cycle = rf.next(price)
    ...
```

**Polars Batch (Python — primary research / feature surface)**

```python
import polars as pl
import quantwave as qw

df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .with_columns([
        pl.col("close").ta.roofing_filter(hp_period=48, ss_period=10).alias("roof"),
    ])
    .collect()
)
```

All surfaces are bit-identical (enforced by the universal `Next<T>` trait and proptests).

## Edge Cases & Limitations

- First 3 bars return 0 (insufficient history for the 2nd-difference HP).
- The filter is a linear IIR; on extremely gappy or zero-volume data the internal state can produce transient artifacts (rare in practice).
- Choosing hp_period too small removes part of the tradable cycle you wanted to keep.
- The output is zero-mean by design; do not expect it to carry absolute price level information.
- Excellent preprocessing step before any bounded oscillator or ML label that cares about cyclic turning points.
- No look-ahead bias.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |
| period > len | When period exceeds series length, output is all NaN. |
| NaN inputs | NaN in input propagates to output (NaN out). |
| Invalid params | Non-positive period or missing required params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

## Related Indicators & See Also

- [SuperSmoother](supersmoother.md) — the noise-removal stage used inside Roofing
- [Cyber Cycle](cyber_cycle.md) — classic Ehlers oscillator that benefits enormously from Roofing pre-filtering
- [Instantaneous Trendline](instantaneous_trendline.md), [Trendflex](trendflex.md) — use Roofing output when you want cycle-aware versions of these
- [Fisher Transform](fisher_transform.md) — apply Fisher to a Roofing-cleaned oscillator for even sharper turns
- [Indicator Gallery](../gallery.md) • [Native Indicators](native/index.md) • [Ehlers DSP Suite](ehlers/index.md)
- `docs/examples/notebooks/multi_indicator_analysis.py` (band-pass concepts)

## Sources & References

**Primary Source**: John Ehlers, *Predictive Indicators for Effective Trading Strategies* (2013) / *Cycle Analytics for Traders* (2013). PDF reference: `references/Ehlers Papers/implemented/PredictiveIndicators.pdf`.

**Implementation Provenance**: `quantwave-core/src/indicators/roofing_filter.rs`. The exact α1 derivation and two-stage recurrence (HP then SS) above is the mathematical source of truth. Full state machine + proptests in the file; `Next<f64>` parity contract in `traits.rs`.

**Visual**: Generated 2026-05-31 IST via `docs/gen_indicator_previews.py` using synthetic multi-frequency price that exercises the precise bandpass behavior.

**Additional Context**: Widely implemented in MQL5 Ehlers libraries and TradingView scripts under the same name.
