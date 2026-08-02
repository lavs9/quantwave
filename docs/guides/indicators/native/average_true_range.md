# Average True Range

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">volatility</span> <span class="kw-badge">atr</span> <span class="kw-badge">classic</span> <span class="kw-badge">range</span></div>

ATR represents the average of true ranges over a specified period.

!!! danger "Two different ATRs live under this name — check which one you are calling"

    `atr` is **not** a single formula in QuantWave. Surfaces split as follows, and
    the two groups disagree by a materially large margin (order of a few percent at
    `period=14`, larger on gappy data):

    | Surface | Smoothing | TA-Lib / TradingView parity |
    |---|---|---|
    | `quantwave.Atr(period)` streaming class | EMA, `alpha = 2/(period+1)` | **No** |
    | `quantwave.atr(period, high, low, close)` batch fn | EMA, `alpha = 2/(period+1)` | **No** |
    | `pl.col("close").ta.atr("high", "low")` Polars plugin | Wilder RMA, `alpha = 1/period` | Yes |
    | `pl.col("high").ta.ta_atr("low", "close")` | Wilder RMA | Yes |
    | `lf.ta().ta_atr(...)` | Wilder RMA | Yes |
    | `quantwave.talib.ATR(...)` | Wilder RMA | Yes |

    **If you want the conventional ATR — the one Wilder defined in 1978 and the one
    TA-Lib, TradingView Pine `ta.atr`, and `pandas.ewm(alpha=1/period, adjust=False)`
    all compute — use `ta_atr`.** The EMA-smoothed variant additionally has no NaN
    warmup and is seeded from the first bar's `high - low`, so early values are
    biased low relative to a Wilder ATR.

    This matters disproportionately because ATR feeds stop distance and
    volatility-targeted position sizing: the error propagates straight into risk.

    No authoritative source has been recorded for the EMA-smoothed variant — the
    `formula_source` recorded for this indicator (Investopedia) describes Wilder's
    RMA. The default has **not** been changed, because
    doing so would move every ATR-composed indicator's output.

    Discover this programmatically rather than trusting prose:

    ```python
    import quantwave as qw

    for note in qw.conventions("atr"):
        print(note.aspect, "->", note.convention)
        print("differs from:", note.differs_from)
        print("guidance:", note.guidance)

    qw.convention_slugs()   # every indicator carrying a convention divergence
    ```

    Indicators that compose the EMA-smoothed `Atr` and therefore inherit the
    divergence: [`supertrend`](supertrend.md), [`keltner`](keltner_channels.md),
    [`atr_ts`](atr_trailing_stop.md), [`ttm_squeeze`](ttm_squeeze.md),
    [`vpn`](volume_positive_negative.md), plus the `sr_monitor` ATR field and the
    volatility-clustering regime model.

## Visual Example

![Average True Range — annotated preview mapping to core implementation](../../../assets/indicator-previews/average_true_range.png)

*Synthetic ideal per library logic. Generated 2026-07-01 IST via `docs/generate_all_previews.py` (reproducible; maps to core `Next<T>` implementation).*

## Description

ATR represents the average of true ranges over a specified period.

Use as the foundational volatility module providing ATR, True Range, and related volatility measures used by higher-level indicators such as SuperTrend and Keltner Channels.

Native Rust implementation with gold-standard or TA-Lib parity tests where applicable.

Average True Range, developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), measures the average of the true range over N bars. True Range accounts for overnight gaps by taking the maximum of: current high minus low, current high minus prior close, prior close minus current low. It remains the industry standard raw volatility measure.

**Typical applications:**

- Size stops and position risk from band width or ATR expansion
- Detect squeeze conditions (narrow bands) before breakout systems
- Warm-up: first `14` bars build rolling volatility state
- Combine with trend direction (SuperTrend, MACD) for breakout bias

QuantWave implements this via the universal `Next<T>` trait. **Unlike every other
indicator in the catalog, `atr` is not bit-identical across surfaces** — see the
callout above. `ta_atr` *is* bit-identical across Rust streaming, Python streaming,
and the Polars `.ta()` plugin, and is parity-tested against `talib_rs` by proptest.

## Formula / Specification

True range is common to both variants:

\[
TR_t = \max\bigl(H_t - L_t,\; |H_t - C_{t-1}|,\; |L_t - C_{t-1}|\bigr)
\]

**`Atr` / `atr()` — EMA smoothing** (`quantwave-core/src/indicators/volatility.rs`,
struct `ATR`):

\[
ATR_t = \alpha\,TR_t + (1-\alpha)\,ATR_{t-1}, \qquad \alpha = \frac{2}{n+1}
\]

seeded with \(ATR_0 = H_0 - L_0\), emitting a value from the first bar (no NaN warmup).

**`ta_atr` — Wilder's RMA, TA-Lib compatible**
(`quantwave-core/src/indicators/incremental/ta_atr.rs`, struct `TaATR`):

\[
ATR_t = \frac{ATR_{t-1}\,(n-1) + TR_t}{n}, \qquad
ATR_n = \frac{1}{n}\sum_{i=1}^{n} TR_i
\]

emitting `NaN` until `n` true ranges have accumulated.

Parity for `ta_atr` is enforced by proptest against `talib_rs::volatility::atr`.
There is no gold-standard vector file for the EMA variant (the metadata references
`atr.json`, which does not exist); its behaviour is pinned by
`tests/python/test_conventions.py` instead.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 14 | Smoothing period |


## Usage Examples

**Streaming (Rust)**

```rust
use quantwave_core::indicators::ATR;
use quantwave_core::traits::Next;

let mut ind = ATR::new(14);
for price in &prices {
    let value = ind.next(price);
}
```

**Streaming (Python)**

```python
from quantwave import ATR

ind = ATR(14)
for price in prices:
    value = ind.next(price)
```

**Polars Batch (Python)**

```python
import polars as pl
import quantwave as qw

def apply_average_true_range(series: pl.Series) -> pl.Series:
    ind = qw.ATR(14)
    return pl.Series([ind.next(float(v)) for v in series.to_list()])

df = (
    pl.read_csv('ohlcv.csv')
    .lazy()
    .with_columns(
        pl.col("close").map_batches(apply_average_true_range, return_dtype=pl.Float64).alias("average_true_range")
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

**Primary Source**: https://www.investopedia.com/terms/a/atr.asp

**Implementation**: `quantwave-core/src/indicators/volatility.rs` (`ATR` / `ATR_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/atr.json`

**Provenance**: Standards bulk upgrade 2026-07-01 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
