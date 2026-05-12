# QuantWave 🌊

**High-performance, Polars-native Technical Analysis for Rust.**

QuantWave is a modern technical analysis library built from the ground up for the Polars ecosystem. It bridges the gap between high-speed batch backtesting and real-time streaming execution by ensuring bit-identical results across both modes.

Whether you are performing quantitative research over terabytes of historical data or deploying a live trading system on a tick-by-tick stream, QuantWave delivers industry-standard accuracy and extreme performance.

## Design Philosophy
1. **Universal Indicator Pattern:** Every indicator guarantees identical results for batch and streaming.
2. **Zero-Copy Performance:** Native Polars plugins operate directly on Arrow memory buffers.
3. **Rigorous Validation:** Every indicator is tested against industry gold-standard data (TradingView, MetaTrader) to ensure correctness.

Select an indicator from the sidebar to view its mathematical formula, parameters, and documentation.
