# Zero Lag EC

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">zero-lag</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">ema</span> <span class="kw-badge">smoothing</span></div>

Zero Lag Error Corrected EMA attempts to eliminate lag by adding an error term to the EMA.

## Visual Example

![Zero Lag EC — annotated preview mapping to core implementation](../../../assets/indicator-previews/zero_lag_ec.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Zero Lag Error Corrected EMA attempts to eliminate lag by adding an error term to the EMA.

Use as a near-zero-lag moving average for trend-following systems. The error-correction term removes the lag inherent in the standard EMA without introducing significant overshoot.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

Ehlers introduces the Zero Lag indicator in Cybernetic Analysis as an EMA with an added error-correction term that subtracts the average lag from the output. The resulting EC (Error Corrected) line tracks price with near-zero delay while the ZL-EMA provides a smoothed reference, with crossovers between them providing trade signals.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `20`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/zero_lag.rs`):

\[
\alpha = \frac{2}{Length + 1}
\]
\[
EMA = \alpha \times Close + (1 - \alpha) \times EMA_{t-1}
\]
\[
EC = \alpha \times (EMA + Gain \times (Close - EC_{t-1})) + (1 - \alpha) \times EC_{t-1}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/zero_lag.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 20 | Equivalent SMA length |
| `gain_limit` | 50.0 | Gain limit (divided by 10 for actual gain) |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ZERO_LAG;
use quantwave_core::traits::Next;

let mut ind = ZERO_LAG::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ZERO_LAG

ind = ZERO_LAG(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_zero_lag_ec(series: pl.Series) -> pl.Series:
    ind = qw.ZERO_LAG(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_zero_lag_ec, return_dtype=pl.Float64).alias("zero_lag_ec")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/ZeroLag.pdf

**Implementation**: `quantwave-core/src/indicators/zero_lag.rs` (`ZERO_LAG` / `ZERO_LAG_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/zero_lag.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
