# TTM Squeeze

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">momentum</span> <span class="kw-badge">breakout</span> <span class="kw-badge">squeeze</span> <span class="kw-badge">classic</span></div>

TTM Squeeze measures the relationship between Bollinger Bands and Keltner Channels to identify volatility consolidations.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **TTM Squeeze** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only ttm_squeeze` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/ttm_squeeze.rs`.*

## Description

TTM Squeeze measures the relationship between Bollinger Bands and Keltner Channels to identify volatility consolidations.

Use to identify periods of compressed volatility (Bollinger Bands inside Keltner Channels) followed by high-energy breakouts. The momentum histogram direction at squeeze release indicates trade direction.

The TTM Squeeze, developed by John Carter, identifies market consolidation by detecting when Bollinger Bands contract inside Keltner Channels — a squeeze condition indicating coiling energy. When the bands expand back outside the Keltner Channels, the squeeze releases and a momentum histogram shows the expected breakout direction. — Mastering the Trade, John Carter

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/ttm_squeeze.rs`):

\[
\text{Squeeze} = BB_{width} < KC_{width}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ttm_squeeze.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `bb_period` | 20 | Bollinger Bands Period |
| `bb_mult` | 2.0 | Bollinger Bands Multiplier |
| `kc_period` | 20 | Keltner Channel Period |
| `kc_mult` | 1.5 | Keltner Channel Multiplier |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::TTM_SQUEEZE;
use quantwave_core::traits::Next;

let mut ind = TTM_SQUEEZE::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import TTM_SQUEEZE

ind = TTM_SQUEEZE(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_ttm_squeeze(series: pl.Series) -> pl.Series:
    ind = qw.TTM_SQUEEZE(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_ttm_squeeze, return_dtype=pl.Float64).alias("ttm_squeeze")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `20` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.investopedia.com/articles/active-trading/110714/intro-ttm-squeeze-indicator.asp

**Implementation**: `quantwave-core/src/indicators/ttm_squeeze.rs` (`TTM_SQUEEZE` / `TTM_SQUEEZE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ttm_squeeze.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
