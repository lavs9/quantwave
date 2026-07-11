# Anchored VWAP

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">volume</span> <span class="kw-badge">classic</span> <span class="kw-badge">support-resistance</span></div>

Volume Weighted Average Price anchored to a specific starting point.

## Visual Example

![Anchored VWAP — annotated preview mapping to core implementation](../../../assets/indicator-previews/anchored_vwap.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Volume Weighted Average Price anchored to a specific starting point.

Use as an intraday fair value benchmark. Institutional traders buy below VWAP and sell above it; breakouts above VWAP on heavy volume signal bullish institutional interest.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Volume Weighted Average Price calculates the average price weighted by volume transacted at each level throughout the trading session. It serves as the primary execution benchmark for institutional orders — TWAP and VWAP algorithms are the two most common order execution strategies in equity markets. — Investopedia

**Typical applications:**

- Trend filter or signal line for systematic entries
- Default lookback `N` — tune per asset volatility
- Cross with faster oscillator for entry timing
- Streaming and Polars paths are bit-identical for production parity

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

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

**Primary Source**: https://www.investopedia.com/terms/v/vwap.asp

**Implementation**: `quantwave-core/src/indicators/vwap.rs` (`VWAP` / `VWAP_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/vwap.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
