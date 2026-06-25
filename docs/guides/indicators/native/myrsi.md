# MyRSI

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">rsi</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">momentum</span> <span class="kw-badge">smoothing</span></div>

Ehlers' version of RSI that swings between -1 and +1.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **MyRSI** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only myrsi` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/my_rsi.rs`.*

## Description

Ehlers' version of RSI that swings between -1 and +1.

Use as Ehlers smoothed RSI variant that applies cycle-aware filtering to reduce whipsaws while maintaining RSI-style overbought/oversold interpretation.

Ehlers presents a smoothed RSI formulation that applies a Laguerre or SuperSmoother filter to the up/down ratio before computing the RSI index. This reduces the noise and oscillation of standard RSI without significantly increasing lag, producing more reliable overbought and oversold readings.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/my_rsi.rs`):

\[
CU = \sum_{i=0}^{length-1} \max(0, Price_i - Price_{i+1})
\]
\[
CD = \sum_{i=0}^{length-1} \max(0, Price_{i+1} - Price_i)
\]
\[
MyRSI = \frac{CU - CD}{CU + CD}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/my_rsi.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 14 | Smoothing length |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::MY_RSI;
use quantwave_core::traits::Next;

let mut ind = MY_RSI::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import MY_RSI

ind = MY_RSI(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_myrsi(series: pl.Series) -> pl.Series:
    ind = qw.MY_RSI(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_myrsi, return_dtype=pl.Float64).alias("myrsi")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Noise%20Elimination%20Technology.pdf

**Implementation**: `quantwave-core/src/indicators/my_rsi.rs` (`MY_RSI` / `MY_RSI_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/my_rsi.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
