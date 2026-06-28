# Projected Moving Average

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">prediction</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">zero-lag</span></div>

A lag-compensated moving average that uses linear regression slope to project the average forward.

## Visual Example

![Projected Moving Average — annotated preview mapping to core implementation](../../../assets/indicator-previews/projected_moving_average.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Projected Moving Average indicator is a technical analysis tool that a lag-compensated moving average that uses linear regression slope to project the average forward.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Use as a predictive moving average that uses linear regression projection to anticipate where price will be rather than where it has been, reducing effective lag.

The Projected Moving Average uses linear regression over the lookback window to project the best-fit line forward to the current bar. This predictive approach shifts the MA output toward the leading edge of price movement, achieving reduced lag compared to conventional MAs of the same period.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/pma.rs`):

\[
Slope = -\frac{n \sum xy - \sum x \sum y}{n \sum x^2 - (\sum x)^2}
\]
\[
PMA = SMA + Slope \cdot \frac{n}{2}
\]
\[
Predict = PMA + 0.5 \cdot (Slope - Slope_{t-2}) \cdot n
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/pma.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 20 | Calculation length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::PROJECTED_MOVING_AVERAGE;
use quantwave_core::traits::Next;

let mut ind = PROJECTED_MOVING_AVERAGE::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import PROJECTED_MOVING_AVERAGE

ind = PROJECTED_MOVING_AVERAGE(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_projected_moving_average(series: pl.Series) -> pl.Series:
    ind = qw.PROJECTED_MOVING_AVERAGE(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_projected_moving_average, return_dtype=pl.Float64).alias("projected_moving_average")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Recursive DSP filters require a warm-up period; first N bars may be unstable or raw-pass-through.
- Designed for cyclic/mean-reverting regimes; trending markets can produce lag or drift.
- Parameter `period` (or equivalent) controls cutoff — too small adds noise, too large adds lag.
- Prefer chaining with other Ehlers tools (Roofing Filter, SuperSmoother) on noisy inputs.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; suitable for live streaming and batch feature pipelines.

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
- [Ehlers DSP guide](../ehlers/index.md)
- [Cyber Cycle](cyber_cycle.md)
- [SuperSmoother](supersmoother.md)

## Sources & References

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20MARCH%202025.html

**Implementation**: `quantwave-core/src/indicators/pma.rs` (`PROJECTED_MOVING_AVERAGE` / `PROJECTED_MOVING_AVERAGE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/pma.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
