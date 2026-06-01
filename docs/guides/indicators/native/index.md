# Native Indicators

QuantWave ships **150+ production-grade indicators** implemented natively in Rust, with first-class exposure through both Polars (`.ta()` expressions) and lightweight streaming wrappers (`Next<T>`).

Every indicator guarantees **bit-identical results** between batch and streaming modes, validated through property-based tests against gold-standard vectors.

All pages in this section follow the project's [Documentation Standards](../../DOCUMENTATION_STANDARDS.md): practical explanations, mathematical definitions or recognition logic, parameter tables, 3-surface usage examples, edge cases, visuals (or high-quality placeholders), and authoritative sources.

## Featured High-Value Areas

These stand out for strategy development, risk management, and ML feature engineering:

- **Price Action & Geometric Patterns** (MQL5 lynnchris toolkit)  
  Market Structure (confirmed BOS), Flags + Head & Shoulders with rich sizing metadata (`pole_length_atr`), and S/R interaction monitoring. See the four dedicated guides below and the [Price Action Patterns notebook](../examples/notebooks/pa_flag_breakout_strategy.md).

- **Ehlers DSP Suite**  
  Low-lag cycle and trend tools (Cyber Cycle, Instantaneous Trendline, Trendflex, Reflex, SuperSmoother, etc.). Exceptional performance and minimal lag compared to traditional indicators.

- **Modern & Regime-Aware Tools**  
  TTM Squeeze, Choppiness Index, Hurst Exponent, multiple Kalman variants, and the full Regimes suite (HMM, GMM, PELT, etc.).

- **Classic Production Workhorses**  
  SuperTrend, Ichimoku, Keltner, Donchian, RSI family, MACD, ATR-based tools, and the complete TA-Lib compatible set.

## Organization

The sidebar is organized into 8 logical top-level groups that match the categories on this page:

- Classic & Overlap
- Ehlers DSP Suite
- Price Action & Geometric Patterns (prominently featured)
- Volatility, Channels & Trend
- Volume & Flow
- Candlestick & Simple Patterns
- Advanced, ML Features & Regimes
- TA-Lib Compatible

Use the search or the categories in the left navigation to browse. Most users will get the most value from the **Price Action** and **Ehlers DSP** groups.

## Featured Individual Indicators (Strong Starting Points)

These specific pages are particularly well-regarded and representative:

- [SuperTrend](native/supertrend.md) — Robust trend-following with dynamic stops (one of the most used in production).
- [Cyber Cycle](native/cyber_cycle.md) — Foundational Ehlers low-lag cycle extractor.
- [Market Structure](native/market_structure.md) — Core for confirmed swings and BOS (foundation for all PA tools).
- [TTM Squeeze](native/ttm_squeeze.md) — Volatility contraction/expansion for high-probability breakouts.
- [Ichimoku Cloud](native/ichimoku_cloud.md) — Complete trend, momentum, and dynamic S/R system.
- [Geometric Patterns](native/geometric_patterns.md) — Flags and H&S with rich `pole_length_atr` for sizing.

## Important Notes

- Detailed formulas, visuals, edge cases, and rich usage examples live on the individual indicator pages (not here). Those pages are the primary reference.
- Many pages now include professional visuals generated via the strategies in `DOCUMENTATION_DECISIONS.md`.
- The auto-generated Python API reference (`/api/`) documents the thin Python surface and is intentionally secondary to the manual guides.

## Getting Started with Native Indicators

- [Indicator Gallery](../gallery.md) — curated high-value starting points
- [Batch & Streaming Examples](../../examples/batch-streaming.md)
- [Documentation Standards](../../DOCUMENTATION_STANDARDS.md) (for contributors)

Browse the full list in the sidebar or use the search box above.