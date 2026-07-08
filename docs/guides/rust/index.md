# Rust Guides

QuantWave's Rust workspace powers **221** native indicators, Polars integration, and the backtest engine. These guides explain *when* and *how* to use Rust directly — without duplicating the full [docs.rs](https://docs.rs/quantwave-core) API reference.

!!! tip "Short answer"
    Use **Python + Polars** (`pip install "quantwave[polars]"`) for research and backtests.
    Use **Rust crates** when embedding indicators in a service, building custom binaries, or contributing to the library.

## Choose your path

| Goal | Start here |
|------|------------|
| First Rust install | [Getting Started: Rust](../../getting-started/rust.md) |
| Which crate to import | [Crate map](crate-map.md) |
| Streaming / live loops | [`Next<T>` pattern](next-trait.md) |
| Simulation without Python | [Rust backtest](backtest.md) |
| Full API reference | [docs.rs — quantwave-core](https://docs.rs/quantwave-core) |

## Workspace layout

```
quantwave-core     → Next<T> indicators, features, regimes (single source of math)
quantwave-polars   → LazyFrame .ta() / .bt() namespace
quantwave-plugins  → Polars expression plugins (vectorized UDFs)
quantwave-backtest → Portfolio simulation engine
quantwave          → Umbrella re-exports for binary crates
```

## Batch vs streaming parity

Every indicator implements [`Next<Input>`](next-trait.md) in `quantwave-core`. That implementation is the **single source of truth** for:

- Streaming structs (`rsi.next(price)`)
- Polars batch columns (`.ta().rsi(...)`)
- Gold-standard validation vectors

Python's `quantwave.assert_parity()` exists because this contract is enforced in CI.

## Next steps

- [Crate map → docs.rs links](crate-map.md)
- [SuperTrend steel-thread reference](../indicators/native/supertrend.md) (formula + parity)
- [Benchmarks](../../benchmarks.md)
- [Contributing](../../contributing.md)