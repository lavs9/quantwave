# Standard Deviation

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">statistics</span> <span class="kw-badge">classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">trend</span></div>

Standard Deviation is a statistical measure of market volatility.

!!! danger "`stddev` is population std (ddof=0) — pandas `.std()` defaults to sample std (ddof=1)"

    QuantWave's `stddev` follows the TA-Lib convention: it divides by `N`, not `N-1`.
    `pandas.Series.rolling(N).std()` divides by `N-1` unless you pass `ddof=0`. The two
    differ by a factor of `sqrt(N / (N-1))`, which shrinks toward 1 as `N` grows but is
    non-trivial at typical indicator windows:

    | | Formula | Result on the same window |
    |---|---|---|
    | `qw stddev` (population, ddof=0) | \(\sqrt{\sum (x_i-\mu)^2 / N}\) | `1.9653244` |
    | `pandas .std()` (sample, ddof=1) | \(\sqrt{\sum (x_i-\mu)^2 / (N-1)}\) | `2.0716338` |

    If you cross-check a `stddev`-derived value (Bollinger-style bands, a z-score, a
    volatility filter) against a pandas prototype, match the `ddof` first — the values
    will not agree otherwise, and nothing raises to tell you why. Use
    `.std(ddof=0)` on the pandas side, or accept the population convention and adjust
    thresholds accordingly.

    See also the [Agent Skill guide](../../agent-skill.md#why-this-exists), which covers
    this and other silent-wrongness cases.

## Visual Example

![Standard Deviation — annotated preview mapping to core implementation](../../../assets/indicator-previews/standard_deviation.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

Standard Deviation is a statistical measure of market volatility.

Use for statistical analysis of price series: linear regression, standard deviation, correlation coefficients, and other descriptive statistics used as indicator inputs.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Standard statistical measures provide the mathematical foundation for many technical indicators. Linear regression finds the best-fit line through price, standard deviation quantifies dispersion, and correlation coefficients measure how closely two series move together — all are essential for quantitative strategy construction.

**Typical applications:**

- Size stops and position risk from band width or ATR expansion
- Detect squeeze conditions (narrow bands) before breakout systems
- Warm-up: first `14` bars build rolling volatility state
- Combine with trend direction (SuperTrend, MACD) for breakout bias

QuantWave implements this via the universal `Next<T>` trait — bit-identical across Rust streaming, Python streaming, and Polars `.ta()` batch plugins.

## Formula / Specification

**Implementation** (`quantwave-core/src/indicators/statistics.rs`):

\[
\sigma = \sqrt{ \frac{\sum (x_i - \mu)^2}{N} }
\]

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/stddev.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::STDDEV;
use quantwave_core::traits::Next;

let mut ind = STDDEV::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import STDDEV

ind = STDDEV(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_standard_deviation(series: pl.Series) -> pl.Series:
    ind = qw.STDDEV(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_standard_deviation, return_dtype=pl.Float64).alias("standard_deviation")
    )
    .collect()
)
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- Warm-up: first `14` bars may return NaN or partial state per implementation.
- Parameter sensitivity: smaller periods increase noise; larger periods increase lag.
- Sudden gaps or bad ticks can distort rolling windows — consider pre-filtering.
- Single-series indicators ignore volume unless otherwise documented.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; streaming and Polars batch paths are bit-identical.

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
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](../supertrend/)

## Sources & References

**Primary Source**: https://www.investopedia.com/terms/s/standarddeviation.asp

**Implementation**: `quantwave-core/src/indicators/statistics.rs` (`STDDEV` / `STDDEV_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/stddev.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
