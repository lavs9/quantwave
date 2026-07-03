# Getting Started with Rust

QuantWave is built in Rust and provides high-performance crates for core logic, Polars integration, and backtesting.

!!! tip "Short answer"
    Most users should start with [Python + Polars](python.md). This page is for Rust-native binaries, embedded services, or library contributors.

## Installation

Add the crates you need to your `Cargo.toml` (workspace version **0.6.0**):

```toml
[dependencies]
quantwave-core = "0.6"
quantwave-polars = "0.6"
# optional:
quantwave-backtest = "0.6"
```

## Quick Start (Polars)

```rust
use polars::prelude::*;
use quantwave_polars::QuantWaveExt;

fn main() -> PolarsResult<()> {
    let df = df!("close" => &[44.0, 44.5, 43.8, 44.2, 45.0])?;
    let out = df.lazy().ta().rsi("close", 14).collect()?;
    println!("{out}");
    Ok(())
}
```

## Quick Start (Streaming)

```rust
use quantwave_core::indicators::RSI;
use quantwave_core::traits::Next;

fn main() {
    let mut rsi = RSI::new(14);
    for price in [44.0, 44.5, 43.8, 44.2, 45.0] {
        println!("RSI: {:?}", rsi.next(price));
    }
}
```

## Rust guides (MkDocs)

Curated concepts — full API depth lives on docs.rs:

| Guide | Topic |
|-------|-------|
| [Rust overview](../guides/rust/index.md) | When to use Rust vs Python |
| [Crate map](../guides/rust/crate-map.md) | Which crate + docs.rs links |
| [`Next<T>` pattern](../guides/rust/next-trait.md) | Streaming indicators |
| [Backtest engine](../guides/rust/backtest.md) | `quantwave-backtest` entry |

## docs.rs API reference

- [quantwave-core](https://docs.rs/quantwave-core) — indicators, `Next<T>`, features
- [quantwave-polars](https://docs.rs/quantwave-polars) — `.ta()` / `.bt()` from Rust
- [quantwave-backtest](https://docs.rs/quantwave-backtest) — simulation engine
- [quantwave-plugins](https://docs.rs/quantwave-plugins) — Polars expression plugins
- [quantwave](https://docs.rs/quantwave) — umbrella crate

## Where to go next

| Goal | Next step |
|------|-----------|
| Polars from Python | [Python getting started](python.md) |
| Indicator catalog | [Full catalog](../guides/indicators/native/) |
| Backtest from Python | [Backtest quickstart](../guides/backtest/quickstart.md) |
| Benchmarks | [Performance numbers](../benchmarks.md) |
| Full funnel | [Getting Started hub](index.md) |