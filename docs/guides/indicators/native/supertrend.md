# SuperTrend

<div class="indicator-meta"><span class="category-badge">Classic</span> <span class="kw-badge">trend</span> <span class="kw-badge">atr</span> <span class="kw-badge">stop-loss</span> <span class="kw-badge">classic</span> <span class="kw-badge">breakout</span></div>

ATR-based trend follower that doubles as a trailing stop — one of QuantWave's flagship indicators and the steel-thread reference for batch ↔ streaming parity.

## Visual Example

![SuperTrend — annotated preview mapping to core implementation](../../../assets/indicator-previews/supertrend.png)

*Synthetic OHLCV with SuperTrend line and direction flips. Generated via `docs/generate_all_previews.py`; maps to the core `Next<(f64,f64,f64)>` implementation.*

## Description

SuperTrend overlays a volatility-adjusted band on price and tracks whether the market is in an **uptrend** or **downtrend**. When trend is up, the active line sits on the **lower** band (support); when trend is down, it sits on the **upper** band (resistance). A close beyond the opposite band flips direction — producing discrete, rule-based entries and exits without repainting past bars.

Production teams use SuperTrend for:

- **Trend gating** — only take long signals when `direction > 0`
- **Dynamic stops** — exit when price closes through the active band
- **Feature engineering** — distance-to-SuperTrend and direction as ML inputs
- **Backtest signals** — feed `direction` into `.bt.backtest()` as exposure (see [Strategy Backtest](../../../examples/notebooks/strategy_backtest.md))

QuantWave implements SuperTrend through the universal `Next<T>` trait. The Polars expression plugin, Python streaming class, and Rust streaming struct all call the same core logic, validated by gold-standard vectors and streaming-batch proptests.

## Formula / Specification

**Source:** [TradingView — SuperTrend by Mobius](https://www.tradingview.com/script/7zF0a4f8-SuperTrend-by-Mobius/)

Let \(m_t = (H_t + L_t) / 2\) (bar midpoint), \(\text{ATR}_t\) the Wilder ATR over `period`, and `multiplier` the band width.

\[
\begin{aligned}
U^{\text{basic}}_t &= m_t + \text{multiplier} \cdot \text{ATR}_t \\
L^{\text{basic}}_t &= m_t - \text{multiplier} \cdot \text{ATR}_t
\end{aligned}
\]

Final bands ratchet (never loosen against the prior bar):

\[
\begin{aligned}
U_t &= \min(U^{\text{basic}}_t,\, U_{t-1}) \quad \text{if } C_{t-1} \le U_{t-1},\ \text{else } U^{\text{basic}}_t \\
L_t &= \max(L^{\text{basic}}_t,\, L_{t-1}) \quad \text{if } C_{t-1} \ge L_{t-1},\ \text{else } L^{\text{basic}}_t
\end{aligned}
\]

Direction flips when price closes through the inactive band; the SuperTrend value is the active band:

\[
\text{SuperTrend}_t = \begin{cases}
L_t & \text{if direction is up (+1)} \\
U_t & \text{if direction is down (−1)}
\end{cases}
\]

**Implementation:** `quantwave-core/src/indicators/supertrend.rs` (`SuperTrend::next((high, low, close))` → `(value, direction)`).

**Gold-standard vectors:** `quantwave-core/tests/gold_standard/supertrend_10_3.json` (period=10, multiplier=3.0).

## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `period` | 10 | ATR lookback length |
| `multiplier` | 3.0 | Band width in ATR units; higher = wider bands, fewer flips |

Typical ranges: `period` 7–14 for intraday, 10–20 for daily; `multiplier` 2.0–4.0. Tighter settings increase whipsaws; wider settings increase lag.

## Usage Examples

**Polars batch (recommended)**

```python
import polars as pl
import quantwave  # registers pl.col().ta

df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .with_columns(
        pl.col("close")
        .ta.supertrend("high", "low", period=10, multiplier=3.0)
        .alias("st")
    )
    .with_columns(
        pl.col("st").struct.field("supertrend").alias("supertrend"),
        pl.col("st").struct.field("direction").alias("supertrend_dir"),
    )
    .collect()
)
```

**Streaming (Python)**

```python
import quantwave as qw

st = qw.streaming_class("supertrend")(period=10, multiplier=3.0)
for row in df.iter_rows(named=True):
    out = st.next(row["high"], row["low"], row["close"])
    # out.value → SuperTrend line, out.direction → +1 / −1
```

**Streaming (Rust)**

```rust
use quantwave_core::indicators::supertrend::SuperTrend;
use quantwave_core::traits::Next;

let mut st = SuperTrend::new(10, 3.0);
for (high, low, close) in bars {
    let (line, direction) = st.next((high, low, close));
}
```

**Backtest wiring**

```python
signal_df = df.with_columns(
    pl.when(pl.col("supertrend_dir") > 0).then(1.0).otherwise(0.0).alias("signal")
)
report = signal_df.lazy().bt.backtest_with_report(signal="signal")
```

All surfaces are bit-identical via the single `Next<T>` implementation and proptests.

## Edge Cases & Limitations

- **Warm-up:** First `period` bars build ATR state; early values follow core warmup semantics (see Boundary Behavior).
- **Whipsaws:** Ranging markets produce frequent direction flips — combine with higher-timeframe trend or [Market Structure](market_structure/) filters.
- **Gaps:** Overnight gaps can trigger immediate flips; consider execution delay in backtests (`execution_delay="next_bar"`).
- **Parameter sensitivity:** Lower `multiplier` or `period` increases noise; higher values increase lag.
- **No look-ahead:** Direction is known only after the bar closes; safe for live streaming and batch features.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars may return partial ATR state until `period` bars are seen. |
| `period` > series length | Output reflects insufficient data; parity tests use explicit vectors. |
| NaN in H/L/C | NaN propagates to output for that bar. |
| Invalid params | Non-positive `period` or `multiplier` raise `ValueError`. |

## Related Indicators & See Also

- [ATR Trailing Stop](atr_trailing_stop/) — alternative ATR-based exit
- [Keltner Channels](keltner_channels/) — envelope bands with similar volatility scaling
- [Parabolic SAR](parabolic_sar/) — complementary stop-and-reverse system
- [Strategy Backtest notebook](../../../examples/notebooks/strategy_backtest.md) — SuperTrend → `.bt` E2E
- [Indicator Gallery](../gallery.md)

## Sources & References

**Primary source:** [TradingView — SuperTrend by Mobius](https://www.tradingview.com/script/7zF0a4f8-SuperTrend-by-Mobius/)

**Implementation:** `quantwave-core/src/indicators/supertrend.rs` (`SuperTrend` / `SUPERTREND_METADATA`)

**Parity:** `quantwave-core/tests/gold_standard/supertrend_10_3.json`; streaming-batch proptest in `supertrend.rs`