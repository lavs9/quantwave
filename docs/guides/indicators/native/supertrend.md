# SuperTrend

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">atr</span> <span class="kw-badge">stop-loss</span> <span class="kw-badge">classic</span> <span class="kw-badge">breakout</span></div>

Trend-following indicator that combines ATR for volatility bands to identify the primary market direction.

## Visual Example

![SuperTrend — annotated preview mapping to core implementation](../../../assets/indicator-previews/supertrend.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Trend-following indicator that combines ATR for volatility bands to identify the primary market direction.

Use as a primary trend-following indicator and dynamic stop-loss. A SuperTrend flip from bearish to bullish (or vice versa) provides a clear, rule-based entry and exit signal.

SuperTrend computes upper and lower ATR-based bands around the midpoint of each bar. The active line flips from upper to lower (and vice versa) only when price closes beyond the band, providing a clean directional bias and a trailing stop level in one indicator. — TradingView Community

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/supertrend.rs`):

\[
\text{SuperTrend} = \begin{cases}
\text{LowerBand} & \text{if trend is up} \\
\text{UpperBand} & \text{if trend is down}
\end{cases}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/supertrend_10_3.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 10 | ATR length |
| `multiplier` | 3.0 | ATR multiplier |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::SuperTrend;
use quantwave_core::traits::Next;

let mut ind = SuperTrend::new(10);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import SuperTrend

ind = SuperTrend(10)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_supertrend(series: pl.Series) -> pl.Series:
    ind = qw.SuperTrend(10)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_supertrend, return_dtype=pl.Float64).alias("supertrend")
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

**Primary Source**: https://www.tradingview.com/script/7zF0a4f8-SuperTrend-by-Mobius/

**Implementation**: `quantwave-core/src/indicators/supertrend.rs` (`SuperTrend` / `SUPERTREND_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/supertrend_10_3.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
