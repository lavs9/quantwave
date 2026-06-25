# Bill Williams Fractals

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">pattern</span> <span class="kw-badge">support-resistance</span> <span class="kw-badge">classic</span> <span class="kw-badge">williams</span></div>

Fractals are indicators on candlestick charts that identify reversal points in the market.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Bill Williams Fractals** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only bill_williams_fractals` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/fractals.rs`.*

## Description

Fractals are indicators on candlestick charts that identify reversal points in the market.

Use to mark potential support and resistance levels at local price extremes. Williams Fractals are commonly combined with Alligator lines to filter valid fractal signals.

Bill Williams introduced Fractals in Trading Chaos (1995) as a pattern-recognition tool identifying local price extremes. A bullish fractal is a bar whose low is lower than the two bars on either side; a bearish fractal is a bar whose high is higher than the two bars on either side. Combined with the Alligator indicator, fractals provide entry triggers. — StockCharts ChartSchool

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/fractals.rs`):

\[
\text{Up Fractal} = \text{High} > \text{High}_{t-1, t-2, t+1, t+2}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/fractals.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::FRACTALS;
use quantwave_core::traits::Next;

let mut ind = FRACTALS::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import FRACTALS

ind = FRACTALS(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_bill_williams_fractals(series: pl.Series) -> pl.Series:
    ind = qw.FRACTALS(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_bill_williams_fractals, return_dtype=pl.Float64).alias("bill_williams_fractals")
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

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend.md)

## Sources & References

**Primary Source**: https://www.investopedia.com/terms/f/fractal.asp

**Implementation**: `quantwave-core/src/indicators/fractals.rs` (`FRACTALS` / `FRACTALS_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/fractals.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
