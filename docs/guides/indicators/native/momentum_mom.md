# Momentum (MOM)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">trend</span></div>

A simple indicator that measures the amount that a security's price has changed over a given span of time.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Momentum (MOM)** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only momentum_mom` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/momentum.rs`.*

## Description

A simple indicator that measures the amount that a security's price has changed over a given span of time.

Use to measure the velocity of price changes. Positive values indicate an uptrend, while negative values indicate a downtrend.

Momentum is one of the most basic and powerful concepts in technical analysis. It measures the rate of change of an asset's price, providing a clear indication of trend strength and potential exhaustion before the actual price reversal occurs. — StockCharts ChartSchool

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/momentum.rs`):

\[
MOM = Price_t - Price_{t-n}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/mom.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 10 | Lookback period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::MOM;
use quantwave_core::traits::Next;

let mut ind = MOM::new(10);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import MOM

ind = MOM(10)
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
        pl.col("close").ta.mom("close", 10).alias("momentum_mom")
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

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend.md)

## Sources & References

**Primary Source**: https://www.investopedia.com/terms/m/momentum.asp

**Implementation**: `quantwave-core/src/indicators/momentum.rs` (`MOM` / `MOM_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/mom.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
