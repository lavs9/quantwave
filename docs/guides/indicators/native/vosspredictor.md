# VossPredictor

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">prediction</span> <span class="kw-badge">cycle</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">filter</span></div>

A predictive filter with negative group delay for band-limited signals.

## Visual Example

![VossPredictor — annotated preview mapping to core implementation](../../../assets/indicator-previews/vosspredictor.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The VossPredictor indicator is a technical analysis tool that a predictive filter with negative group delay for band-limited signals.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Use for multi-bar price prediction based on a bandpass-filtered dominant cycle. More accurate than simple linear extrapolation due to its IIR filter pole placement.

The Voss Predictor is a predictive filter developed by J.F. Voss and adapted by Ehlers in Cycle Analytics for Traders. Its IIR bandpass design inherently extrapolates the filtered signal several bars into the future by virtue of pole placement inside the unit circle, enabling lookahead without buffer access.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/voss_predictor.rs`):

\[
Filt = \text{BandPass}(Price, Period, 0.25)
\]
\[
Order = 3 \cdot Predict
\]
\[
SumC = \sum_{n=0}^{Order-1} \frac{n+1}{Order} Voss_{t-(Order-n)}
\]
\[
Voss = \frac{3 + Order}{2} Filt - SumC
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/voss_predictor.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 20 | Center period of the BandPass filter |
| `predict` | 3 | Number of bars of prediction |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::VOSS_PREDICTOR;
use quantwave_core::traits::Next;

let mut ind = VOSS_PREDICTOR::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import VOSS_PREDICTOR

ind = VOSS_PREDICTOR(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_vosspredictor(series: pl.Series) -> pl.Series:
    ind = qw.VOSS_PREDICTOR(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_vosspredictor, return_dtype=pl.Float64).alias("vosspredictor")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/A%20PEEK%20INTO%20THE%20FUTURE.pdf

**Implementation**: `quantwave-core/src/indicators/voss_predictor.rs` (`VOSS_PREDICTOR` / `VOSS_PREDICTOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/voss_predictor.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
