# On-Balance Volume (OBV)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volume</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">accumulation</span> <span class="kw-badge">distribution</span></div>

A momentum indicator that uses volume flow to predict changes in stock price.

## Visual Example

![On-Balance Volume (OBV) — annotated preview mapping to core implementation](../../../assets/indicator-previews/on_balance_volume_obv.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A momentum indicator that uses volume flow to predict changes in stock price.

Use to identify accumulation by institutions. When price is flat but OBV is rising, a breakout to the upside is likely. Conversely, when price is flat but OBV is falling, a breakdown is likely.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Introduced by Joe Granville in his 1963 book 'Granville's New Key to Stock Market Profits', OBV is one of the oldest and most respected volume indicators. It operates on the principle that volume precedes price, and that institutional money flow leaves a detectable trail in the volume data before the price move occurs. — StockCharts ChartSchool

**Typical applications:**

- Fade extremes in ranges; trade with trend on recoveries from oversold/overbought
- Use divergences as early warning — confirm with structure or volume
- Parameter default `N` — shorten for sensitivity, lengthen for stability
- Drop into `build_feature_matrix()` for ML research

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/volume.rs`):

\[
OBV_t = OBV_{t-1} + \begin{cases} Volume & \text{if } Close_t > Close_{t-1} \\ 0 & \text{if } Close_t = Close_{t-1} \\ -Volume & \text{if } Close_t < Close_{t-1} \end{cases}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/obv.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::OBV;
use quantwave_core::traits::Next;

let mut ind = OBV::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import OBV

ind = OBV(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_on_balance_volume_obv(series: pl.Series) -> pl.Series:
    ind = qw.OBV(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_on_balance_volume_obv, return_dtype=pl.Float64).alias("on_balance_volume_obv")
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

**Primary Source**: https://www.investopedia.com/terms/o/onbalancevolume.asp

**Implementation**: `quantwave-core/src/indicators/volume.rs` (`OBV` / `OBV_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/obv.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
