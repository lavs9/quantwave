# Cyber Cycle

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

An oscillator introduced by John Ehlers that models the cyclical component of a time series using FIR smoothing.

## Visual Example

![Cyber Cycle — annotated preview mapping to core implementation](../../../assets/indicator-previews/cyber_cycle.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

An oscillator introduced by John Ehlers that models the cyclical component of a time series using FIR smoothing.

Use as a high-resolution short-term cycle oscillator to time entries and exits around cycle turns. Pair with a trend classifier to suppress signals in trending conditions.

Ehlers introduces the Cyber Cycle in Cybernetic Analysis (2004) as a bandpass-like filter isolating the short-term cyclical component. The trigger line is the Cyber Cycle delayed by one bar, creating a clean crossover signal without derivative noise.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/cyber_cycle.rs`):

\[
\alpha = \frac{2}{\text{Length} + 1}
\]
\[
\text{Smooth} = \frac{X_t + 2X_{t-1} + 2X_{t-2} + X_{t-3}}{6}
\]
\[
CC_t = \left(1 - \frac{\alpha}{2}\right)^2 (\text{Smooth}_t - 2\text{Smooth}_{t-1} + \text{Smooth}_{t-2}) + 2(1 - \alpha)CC_{t-1} - (1 - \alpha)^2 CC_{t-2}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/cyber_cycle.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 14 | Alpha smoothing length parameter |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::CYBER_CYCLE;
use quantwave_core::traits::Next;

let mut ind = CYBER_CYCLE::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import CYBER_CYCLE

ind = CYBER_CYCLE(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_cyber_cycle(series: pl.Series) -> pl.Series:
    ind = qw.CYBER_CYCLE(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_cyber_cycle, return_dtype=pl.Float64).alias("cyber_cycle")
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

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Ehlers DSP guide](../ehlers/index.md)
- [Cyber Cycle](cyber_cycle.md)
- [SuperSmoother](supersmoother.md)

## Sources & References

**Primary Source**: Cybernetic Analysis for Stocks and Futures, John Ehlers, 2004, Chapter 4

**Implementation**: `quantwave-core/src/indicators/cyber_cycle.rs` (`CYBER_CYCLE` / `CYBER_CYCLE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/cyber_cycle.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
