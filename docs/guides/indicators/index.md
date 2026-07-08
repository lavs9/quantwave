# Indicators

!!! tip "Short answer"
    **221 native Rust indicators** — search by name, browse the [catalog](native/), or start from the [gallery](gallery.md). Every indicator shares one `Next<T>` core for batch and streaming parity.

QuantWave ships **221 native Rust indicators** with bit-identical batch (Polars `.ta()`) and streaming (`Next<T>`) parity — validated by gold-standard vectors and proptests.

!!! tip "Find any indicator fast"
    Use the **search box** (top right) and type a name or slug — e.g. `rsi`, `cyber_cycle`, `market_structure`.  
    For browsing by category, open the [Full Catalog](native/).

## Learning paths

<div class="qw-grid" markdown="1">

<div class="qw-card" markdown="1">

### New to QuantWave?
Start with the [Gallery](gallery.md) — ten curated indicators with real strategy context (SuperTrend, Market Structure, Cyber Cycle, …).

</div>

<div class="qw-card" markdown="1">

### Building a trend system
[SuperTrend](native/supertrend/) → [ADX](native/average_directional_index_adx/) → [Strategy Backtest notebook](../../examples/notebooks/strategy_backtest.md)

</div>

<div class="qw-card" markdown="1">

### Price action & structure
[Market Structure](native/market_structure/) → [Geometric Patterns](native/geometric_patterns/) → [PA Flag Breakout notebook](../../examples/notebooks/pa_flag_breakout_strategy.md)

</div>

<div class="qw-card" markdown="1">

### Ehlers / cycle research
[Ehlers DSP guide](ehlers/index.md) → [Cyber Cycle](native/cyber_cycle/) → [Regime Detection](regimes/index.md)

</div>

<div class="qw-card" markdown="1">

### ML feature pipelines
[ML Features guide](../ml_features.md) → [Hurst](native/hurst_exponent/) → [Fractional Differentiation](native/fractional_differentiation/)

</div>

<div class="qw-card" markdown="1">

### Full reference
[Complete catalog](native/) — all 221 indicators by category with slug lookup tables.

</div>

</div>

## Flagship indicators

These pages are written to **gold standard** depth (formula, correct Polars API, edge cases, strategy context):

| Indicator | Why it matters |
|-----------|----------------|
| [SuperTrend](native/supertrend/) | ATR trend + trailing stop; steel-thread parity reference |
| [Market Structure](native/market_structure/) | MQL5 Part 21 — bias + confirmed BOS; gates all PA signals |
| [Relative Strength Index](native/relative_strength_index_rsi/) | Wilder momentum oscillator; ML and mean-reversion staple |
| [MACD](native/moving_average_convergence_divergence_macd/) | Trend-momentum struct (line, signal, histogram) |
| [Cyber Cycle](native/cyber_cycle/) | Ehlers low-lag cycle; regime-aware timing |
| [Geometric Patterns](native/geometric_patterns/) | Flags + H&S with rich struct metadata for sizing |

## Browse by area

| Area | Entry points |
|------|----------------|
| **Classics** | [RSI](native/relative_strength_index_rsi/), [MACD](native/moving_average_convergence_divergence_macd/), [Bollinger Bands](native/bollinger_bands/) |
| **Ehlers DSP** | [Ehlers guide](ehlers/index.md), [Instantaneous Trendline](native/instantaneous_trendline/), [Trendflex](native/trendflex/) |
| **Volatility** | [ATR](native/average_true_range/), [TTM Squeeze](native/ttm_squeeze/), [Keltner](native/keltner_channels/) |
| **Volume** | [OBV](native/on_balance_volume_obv/), [MFI](native/money_flow_index_mfi/), [VWAP](native/anchored_vwap/) |
| **Patterns** | 50+ candlestick pages in the [Full Catalog](native/) |
| **Regimes** | [HMM / GMM / changepoints](regimes/index.md) |

## Guarantees

- **One math path** — `Next<T>` powers Rust streaming, Python streaming, and Polars plugins
- **Gold-standard tests** — industry reference vectors in `quantwave-core/tests/gold_standard/`
- **Documented boundaries** — warmup, NaN, and parameter rules on every page