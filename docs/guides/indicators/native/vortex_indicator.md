# Vortex Indicator

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">breakout</span></div>

The Vortex Indicator helps identify the start of a new trend or the continuation of an existing one.

## Visual Example

![Vortex Indicator — annotated preview mapping to core implementation](../../../assets/indicator-previews/vortex_indicator.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Vortex Indicator helps identify the start of a new trend or the continuation of an existing one.

Use to detect the start of new trends. A Vortex Indicator crossover (VI+ crossing above VI-) signals the beginning of an uptrend; the reverse signals a downtrend.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

The Vortex Indicator, developed by Etienne Botes and Douglas Siepman (2010), is inspired by the vortex flow of water discovered by Viktor Schauberger. VI+ measures upward movement relative to the prior bar low; VI- measures downward movement relative to the prior bar high. Normalized by ATR, they produce two oscillating lines whose crossovers signal trend changes. — Technical Analysis of Stocks and Commodities, 2010

**Typical applications:**

- Fade extremes in ranges; trade with trend on recoveries from oversold/overbought
- Use divergences as early warning — confirm with structure or volume
- Parameter default `14` — shorten for sensitivity, lengthen for stability
- Drop into `build_feature_matrix()` for ML research

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/vortex.rs`):

\[
VI+ = \frac{\sum VM+}{\sum TR} \\ VI- = \frac{\sum VM-}{\sum TR}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/vortex.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::VORTEX;
use quantwave_core::traits::Next;

let mut ind = VORTEX::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import VORTEX

ind = VORTEX(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_vortex_indicator(series: pl.Series) -> pl.Series:
    ind = qw.VORTEX(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_vortex_indicator, return_dtype=pl.Float64).alias("vortex_indicator")
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

**Primary Source**: https://www.investopedia.com/terms/v/vortex-indicator-vi.asp

**Implementation**: `quantwave-core/src/indicators/vortex.rs` (`VORTEX` / `VORTEX_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/vortex.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
