# Accumulation/Distribution Line (AD)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volume</span> <span class="kw-badge">momentum</span> <span class="kw-badge">classic</span> <span class="kw-badge">accumulation</span> <span class="kw-badge">distribution</span></div>

A volume-based indicator designed to measure the cumulative flow of money into and out of a security.

## Visual Example

![Accumulation/Distribution Line (AD) — annotated preview mapping to core implementation](../../../assets/indicator-previews/accumulation_distribution_line_ad.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

A volume-based indicator designed to measure the cumulative flow of money into and out of a security.

Use to confirm price trends or identify potential reversals through divergences. Rising AD confirms an uptrend; falling AD confirms a downtrend.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Developed by Marc Chaikin, the AD line uses the relationship between price and volume to determine whether a security is being accumulated or distributed. It is calculated by multiplying the Money Flow Multiplier by the period's volume and adding it to a cumulative total. — StockCharts ChartSchool

**Typical applications:**

- Fade extremes in ranges; trade with trend on recoveries from oversold/overbought
- Use divergences as early warning — confirm with structure or volume
- Parameter default `N` — shorten for sensitivity, lengthen for stability
- Drop into `build_feature_matrix()` for ML research

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/volume.rs`):

\[
\text{MFM} = \frac{(Close - Low) - (High - Close)}{High - Low} \\ \text{MFV} = \text{MFM} \times Volume \\ AD_t = AD_{t-1} + \text{MFV}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ad.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::AD;
use quantwave_core::traits::Next;

let mut ind = AD::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import AD

ind = AD(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave  # registers pl.col().ta

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("open").ta.ad("open", "high", "low", "close").alias("accumulation_distribution_line_ad")
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

**Primary Source**: https://www.investopedia.com/terms/a/accumulationdistributioncurve.asp

**Implementation**: `quantwave-core/src/indicators/volume.rs` (`AD` / `AD_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ad.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
