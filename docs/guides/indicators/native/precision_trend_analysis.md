# Precision Trend Analysis

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">trend</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">filter</span></div>

Trend identification using the difference between two high-pass filters.

## Visual Example

![Precision Trend Analysis — annotated preview mapping to core implementation](../../../assets/indicator-previews/precision_trend_analysis.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Precision Trend Analysis indicator is a technical analysis tool that trend identification using the difference between two high-pass filters.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Use as a high-precision trend indicator that applies DSP filtering to remove cycle noise before measuring trend direction, giving fewer but more reliable trend signals.

Ehlers Precision Trend analysis applies a roofing-filter style preprocessing to price before computing the trend indicator, removing the cyclical component that causes premature trend reversals in standard indicators. The result is a trend signal that changes state only when the genuine trend direction changes.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/precision_trend.rs`):

\[
HP1 = HighPass(Price, Length1)
\]
\[
HP2 = HighPass(Price, Length2)
\]
\[
Trend = HP1 - HP2
\]
\[
ROC = \frac{Length2}{6.28} \cdot (Trend - Trend_{t-1})
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/precision_trend.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length1` | 250 | First HighPass filter period |
| `length2` | 40 | Second HighPass filter period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::PRECISION_TREND_ANALYSIS;
use quantwave_core::traits::Next;

let mut ind = PRECISION_TREND_ANALYSIS::new(250);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import PRECISION_TREND_ANALYSIS

ind = PRECISION_TREND_ANALYSIS(250)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_precision_trend_analysis(series: pl.Series) -> pl.Series:
    ind = qw.PRECISION_TREND_ANALYSIS(250)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_precision_trend_analysis, return_dtype=pl.Float64).alias("precision_trend_analysis")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20SEPTEMBER%202024.html

**Implementation**: `quantwave-core/src/indicators/precision_trend.rs` (`PRECISION_TREND_ANALYSIS` / `PRECISION_TREND_ANALYSIS_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/precision_trend.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
