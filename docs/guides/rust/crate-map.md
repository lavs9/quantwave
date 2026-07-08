# Rust Crate Map

QuantWave is a Cargo workspace. Pick the smallest crate that matches your integration layer.

## Crates at a glance

| Crate | Use when | docs.rs |
|-------|----------|---------|
| **quantwave-core** | Streaming indicators, features, regimes, `Next<T>` | [docs.rs/quantwave-core](https://docs.rs/quantwave-core) |
| **quantwave-polars** | `LazyFrame.ta()` / `.bt()` from Rust binaries | [docs.rs/quantwave-polars](https://docs.rs/quantwave-polars) |
| **quantwave-plugins** | Custom Polars expression plugins, zero-copy UDFs | [docs.rs/quantwave-plugins](https://docs.rs/quantwave-plugins) |
| **quantwave-backtest** | Vectorized portfolio simulation (no Python) | [docs.rs/quantwave-backtest](https://docs.rs/quantwave-backtest) |
| **quantwave** | Umbrella crate — one dependency for apps/examples | [docs.rs/quantwave](https://docs.rs/quantwave) |

Current workspace version: **0.6.0** (see [releases](../../releases/0.6.0.md)).

## Dependency recipes

### Indicators only (streaming)

```toml
[dependencies]
quantwave-core = "0.6"
```

### Polars batch pipeline

```toml
[dependencies]
quantwave-polars = "0.6"
polars = { version = "1", features = ["lazy"] }
```

### Backtest engine (Rust-native)

```toml
[dependencies]
quantwave-backtest = "0.6"
polars = { version = "1", features = ["lazy"] }
```

### Everything (binary / research tool)

```toml
[dependencies]
quantwave = "0.6"
polars = { version = "1", features = ["lazy"] }
```

## Python vs Rust boundary

| Capability | Python (`pip install quantwave`) | Rust crate |
|------------|----------------------------------|------------|
| `.ta()` on LazyFrame | ✅ primary DX | `quantwave-polars` |
| `.bt()` backtest | ✅ `quantwave._backtest` | `quantwave-backtest` |
| Streaming `Next<T>` | via parity / future bindings | `quantwave-core` |
| 221 indicator catalog | metadata + `.ta()` | `quantwave-core` + plugins |

For agent/LLM discovery, prefer MkDocs narrative pages + this map; use docs.rs for type-level API depth.

## Related

- [Rust guides index](index.md)
- [`Next<T>` streaming](next-trait.md)
- [Rust backtest entry](backtest.md)
- [Python getting started](../../getting-started/python.md)