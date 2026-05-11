# QuantWave 🌊

**High-performance, Polars-native Technical Analysis for Rust.**

QuantWave is a modern technical analysis library built from the ground up for the [Polars](https://github.com/pola-rs/polars) ecosystem. It bridges the gap between **high-speed batch backtesting** and **real-time streaming execution** by ensuring bit-identical results across both modes.

---

## 🚀 Key Features

- **Polars Native:** Built specifically for Polars `LazyFrame` and `Series` with zero-copy expression plugins.
- **Streaming-Batch Parity:** Every indicator implements the "Universal Indicator" pattern, guaranteeing identical results for batch (backtesting) and streaming (live trading).
- **SIMD Optimized:** Leverages `talib-rs` and `talib-rs-core` for industry-standard performance.
- **Modern Indicator Suite:** Beyond classic TA-Lib, QuantWave includes modern DSP suites (Ehlers), ML feature engineering tools, and market structure indicators.
- **Bit-Identical Validation:** All indicators are verified against a "Gold Standard" test suite using `proptest` and industry reference vectors.

---

## 🏗 Architecture

QuantWave is structured as a modular Rust workspace:

- **`quantwave-core`**: The foundational engine. Contains the `Next<T>` trait, streaming state machines, and mathematical primitives.
- **`quantwave-polars`**: High-level ergonomics. Provides the `.ta()` namespace extension for Polars objects.
- **`quantwave-plugins`**: Performance-critical Polars Expression Plugins (UDFs) that operate directly on Arrow buffers.

---

## 🛠 Installation

Add QuantWave to your `Cargo.toml`:

```toml
[dependencies]
quantwave = "0.1"
polars = "0.46"
```

---

## 📖 Usage

### Batch Processing (Polars)

```rust
use polars::prelude::*;
use quantwave_polars::TA;

let df = df.lazy()
    .with_column(
        col("close").ta().supertrend(10, 3.0).alias("supertrend")
    )
    .collect()?;
```

### Streaming Processing

```rust
use quantwave_core::indicators::SuperTrend;
use quantwave_core::traits::Next;

let mut st = SuperTrend::new(10, 3.0);
for price in prices {
    let signal = st.next(price);
    println!("Signal: {:?}", signal);
}
```

---

## 🗺 Roadmap

### Phase 1: Foundation (In Progress)
- [x] Workspace & Core Traits (`Next<T>`)
- [x] Gold Standard Testing Framework
- [x] Steel Thread: SuperTrend, VWAP, HMA, ALMA
- [ ] Integration of 158 `talib-rs` indicators

### Phase 2: Modern Suite
- [ ] Ehlers DSP Suite (Cyber Cycle, Laguerre RSI)
- [ ] WaveTrend Oscillator
- [ ] Ichimoku Cloud

### Phase 3: ML & Market Structure
- [ ] Volume Profile & Order Flow tools
- [ ] Kalman Filters & Hurst Exponent
- [ ] Fractal-based Market Structure

---

## 🧪 Development & Testing

QuantWave uses a rigorous validation pipeline:

1. **Unit Tests:** Traditional logic verification.
2. **Gold Standard:** Verification against JSON-encoded reference vectors.
3. **Parity Tests:** `proptest` ensures `Batch(data) == Streaming.collect(data)`.

```bash
# Run all tests
cargo test

# Check linting
cargo clippy
```

---

## 🤝 Contributing

QuantWave uses **Beads** for issue tracking. 

- Check for ready work: `bd ready`
- Claim a task: `bd update <id> --claim`

---

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
