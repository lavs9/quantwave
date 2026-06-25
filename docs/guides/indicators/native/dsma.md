# DSMA

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">dominant-cycle</span></div>

Deviation Scaled Moving Average adapts to price variations using standard deviation scaled oscillators.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **DSMA** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only dsma` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/dsma.rs`.*

## Description

Deviation Scaled Moving Average adapts to price variations using standard deviation scaled oscillators.

Use as a highly adaptive moving average that tracks price closely during trends and large moves but provides heavy filtering during consolidation. Ideal for trend-following entries and trailing stops.

In 'The Deviation-Scaled Moving Average' (2018), Ehlers introduces an adaptive EMA where the alpha (smoothing factor) is dynamically adjusted based on a deviation-scaled oscillator. By scaling the SuperSmoother-filtered momentum by its RMS, the indicator becomes reactive to significant price deviations while remaining smooth during low-volatility periods.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/dsma.rs`):

\[
Zeros = Close - Close_{t-2}
\]
\[
Filt = c_1 \frac{Zeros + Zeros_{t-1}}{2} + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]
\[
RMS = \sqrt{\frac{1}{P} \sum_{i=0}^{P-1} Filt_{t-i}^2}
\]
\[
\alpha = \min\left(1.0, \left| \frac{Filt}{RMS} \right| \frac{5}{P}\right)
\]
\[
DSMA = \alpha \cdot Close + (1 - \alpha) \cdot DSMA_{t-1}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/dsma.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 40 | Critical period for smoothing and RMS calculation |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::DSMA;
use quantwave_core::traits::Next;

let mut ind = DSMA::new(40);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import DSMA

ind = DSMA(40)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_dsma(series: pl.Series) -> pl.Series:
    ind = qw.DSMA(40)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_dsma, return_dtype=pl.Float64).alias("dsma")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/DEVIATION%20SCALED%20MOVING%20AVERAGE.pdf

**Implementation**: `quantwave-core/src/indicators/dsma.rs` (`DSMA` / `DSMA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/dsma.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
