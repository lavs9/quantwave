# EMD

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">decomposition</span> <span class="kw-badge">cycle</span> <span class="kw-badge">spectral</span> <span class="kw-badge">dsp</span></div>

Empirical Mode Decomposition separates cycles from trends using bandpass filtering and identifies market modes via adaptive thresholds.

## Visual Example

![EMD — annotated preview mapping to core implementation](../../../assets/indicator-previews/emd.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Empirical Mode Decomposition separates cycles from trends using bandpass filtering and identifies market modes via adaptive thresholds.

Use to decompose price into Intrinsic Mode Functions to separate cycles of different periods without any a priori period assumption. Useful for multi-timescale analysis.

Part of QuantWave's Ehlers digital signal processing suite. Designed for low-lag cycle and trend work — pair with Roofing Filter or SuperSmoother on noisy inputs.

Empirical Mode Decomposition is a data-driven method developed by Huang et al. (1998) that decomposes a signal into Intrinsic Mode Functions by iteratively sifting local extrema. Unlike Fourier methods, it requires no predetermined basis functions, making it adaptive to non-stationary market data.

**Typical applications:**

- Use for cycle timing in mean-reverting regimes
- Gate with Hurst exponent or ADX before taking cycle signals
- Allow `20`+ bars warm-up for filter state to stabilise
- Chain with Roofing Filter when input is noisy

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/emd.rs`):

\[
\beta = \cos\left(\frac{360}{P}\right), \gamma = \frac{1}{\cos\left(\frac{720\delta}{P}\right)}, \alpha = \gamma - \sqrt{\gamma^2 - 1}
\]
\[
BP = 0.5(1 - \alpha)(Price - Price_{t-2}) + \beta(1 + \alpha)BP_{t-1} - \alpha BP_{t-2}
\]
\[
Mean = \text{SMA}(BP, 2P)
\]
\[
Threshold = \text{Fraction} \cdot \text{SMA}(\text{Peak/Valley}, 50)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/emd.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 20 | Bandpass center period |
| `delta` | 0.5 | Bandwidth half-width |
| `fraction` | 0.1 | Threshold multiplier for peaks/valleys |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::EMD;
use quantwave_core::traits::Next;

let mut ind = EMD::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import EMD

ind = EMD(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_emd(series: pl.Series) -> pl.Series:
    ind = qw.EMD(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_emd, return_dtype=pl.Float64).alias("emd")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EmpiricalModeDecomposition.pdf

**Implementation**: `quantwave-core/src/indicators/emd.rs` (`EMD` / `EMD_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/emd.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
