# Getting Started with Rust

QuantWave is built in Rust and provides high-performance crates for both core logic and Polars integration.

!!! tip "New here?"
    Start with the [Getting Started funnel](../index.md) if you have not installed yet.
    Python/Polars is the primary DX path — this page is for Rust-native or embedded use.

## Installation

Add the crates you need to your `Cargo.toml`:

```toml
[dependencies]
quantwave-core = "0.1"
quantwave-polars = "0.1"
```

## Quick Start (Polars)

```rust
use polars::prelude::*;
use quantwave_polars::TA;

fn main() -> PolarsResult<()> {
    let df = df.lazy()
        .ta()
        .rsi("close", 14)
        .collect()?;
    
    Ok(())
}
```

## Quick Start (Streaming)

```rust
use quantwave_core::indicators::RSI;
use quantwave_core::traits::Next;

fn main() {
    let mut rsi = RSI::new(14);
    
    for price in prices {
        let value = rsi.next(price);
        println!("RSI: {:?}", value);
    }
}
```

## Where to go next

| Goal | Next step |
|------|-----------|
| Polars from Python | [Python getting started](../python.md) |
| Indicator catalog | [Full catalog](../../guides/indicators/native/) |
| Architecture | `quantwave-core` traits (`Next<T>`) power both streaming structs and Polars plugins |
| Benchmarks | [Performance numbers](../../benchmarks.md) |
| Full funnel | [Getting Started hub](../index.md) |
