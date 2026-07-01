# TRIX

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

A momentum oscillator that shows the percent rate of change of a triple exponentially smoothed moving average.

## Visual Example

![TRIX — annotated preview mapping to core implementation](../../../assets/indicator-previews/trix.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A momentum oscillator that shows the percent rate of change of a triple exponentially smoothed moving average.

Use to filter out market noise and identify trend reversals. TRIX crossings of the zero line or a signal line can provide trade entries.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Developed by Jack Hutson in the early 1980s, TRIX is a powerful momentum oscillator that effectively filters out minor price fluctuations. By triple-smoothing an EMA, it emphasizes the underlying trend and provides a clear signal when the trend changes direction. — StockCharts ChartSchool

**Typical applications:**

- Fade extremes in ranges; trade with trend on recoveries from oversold/overbought
- Use divergences as early warning — confirm with structure or volume
- Parameter default `15` — shorten for sensitivity, lengthen for stability
- Drop into `build_feature_matrix()` for ML research

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/momentum.rs`):

\[
TRIX = \frac{EMA3_t - EMA3_{t-1}}{EMA3_{t-1}} \times 100
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/trix.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 15 | Smoothing period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::TRIX;
use quantwave_core::traits::Next;

let mut ind = TRIX::new(15);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import TRIX

ind = TRIX(15)
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
        pl.col("close").ta.trix(15).alias("trix")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `15` bars may return NaN or partial state per implementation.
- Parameter sensitivity: smaller periods increase noise; larger periods increase lag.
- Sudden gaps or bad ticks can distort rolling windows — consider pre-filtering.
- Single-series indicators ignore volume unless otherwise documented.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; streaming and Polars batch paths are bit-identical.

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
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend/)

## Sources & References

**Primary Source**: https://www.investopedia.com/terms/t/trix.asp

**Implementation**: `quantwave-core/src/indicators/momentum.rs` (`TRIX` / `TRIX_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/trix.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
