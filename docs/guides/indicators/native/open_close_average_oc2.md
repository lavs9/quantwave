# Open-Close Average (OC2)

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">price-transform</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">dsp</span></div>

A simple average of the Open and Close prices.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Open-Close Average (OC2)** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only open_close_average_oc2` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/price_transform.rs`.*

## Description

A simple average of the Open and Close prices.

Use to reduce noise in technical indicators. Based on John Ehlers' recent research, averaging the open and close can significantly improve signal-to-noise ratios in DSP-based indicators.

In his 2023 paper 'Every Little Bit Helps', John Ehlers demonstrates that using the average of the Open and Close as an input can enhance the performance of various filters and oscillators by providing a cleaner signal with reduced aliasing. — John Ehlers

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/price_transform.rs`):

\[
OC2 = \frac{Open + Close}{2}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/oc2.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| (none) | — | No tunable parameters for this detector. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::OC2;
use quantwave_core::traits::Next;

let mut ind = OC2::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import OC2

ind = OC2(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_open_close_average_oc2(series: pl.Series) -> pl.Series:
    ind = qw.OC2(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_open_close_average_oc2, return_dtype=pl.Float64).alias("open_close_average_oc2")
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

**Primary Source**: Every Little Bit Helps (John Ehlers, 2023)

**Implementation**: `quantwave-core/src/indicators/price_transform.rs` (`OC2` / `OC2_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/oc2.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
