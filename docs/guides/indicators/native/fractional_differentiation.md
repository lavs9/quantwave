# Fractional Differentiation

<div class="indicator-meta"><span class="category-badge">ML Features</span> <span class="kw-badge">ml</span> <span class="kw-badge">stationarity</span> <span class="kw-badge">prado</span> <span class="kw-badge">feature-engineering</span> <span class="kw-badge">fractional</span></div>

Applies Prado-style fractional differencing to preserve memory while reducing non-stationarity in price series.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Fractional Differentiation** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only fractional_differentiation` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-28 IST. Core logic in `frac_diff`.*

## Description

Applies Prado-style fractional differencing to preserve memory while reducing non-stationarity in price series.

Use as an ML feature primitive on log-prices or returns. Lower d (e.g. 0.3–0.5) retains more memory than integer differencing while improving stationarity for tree models and neural nets.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`frac_diff`):

\[
w_0 = 1,\quad w_k = -w_{k-1}\frac{d - k + 1}{k},\quad
\tilde{X}_t = \sum_{k=0}^{K} w_k X_{t-k}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/frac_diff.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `d` | 0.4 | Fractional differentiation order (0 = identity, 1 = full integer diff) |
| `threshold` | 1e-5 | Truncate weights when |w_k| falls below this value |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::FracDiff;
use quantwave_core::traits::Next;

let mut ind = FracDiff::new(0.4);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import FracDiff

ind = FracDiff(0.4)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_fractional_differentiation(series: pl.Series) -> pl.Series:
    ind = qw.FracDiff(0.4)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_fractional_differentiation, return_dtype=pl.Float64).alias("fractional_differentiation")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `0.4` bars may return NaN or partial state per implementation.
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
- [SuperTrend](supertrend.md)

## Sources & References

**Primary Source**: https://www.wiley.com/en-us/Advances+in+Financial+Machine+Learning-p-9781119482086

**Implementation**: `quantwave-core/src/indicators/frac_diff` (`FracDiff` / `_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/frac_diff.json`

**Provenance**: Standards bulk upgrade 2026-06-28 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
