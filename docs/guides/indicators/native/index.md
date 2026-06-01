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

The sidebar is organized into logical groups (Classic, Ehlers DSP, Patterns, Volatility, Volume & Flow, ML Features & Regimes, Options India, etc.).

Use the search or the categories in the left navigation to browse. Most users will get the most value from:

- The **Price Action** subsection (for event-driven and ML work)
- The **Ehlers DSP** subsection (for low-lag cycle analysis)
- High-signal classics such as SuperTrend, TTM Squeeze, and Ichimoku

## Important Notes

- Detailed formulas, visuals, edge cases, and rich usage examples live on the individual indicator pages (not here). Those pages are the primary reference.
- Many pages now include professional visuals generated via the strategies in `DOCUMENTATION_DECISIONS.md`.
- The auto-generated Python API reference (`/api/`) documents the thin Python surface and is intentionally secondary to the manual guides.

## Getting Started with Native Indicators

- [Indicator Gallery](../gallery.md) — curated high-value starting points
- [Batch & Streaming Examples](../../examples/batch-streaming.md)
- [Documentation Standards](../../DOCUMENTATION_STANDARDS.md) (for contributors)

Browse the full list in the sidebar or use the search box above.