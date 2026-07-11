# Commodity Channel Index (CCI)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">classic</span> <span class="kw-badge">mean-reversion</span></div>

A versatile indicator that can be used to identify a new trend or warn of extreme conditions.

## Visual Example

![Commodity Channel Index (CCI) — annotated preview mapping to core implementation](../../../assets/indicator-previews/commodity_channel_index_cci.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A versatile indicator that can be used to identify a new trend or warn of extreme conditions.

Use to identify cyclical turns in commodities or stocks. Readings above +100 imply a strong uptrend, while readings below -100 imply a strong downtrend.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Developed by Donald Lambert in 1980, the CCI measures the current price level relative to an average price level over a given period. CCI is relatively high when prices are far above their average and relatively low when prices are far below their average. — StockCharts ChartSchool

**Typical applications:**

- Fade extremes in ranges; trade with trend on recoveries from oversold/overbought
- Use divergences as early warning — confirm with structure or volume
- Parameter default `14` — shorten for sensitivity, lengthen for stability
- Drop into `build_feature_matrix()` for ML research

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/momentum.rs`):

\[
CCI = \frac{Price - SMA}{0.015 \times \text{Mean Deviation}}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/cci.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 14 | Lookback period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::CCI;
use quantwave_core::traits::Next;

let mut ind = CCI::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import CCI

ind = CCI(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_commodity_channel_index_cci(series: pl.Series) -> pl.Series:
    ind = qw.CCI(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_commodity_channel_index_cci, return_dtype=pl.Float64).alias("commodity_channel_index_cci")
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

**Primary Source**: https://www.investopedia.com/terms/c/commoditychannelindex.asp

**Implementation**: `quantwave-core/src/indicators/momentum.rs` (`CCI` / `CCI_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/cci.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
