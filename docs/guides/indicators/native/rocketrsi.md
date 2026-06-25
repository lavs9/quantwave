# RocketRSI

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">rsi</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">fisher</span> <span class="kw-badge">momentum</span></div>

Highly responsive RSI variant using SuperSmoother and Fisher Transform.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **RocketRSI** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only rocketrsi` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/rocket_rsi.rs`.*

## Description

Highly responsive RSI variant using SuperSmoother and Fisher Transform.

Use for rapid cycle identification and reversal detection. The Fisher Transform converts the RSI distribution into a Gaussian-like distribution with sharp peaks at reversals.

RocketRSI improves upon standard RSI by first smoothing the momentum with a SuperSmoother filter to eliminate high-frequency noise. The resulting RSI is then passed through a Fisher Transform to create clear, actionable signals at cyclical turning points.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/rocket_rsi.rs`):

\[
Mom = Price - Price_{t-(L-1)}
\]
\[
Filt = \text{SuperSmoother}(Mom, SL)
\]
\[
MyRSI = \frac{\sum \max(0, \Delta Filt) - \sum \max(0, -\Delta Filt)}{\sum |\Delta Filt|}
\]
\[
RocketRSI = 0.5 \cdot \ln\left(\frac{1 + MyRSI}{1 - MyRSI}\right)
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/rocket_rsi.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `rsi_length` | 8 | RSI calculation period |
| `smooth_length` | 10 | SuperSmoother filter period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ROCKET_RSI;
use quantwave_core::traits::Next;

let mut ind = ROCKET_RSI::new(8);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ROCKET_RSI

ind = ROCKET_RSI(8)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_rocketrsi(series: pl.Series) -> pl.Series:
    ind = qw.ROCKET_RSI(8)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_rocketrsi, return_dtype=pl.Float64).alias("rocketrsi")
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

**Primary Source**: https://www.traders.com/Documentation/FEEDbk_docs/2018/05/TradersTips.html

**Implementation**: `quantwave-core/src/indicators/rocket_rsi.rs` (`ROCKET_RSI` / `ROCKET_RSI_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/rocket_rsi.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
