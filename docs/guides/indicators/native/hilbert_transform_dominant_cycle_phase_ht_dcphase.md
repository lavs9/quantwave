# Hilbert Transform - Dominant Cycle Phase (HT_DCPHASE)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">hilbert</span> <span class="kw-badge">phase</span> <span class="kw-badge">dsp</span></div>

Calculates the phase angle (0 to 360 degrees) of the dominant cycle identified by the Hilbert Transform.

## Visual Example

![Hilbert Transform - Dominant Cycle Phase (HT_DCPHASE) — annotated preview mapping to core implementation](../../../assets/indicator-previews/hilbert_transform_dominant_cycle_phase_ht_dcphase.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Calculates the phase angle (0 to 360 degrees) of the dominant cycle identified by the Hilbert Transform.

Use to identify the current position within a market cycle. It is the core component for generating the Hilbert Sine Wave indicator, which signals trend vs. cycle regimes.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

The Dominant Cycle Phase represents the instantaneous position within a detected cycle. By measuring the phase angle, traders can determine if the market is at a peak, trough, or mid-cycle, enabling more precise timing for entry and exit signals. — Rocket Science for Traders

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `N`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/cycle.rs`):

\[
Phase = \arctan\left(\frac{\text{Quadrature}}{\text{InPhase}}\right)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ht_dcphase.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::HT_DCPHASE;
use quantwave_core::traits::Next;

let mut ind = HT_DCPHASE::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import HT_DCPHASE

ind = HT_DCPHASE(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave  # registers pl.col().ta

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").ta.ht_dcphase(14).alias("hilbert_transform_dominant_cycle_phase_ht_dcphase")
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

**Primary Source**: https://www.tradingview.com/support/solutions/43000502010-hilbert-transform-dominant-cycle-phase-ht-dcphase/

**Implementation**: `quantwave-core/src/indicators/cycle.rs` (`HT_DCPHASE` / `HT_DCPHASE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ht_dcphase.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
