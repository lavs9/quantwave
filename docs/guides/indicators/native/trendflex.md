# Trendflex

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">zero-lag</span> <span class="kw-badge">trend</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">oscillator</span></div>

A zero-lag trend indicator that retains the trend component by measuring the normalized cumulative deviation of a SuperSmoother-filtered price from its recent history. Companion to Reflex (which removes the trend slope to isolate the cycle).

## Visual Example

![Trendflex: price with emerging trend plus noise. The SuperSmoother (orange) provides clean input; the Trendflex line (green) rises early and decisively at trend onset with minimal lag and normalized amplitude. Annotations highlight the cumulative Sum and MS variance normalization stages.](../../../assets/indicator-previews/trendflex.png)

*Synthetic ideal showing early trend detection via cumulative smoothed deviation after SuperSmoother(Length/2). Matches the exact Filt + Sum + MS normalization in `quantwave-core/src/indicators/trendflex.rs` (and its dependency on SuperSmoother). Generated 2026-05-31 IST via `docs/gen_indicator_previews.py`.*

## Description

Trendflex provides a near-zero-lag measurement of trend strength and direction. By first applying the SuperSmoother (a critically-damped Butterworth filter) at half the assumed cycle length, it removes high-frequency noise while preserving the trend slope. The subsequent summation of deviations from the recent smoothed history, normalized by a running mean-square, produces an oscillator-like output that leads conventional moving averages at the start of sustained moves.

It is the natural counterpart to Reflex: use Trendflex to stay with the trend and Reflex (or Cyber Cycle) to detect when the cycle component is re-asserting itself. The single tunable `length` parameter represents the "assumed cycle period" and is used only to size the SuperSmoother; the normalization makes the output relatively scale-free.

## Formula / Specification

**Exact steps (from the reference implementation)**:

1. `Filt = SuperSmoother(Price, Length / 2)`
2. Maintain a rolling history of `Filt`. For each bar after warmup:
   `Sum = (1 / Length) * Σ (Filt_t - Filt_{t-n})` for n = 1..Length
3. `MS = 0.04 * Sum² + 0.96 * MS_{t-1}` (exponential variance estimate)
4. `Trendflex = Sum / sqrt(MS)` (or 0 when MS is negligible)

\[
Filt = 	ext{SuperSmoother}(Price, Length/2)
\]
\[
Sum = rac{1}{Length} \sum_{n=1}^{Length} (Filt_t - Filt_{t-n})
\]
\[
MS = 0.04 \cdot Sum^2 + 0.96 \cdot MS_{t-1}
\]
\[
Trendflex = rac{Sum}{\sqrt{MS}}
\]

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 20 | Assumed cycle period. Controls the SuperSmoother pre-filter (internally `period = length / 2`). Larger values increase smoothness at the cost of slightly more lag on very long trends. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::Trendflex;
use quantwave_core::traits::Next;

let mut tf = Trendflex::new(20);
for price in prices {
    let val = tf.next(price);
    // Positive and rising → strong new trend; use for direction + strength gating
}
```

**Streaming (Python)**

```python
from quantwave import Trendflex

tf = Trendflex(20)
for price in prices:
    val = tf.next(price)
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
        pl.col("close").ta.trendflex(length=20).alias("trendflex"),
    ])
    .collect()
)
```

All surfaces are bit-identical (enforced by the universal `Next<T>` trait and proptests).

## Edge Cases & Limitations

- Returns 0.0 during the first `length + 1` bars (history fill).
- The normalization (division by sqrt(MS)) can produce large spikes when MS is near zero (very quiet markets); clamp or require minimum variance in production.
- `length` should be chosen to match the dominant cycle of the instrument/timescale; 14–30 is typical for daily/4h equities and futures.
- Because it relies on SuperSmoother, any instability in the pre-filter (rare) propagates.
- Excellent for feature engineering (monotonic rising values often precede sustained moves); pair with volatility or volume filters.
- No look-ahead bias.

## Related Indicators & See Also

- [Reflex](reflex.md) — the cycle-extraction sibling (removes trend slope)
- [SuperSmoother](supersmoother.md) — the critical pre-filter used internally
- [Instantaneous Trendline](instantaneous_trendline.md) — fully adaptive (no length param) alternative
- [Cyber Cycle](cyber_cycle.md), [Roofing Filter](roofing_filter.md) — cycle isolation tools that complement trend measures
- [Market Structure (Swings + BOS)](market_structure.md) — confirm trend direction with structure before acting on Trendflex slope
- [Indicator Gallery](../gallery.md) • [Native Indicators](native/index.md) • [Ehlers DSP Suite](ehlers/index.md)
- Strategy notebooks: `docs/examples/notebooks/pa_flag_breakout_strategy.md`

## Sources & References

**Primary Source**: John Ehlers, "Reflex: A New Zero-Lag Indicator" (2020 article). Published in Trader's Tips, February 2020. HTML reference: `references/traderstipsreference/implemented/TRADERS' TIPS - FEBRUARY 2020.html`.

**Implementation Provenance**: `quantwave-core/src/indicators/trendflex.rs` (depends on `SuperSmoother`). The exact Sum + MS recurrence above is the single source of mathematical truth. Parity proptests and `Next<f64>` contract in the same file + `traits.rs`.

**Visual**: Generated 2026-05-31 IST via `docs/gen_indicator_previews.py` with synthetic data exercising the cumulative deviation path.

**Additional Context**: Cross-referenced against Ehlers' later Cycle Analytics writings and multiple MQL5 ports of the Reflex/Trendflex pair.
