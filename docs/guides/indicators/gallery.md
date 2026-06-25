# Indicator Gallery

QuantWave delivers **150+ production-grade technical indicators** in high-performance Rust, with first-class exposure through Polars (`.ta()` namespace) and streaming `Next<T>` implementations. **Every indicator guarantees bit-identical results** between batch and streaming modes, validated via proptests against gold-standard vectors.

All dedicated indicator pages adhere to QuantWave's [Documentation Standards](../../DOCUMENTATION_STANDARDS.md): practical user-focused explanations, mathematical definitions or recognition logic, parameter tables, usage examples across Rust streaming / Python streaming / Polars batch, edge cases, visuals (or precise placeholders), and authoritative sources (Ehlers papers, MQL5 series, TA-Lib, TradingView, etc.).

Use the sidebar, search, or the categorized overview below to explore. Click any link to reach the dedicated page.

## Featured High-Value Indicators

These tools deliver outsized practical value for strategies, risk management, and ML feature engineering:

- **[SuperTrend](native/supertrend.md)**: ATR-based trend-following with dynamic stops. One of the most widely used signals in production. Exceptional performance (~27× faster than common Python implementations on large datasets).

- **[Cyber Cycle](native/cyber_cycle.md)** (Ehlers DSP): Low-lag cycle extractor that cleanly separates trend from cycle. A foundational tool for regime-aware systems (~100× speedup vs pandas on 1M rows).

- **[Market Structure](native/market_structure.md)** (MQL5 Part 21): Adaptive swing detection with Bullish/Bearish/Neutral bias and confirmed Break-of-Structure (BOS) flips (only after structure established). Rich `MarketStructureState` + `PAEvent` for gating strategies and ML. See dedicated guide + [notebook](../../examples/notebooks/pa_flag_breakout_strategy.md).

- **[Geometric Patterns (Flags + H&S)](native/geometric_patterns.md)** (MQL5 Parts 66 & 69): Continuation Flags and reversal Head & Shoulders built on Market Structure. Rich structs with `pole_length_atr` (sizing), `score`, symmetry, `breakout_confirmed`. Ideal for ML features and dynamic risk. Full guide + examples in the dedicated page and [notebook](../../examples/notebooks/pa_flag_breakout_strategy.md).

- **[S/R Interactions](native/sr_monitor.md)** (MQL5 Part 67): Real-time classification of horizontal level interactions (Approach/Touch/Breakout/Reversal/Retest) with rich provenance. Perfect confluence with geometric patterns.

- **[Pivot Points](native/pivot_points.md)**: Classic and variant floor-trader levels with full integration to structure and S/R workflows.

- **[Ichimoku Cloud](native/ichimoku_cloud.md)**: Complete trend, momentum, and dynamic S/R system in a single indicator.

- **[TTM Squeeze](native/ttm_squeeze.md)**: Volatility contraction/expansion detector prized for high-probability breakout entries.

- **[Swiss Army Knife Indicator](native/swiss_army_knife_indicator.md)**: Versatile Ehlers multi-measurement tool combining several analytical dimensions.

Dozens more classics, filters, and unique implementations are available.

## Price Action & Geometric Patterns

QuantWave's most distinctive modern capability: faithful, production-ready ports of the MQL5 "Price Action Analysis Toolkit" series (lynnchris).

**Core components** (all with full streaming + Polars batch parity and rich, serializable event structs):

- **Market Structure (Part 21)**: Adaptive HH/HL/LH/LL swing detection, persistent bias, and confirmed BOS flips. Sparse, high-quality events only after bias establishment. Primary gating mechanism for every other PA signal.

- **Geometric Pattern Scanner (Parts 66 + 69)**: Bull/Bear Flags (pole + qualified consolidation with retrace ≤ 61.8%) and classic Head & Shoulders patterns. Rich metadata enables precise risk and quality filters.

- **S/R Monitoring (Part 67)**: Automatic or seeded horizontal level detection with classified interactions (Approach / Touch / Breakout / Reversal / Retest) carrying distance, strength, and provenance.

**Authoritative sources** (required reading for implementation details):
- Part 21: https://www.mql5.com/en/articles/17891
- Part 66: https://www.mql5.com/en/articles/22194
- Part 69: https://www.mql5.com/en/articles/22503
- Part 67: https://www.mql5.com/en/articles/21961

Archived reference `.mq5` files live in `references/MQL5/lynnchris/implemented/`.

**Practical usage**: The [Price Action Patterns notebook](../../examples/notebooks/pa_flag_breakout_strategy.md) demonstrates complete batch + streaming examples, flag breakout strategy with sizing derived from `pole_length_atr`, regime + ML filters, and visual descriptions of every pattern and event type. This is the recommended starting point for the PA suite.

