# Bill Williams Alligator

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">classic</span> <span class="kw-badge">williams</span></div>

Trend-following indicator using three delayed smoothed moving averages.

## Visual Example

![Bill Williams Alligator — annotated preview mapping to core implementation](../../../assets/indicator-previews/bill_williams_alligator.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Trend-following indicator using three delayed smoothed moving averages.

Use to identify trend presence and direction. When the three Alligator lines are separated and fanning, the market is trending; when they converge or intertwine, the market is ranging.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Bill Williams introduced the Alligator in Trading Chaos (1995) as three offset SMAs with periods 13, 8, and 5 and offsets of 8, 5, and 3 bars. The three lines represent the Jaw, Teeth, and Lips of the Alligator. When the Alligator is sleeping (lines intertwined) no trade is taken; when it wakes and opens its mouth a trend trade is entered. — StockCharts ChartSchool

**Typical applications:**

- Trend filter or signal line for systematic entries
- Default lookback `N` — tune per asset volatility
- Cross with faster oscillator for entry timing
- Streaming and Polars paths are bit-identical for production parity

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/alligator.rs`):

\[
\text{Jaw} = \text{SMMA}(13, 8), \text{Teeth} = \text{SMMA}(8, 5), \text{Lips} = \text{SMMA}(5, 3)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/alligator.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ALLIGATOR;
use quantwave_core::traits::Next;

let mut ind = ALLIGATOR::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ALLIGATOR

ind = ALLIGATOR(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_bill_williams_alligator(series: pl.Series) -> pl.Series:
    ind = qw.ALLIGATOR(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_bill_williams_alligator, return_dtype=pl.Float64).alias("bill_williams_alligator")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `N` bars may return NaN or partial state per implementation.
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
- [SuperTrend](../supertrend/)

## Sources & References

**Primary Source**: https://chartschool.stockcharts.com/table-of-contents/technical-indicators-and-overlays/alligator

**Implementation**: `quantwave-core/src/indicators/alligator.rs` (`ALLIGATOR` / `ALLIGATOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/alligator.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
