# Hilbert Transform - Trend vs. Cycle Mode (HT_TRENDMODE)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">trend</span> <span class="kw-badge">hilbert</span> <span class="kw-badge">regime-detection</span> <span class="kw-badge">dsp</span></div>

A binary indicator that determines if the market is currently in a trending state (1) or a cyclical state (0).

## Visual Example

![Hilbert Transform - Trend vs. Cycle Mode (HT_TRENDMODE) — annotated preview mapping to core implementation](../../../assets/indicator-previews/hilbert_transform_trend_vs_cycle_mode_ht_trendmode.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Hilbert Transform - Trend vs. Cycle Mode (HT_TRENDMODE) indicator is a technical analysis tool that a binary indicator that determines if the market is currently in a trending state (1) or a cyclical state (0).

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Use as a master filter for strategy selection. Deploy trend-following tools when TRENDMODE is 1, and mean-reversion tools when TRENDMODE is 0.

Determining the current market regime is the 'holy grail' of technical analysis. The HT_TRENDMODE indicator uses the rate of change of the dominant cycle phase to distinguish between trending and ranging price action, allowing traders to avoid 'whipsaws' in non-conducive environments. — Rocket Science for Traders

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/cycle.rs`):

\[
\text{TRENDMODE} = \begin{cases} 1 & \text{if trend detected} \\ 0 & \text{if cycle detected} \end{cases}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ht_trendmode.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::HT_TRENDMODE;
use quantwave_core::traits::Next;

let mut ind = HT_TRENDMODE::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import HT_TRENDMODE

ind = HT_TRENDMODE(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").ta.ht_trendmode(14).alias("hilbert_transform_trend_vs_cycle_mode_ht_trendmode")
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

**Primary Source**: https://www.tradingview.com/support/solutions/43000502014-hilbert-transform-trend-vs-cycle-mode-ht-trendmode/

**Implementation**: `quantwave-core/src/indicators/cycle.rs` (`HT_TRENDMODE` / `HT_TRENDMODE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ht_trendmode.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
