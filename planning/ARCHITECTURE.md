# QuantWave Architecture Design

## 1. Overview
QuantWave is a high-performance, Polars-native technical analysis library for Rust. It is designed to provide bit-identical results between batch processing (backtesting) and streaming processing (live trading).

## 2. Crate Structure (Workspace)
- `quantwave-rs` (Root): Workspace configuration.
- `quantwave-core`: The primary engine. Contains traits, state machines, and streaming implementations.
- `quantwave-polars`: Polars-specific integration. Defines the `ta()` namespace on LazyFrame/Series.
- `quantwave-plugins`: A specialized crate for Polars Expression Plugins (UDFs) to ensure maximum performance.

## 3. Core Traits & Design Patterns

### A. The `Next<I, O>` Trait (Streaming)
The foundation of every indicator. It maintains internal state and processes data points one by one.
```rust
pub trait Next<Input> {
    type Output;
    fn next(&mut self, input: Input) -> Self::Output;
}
```

### B. The `IndicatorConfig` Trait (Generics)
Enables "Depth over Breadth" by allowing indicators to be generic over their smoothing algorithms.
```rust
pub trait IndicatorConfig {
    type Indicator: Next<f64, Output = f64>;
    fn build(&self) -> Self::Indicator;
}
```

### C. Polars Expression Plugins
Instead of `map_batches`, custom indicators are registered as Polars plugins. This allows zero-copy access to Arrow buffers and bypasses the GIL when called from Python.

## 4. Integration with `talib-rs-core`
- **Classic Indicators:** QuantWave wraps `talib-rs-core` functions using macros to provide a Polars-native API.
- **Modern/Custom Indicators:** Built from scratch using the `Next` trait and optimized for Polars via the plugin system.

## 5. Performance Strategy
- **SIMD:** Utilize `wide` or `packed_simd` (via `talib-rs-core`) for vectorized aggregates.
- **Incremental Math:** All recursive indicators must use O(1) or O(log N) update formulas where mathematically possible.
- **Zero-Copy:** Polars Plugins operate directly on the underlying `ChunkedArray` memory.

## 6. Testing & Validation
- **Parity:** `proptest` ensures `Batch(data) == Streaming.collect(data)`.
- **Fidelity:** `tests/gold_standard/*.json` provides ground-truth validation against industry references.
- **Tolerance:** Use the `approx` crate with relative/absolute tolerances for floating-point comparisons.

## 7. Backtester Streaming Simulation + Batch/Streaming Parity (quantwave-ug9t)
The `quantwave-backtest` crate (delivered via quantwave-1hr + ug9t) enforces the project's core promise for strategies:

- **Batch path**: `BacktestEngine::run` (and convenience `backtest_simple_bool_signal`) on long-format DF with pre-computed scalar signal column (now generalized: f64 value = desired exposure for sizing).
- **Streaming path**: `run_streaming_simulation(bars, generator: impl Next<&Bar, Output=StrategySignal>, config)` — drives exact same causal execution using user Next<T> state machines (features, regimes, rich PA).
- **Parity guarantee**: Single `run_simulation` helper (private) is the source of truth for costs, fills, equity, Trade recording (incl. quantity from exposure + entry_metadata). Same signals → bit-identical equity curves, trade lists (within tol), stats.
- **Rich support (for 06sz/366/4ps)**: `StrategySignal` carries `exposure` (pole-height sized) + `metadata: HashMap` (pole_height, regime, cycle_momentum etc). `Trade` now stores `quantity` + `entry_metadata`.
  The canonical end-to-end exercising the 4ps ML features surface (.ta().features()) + rich metadata flow + batch/streaming parity is docs/examples/notebooks/ml_feature_backtest_parity.py (primary closure evidence for quantwave-4ps + quantwave-gwx).
- **Mandatory verification**: At least one test exercising regime filter (TAR) + feature threshold (CyberCycle momentum) + rich PA pole-height sizing struct. Equity tol 1e-8 rel/eps; stats/trade pnls 1e-6; exact counts. Documented failure modes in lib.rs.
- **Sources**: See quantwave-backtest/src/lib.rs header (Yvictor/polars-backtest, vectorbt clean-room, MQL5 PA via 366, Ehlers artifacts, core test_utils parity pattern + TAR/CyberCycle impls).
- **No new files**: All in lib.rs (per "never create unless necessary"). Tests inside crate (#[cfg(test)]). Uses cargo nextest + clippy clean.
- **Date note (per conventions)**: Work completed 2026-05-30 IST.

This separates QuantWave backtesting: any strategy using ML features or rich PA is proven identical in research (batch) vs live-like (streaming) modes.
