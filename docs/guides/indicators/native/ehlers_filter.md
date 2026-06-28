# Ehlers Filter

<div class="indicator-meta"><span class="category-badge">Ehlers DSP</span> <span class="kw-badge">filter</span> <span class="kw-badge">ehlers</span> <span class="kw-badge">dsp</span> <span class="kw-badge">adaptive</span> <span class="kw-badge">smoothing</span></div>

A non-linear FIR filter that computes adaptive coefficients from the sum of squared distances between recent prices. It delivers smooth output during low-volatility periods and rapid response when price makes sharp transitions.

## Visual Example

![Ehlers Filter (indigo) overlaid on synthetic cyclic price series containing a dominant cycle, slow drift, and moderate noise. The output remains smooth where price distances are small and adapts its effective weighting when larger jumps increase the distance coefficients.](../../../assets/indicator-previews/ehlers_filter.png)

*Synthetic ideal price series (dominant cycle + linear drift + Gaussian noise) engineered to exercise the distance-coefficient adaptation logic. Matches the exact FIR windowing, double-loop coefficient computation, and early-bar passthrough behavior implemented in `quantwave-core/src/indicators/ehlers_filter.rs` (Next<f64> at lines 29-58 and the matching proptest batch reference implementation). Generated 2026-05-31 IST via `docs/gen_indicator_previews.py`.*

## Description

The Ehlers Filter is a data-driven FIR smoother whose coefficients are derived from the local "distance" structure of the price series rather than fixed window weights. In quiet, range-bound markets the squared differences remain small and the filter behaves like a conventional average. When a strong directional move or volatility spike occurs, the coefficients automatically de-emphasize older samples that are now distant, producing a faster reaction without the lag penalty of a shorter fixed window.

Practitioners use it as a drop-in replacement for EMA or SuperSmoother when the market regime is unknown in advance or when they want a single filter that self-tunes across regimes. It is especially valuable upstream of cycle oscillators, regime detectors, or as a low-noise input to ML feature pipelines. Because the adaptation is purely local and causal, the filter introduces no look-ahead bias and preserves the streaming / batch parity contract.

## Formula / Specification

**Exact implementation in QuantWave** (`quantwave-core/src/indicators/ehlers_filter.rs`):

1. Maintain a sliding window of the most recent `2 × length − 1` prices (front-pushed deque).
2. While the window is shorter than the required history, return the raw input (passthrough warm-up).
3. For each new bar, for every position `count` in the first `length` samples of the window:
   - Compute the sum of squared differences (`distance2`) between that sample and the next `length−1` samples behind it.
   - Use `distance2` directly as the coefficient for that sample.
4. The filtered value is the weighted sum of the recent `length` prices divided by the sum of the coefficients. When the sum of coefficients is zero (degenerate flat window), fall back to the raw input.
5. The streaming `Next<f64>` and the proptest reference batch implementation are bit-identical (enforced with `approx` tolerance `1e-10`).

The LaTeX form recorded in `EHLERS_FILTER_METADATA` matches the above exactly.

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `length`  | 15      | Filter window length used both for the coefficient lookback and the FIR span. Larger values increase smoothness at the cost of additional lag on true regime shifts. |

## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::EhlersFilter;
use quantwave_core::traits::Next;

let mut filt = EhlersFilter::new(15);
for price in price_series {
    let smooth = filt.next(price);
    // Use smooth as denoised input to cycle detectors or ML features
}
```

**Streaming (Python)**

```python
from quantwave import EhlersFilter

filt = EhlersFilter(15)
for price in price_series:
    smooth = filt.next(price)
```

**Polars Batch (Python — primary research / feature surface)**

```python
import polars as pl
import quantwave as qw   # exposes the streaming classes for UDFs / research

# For full production Polars workloads the same Next logic is wired via
# custom expressions or the features namespace (see features.rs for patterns).
# The example below demonstrates parity-preserving usage with the Python surface:

def ehlers_filter_expr(col: str, length: int = 15):
    filt = qw.EhlersFilter(length)
    def _apply(s: pl.Series) -> pl.Series:
        return pl.Series([filt.next(float(v)) for v in s.to_list()])
    return pl.col(col).map_batches(_apply, return_dtype=pl.Float64)

df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .with_columns([
        ehlers_filter_expr("close", 15).alias("ehlers_filter_15"),
    ])
    .collect()
)
```

All three surfaces are bit-identical by construction: the Python and Rust classes are thin wrappers around the identical `Next<f64>` implementation whose parity is proven by the proptest in the core source.

## Edge Cases & Limitations

- The first `2 × length − 2` bars return the raw price (insufficient history for coefficient calculation).
- In perfectly flat windows the coefficient sum can become zero; the implementation correctly falls back to the input value.
- Very large `length` relative to the series can make the filter overly sluggish on genuine structural breaks.
- The adaptation is based on squared Euclidean distance in price space only; it does not incorporate volume or volatility scaling unless the caller pre-processes the input.
- Because the filter is FIR with data-dependent weights it can still exhibit some overshoot on the first large move after a long quiet period.
- Not a replacement for a true low-pass with known cutoff; for fixed frequency response prefer SuperSmoother, Butterworth, or RoofingFilter.
- Best used in confluence with a regime classifier (e.g. Cyber Cycle or market structure bias) so that aggressive adaptation is only trusted when the regime actually supports it.
- No look-ahead bias; every output depends only on data up to and including the current bar.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |
| period > len | When period exceeds series length, output is all NaN. |
| NaN inputs | NaN in input propagates to output (NaN out). |
| Invalid params | Non-positive period or missing required params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

## Related Indicators & See Also

- [UltimateSmoother](ultimatesmoother.md) — preferred zero-lag general-purpose smoother from the same author
- [Super Smoother](supersmoother.md), [Butterworth](butterworth2.md) — fixed-response alternatives when adaptation is not desired
- [Reflex](reflex.md) — zero-lag cycle synchronizer that internally uses a SuperSmoother
- [Ehlers Stochastic](ehlers_stochastic.md) — cycle-aware oscillator that benefits from clean upstream filtering
- [Market Structure](market_structure.md) — use bias / BOS state to gate or weight the filter output
- [Indicator Gallery](../gallery.md) • [Native Indicators](native/index.md) • [Ehlers DSP Suite](../ehlers/index.md)
- Full runnable examples: `docs/examples/notebooks/multi_indicator_analysis.ipynb` (and the ML feature parity notebook)

## Sources & References

**Primary Source**: `quantwave-core/src/indicators/ehlers_filter.rs` (EHLERS_FILTER_METADATA + Next<f64> implementation and proptest parity harness). Formula source recorded in metadata: https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EhlersFilters.pdf (John Ehlers, "Ehlers Filters", 2001).

**Visual**: Generated 2026-05-31 IST via `docs/gen_indicator_previews.py` using synthetic data that exercises the exact coefficient logic implemented in the core.

**Additional Context**: Ehlers, *Cybernetic Analysis for Stocks and Futures* (2004) and *Cycle Analytics for Traders* (2013) for the broader DSP philosophy and when adaptive vs fixed filters are preferable. TradersTips archive articles for practical EasyLanguage / Excel ports that match the coefficient construction.

**Implementation Provenance**: Universal `Next<T>` contract and batch/streaming parity documented in `quantwave-core/src/traits.rs` and the proptest block inside `ehlers_filter.rs`. All higher-level surfaces (Python bindings, Polars UDFs) delegate to this single source of truth.
