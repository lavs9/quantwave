# Average True Range

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">atr</span> <span class="kw-badge">classic</span> <span class="kw-badge">range</span></div>

ATR represents the average of true ranges over a specified period.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Average True Range** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only average_true_range` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/volatility.rs`.*

## Description

ATR represents the average of true ranges over a specified period.

Use as the foundational volatility module providing ATR, True Range, and related volatility measures used by higher-level indicators such as SuperTrend and Keltner Channels.

Average True Range, developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), measures the average of the true range over N bars. True Range accounts for overnight gaps by taking the maximum of: current high minus low, current high minus prior close, prior close minus current low. It remains the industry standard raw volatility measure.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/volatility.rs`):

\[
ATR = \frac{ATR_{t-1} \times (n-1) + TR_t}{n}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/atr.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Smoothing period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ATR;
use quantwave_core::traits::Next;

let mut ind = ATR::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ATR

ind = ATR(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_average_true_range(series: pl.Series) -> pl.Series:
    ind = qw.ATR(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_average_true_range, return_dtype=pl.Float64).alias("average_true_range")
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

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend.md)

## Sources & References

**Primary Source**: https://www.investopedia.com/terms/a/atr.asp

**Implementation**: `quantwave-core/src/indicators/volatility.rs` (`ATR` / `ATR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/atr.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
