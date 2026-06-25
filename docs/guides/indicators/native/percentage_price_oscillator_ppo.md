# Percentage Price Oscillator (PPO)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">classic</span> <span class="kw-badge">normalization</span></div>

A momentum oscillator that measures the difference between two moving averages as a percentage of the larger moving average.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Percentage Price Oscillator (PPO)** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only percentage_price_oscillator_ppo` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/momentum.rs`.*

## Description

A momentum oscillator that measures the difference between two moving averages as a percentage of the larger moving average.

Use to compare trend momentum across different securities with varying price levels. PPO is the percentage version of MACD.

The Percentage Price Oscillator (PPO) is identical to the MACD, except that it measures the difference between two moving averages as a percentage. This allows for comparison across different stocks regardless of their price, making it a superior tool for relative strength analysis. — StockCharts ChartSchool

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/momentum.rs`):

\[
PPO = \frac{EMA(12) - EMA(26)}{EMA(26)} \times 100
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ppo.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `fastperiod` | 12 | Fast period |
| `slowperiod` | 26 | Slow period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::PPO;
use quantwave_core::traits::Next;

let mut ind = PPO::new(12);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import PPO

ind = PPO(12)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_percentage_price_oscillator_ppo(series: pl.Series) -> pl.Series:
    ind = qw.PPO(12)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_percentage_price_oscillator_ppo, return_dtype=pl.Float64).alias("percentage_price_oscillator_ppo")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `12` bars may return NaN or partial state per implementation.
- Parameter sensitivity: smaller periods increase noise; larger periods increase lag.
- Sudden gaps or bad ticks can distort rolling windows — consider pre-filtering.
- Single-series indicators ignore volume unless otherwise documented.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; streaming and Polars batch paths are bit-identical.

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend.md)

## Sources & References

**Primary Source**: https://www.investopedia.com/terms/p/ppo.asp

**Implementation**: `quantwave-core/src/indicators/momentum.rs` (`PPO` / `PPO_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ppo.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