In addition, QuantWave ships 50+ classic candlestick patterns (Engulfing, Morning/Evening Star families, Harami, Hikkake variants, Three White Soldiers, etc.) plus Bill Williams Fractals — all documented individually under the Patterns section of the native index.

## Categories

### Classic & Overlap
Moving averages (SMA, EMA, HMA, KAMA, ALMA, FRAMA, T3, DEMA, TEMA, Zero-Lag variants), MACD, RSI, Stochastic, CCI, ADX, Bollinger Bands, ATR, Parabolic SAR, Vortex, and many more. See the Classic subsection in [Native Indicators](native/index.md).

### Ehlers DSP Suite
30+ specialized low-lag tools from John Ehlers: Cyber Cycle, Instantaneous Trendline, Trendflex, Reflex, Ehlers Filter, Ehlers Stochastic, Ehlers Loops, UltimateSmoother, Fisher & Inverse Fisher, MAMA/MESA, Mesa Stochastic, Hilbert Transform variants (dominant cycle, phasor, sine wave, trend vs cycle), Autocorrelation, Roofing Filter, SuperSmoother, Butterworth, Laguerre family, and more. Every native Ehlers page now follows [DOCUMENTATION_STANDARDS.md](../../DOCUMENTATION_STANDARDS.md) (mandatory sections, 3-surface code + parity, edge cases, core-path sources; visuals via `gen_indicator_previews.py` or precise placeholders). See [Ehlers DSP guide](ehlers/index.md) and the Ehlers DSP subsection in the native index.

### Volatility, Channels & Bands
Donchian Channels, Keltner Channels, TTM Squeeze, Ultimate Bands, SVE Volatility Bands, Exponential Deviation Bands, ATR Trailing Stop, and related.

### Volume & Flow
OBV, MFI, Accumulation/Distribution, Chaikin Oscillator, VFI, Volume Profile, Positive/Negative Volume, Anchored VWAP, and more.

### Patterns
- 50+ candlestick and simple patterns — all conform to [DOCUMENTATION_STANDARDS.md](../DOCUMENTATION_STANDARDS.md) (TA-Lib recognition rules from core, 3-surface examples, edge cases, no Nison boilerplate; visuals via `gen_candle_previews.py` where generators exist, otherwise precise placeholders). Exemplars: Doji family, Harami, Three Black Crows / White Soldiers, Abandoned Baby, Engulfing. See [Native Indicators → Patterns](native/index.md).
- Geometric chart patterns and S/R (detailed above)
- Bill Williams Fractals, Pivot Points, Heikin-Ashi, Ichimoku

### Advanced, Filters, Statistics & ML Features
Precision Trend Analysis, Choppiness Index, Schaff Trend Cycle, Gap Momentum, Hurst Exponent, multiple Kalman variants, One Euro Filter, Noise Elimination Technology, System Evaluator, Reversion Index, and the complete Regime Detection suite (HMM, GMM, PELT, etc. — see dedicated [Regimes guide](../regimes/index.md)).

### TA-Lib Compatible
Native, high-performance replacements for the complete TA-Lib function set (see [TA-Lib section](talib/index.md)).

## Getting the Most from QuantWave Indicators

- **Parity guarantee** — The same `Next<T>` implementation powers both the Polars plugins and all streaming structs.
- **Rich outputs where it matters** — Scalar values for classic indicators; full event structs with metadata for PA tools.
- **Production ready** — Zero-copy where possible, extensively proptested, and benchmarked (see [Benchmarks](../../benchmarks.md)).
- **Source transparency** — Every implementation records its primary reference per project standards.

## Next Steps

1. **Start coding immediately**
   - [Getting Started (Python)](../../getting-started/python.md)
   - [Getting Started (Rust)](../../getting-started/rust.md)

2. **See indicators in action**
   - [Batch & Streaming Examples](../../examples/batch-streaming.md)
   - [Price Action Flag Breakout Strategy Notebook](../../examples/notebooks/pa_flag_breakout_strategy.md) (essential for Geometric / Market Structure usage)

3. **Deepen your understanding**
   - [Native Indicators](native/index.md) — the complete browsable catalog with SUMMARY navigation
   - [Ehlers DSP Suite](ehlers/index.md)
   - [Documentation Standards](../../DOCUMENTATION_STANDARDS.md) (for contributors and page quality expectations)
   - [Benchmarks](../../benchmarks.md)

4. **Participate**
   - [Contributing](../../contributing.md)
   - GitHub repository for discussions and issues

The gallery is your map. The individual pages, notebooks, and source code are where the real depth lives. Everything is built for quantitative developers who need both correctness at scale and rich, actionable signals.

Browse the sidebar or search to jump straight to any indicator.
