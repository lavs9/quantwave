# Williams %R

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">overbought</span> <span class="kw-badge">oversold</span> <span class="kw-badge">classic</span></div>

A momentum indicator that measures overbought and oversold levels, similar to a stochastic oscillator.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Williams %R** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only williams_r` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/momentum.rs`.*

## Description

A momentum indicator that measures overbought and oversold levels, similar to a stochastic oscillator.

Use to identify entry and exit points in the market. Readings from 0 to -20 are considered overbought, while readings from -80 to -100 are considered oversold.

Developed by Larry Williams, %R compares the closing price of a stock to the high-low range over a specific period, typically 14 days. It is used to determine when a stock might be overbought or oversold and to identify potential trend reversals. — StockCharts ChartSchool

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/momentum.rs`):

\[
\%R = \frac{\text{Highest High} - \text{Close}}{\text{Highest High} - \text{Lowest Low}} \times -100
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/willr.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 14 | Lookback period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::WILLR;
use quantwave_core::traits::Next;

let mut ind = WILLR::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import WILLR

ind = WILLR(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_williams_r(series: pl.Series) -> pl.Series:
    ind = qw.WILLR(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_williams_r, return_dtype=pl.Float64).alias("williams_r")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `14` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.investopedia.com/terms/w/williamsr.asp

**Implementation**: `quantwave-core/src/indicators/momentum.rs` (`WILLR` / `WILLR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/willr.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
