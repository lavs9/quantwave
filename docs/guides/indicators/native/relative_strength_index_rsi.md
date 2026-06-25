# Relative Strength Index (RSI)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">overbought</span> <span class="kw-badge">oversold</span> <span class="kw-badge">classic</span></div>

A momentum oscillator that measures the speed and change of price movements.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Relative Strength Index (RSI)** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only relative_strength_index_rsi` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/momentum.rs`.*

## Description

A momentum oscillator that measures the speed and change of price movements.

Use to identify overbought (>70) and oversold (<30) conditions. RSI divergences against price often signal impending trend reversals.

Developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), the RSI compares the magnitude of recent gains to recent losses to determine overbought and oversold conditions of an asset. It remains the most widely used momentum oscillator in modern technical analysis.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/momentum.rs`):

\[
RS = \frac{Average Gain}{Average Loss} \\ RSI = 100 - \frac{100}{1 + RS}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/rsi.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 14 | Lookback period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::RSI;
use quantwave_core::traits::Next;

let mut ind = RSI::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import RSI

ind = RSI(14)
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
        pl.col("close").ta.rsi("close", 14).alias("relative_strength_index_rsi")
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

**Primary Source**: https://www.investopedia.com/terms/r/rsi.asp

**Implementation**: `quantwave-core/src/indicators/momentum.rs` (`RSI` / `RSI_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/rsi.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
