# Native Indicators

QuantWave implements a large set of technical indicators natively in Rust. These are exposed both as high-performance Polars expressions and as streaming `Next<T>` implementations.

All native indicators follow the project's core principles:
- Bit-identical results between batch (Polars) and streaming modes
- Full support for the `.ta()` namespace in Polars
- Rich metadata where applicable

## Categories

- **Classic Indicators** — SMA, EMA, RSI, MACD, ATR, SuperTrend, etc.
- **Pattern Recognition** — 50+ candlestick patterns (progressively professionalized under p1k6 to full DOCUMENTATION_STANDARDS.md conformance with visuals, exact TA-Lib rules, 3-surface examples, edges) + geometric price action detectors (Flags, Head & Shoulders, Market Structure, S/R). See dedicated exemplars in the Patterns subsection of the sidebar and the new visuals in `docs/assets/candlestick-previews/`.
- **Ehlers DSP Suite** — Advanced cycle and trend tools from John Ehlers (Cyber Cycle, Instantaneous Trendline, etc.)
- **Volatility & Trend** — Keltner, Donchian, TTM Squeeze, etc.
- **Price Action Foundation** — MarketStructure (swings + confirmed BOS), GeometricPatternScanner (Flags + H&S), SRMonitor.

## Price Action & Geometric Patterns

QuantWave provides production-ready implementations of the MQL5 "Price Action Analysis Toolkit" series (lynnchris):

- **Market Structure** (Part 21): Adaptive swing detection + bias tracking + confirmed Break of Structure flips (noise-free by design). Rich `MarketStructureState` + `PAEvent` output for strategies and ML.
- **Geometric Patterns** (Parts 66 + 69): Flag (continuation) and Head & Shoulders (reversal) detectors built on the Market Structure foundation. Emits rich `FlagPattern` / `HsPattern` structs with `pole_length_atr`, `height_atr`, symmetry, score, breakout flags — directly usable for position sizing and feature engineering.
- **S/R Monitoring** (Part 67): Real-time horizontal support/resistance interaction detection (touch, breakout, retest, reversal) with rich event metadata.

All are exposed via:
- Rust: `MarketStructure`, `GeometricPatternScanner`, `SRMonitor` (implement `Next`)
- Polars: `.ta().market_structure(...)`, `.ta().geometric_patterns(...)`
- Python streaming wrappers (parity with Rust)

**Key design principles**:
- Streaming-first with exact batch parity (proptests)
- Rich, serializable structs (no reliance on drawing objects)
- Confirmed signals only (bias must be established before flips/patterns)
- Perfect for event-driven backtesters and ML (features at event bars)

**Dedicated user guides** (recommended starting point for strategy & ML developers):
- [Market Structure (Swings + Confirmed BOS)](market_structure.md)
- [Geometric Patterns (Flags + Head & Shoulders)](geometric_patterns.md)
- [S/R Interactions](sr_monitor.md)
- [Using Rich PA Events & Metadata for Strategies and ML](pa_events_strategies.md)

See also the runnable strategy deep-dive: [Price Action Patterns notebook & examples](../examples/notebooks/pa_flag_breakout_strategy.md) (includes visual placeholders, sizing math from `pole_length_atr`, regime/ML filters, and Polars + streaming examples).

MQL5 sources (authoritative):
- Part 21: https://www.mql5.com/en/articles/17891
- Part 66: https://www.mql5.com/en/articles/22194
- Part 69: https://www.mql5.com/en/articles/22503
- Part 67: https://www.mql5.com/en/articles/21961

Archived .mq5 in `references/MQL5/lynnchris/implemented/`.

---

*Browse the full list of native indicators in the sidebar or use the search.*