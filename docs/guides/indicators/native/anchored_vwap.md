# Anchored VWAP

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">volume</span> <span class="kw-badge">classic</span> <span class="kw-badge">support-resistance</span></div>

Volume Weighted Average Price anchored to a specific starting point.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Anchored VWAP** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only anchored_vwap` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/vwap.rs`.*

## Description

Volume Weighted Average Price anchored to a specific starting point.

Use as an intraday fair value benchmark. Institutional traders buy below VWAP and sell above it; breakouts above VWAP on heavy volume signal bullish institutional interest.

Volume Weighted Average Price calculates the average price weighted by volume transacted at each level throughout the trading session. It serves as the primary execution benchmark for institutional orders — TWAP and VWAP algorithms are the two most common order execution strategies in equity markets. — Investopedia

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/vwap.rs`):

\[
VWAP = \frac{\sum (Price \times Volume)}{\sum Volume}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/vwap.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::VWAP;
use quantwave_core::traits::Next;

let mut ind = VWAP::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import VWAP

ind = VWAP(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_anchored_vwap(series: pl.Series) -> pl.Series:
    ind = qw.VWAP(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_anchored_vwap, return_dtype=pl.Float64).alias("anchored_vwap")
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

**Primary Source**: https://www.investopedia.com/terms/v/vwap.asp

**Implementation**: `quantwave-core/src/indicators/vwap.rs` (`VWAP` / `VWAP_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/vwap.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
