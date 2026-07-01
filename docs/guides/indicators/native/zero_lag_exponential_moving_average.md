# Zero Lag Exponential Moving Average

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">low-lag</span> <span class="kw-badge">ema</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

ZLEMA attempts to eliminate the inherent lag associated with moving averages.

## Visual Example

![Zero Lag Exponential Moving Average — annotated preview mapping to core implementation](../../../assets/indicator-previews/zero_lag_exponential_moving_average.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

ZLEMA attempts to eliminate the inherent lag associated with moving averages.

Use to reduce the lag of a standard EMA by approximately two thirds. Drop-in replacement for EMA in trend-following systems where responsiveness is more important than smoothness.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Patrick Mulloy introduced Triple EMA in Technical Analysis of Stocks and Commodities (1994) as a practical lag-reduction technique. TEMA = 3*EMA - 3*EMA(EMA) + EMA(EMA(EMA)), subtracting out two orders of the EMA lag while preserving most of the noise reduction.

**Typical applications:**

- Trend filter or signal line for systematic entries
- Default lookback `14` — tune per asset volatility
- Cross with faster oscillator for entry timing
- Streaming and Polars paths are bit-identical for production parity

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/tema.rs`):

\[
ZLEMA = EMA(Price + (Price - Price_{t - (period - 1)/2}))
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/zlema.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Smoothing period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ZLEMA;
use quantwave_core::traits::Next;

let mut ind = ZLEMA::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ZLEMA

ind = ZLEMA(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_zero_lag_exponential_moving_average(series: pl.Series) -> pl.Series:
    ind = qw.ZLEMA(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_zero_lag_exponential_moving_average, return_dtype=pl.Float64).alias("zero_lag_exponential_moving_average")
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
- [SuperTrend](supertrend/)

## Sources & References

**Primary Source**: https://en.wikipedia.org/wiki/Zero_lag_exponential_moving_average

**Implementation**: `quantwave-core/src/indicators/tema.rs` (`ZLEMA` / `ZLEMA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/zlema.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
