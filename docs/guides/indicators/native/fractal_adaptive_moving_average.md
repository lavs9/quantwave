# Fractal Adaptive Moving Average

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">fractal</span> <span class="kw-badge">smoothing</span></div>

An adaptive moving average that uses the fractal dimension of prices to dynamically change its smoothing constant.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Fractal Adaptive Moving Average** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only fractal_adaptive_moving_average` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/frama.rs`.*

## Description

An adaptive moving average that uses the fractal dimension of prices to dynamically change its smoothing constant.

Use as an adaptive moving average that slows dramatically during consolidation and speeds up during trending phases. Outperforms fixed-period MAs in ranging markets by avoiding false crossovers.

The Fractal Adaptive Moving Average uses the fractal dimension of recent price action to adapt its smoothing constant. During trending markets the fractal dimension approaches 1 (a line) producing a fast-reacting EMA; during ranging markets the dimension approaches 2 (a plane) slowing the average dramatically to filter chop.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/frama.rs`):

\[
D = \frac{\log(N_1 + N_2) - \log(N_3)}{\log(2)}
\]
\[
\alpha = \exp(-4.6 (D - 1))
\]
\[
\text{FRAMA}_t = \alpha P_t + (1 - \alpha) \text{FRAMA}_{t-1}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/frama.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 16 | Length (must be an even number; odd values will be incremented by 1). |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::FRAMA;
use quantwave_core::traits::Next;

let mut ind = FRAMA::new(16);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import FRAMA

ind = FRAMA(16)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_fractal_adaptive_moving_average(series: pl.Series) -> pl.Series:
    ind = qw.FRAMA(16)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_fractal_adaptive_moving_average, return_dtype=pl.Float64).alias("fractal_adaptive_moving_average")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/FRAMA.pdf

**Implementation**: `quantwave-core/src/indicators/frama.rs` (`FRAMA` / `FRAMA_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/frama.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
