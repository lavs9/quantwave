# Moving Average Convergence Divergence (MACD)

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">momentum</span> <span class="kw-badge">moving-average</span> <span class="kw-badge">classic</span></div>

Gerald Appel's trend-momentum system — MACD line, signal line, and histogram in one struct, TA-Lib parity guaranteed.

## Visual Example

![Moving Average Convergence Divergence (MACD) — annotated preview mapping to core implementation](../../../assets/indicator-previews/moving_average_convergence_divergence_macd.png)

*Synthetic price with MACD line, signal, and histogram. Generated via `docs/generate_all_previews.py`.*

## Description

MACD plots the **difference between fast and slow EMAs** (default 12 vs 26). A **signal line** (EMA of MACD, default 9) smooths the oscillator; the **histogram** (`MACD − Signal`) shows momentum acceleration.

Production uses:

- **Signal crossovers** — MACD crosses above signal → bullish momentum shift (confirm with trend)
- **Histogram slope** — rising histogram confirms strengthening trend; shrinking histogram warns of exhaustion
- **Zero-line context** — MACD above zero supports long bias on higher timeframes
- **ML features** — struct output (`macd`, `signal`, `hist`) drops cleanly into feature matrices

QuantWave uses TA-Lib-aligned EMA seeding in `incremental/macd.rs`. Streaming returns `(macd, signal, hist)`; Polars `.ta.macd()` returns the same struct. Proptests assert parity against `talib_rs::momentum::macd`.

## Formula / Specification

**Source:** Gerald Appel (1970s); [Investopedia MACD](https://www.investopedia.com/terms/m/macd.asp)

\[
\text{MACD}_t = EMA_{fast}(C)_t - EMA_{slow}(C)_t
\]
\[
\text{Signal}_t = EMA_{signal}(\text{MACD})_t
\]
\[
\text{Hist}_t = \text{MACD}_t - \text{Signal}_t
\]

Defaults: fast = 12, slow = 26, signal = 9.

**Implementation:** `quantwave-core/src/indicators/incremental/macd.rs`

**Gold-standard vectors:** `quantwave-core/tests/gold_standard/macd.json`

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `fastperiod` | 12 | Fast EMA length |
| `slowperiod` | 26 | Slow EMA length |
| `signalperiod` | 9 | Signal EMA length |

Faster settings increase whipsaws; slower settings lag turning points. PPO offers a percentage-scaled variant for cross-asset comparison.

## Usage Examples

**Polars batch (recommended)**

```python
import polars as pl
import quantwave  # registers pl.col().ta

df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .with_columns(
        pl.col("close").ta.macd(12, 26, 9).alias("macd_struct")
    )
    .with_columns(
        pl.col("macd_struct").struct.field("macd").alias("macd"),
        pl.col("macd_struct").struct.field("signal").alias("macd_signal"),
        pl.col("macd_struct").struct.field("hist").alias("macd_hist"),
    )
    .collect()
)
```

**Streaming (Python)**

```python
import quantwave as qw

macd = qw.streaming_class("macd")(fastperiod=12, slowperiod=26, signalperiod=9)
for price in closes:
    out = macd.next(price)  # macd, signal, hist
```

**Streaming (Rust)**

```rust
use quantwave_core::indicators::MACD;
use quantwave_core::traits::Next;

let mut macd = MACD::new(12, 26, 9);
for price in &closes {
    let (line, signal, hist) = macd.next(*price);
}
```

**Signal wiring (crossover)**

```python
df = df.with_columns(
    (pl.col("macd") > pl.col("macd_signal")).alias("macd_bullish")
)
```

## Edge Cases & Limitations

- **Range markets:** Frequent signal-line crossovers without trend — gate with [ADX](../average_directional_index_adx/) or [SuperTrend](../supertrend/).
- **Warm-up:** Needs `slowperiod + signalperiod` bars for stable signal EMA.
- **Lag:** Inherent to EMA construction; Ehlers tools ([Cyber Cycle](../cyber_cycle/)) offer lower-lag alternatives for timing.
- **Single price input:** Volume-less; combine with [MFI](../money_flow_index_mfi/) or OBV for flow confirmation.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN triple until EMAs seed. |
| `slowperiod` > series length | Insufficient data → NaN outputs. |
| NaN in close | NaN propagates through EMA chain. |
| Invalid params | Non-positive periods raise `ValueError`. |

## Related Indicators & See Also

- [PPO](../percentage_price_oscillator_ppo/) — percentage-scaled MACD for cross-asset ranks
- [RSI](../relative_strength_index_rsi/) — complementary momentum oscillator
- [APO](../absolute_price_oscillator_apo/) — MACD without signal line
- [Multi-Indicator Analysis notebook](../../../examples/notebooks/multi_indicator_analysis.md)

## Sources & References

**Primary source:** Appel; [Investopedia MACD](https://www.investopedia.com/terms/m/macd.asp)

**Implementation:** `quantwave-core/src/indicators/incremental/macd.rs` (`MACD` / `MACD_METADATA`)

**Parity:** TA-Lib proptest in `momentum.rs`; gold-standard `macd.json`