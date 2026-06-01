# SuperSmoother

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">smoothing</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">low-pass</span></div>

A second-order IIR filter with a maximally flat Butterworth response for superior smoothing with minimal lag. The gold-standard low-lag smoother in the Ehlers DSP suite and the foundation for many higher-level indicators (Trendflex, Roofing Filter, etc.).

## Visual Example

![SuperSmoother: noisy price (gray) with a short cycle. SMA(20) (dashed red) lags and can overshoot. SuperSmoother (blue) hugs the underlying movement with far less lag and no ringing/overshoot thanks to its critically-damped 2-pole design. Annotation calls out the response advantage.](../../../assets/indicator-previews/supersmoother.png)

*Synthetic noisy input with short cycle. Critically-damped 2-pole Butterworth (period=20) vs SMA of comparable lag. Exact c1/c2/c3 recurrence and 4-bar warmup from `quantwave-core/src/indicators/super_smoother.rs`. Generated 2026-05-31 IST via `docs/gen_indicator_previews.py`.*

## Description

SuperSmoother is Ehlers' answer to the classic moving-average compromise: longer periods give better noise rejection but unacceptable lag. By using a 2-pole Butterworth design that is critically damped, it achieves the smoothing power of a much longer SMA while responding almost as fast as a short EMA — without the Gibbs ringing/overshoot that plagues many IIR filters on price data.

It is the default pre-filter recommended before virtually any oscillator or cycle tool in the Ehlers family and a drop-in, higher-performance replacement for SMA/EMA in trend-following systems when minimum lag is required.

## Formula / Specification

**Exact coefficients and recurrence** (period = critical wavelength):

```
a1  = exp(-1.414 * π / Period)
c2  = 2 * a1 * cos(1.414 * π / Period)
c3  = -a1 * a1
c1  = 1 - c2 - c3
SS  = c1 * (Price + Price[t-1]) / 2 + c2 * SS[t-1] + c3 * SS[t-2]
```

Warmup: first 3 bars return the raw price (4th bar and beyond use the filter).

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 20 | Critical period (wavelength) of the filter. Controls the cutoff frequency of the maximally-flat Butterworth response. Common values: 10 (very responsive), 20 (balanced), 30–40 (strong smoothing for higher-timeframe trend). |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::SuperSmoother;
use quantwave_core::traits::Next;

let mut ss = SuperSmoother::new(20);
for price in prices {
    let smooth = ss.next(price);
    // Use as low-lag replacement for any moving average
}
```

**Streaming (Python)**

```python
from quantwave import SuperSmoother

ss = SuperSmoother(20)
for price in prices:
    smooth = ss.next(price)
    ...
```

**Polars Batch (Python — primary research / feature surface)**

```python
import polars as pl
import quantwave as qw

df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .with_columns([
        pl.col("close").ta.super_smoother(period=20).alias("ss"),
    ])
    .collect()
)
```

All surfaces are bit-identical (enforced by the universal `Next<T>` trait and proptests).

## Edge Cases & Limitations

- First three bars return the raw price (insufficient history for the 2nd-order recurrence).
- On extremely low-volatility or constant-price series the filter still converges correctly but offers no advantage.
- Because it is IIR, a single bad tick (outlier) has a decaying but non-zero effect on subsequent bars (use with robust pre-cleaning or median pre-filter when data quality is suspect).
- The "period" is not a lookback window in the SMA sense; it is the wavelength at which the filter response is -3 dB.
- Outstanding building block — many Ehlers indicators (Trendflex, Roofing, etc.) are built on top of it.
- No look-ahead bias.

## Related Indicators & See Also

- [Roofing Filter](roofing_filter.md) — uses SuperSmoother as its second stage
- [Trendflex](trendflex.md) — uses SuperSmoother(Length/2) as pre-filter
- [Instantaneous Trendline](instantaneous_trendline.md) — heavier adaptive smoother; SuperSmoother is the lightweight workhorse
- [Cyber Cycle](cyber_cycle.md) — FIR smoother stage + recursive; often paired with SuperSmoother pre-filtering in practice
- All classic overlap indicators when lag reduction is priority
- [Indicator Gallery](../gallery.md) • [Native Indicators](native/index.md) • [Ehlers DSP Suite](ehlers/index.md)

## Sources & References

**Primary Source**: John Ehlers, "The Ultimate Smoother" (article) and *Cybernetic Analysis for Stocks and Futures* (2004). PDF reference: `references/Ehlers Papers/implemented/UltimateSmoother.pdf`.

**Implementation Provenance**: `quantwave-core/src/indicators/super_smoother.rs`. The exact a1/c1/c2/c3 derivation and recurrence above is the single source of mathematical truth. Full warmup logic + exhaustive proptests in the file; parity via `Next<f64>` in `traits.rs`.

**Visual**: Generated 2026-05-31 IST via `docs/gen_indicator_previews.py` with synthetic data chosen to highlight lag and ringing differences.

**Additional Context**: The filter is also described in later Ehlers works (Cycle Analytics for Traders) as the preferred replacement for traditional moving averages in DSP-based indicators.
