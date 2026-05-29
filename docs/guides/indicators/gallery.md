# Indicator Gallery

Explore the wide range of technical indicators available in QuantWave. Every indicator is optimized for both Polars batch processing and real-time streaming.

> **Prototype Preview Generator (quantwave-7x1 spike)** — This section shows the first automated static previews generated during development. These are synthetic for the spike but demonstrate the visual direction (small, clean, consistent thumbnails that will later be produced from real QuantWave calculations during `mkdocs build`).

<div style="display: flex; gap: 16px; flex-wrap: wrap; margin: 16px 0;">
  <div style="text-align: center;">
    ![SuperTrend preview](../../assets/indicator-previews/supertrend.png){ width="220" }
    <div style="font-size: 0.75rem; color: #64748b; margin-top: 4px;">SuperTrend</div>
  </div>
  <div style="text-align: center;">
    ![Cyber Cycle preview](../../assets/indicator-previews/cyber_cycle.png){ width="220" }
    <div style="font-size: 0.75rem; color: #64748b; margin-top: 4px;">Cyber Cycle (Ehlers)</div>
  </div>
  <div style="text-align: center;">
    ![RSI preview](../../assets/indicator-previews/rsi.png){ width="220" }
    <div style="font-size: 0.75rem; color: #64748b; margin-top: 4px;">RSI (14)</div>
  </div>
</div>

## Interactive Gallery

[Placeholder for interactive gallery component]

> **Future-proof**: We plan to auto-generate preview charts via `docs/gen_python_api.py` or a Rust script so this gallery always stays in sync with the 150+ indicators.

## Categories

### Native Indicators
Classic indicators like SMA, EMA, RSI, and MACD, re-implemented in high-performance Rust.

### Ehlers DSP Suite
Advanced digital signal processing indicators by John Ehlers, including CyberCycle, Laguerre RSI, and more.

### Trend & Volatility
Indicators designed to identify market trends and measure volatility, such as SuperTrend and ATR.

---

*Explore individual guides for detailed formulas and usage.*
