# Harrington ADX Oscillator

<div class="indicator-meta"><span class="category-badge">Wilder</span> <span class="kw-badge">adx</span> <span class="kw-badge">dmi</span> <span class="kw-badge">oscillator</span> <span class="kw-badge">wilder</span> <span class="kw-badge">momentum</span></div>

An oscillator variant of the ADX where the sign reflects trend direction determined by DMI+ and DMI-.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Harrington ADX Oscillator** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only harrington_adx_oscillator` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/harrington_adx.rs`.*

## Description

An oscillator variant of the ADX where the sign reflects trend direction determined by DMI+ and DMI-.

The oscillator is positive when DMI+ > DMI- and negative when DMI- > DMI+. The magnitude represents trend strength (ADX). Thresholds at 15 and 40 are often used to identify trend initiation and overextended states.

While originally created by Wilder, this revisualization by Harrington transforms the unipolar ADX into a bipolar oscillator. This allows for simultaneous identification of trend strength and direction in a single histogram display, simplifying the interpretation of complex directional movement data.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/harrington_adx.rs`):

\[
TR = \max(H-L, |H-C_{t-1}|, |L-C_{t-1}|)
\]
\[
+DM = (H-H_{t-1} > L_{t-1}-L) \text{ and } (H-H_{t-1} > 0) ? H-H_{t-1} : 0
\]
\[
-DM = (L_{t-1}-L > H-H_{t-1}) \text{ and } (L_{t-1}-L > 0) ? L_{t-1}-L : 0
\]
\[
+DI = 100 \cdot \frac{EMA(+DM, 1/L)}{EMA(TR, 1/L)}
\]
\[
-DI = 100 \cdot \frac{EMA(-DM, 1/L)}{EMA(TR, 1/L)}
\]
\[
DX = 100 \cdot \frac{|+DI - -DI|}{+DI + -DI}
\]
\[
ADX = EMA(DX, 1/L)
\]
\[
Result = (SMA(+DI, S) \ge SMA(-DI, S)) ? ADX : -ADX
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/harrington_adx.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `adx_length` | 10 | Wilder's ADX period |
| `adx_smooth_length` | 1 | SMA period for DMI components smoothing |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::HARRINGTON_ADX;
use quantwave_core::traits::Next;

let mut ind = HARRINGTON_ADX::new(10);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import HARRINGTON_ADX

ind = HARRINGTON_ADX(10)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_harrington_adx_oscillator(series: pl.Series) -> pl.Series:
    ind = qw.HARRINGTON_ADX(10)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_harrington_adx_oscillator, return_dtype=pl.Float64).alias("harrington_adx_oscillator")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `10` bars may return NaN or partial state per implementation.
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20DECEMBER%202024.html

**Implementation**: `quantwave-core/src/indicators/harrington_adx.rs` (`HARRINGTON_ADX` / `HARRINGTON_ADX_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/harrington_adx.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
