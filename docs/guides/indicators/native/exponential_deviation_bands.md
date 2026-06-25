# Exponential Deviation Bands

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">bands</span> <span class="kw-badge">volatility</span> <span class="kw-badge">exponential-deviation</span> <span class="kw-badge">trend</span></div>

A price band indicator based on exponential deviation that applies more weight to recent data and generates fewer breakouts than standard deviation bands.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Exponential Deviation Bands** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only exponential_deviation_bands` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/exp_dev_bands.rs`.*

## Description

A price band indicator based on exponential deviation that applies more weight to recent data and generates fewer breakouts than standard deviation bands.

Use as a tool to identify trends and potential trend reversals. Prices consistently above the upper band indicate a strong uptrend, while prices below the lower band indicate a strong downtrend.

Introduced by Vitali Apirine, Exponential Deviation Bands use an EMA of the absolute deviation from a base moving average (SMA or EMA) to create volatility bands. This approach is more responsive to recent price changes than standard deviation-based Bollinger Bands.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/exp_dev_bands.rs`):

\[
BaseMA = \text{SMA or EMA}(Price, n) \\
Deviation = |BaseMA - Price| \\
ExpDev = EMA(Deviation, n) \\
Upper = BaseMA + ExpDev \times multiplier \\
Lower = BaseMA - ExpDev \times multiplier
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/exp_dev_bands_20_2.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 20 | Period for the base moving average and exponential deviation. |
| `dev_mult` | 2.0 | Multiplier for the exponential deviation. |
| `use_sma` | false | Whether to use SMA (true) or EMA (false) as the base moving average. |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ExpDevBands;
use quantwave_core::traits::Next;

let mut ind = ExpDevBands::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ExpDevBands

ind = ExpDevBands(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_exponential_deviation_bands(series: pl.Series) -> pl.Series:
    ind = qw.ExpDevBands(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_exponential_deviation_bands, return_dtype=pl.Float64).alias("exponential_deviation_bands")
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

**Primary Source**: Technical Analysis of Stocks & Commodities, July 2019

**Implementation**: `quantwave-core/src/indicators/exp_dev_bands.rs` (`ExpDevBands` / `EXPDEVBANDS_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/exp_dev_bands_20_2.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
