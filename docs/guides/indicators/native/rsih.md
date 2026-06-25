# RSIH

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">rsi</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">high-pass</span> <span class="kw-badge">cycle</span></div>

RSI enhanced with Hann windowing for superior smoothing and zero-centering.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **RSIH** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only rsih` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/rsih.rs`.*

## Description

RSI enhanced with Hann windowing for superior smoothing and zero-centering.

Use to measure momentum exclusively on the cyclical (high-pass filtered) component of price, eliminating the trend bias that makes standard RSI drift.

RSIH applies RSI computation to the high-pass filtered price rather than raw price. By removing the trend component first, the RSI calculation operates only on the cyclical content of the market, producing an oscillator that is centered around zero regardless of the prevailing trend direction.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/rsih.rs`):

\[
CU = \sum_{n=1}^L (1 - \cos\left(\frac{2\pi n}{L+1}\right)) \cdot \max(0, Close_{t-n+1} - Close_{t-n})
\]
\[
CD = \sum_{n=1}^L (1 - \cos\left(\frac{2\pi n}{L+1}\right)) \cdot \max(0, Close_{t-n} - Close_{t-n+1})
\]
\[
RSIH = \frac{CU - CD}{CU + CD}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/rsih.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 14 | RSI length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::RSIH;
use quantwave_core::traits::Next;

let mut ind = RSIH::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import RSIH

ind = RSIH(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_rsih(series: pl.Series) -> pl.Series:
    ind = qw.RSIH(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_rsih, return_dtype=pl.Float64).alias("rsih")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202022.html

**Implementation**: `quantwave-core/src/indicators/rsih.rs` (`RSIH` / `RSIH_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/rsih.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
