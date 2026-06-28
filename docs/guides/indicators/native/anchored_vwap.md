# Anchored VWAP

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">volume</span> <span class="kw-badge">classic</span> <span class="kw-badge">support-resistance</span></div>

Volume Weighted Average Price anchored to a specific starting point.

## Visual Example

![Anchored VWAP — annotated preview mapping to core implementation](../../../assets/indicator-previews/anchored_vwap.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Anchored VWAP indicator is a technical analysis tool that volume weighted average price anchored to a specific starting point.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

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

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Output starts from bar 1; warmup_bars marks period-stability, not NaN. |
| period > len | Cumulative sum continues; period only affects smoothed variants. |
| NaN inputs | NaN inputs may produce NaN or skip depending on indicator. |
| Invalid params | Invalid params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

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
