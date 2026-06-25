# MarketState

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">trend</span> <span class="kw-badge">cycle</span> <span class="kw-badge">regime</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span></div>

Identifies trend vs cycle regimes using Correlation Cycle phase angle.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **MarketState** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only marketstate` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/market_state.rs`.*

## Description

Identifies trend vs cycle regimes using Correlation Cycle phase angle.

Returns 1 for uptrend, -1 for downtrend, and 0 for cycle mode. Use to switch between trend-following and mean-reversion strategies.

In 'Correlation As A Cycle Indicator' (2020), Ehlers defines a Market State variable based on the rate of change of the Correlation Cycle phase angle. When the angle changes slowly (less than 9 degrees per bar), the market is in a trend regime (positive angle for uptrend, negative for downtrend). Rapid angle changes indicate a cycle regime.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/market_state.rs`):

\[
\text{State} = 
\begin{cases} 
1 & \text{if } |\Delta \text{Angle}| < \text{Threshold} \text{ and Angle} \geq 0 \\
-1 & \text{if } |\Delta \text{Angle}| < \text{Threshold} \text{ and Angle} < 0 \\
0 & \text{otherwise}
\end{cases}
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/market_state.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Correlation wavelength |
| `threshold` | 9.0 | Angle rate of change threshold for trend detection |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::MARKET_STATE;
use quantwave_core::traits::Next;

let mut ind = MARKET_STATE::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import MARKET_STATE

ind = MARKET_STATE(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_marketstate(series: pl.Series) -> pl.Series:
    ind = qw.MARKET_STATE(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_marketstate, return_dtype=pl.Float64).alias("marketstate")
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

**Primary Source**: https://www.traders.com/Documentation/FEEDbk_docs/2020/06/TradersTips.html

**Implementation**: `quantwave-core/src/indicators/market_state.rs` (`MARKET_STATE` / `MARKET_STATE_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/market_state.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
