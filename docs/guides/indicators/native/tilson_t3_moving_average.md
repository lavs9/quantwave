# Tilson T3 Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">lag-reduction</span> <span class="kw-badge">classic</span></div>

A smooth, low-lag moving average that uses multiple exponential smoothing.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Tilson T3 Moving Average** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only tilson_t3_moving_average` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/overlap.rs`.*

## Description

A smooth, low-lag moving average that uses multiple exponential smoothing.

Use for trend following in noisy markets. T3 offers a superior balance between lag reduction and smoothness compared to DEMA or TEMA.

Developed by Tim Tilson in 1998, the T3 moving average uses a 'v-factor' (volume factor) to control how much the indicator reacts to price changes. It is essentially a sextuple EMA smoothing process that provides a very smooth curve with remarkably little lag. — Technical Analysis of Stocks & Commodities

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/overlap.rs`):

\[
e1 = EMA(Price, n) \\ e2 = EMA(e1, n) \\ \dots \\ e6 = EMA(e5, n) \\ T3 = c1 \times e6 + c2 \times e5 + c3 \times e4 + c4 \times e3
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/t3.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 5 | Smoothing period |
| `v_factor` | 0.7 | Volume factor (0.0 to 1.0) |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::T3;
use quantwave_core::traits::Next;

let mut ind = T3::new(5);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import T3

ind = T3(5)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_tilson_t3_moving_average(series: pl.Series) -> pl.Series:
    ind = qw.T3(5)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_tilson_t3_moving_average, return_dtype=pl.Float64).alias("tilson_t3_moving_average")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `5` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.tradingview.com/script/667W2a8n-T3-Moving-Average/

**Implementation**: `quantwave-core/src/indicators/overlap.rs` (`T3` / `T3_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/t3.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
