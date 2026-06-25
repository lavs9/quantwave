# Money Flow Index (MFI)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">momentum</span> <span class="kw-badge">volume</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">classic</span></div>

A technical oscillator that uses price and volume data for identifying overbought or oversold signals.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Money Flow Index (MFI)** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only money_flow_index_mfi` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/momentum.rs`.*

## Description

A technical oscillator that uses price and volume data for identifying overbought or oversold signals.

Use as a volume-weighted RSI. Divergences between MFI and price can signal potential reversals, especially when the MFI is in extreme territory (>80 or <20).

The Money Flow Index (MFI) is a momentum indicator that measures the inflow and outflow of money into an asset over a specific period of time. It is related to the RSI but incorporates volume, whereas the RSI only considers price. — Investopedia

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/momentum.rs`):

\[
\text{Money Ratio} = \frac{\text{Positive Money Flow}}{\text{Negative Money Flow}} \\ MFI = 100 - \frac{100}{1 + \text{Money Ratio}}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/mfi.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `timeperiod` | 14 | Lookback period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::MFI;
use quantwave_core::traits::Next;

let mut ind = MFI::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import MFI

ind = MFI(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_money_flow_index_mfi(series: pl.Series) -> pl.Series:
    ind = qw.MFI(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_money_flow_index_mfi, return_dtype=pl.Float64).alias("money_flow_index_mfi")
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

**Primary Source**: https://www.investopedia.com/terms/m/mfi.asp

**Implementation**: `quantwave-core/src/indicators/momentum.rs` (`MFI` / `MFI_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/mfi.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
