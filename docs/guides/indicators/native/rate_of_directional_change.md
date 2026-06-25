# Rate of Directional Change

<div class="indicator-meta"><span class="category-badge">Volatility</span> <span class="kw-badge">zigzag</span> <span class="kw-badge">whipsaw</span> <span class="kw-badge">momentum</span> <span class="kw-badge">volatility</span> <span class="kw-badge">directional change</span></div>

Measures the frequency of directional changes (zigzag flips) within a moving window to identify whipsaw market conditions.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Rate of Directional Change** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only rate_of_directional_change` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/rodc.rs`.*

## Description

Measures the frequency of directional changes (zigzag flips) within a moving window to identify whipsaw market conditions.

Use to filter out false signals in trend-following strategies. High RODC values indicate a whipsaw environment, while low values suggest a trending market.

RODC tracks the number of alternating up and down zigzag segments within a fixed window. By normalizing this count and smoothing it, the indicator provides a measure of how 'noisy' the price action is. It declines in trending environments and increases during whipsaws. — Richard Poster, TASC March 2024

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/rodc.rs`):

\[
RODC = SMA(100 \times \frac{NumUD}{WindowSize}, SmoothPeriod)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/rodc_30_15_3.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `window_size` | 30 | Lookback window for zigzag calculation |
| `threshold` | 0.0015 | Zigzag reversal threshold (absolute price change) |
| `smooth_period` | 3 | Smoothing period for the resulting rate |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::RODC;
use quantwave_core::traits::Next;

let mut ind = RODC::new(30);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import RODC

ind = RODC(30)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_rate_of_directional_change(series: pl.Series) -> pl.Series:
    ind = qw.RODC(30)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_rate_of_directional_change, return_dtype=pl.Float64).alias("rate_of_directional_change")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `30` bars may return NaN or partial state per implementation.
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

**Primary Source**: TASC March 2024

**Implementation**: `quantwave-core/src/indicators/rodc.rs` (`RODC` / `RODC_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/rodc_30_15_3.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
