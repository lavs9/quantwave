# MADH

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">volatility</span> <span class="kw-badge">statistics</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">high-pass</span></div>

Moving Average Difference with Hann Windowing: 100 * (Hann(short) - Hann(long)) / Hann(long)

## Visual Example

![MADH — annotated preview mapping to core implementation](../../../assets/indicator-previews/madh.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Moving Average Difference with Hann Windowing: 100 * (Hann(short) - Hann(long)) / Hann(long)

Use to measure the volatility of the cyclical price component only, filtering out trend-driven amplitude changes that inflate standard volatility measures in trending markets.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

MADH applies Mean Absolute Deviation to the high-pass filtered price series rather than raw price. By isolating the cyclical component before measuring dispersion, it quantifies the noise level within the current market cycle rather than conflating it with trend amplitude.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `8`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/madh.rs`):

\[
LongLength = \lfloor ShortLength + DominantCycle / 2 \rfloor
\]
\[
Filt1 = HannWindow(Price, ShortLength)
\]
\[
Filt2 = HannWindow(Price, LongLength)
\]
\[
MADH = 100 \times \frac{Filt1 - Filt2}{Filt2}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/madh.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `short_length` | 8 | Short-term filter length |
| `dominant_cycle` | 27 | Dominant cycle for calculating long length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::MADH;
use quantwave_core::traits::Next;

let mut ind = MADH::new(8);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import MADH

ind = MADH(8)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_madh(series: pl.Series) -> pl.Series:
    ind = qw.MADH(8)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_madh, return_dtype=pl.Float64).alias("madh")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - NOVEMBER 2021.html

**Implementation**: `quantwave-core/src/indicators/madh.rs` (`MADH` / `MADH_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/madh.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
