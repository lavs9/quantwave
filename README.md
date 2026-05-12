# QuantWave 🌊

[![DeepWiki](https://deepwiki.com/badge-maker?url=https%3A%2F%2Fdeepwiki.com%2Flavs9%2Fquantwave)](https://deepwiki.com/lavs9/quantwave)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Documentation](https://img.shields.io/badge/docs-mdBook-blue)](https://lavs9.github.io/quantwave/)

**High-performance, Polars-native Technical Analysis for Rust.**

QuantWave is a modern technical analysis library built from the ground up for the [Polars](https://github.com/pola-rs/polars) ecosystem. It bridges the gap between **high-speed batch backtesting** and **real-time streaming execution** by ensuring bit-identical results across both modes.

Whether you are performing quantitative research over terabytes of historical data or deploying a live trading system on a tick-by-tick stream, QuantWave delivers industry-standard accuracy and extreme performance.

---

## 🚀 Why QuantWave?

- **Polars Native:** Built specifically for Polars `LazyFrame` and `Series` with zero-copy expression plugins. Say goodbye to converting to/from `Vec<f64>` or `ndarray` to calculate your indicators.
- **Streaming-Batch Parity:** Every indicator implements the "Universal Indicator" pattern, guaranteeing mathematically identical results for batch (backtesting) and streaming (live trading).
- **Comprehensive Suite:** Featuring 150+ standard indicators (via robust TA-Lib wrapping) alongside modern DSP suites (Ehlers), ML feature engineering tools, and market structure algorithms.
- **Bit-Identical Validation:** Sleep well at night. All indicators are rigorously verified against an extensive "Gold Standard" test suite using `proptest` and industry reference vectors.

---

## 📚 Documentation & Resources

For detailed indicator formulas, parameter definitions, and architectural deep-dives, please refer to our official documentation sites:

- 📖 **[QuantWave Indicator Bible (mdBook)](https://lavs9.github.io/quantwave/)**: Comprehensive reference for all native and TA-Lib indicators, complete with LaTeX math formulas.
- 🧠 **[DeepWiki Integration](https://deepwiki.com/lavs9/quantwave)**: Explore our system architecture, decision logs, and agentic workflows.

---

## 🛠 Installation

Add QuantWave to your `Cargo.toml`:

```toml
[dependencies]
quantwave = "0.1"
polars = "0.46"
```

---

## 📖 Quick Start

QuantWave is designed to be completely intuitive whether you are processing historical dataframes or processing live WebSocket streams.

### 1. Batch Processing (Backtesting / Research)

Extend Polars with the `.ta()` namespace to rapidly compute indicators across your entire dataset.

```rust
use polars::prelude::*;
use quantwave_polars::TA;

let df = df.lazy()
    // Calculate SuperTrend with Period=10, Multiplier=3.0
    .with_column(
        col("close").ta().supertrend(10, 3.0).alias("supertrend")
    )
    .collect()?;
```

### 2. Streaming Processing (Live Trading)

Use the core structs directly to process incoming ticks one by one without reallocating arrays or maintaining complex state buffers.

```rust
use quantwave_core::indicators::SuperTrend;
use quantwave_core::traits::Next;

// Initialize state machine once
let mut st = SuperTrend::new(10, 3.0);

// Feed it tick by tick in your live event loop
for price in live_price_stream {
    let signal = st.next(price);
    println!("Latest SuperTrend Value: {:?}", signal);
}
```

---

## 🧪 Rigorous Validation

QuantWave is built for institutional-grade reliability. We validate our calculations through a rigorous three-tier pipeline:

1. **Unit Tests:** Ensuring edge cases and bounds are handled safely.
2. **Gold Standard Verification:** Comparing outputs against JSON-encoded reference vectors sourced from TradingView, MetaTrader, and established platforms.
3. **Parity Tests:** Proptest suites that continuously enforce `Batch(data) == Streaming.collect(data)`.

---

## 🤝 Contributing & Issue Tracking

QuantWave uses **Beads** (`bd`) for deterministic, graph-aware issue tracking to ensure high-velocity agentic and human collaboration.

- Check for ready work: `bd ready`
- Claim a task: `bd update <id> --claim`

---

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
