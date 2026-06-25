# VFI

<div class="indicator-meta"><span class="category-badge">Volume Indicators</span> <span class="kw-badge">volume</span> <span class="kw-badge">vfi</span> <span class="kw-badge">money-flow</span> <span class="kw-badge">katsanos</span> <span class="kw-badge">oscillator</span></div>

Volume Flow Indicator - a volume-based indicator that uses price and volume relative to a cutoff to measure money flow.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **VFI** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only vfi` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/vfi.rs`.*

## Description

Volume Flow Indicator - a volume-based indicator that uses price and volume relative to a cutoff to measure money flow.

Used to identify trend direction and potential reversals. Values above 0 are bullish, below 0 are bearish. Extreme readings and divergences are also significant.

Katsanos' Volume Flow Indicator (VFI) is based on the popular On Balance Volume (OBV) but with three main modifications: it is bounded, it filters out small price changes, and it caps volume extremes. It provides a more balanced view of buying and selling pressure by accounting for price volatility and volume outliers. — TASC June 2004

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/vfi.rs`):

\begin{aligned}
TP &= \frac{H+L+C}{3} \\
Inter &= \ln(TP) - \ln(TP_{t-1}) \\
VInter &= StdDev(Inter, 30) \\
Cutoff &= Coef \cdot VInter \cdot Close \\
Vave &= SMA(Volume, Period)_{t-1} \\
Vmax &= Vave \cdot Vcoef \\
VC &= \min(Volume, Vmax) \\
MF &= TP - TP_{t-1} \\
VCP &= \begin{cases} VC, & \text{if } MF > Cutoff \\ -VC, & \text{if } MF < -Cutoff \\ 0, & \text{otherwise} \end{cases} \\
VFI_{raw} &= \frac{\sum_{i=0}^{Period-1} VCP_{t-i}}{Vave} \\
VFI &= EMA(VFI_{raw}, 3)
\end{aligned}

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/vfi.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 130 | Lookback period for Vave and Summation |
| `coef` | 0.2 | Coefficient for minimal price cut-off |
| `vcoef` | 2.5 | Coefficient for volume cut-off |
| `smoothing_period` | 3 | EMA period for final smoothing |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::Vfi;
use quantwave_core::traits::Next;

let mut ind = Vfi::new(130);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import Vfi

ind = Vfi(130)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_vfi(series: pl.Series) -> pl.Series:
    ind = qw.Vfi(130)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_vfi, return_dtype=pl.Float64).alias("vfi")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `130` bars may return NaN or partial state per implementation.
- Parameter sensitivity: smaller periods increase noise; larger periods increase lag.
- Sudden gaps or bad ticks can distort rolling windows — consider pre-filtering.
- Single-series indicators ignore volume unless otherwise documented.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; streaming and Polars batch paths are bit-identical.

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend.md)

## Sources & References

**Primary Source**: https://www.traders.com/Documentation/FEEDbk_docs/2022/04/TradersTips.html

**Implementation**: `quantwave-core/src/indicators/vfi.rs` (`Vfi` / `VFI_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/vfi.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
