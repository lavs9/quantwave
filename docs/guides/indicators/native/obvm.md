# OBVM

<div class="indicator-meta"><span class="category-badge">Volume Indicators</span> <span class="kw-badge">volume</span> <span class="kw-badge">obv</span> <span class="kw-badge">momentum</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">apirine</span></div>

On-Balance Volume Modified - a smoothed version of OBV with an additional signal line.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **OBVM** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only obvm` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/obvm.rs`.*

## Description

On-Balance Volume Modified - a smoothed version of OBV with an additional signal line.

Used to identify divergences between price and volume flow, and to generate signals via crossovers with its signal line. Values typically follow the trend of buying and selling pressure.

While originally developed by Joe Granville, this modified version by Vitali Apirine applies exponential smoothing to the OBV values to filter out noise and adds a signal line for better trend identification and crossover signals. It provides a clearer picture of volume-price relationships by reducing high-frequency fluctuations. — TASC April 2020

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/obvm.rs`):

\begin{aligned}
TP &= \frac{High + Low + Close}{3} \\
OBV_t &= OBV_{t-1} + \begin{cases} Volume, & \text{if } TP_t > TP_{t-1} \\ -Volume, & \text{if } TP_t < TP_{t-1} \\ 0, & \text{otherwise} \end{cases} \\
OBVM &= EMA(OBV, Period_1) \\
Signal &= EMA(OBVM, Period_2)
\end{aligned}

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/obvm.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `obvm_period` | 7 | EMA period for smoothing OBV |
| `signal_period` | 10 | EMA period for the signal line |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::Obvm;
use quantwave_core::traits::Next;

let mut ind = Obvm::new(7);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import Obvm

ind = Obvm(7)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_obvm(series: pl.Series) -> pl.Series:
    ind = qw.Obvm(7)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_obvm, return_dtype=pl.Float64).alias("obvm")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `7` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://www.traders.com/Documentation/FEEDbk_docs/2020/04/TradersTips.html

**Implementation**: `quantwave-core/src/indicators/obvm.rs` (`Obvm` / `OBVM_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/obvm.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
