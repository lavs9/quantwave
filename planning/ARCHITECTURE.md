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
