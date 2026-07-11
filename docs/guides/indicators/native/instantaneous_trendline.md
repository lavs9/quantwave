# Instantaneous Trendline

<div class="indicator-meta"><span class="category-badge">Rocket Science</span> <span class="kw-badge">trend</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

Removes the dominant cycle component identified via the Hilbert Transform to reveal the underlying trend with minimal lag. The indicator is fully adaptive — it self-tunes its smoothing length to the market's current dominant cycle period (typically clamped 6–50 bars).

## Visual Example

![Instantaneous Trendline: synthetic price series containing a clear dominant cycle overlaid on a slow upward drift. The Instantaneous Trendline (bold blue) closely tracks the underlying drift with minimal lag after cycle removal. Annotations label the rejected cycle band, estimated DCPeriod, and the final WMA4 stage.](../../../assets/indicator-previews/instantaneous_trendline.png)

*Synthetic ideal engineered to exhibit a dominant ~18-bar cycle + linear drift. Demonstrates the Hilbert-based adaptive DCPeriod estimation followed by SMA(DCPeriod) then 4-bar WMA smoothing. Matches the exact HilbertFIR + EhlersWma4 logic and state machine in `quantwave-core/src/indicators/instantaneous_trendline.rs`. Generated 2026-05-31 IST via `docs/gen_indicator_previews.py`.*

## Description

The Instantaneous Trendline is one of John Ehlers' most sophisticated trend tools. By continuously estimating the dominant market cycle using phasor arithmetic (homodyne discriminator on the Hilbert Transform outputs), it applies a smoothing window whose length is dictated by the market itself rather than a fixed user parameter. The result is a trend estimate that remains responsive yet smooth, free of the lag inherent in fixed-period moving averages.

It is particularly valuable in regime-aware systems: pair its output with a cycle oscillator (e.g., Cyber Cycle) or Market Structure bias to trade only in the direction of the extracted trend while the cycle component is subordinate. Because the computation is stateless beyond a modest history buffer and the universal `Next<f64>` contract, the same logic powers both real-time decision engines and large-scale Polars feature engineering.

## Formula / Specification

**Core recurrence (simplified from the full Hilbert + homodyne implementation)**:

1. Smooth price with a 4-bar WMA (initial conditioning).
2. Apply the Hilbert Transform (detrender, quadrature, phase advance components) using the previous dominant period estimate to produce in-phase (I) and quadrature (Q) signals.
3. Smooth I/Q components (0.2/0.8 EMA) and compute the homodyne discriminator: `Re = I2·I2_prev + Q2·Q2_prev`, `Im = I2·Q2_prev - Q2·I2_prev`.
4. Derive instantaneous period: `Period = 360 / atan(Im/Re)` (in degrees), with median filtering, rate limiting (0.67×–1.5× prior), and clamping to [6, 50].
5. Compute `DCPeriod = int(SmoothPeriod + 0.5)`.
6. `Trendline = WMA( SMA(Price, DCPeriod), 4 )`.

The full adaptive feedback loop (period_prev, smooth_period_prev, multiple history deques) ensures stability and zero phase distortion on the trend component.

\[
\text{Trendline} = \text{WMA}(\text{SMA}(\text{Price}, \text{DCPeriod}), 4)
\]

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | Fully adaptive. All smoothing lengths are derived internally from the Hilbert-derived dominant cycle estimate. The struct maintains ~50 bars of price history plus auxiliary deques for the phasor pipeline. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::InstantaneousTrendline;
use quantwave_core::traits::Next;

let mut it = InstantaneousTrendline::new();
for price in prices {
    let trend = it.next(price);
    // Use trend as adaptive zero-lag baseline for entries, stops, or regime filters
}
```

**Streaming (Python)**

```python
from quantwave import InstantaneousTrendline

it = InstantaneousTrendline()
for price in prices:
    trend = it.next(price)
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
        pl.col("close").ta.instantaneous_trendline().alias("itrend"),
    ])
    .collect()
)
# "itrend" column is the adaptive trend estimate; bit-identical to streaming
```

All surfaces are bit-identical (enforced by the universal `Next<T>` trait and proptests in `quantwave-core/src/indicators/instantaneous_trendline.rs`).

## Edge Cases & Limitations

- Requires ~12 bars of history before producing a non-price output (early bars return raw price or zeroed state).
- Computationally heavier than simple MAs (multiple Hilbert FIR stages + phasor feedback). Still extremely fast in Rust (~74 ms per million rows).
- DCPeriod estimate can jump; the 0.2/0.8 smoothing + rate limits prevent most instability, but very noisy or gapped data can produce brief artifacts.
- Clamping [6,50] means the indicator is not suitable for ultra-short (sub-6 bar) or very long (multi-month) trend extraction without additional regime logic.
- Best used on liquid instruments with reliable OHLC; thin markets or single-tick feeds can produce erratic period estimates.
- No look-ahead bias by construction.
- When the market is in a strong pure trend (cycle power near zero), the DCPeriod lengthens and the output behaves like a longer adaptive SMA.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |
| period > len | When period exceeds series length, output is all NaN. |
| NaN inputs | NaN in input propagates to output (NaN out). |
| Invalid params | Non-positive period or missing required params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

## Related Indicators & See Also

- [Cyber Cycle](cyber_cycle.md) — companion cycle oscillator; use together for regime filtering
- [Trendflex](trendflex.md), [Reflex](reflex.md) — zero-lag trend/cycle family
- [Roofing Filter](roofing_filter.md) — pre-filter before feeding oscillators or this trendline
- [SuperSmoother](supersmoother.md) — foundational low-lag smoother used internally by many Ehlers tools
- [Market Structure (Swings + BOS)](market_structure.md) — establish directional bias before trusting trendline slope
- [Geometric Patterns (Flags + H&S)](geometric_patterns.md)
- [Indicator Gallery](../gallery.md) • [Native Indicators](../native/index.md) • [Ehlers DSP Suite](../ehlers/index.md)
- Full strategy + ML examples: `docs/examples/notebooks/pa_flag_breakout_strategy.md` and `ml_feature_backtest_parity.md`

## Sources & References

**Primary Source**: John Ehlers, *Rocket Science for Traders* (2001), Chapter 10 — "The Instantaneous Trendline". PDF reference: `references/Ehlers Papers/implemented/ROCKET SCIENCE FOR TRADER.pdf`.

**Implementation Provenance**: Full source of mathematical truth and state machine is `quantwave-core/src/indicators/instantaneous_trendline.rs` (uses `HilbertFIR`, `EhlersWma4`, homodyne discriminator, and the exact recurrence shown above). Universal `Next<f64>` contract and batch/streaming parity documented in `quantwave-core/src/traits.rs` and Agents.md. Metadata + formula_latex recorded in the same file.

**Visual**: Generated 2026-05-31 IST via `docs/gen_indicator_previews.py` using synthetic data engineered to exercise the adaptive DCPeriod + WMA path.

**Additional Context**: Trader's Tips reprints and MQL5 Ehlers ports for practical usage patterns; cross-checked against the core proptests (no gold_standard JSON for this indicator, but exhaustive parity coverage exists).
