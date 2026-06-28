# Ultimate Bands

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">bands</span> <span class="kw-badge">volatility</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">adaptive</span></div>

A Bollinger-style band using UltimateSmoother for the center line and standard deviation of the price-smooth difference for width.

## Visual Example

![Ultimate Bands — annotated preview mapping to core implementation](../../../assets/indicator-previews/ultimate_bands.png)

*Synthetic ideal per library logic. Generated 2026-06-25 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

The Ultimate Bands indicator is a technical analysis tool that a bollinger-style band using ultimatesmoother for the center line and standard deviation of the price-smooth difference for width.

This indicator is primarily used for identifying key market conditions. It provides a robust signal that can be easily integrated into both simple strategies and more complex machine learning feature pipelines. Compared to its alternatives, it offers a distinct balance of responsiveness and stability.

Traders often combine this with other metrics to confirm signals and avoid false positives during sideways market regimes. It remains a standard tool for systematic trading models.

Use as volatility bands that automatically widen during high-energy cycle phases and narrow during quiet phases. Better than fixed-multiple ATR bands in strongly cyclical markets.

Ehlers Ultimate Bands compute upper and lower price envelopes using the RMS amplitude of the dominant cycle rather than a fixed ATR multiple. This makes the bands proportional to the current cycle energy, expanding when the market is actively cycling and contracting when it enters a low-energy consolidation.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.


## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/ultimate_bands.rs`):

\[
Smooth = UltimateSmoother(Close, Length)
\]
\[
SD = \sqrt{\frac{1}{n}\sum_{i=0}^{n-1} (Close_{t-i} - Smooth_{t-i})^2}
\]
\[
Upper = Smooth + NumSDs \times SD
\]
\[
Lower = Smooth - NumSDs \times SD
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ultimate_bands.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 20 | Smoothing and SD period |
| `num_sds` | 1.0 | Standard Deviation multiplier |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ULTIMATE_BANDS;
use quantwave_core::traits::Next;

let mut ind = ULTIMATE_BANDS::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ULTIMATE_BANDS

ind = ULTIMATE_BANDS(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_ultimate_bands(series: pl.Series) -> pl.Series:
    ind = qw.ULTIMATE_BANDS(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_ultimate_bands, return_dtype=pl.Float64).alias("ultimate_bands")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UltimateChannel.pdf

**Implementation**: `quantwave-core/src/indicators/ultimate_bands.rs` (`ULTIMATE_BANDS` / `ULTIMATE_BANDS_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ultimate_bands.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
