# Native Indicators

QuantWave implements a large set of technical indicators natively in Rust. These are exposed both as high-performance Polars expressions and as streaming `Next<T>` implementations.

All native indicators follow the project's core principles:
- Bit-identical results between batch (Polars) and streaming modes
- Full support for the `.ta()` namespace in Polars
- Rich metadata where applicable

## Categories

- **Classic Indicators** — SMA, EMA, RSI, MACD, ATR, SuperTrend, etc.
- **Pattern Recognition** — Candlestick patterns and geometric price action detectors (Flags, Head & Shoulders, etc.)
- **Ehlers DSP Suite** — Advanced cycle and trend tools from John Ehlers (Cyber Cycle, Instantaneous Trendline, etc.)
- **Volatility & Trend** — Keltner, Donchian, TTM Squeeze, etc.

## Research & Implementation Notes

Many of the more advanced geometric and price-action patterns (e.g. Flags, Head & Shoulders) are based on the excellent MQL5 research series by lynnchris:

- Part 66 (Head & Shoulders)
- Part 69 (Flag Pattern Detection)
- Part 21 (Market Structure)

The geometric pattern detectors (Flags, Head & Shoulders) draw from the MQL5 Price Action series by lynnchris (Parts 66 and 69), with market structure foundations from Part 21.

---

*Browse the full list of native indicators in the sidebar or use the search.*