# Cyber Cycle

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

John Ehlers' bandpass-style cycle oscillator — isolates short-term cyclic component with dramatically less lag than classic momentum tools.

## Visual Example

![Cyber Cycle — annotated preview mapping to core implementation](../../../assets/indicator-previews/cyber_cycle.png)

*Synthetic cycle with Cyber Cycle and trigger line. Generated via `docs/generate_all_previews.py`; maps to core `Next<f64>` → `(cycle, trigger)`.*

## Description

Cyber Cycle applies a **symmetrical 4-bar FIR smoother** and a second-order IIR bandpass to extract the cyclic component of price. The **trigger** line is the cycle delayed one bar — crossovers produce timing signals with less derivative noise than MACD-style constructions.

Use Cyber Cycle when you need:

- **Cycle timing** — entries/exits around cycle turns in mean-reverting regimes
- **Regime gating** — suppress cycle trades when [Hurst](../hurst_exponent/) or trend tools show persistence
- **ML features** — rich struct output via `.ta.features.cyber_cycle()` (cycle, trigger, momentum, signal)
- **Ehlers stacks** — chain with [Roofing Filter](../roofing_filter/), [SuperSmoother](../super_smoother/), [Instantaneous Trendline](../instantaneous_trendline/)

QuantWave sources the math from Ehlers' *Cybernetic Analysis for Stocks and Futures* (2004), Chapter 4. The streaming indicator returns `(cycle, trigger)`; the feature extractor adds momentum and signal fields for ML pipelines.

## Formula / Specification

**Source:** John Ehlers, *Cybernetic Analysis for Stocks and Futures* (2004), Ch. 4

Let `length` control \(\alpha = 2 / (length + 1)\). Four-bar smooth:

\[
\text{Smooth}_t = \frac{X_t + 2X_{t-1} + 2X_{t-2} + X_{t-3}}{6}
\]

Cyber Cycle recurrence (bandpass isolation):

\[
CC_t = \left(1 - \frac{\alpha}{2}\right)^2 (\text{Smooth}_t - 2\text{Smooth}_{t-1} + \text{Smooth}_{t-2})
+ 2(1-\alpha) CC_{t-1} - (1-\alpha)^2 CC_{t-2}
\]

\[
\text{Trigger}_t = CC_{t-1}
\]

**Implementation:** `quantwave-core/src/indicators/cyber_cycle.rs`  
**Feature struct:** `quantwave-core/src/features/cyber_cycle.rs` (momentum, signal fields)

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 14 | Controls \(\alpha\); higher = smoother, more lag |

Ehlers examples often use 10–20 on daily data; intraday may need shorter lengths with Roofing pre-filtering.

## Usage Examples

**Polars features (ML / multi-output)**

```python
import polars as pl
import quantwave  # registers LazyFrame.ta.features

df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .ta.features()
    .cyber_cycle(14)
    .collect()
)
# Struct column "cyber_cycle" — unnest for cycle, trigger, momentum, signal
```

**Streaming indicator (cycle + trigger)**

```python
import quantwave as qw

cc = qw.streaming_class("cyber_cycle")(length=14)
for price in closes:
    out = cc.next(price)
    # out.cycle, out.trigger (or tuple depending on binding)
```

**Streaming (Rust)**

```rust
use quantwave_core::indicators::cyber_cycle::CyberCycle;
use quantwave_core::traits::Next;

let mut cc = CyberCycle::new(14);
for price in &closes {
    let (cycle, trigger) = cc.next(*price);
}
```

**Feature matrix (batch research)**

```python
import quantwave as qw

matrix = qw.build_feature_matrix(df, specs=[
    qw.FeatureSpec("cyber_cycle", {"length": 30}),
    qw.FeatureSpec("hurst", {"window": 100}),
])
```

See [ML Features → Backtest E2E](../../../examples/notebooks/ml_feature_backtest_parity.md) for parity-proof pipeline.

## Edge Cases & Limitations

- **Trending markets:** Bandpass assumes cyclic component exists — trending data produces drift; gate with trend/regime filters.
- **Warm-up:** FIR + IIR state needs several bars; early outputs are unstable.
- **Noisy inputs:** Pre-filter with Roofing or SuperSmoother on very choppy series.
- **Not a standalone system:** Pair with structure ([Market Structure](../market_structure/)) or regime tools.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars reflect partial filter state. |
| `length` > series length | Insufficient data for stable cycle extraction. |
| NaN in close | NaN propagates through filter chain. |
| Invalid params | Non-positive `length` raises `ValueError`. |

## Related Indicators & See Also

- [Ehlers DSP guide](../ehlers/index.md)
- [Instantaneous Trendline](../instantaneous_trendline/) — complementary trend/cycle separator
- [Trendflex](../trendflex/) — adaptive trend/cycle decomposition
- [Roofing Filter](../roofing_filter/) — recommended pre-filter
- [ML Feature Stability notebook](../../../examples/notebooks/ml_feature_stability.md)

## Sources & References

**Primary source:** Ehlers (2004) *Cybernetic Analysis for Stocks and Futures*, Chapter 4

**Implementation:** `quantwave-core/src/indicators/cyber_cycle.rs` (`CyberCycle` / `CYBER_CYCLE_METADATA`)

**Parity:** `quantwave-core/tests/test_ml_feature_validation.rs` — batch vs streaming, no look-ahead