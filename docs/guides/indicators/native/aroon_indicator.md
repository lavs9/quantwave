# Aroon Indicator

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">classic</span> <span class="kw-badge">breakout</span></div>

An indicator system that identifies when a new trend is beginning and the strength of the trend.

## Visual Example

![Aroon Indicator — annotated preview mapping to core implementation](../../../assets/indicator-previews/aroon_indicator.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

An indicator system that identifies when a new trend is beginning and the strength of the trend.

Use to identify when a security is trending and when it is in a range-bound period. Aroon Up crossing above Aroon Down signals the start of a new uptrend.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Developed by Tushar Chande in 1995, the Aroon indicator focuses on the time between highs and the time between lows over a given period. The idea is that strong uptrends will regularly see new highs, and strong downtrends will regularly see new lows. — StockCharts ChartSchool

**Typical applications:**

- Trend filter or signal line for systematic entries
- Default lookback `25` — tune per asset volatility
- Cross with faster oscillator for entry timing
- Streaming and Polars paths are bit-identical for production parity

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/momentum.rs`):

\[
\text{Aroon Up} = \frac{n - \text{Periods since n-period High}}{n} \times 100
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/aroon.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 25 | Lookback period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::AROON;
use quantwave_core::traits::Next;

let mut ind = AROON::new(25);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import AROON

ind = AROON(25)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_aroon_indicator(series: pl.Series) -> pl.Series:
    ind = qw.AROON(25)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_aroon_indicator, return_dtype=pl.Float64).alias("aroon_indicator")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `25` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.investopedia.com/terms/a/aroon.asp

**Implementation**: `quantwave-core/src/indicators/momentum.rs` (`AROON` / `AROON_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/aroon.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
