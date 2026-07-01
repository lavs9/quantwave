# Triangular Moving Average (TRIMA)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">classic</span></div>

A double-smoothed simple moving average that gives more weight to the middle of the lookback period.

## Visual Example

![Triangular Moving Average (TRIMA) — annotated preview mapping to core implementation](../../../assets/indicator-previews/triangular_moving_average_trima.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Triangular Moving Average (TRIMA) indicator is a technical analysis tool that a double-smoothed simple moving average that gives more weight to the middle of the lookback period.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Use for extremely smooth trend identification. TRIMA is significantly smoother than a standard SMA but introduces more lag; it is ideal for identifying long-term cycles.

The Triangular Moving Average is an SMA of an SMA. For a period N, it averages the values over N/2 periods twice. This results in a weight distribution that is triangular, peaking at the center of the window, making it very effective at filtering out high-frequency noise. — StockCharts ChartSchool

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/overlap.rs`):

\[
TRIMA = SMA(SMA(Price, n/2), n/2)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/trima.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 30 | Smoothing period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::TRIMA;
use quantwave_core::traits::Next;

let mut ind = TRIMA::new(30);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import TRIMA

ind = TRIMA(30)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").ta.trima(30).alias("triangular_moving_average_trima")
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

**Primary Source**: https://www.tradingview.com/support/solutions/43000591273-triangular-moving-average-tma/

**Implementation**: `quantwave-core/src/indicators/overlap.rs` (`TRIMA` / `TRIMA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/trima.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
