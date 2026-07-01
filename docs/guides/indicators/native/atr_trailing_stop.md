# ATR Trailing Stop

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span> <span class="kw-badge">stop-loss</span> <span class="kw-badge">atr</span> <span class="kw-badge">classic</span></div>

A trailing stop based on Average True Range to keep trades in a trend.

## Visual Example

![ATR Trailing Stop — annotated preview mapping to core implementation](../../../assets/indicator-previews/atr_trailing_stop.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The ATR Trailing Stop indicator is a technical analysis tool that a trailing stop based on average true range to keep trades in a trend.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Use as a dynamic trailing stop that widens in volatile markets and tightens in calm ones, automatically adjusting stop distance to current market conditions.

ATR Trailing Stop uses Average True Range to set a stop distance that scales with market volatility. During high-volatility regimes the stop moves further from price to avoid premature exit; during low-volatility regimes it tightens to lock in more profit. It is one of the most robust mechanical stop methods in systematic trading.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/atr_ts.rs`):

\[
Stop = P_{high} - (Multiplier \times ATR)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/atr_ts.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 10 | ATR period |
| `multiplier` | 3.0 | ATR Multiplier |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ATR_TS;
use quantwave_core::traits::Next;

let mut ind = ATR_TS::new(10);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ATR_TS

ind = ATR_TS(10)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_atr_trailing_stop(series: pl.Series) -> pl.Series:
    ind = qw.ATR_TS(10)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_atr_trailing_stop, return_dtype=pl.Float64).alias("atr_trailing_stop")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `10` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.tradingview.com/support/solutions/43000589105-average-true-range-atr/

**Implementation**: `quantwave-core/src/indicators/atr_ts.rs` (`ATR_TS` / `ATR_TS_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/atr_ts.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
