# Stochastic Distance Oscillator

<div class="indicator-meta"><span class="category-badge">Momentum</span> <span class="kw-badge">momentum</span> <span class="kw-badge">stochastic</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">apirine</span> <span class="kw-badge">trend</span></div>

A momentum indicator based on the classic stochastic oscillator applied to price distances.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Stochastic Distance Oscillator** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only stochastic_distance_oscillator` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/sdo.rs`.*

## Description

A momentum indicator based on the classic stochastic oscillator applied to price distances.

Identify bull and bear trend changes through overbought (+40) and oversold (-40) levels. Suitable for both trending and ranging markets.

The Stochastic Distance Oscillator (SDO) by Vitali Apirine adapts the stochastic formula to measure the current price distance relative to its historical range. By smoothing this relative distance with an EMA, it provides a cleaner momentum signal that identifies potential trend reversals when crossing extreme thresholds.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/sdo.rs`):

\[
Dist = |Price_t - Price_{t-n}|
\]
\[
DVal = \frac{Dist - \min(Dist_{lookback})}{\max(Dist_{lookback}) - \min(Dist_{lookback})}
\]
\[
DDVal = \begin{cases} DVal & \text{if } Price_t > Price_{t-n} \\ -DVal & \text{if } Price_t < Price_{t-n} \\ 0 & \text{otherwise} \end{cases}
\]
\[
SDO = EMA(DDVal, smoothing) \times 100
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/sdo.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `lookback_period` | 200 | Range lookback for stochastic calculation |
| `period` | 12 | Distance calculation period |
| `ema_pds` | 3 | Smoothing EMA period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::SDO;
use quantwave_core::traits::Next;

let mut ind = SDO::new(200);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import SDO

ind = SDO(200)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_stochastic_distance_oscillator(series: pl.Series) -> pl.Series:
    ind = qw.SDO(200)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_stochastic_distance_oscillator, return_dtype=pl.Float64).alias("stochastic_distance_oscillator")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `200` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://traders.com/Documentation/FEEDbk_docs/2023/06/TradersTips.html

**Implementation**: `quantwave-core/src/indicators/sdo.rs` (`SDO` / `SDO_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/sdo.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
