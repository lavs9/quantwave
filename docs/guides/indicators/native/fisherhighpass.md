# FisherHighPass

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">high-pass</span> <span class="kw-badge">momentum</span></div>

Fisher Transform applied to normalized HighPass filtered prices.

## Visual Example

![FisherHighPass — annotated preview mapping to core implementation](../../../assets/indicator-previews/fisherhighpass.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The FisherHighPass indicator is a technical analysis tool that fisher transform applied to normalized highpass filtered prices.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Use to isolate high-frequency momentum from the cyclical component of price after trend removal. Provides a purer momentum signal than standard Fisher Transform applied to raw price.

FisherHighPass applies the Fisher Transform to the high-pass filtered price rather than raw price. By first removing the low-frequency trend component with a high-pass filter, the resulting Fisher output captures only the cycle-domain momentum, producing an oscillator that is unaffected by the prevailing trend direction.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/fisher_high_pass.rs`):

\[
HP = \text{HighPass}(Price, hp\_len)
\]
\[
N = 2 \cdot \frac{HP - Low(HP, norm\_len)}{High(HP, norm\_len) - Low(HP, norm\_len)} - 1
\]
\[
S = \frac{N + N_{t-1} + N_{t-2}}{3}
\]
\[
Fisher = 0.5 \cdot \ln\left(\frac{1+S}{1-S}\right)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/fisher_high_pass.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `hp_len` | 20 | HighPass filter length |
| `norm_len` | 20 | Normalization lookback period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::FISHER_HIGH_PASS;
use quantwave_core::traits::Next;

let mut ind = FISHER_HIGH_PASS::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import FISHER_HIGH_PASS

ind = FISHER_HIGH_PASS(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_fisherhighpass(series: pl.Series) -> pl.Series:
    ind = qw.FISHER_HIGH_PASS(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_fisherhighpass, return_dtype=pl.Float64).alias("fisherhighpass")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/InferringTradingStrategies.pdf

**Implementation**: `quantwave-core/src/indicators/fisher_high_pass.rs` (`FISHER_HIGH_PASS` / `FISHER_HIGH_PASS_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/fisher_high_pass.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
