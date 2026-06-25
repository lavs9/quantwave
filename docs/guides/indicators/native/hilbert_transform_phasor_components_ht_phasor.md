# Hilbert Transform - Phasor Components (HT_PHASOR)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">cycle</span> <span class="kw-badge">hilbert</span> <span class="kw-badge">phasor</span> <span class="kw-badge">dsp</span></div>

Outputs the In-Phase and Quadrature components of the signal, which are used to calculate phase and amplitude.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Hilbert Transform - Phasor Components (HT_PHASOR)** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only hilbert_transform_phasor_components_ht_phasor` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/cycle.rs`.*

## Description

Outputs the In-Phase and Quadrature components of the signal, which are used to calculate phase and amplitude.

Use as building blocks for custom DSP indicators. The In-Phase component is the signal itself, while the Quadrature component is shifted by 90 degrees.

The Phasor components (In-Phase and Quadrature) are the fundamental outputs of the Hilbert Transform. They allow for the decomposition of a signal into its vector representation, which is essential for advanced cycle analysis and the creation of lag-free filters. — Rocket Science for Traders

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/cycle.rs`):

\[
\text{Result} = (InPhase, Quadrature)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ht_phasor.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::HT_PHASOR;
use quantwave_core::traits::Next;

let mut ind = HT_PHASOR::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import HT_PHASOR

ind = HT_PHASOR(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_hilbert_transform_phasor_components_ht_phasor(series: pl.Series) -> pl.Series:
    ind = qw.HT_PHASOR(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_hilbert_transform_phasor_components_ht_phasor, return_dtype=pl.Float64).alias("hilbert_transform_phasor_components_ht_phasor")
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

**Primary Source**: https://www.tradingview.com/support/solutions/43000502012-hilbert-transform-phasor-components-ht-phasor/

**Implementation**: `quantwave-core/src/indicators/cycle.rs` (`HT_PHASOR` / `HT_PHASOR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ht_phasor.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
