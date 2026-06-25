# Ultimate Channel

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">channel</span> <span class="kw-badge">volatility</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">breakout</span></div>

A Keltner-style channel using UltimateSmoothers for both the center line and the volatility range to minimize lag.

## Visual Example

> **Chart**: Sparkline or annotated price series showing **Ultimate Channel** behaviour on synthetic trending + cyclic data. Run `python docs/gen_indicator_previews.py --only ultimate_channel` after extending the generator.

*Visual placeholder — standards bulk upgrade 2026-06-25 IST. Core logic in `quantwave-core/src/indicators/ultimate_channel.rs`.*

## Description

A Keltner-style channel using UltimateSmoothers for both the center line and the volatility range to minimize lag.

Use as a dynamic price channel whose width scales with the current dominant cycle amplitude, providing adaptive support and resistance levels for breakout trading.

The Ultimate Channel uses the measured dominant cycle amplitude to set channel width, analogous to Keltner Channels but cycle-aware rather than ATR-based. When price breaks beyond the channel boundary, it signals that cycle amplitude has expanded enough to suggest a genuine directional move.

QuantWave implements this indicator via the universal `Next<T>` trait, guaranteeing bit-identical results between Rust streaming, Python streaming, and Polars batch (`.ta()` / `map_batches`) surfaces.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/ultimate_channel.rs`):

\[
TH = \max(High, Close_{t-1})
\]
\[
TL = \min(Low, Close_{t-1})
\]
\[
STR = UltimateSmoother(TH - TL, STRLength)
\]
\[
Center = UltimateSmoother(Close, Length)
\]
\[
Upper = Center + NumSTRs \times STR
\]
\[
Lower = Center - NumSTRs \times STR
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/ultimate_channel.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length` | 20 | Center line smoothing period |
| `str_length` | 20 | Smooth True Range (STR) period |
| `num_strs` | 1.0 | Channel width multiplier |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ULTIMATE_CHANNEL;
use quantwave_core::traits::Next;

let mut ind = ULTIMATE_CHANNEL::new(20);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ULTIMATE_CHANNEL

ind = ULTIMATE_CHANNEL(20)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_ultimate_channel(series: pl.Series) -> pl.Series:
    ind = qw.ULTIMATE_CHANNEL(20)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_ultimate_channel, return_dtype=pl.Float64).alias("ultimate_channel")
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

**Primary Source**: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UltimateChannel.pdf

**Implementation**: `quantwave-core/src/indicators/ultimate_channel.rs` (`ULTIMATE_CHANNEL` / `ULTIMATE_CHANNEL_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/ultimate_channel.json`

**Provenance**: Standards bulk upgrade 2026-06-25 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
