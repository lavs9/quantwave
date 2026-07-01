# FisherHighPass

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">high-pass</span> <span class="kw-badge">momentum</span></div>

Fisher Transform applied to normalized HighPass filtered prices.

## Visual Example

![FisherHighPass — annotated preview mapping to core implementation](../../../assets/indicator-previews/fisherhighpass.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Fisher Transform applied to normalized HighPass filtered prices.

Use to isolate high-frequency momentum from the cyclical component of price after trend removal. Provides a purer momentum signal than standard Fisher Transform applied to raw price.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

FisherHighPass applies the Fisher Transform to the high-pass filtered price rather than raw price. By first removing the low-frequency trend component with a high-pass filter, the resulting Fisher output captures only the cycle-domain momentum, producing an oscillator that is unaffected by the prevailing trend direction.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `20`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

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

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
