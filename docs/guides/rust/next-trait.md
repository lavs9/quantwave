# The `Next<T>` Pattern

`Next<Input>` is QuantWave's universal indicator trait. One implementation powers streaming state machines, Polars batch columns, and gold-standard tests.

!!! tip "Short answer"
    Implement `Next<f64>` (or `Next<&Bar>` for rich signals), call `.next(input)` per bar, and batch Polars paths will match if they share the same core logic.

## Trait definition

```rust
pub trait Next<Input> {
    type Output;
    fn next(&mut self, input: Input) -> Self::Output;
}
```

Source: `quantwave_core::traits::Next` — see [docs.rs](https://docs.rs/quantwave-core/latest/quantwave_core/traits/trait.Next.html).

## Streaming RSI example

```rust
use quantwave_core::indicators::RSI;
use quantwave_core::traits::Next;

fn main() {
    let mut rsi = RSI::new(14);
    let prices = [44.0, 44.5, 43.8, 44.2, 45.0];

    for price in prices {
        let value = rsi.next(price);
        println!("RSI: {value:?}");
    }
}
```

Warmup bars return `None` until the lookback is satisfied — same semantics as Polars `.ta().rsi(period=14)`.

## Why it matters

| Layer | How `Next<T>` is used |
|-------|----------------------|
| **Streaming** | Stateful struct, one `next()` per tick |
| **Polars batch** | Expression plugin walks the series with identical math |
| **Tests** | `proptest` asserts `batch == streaming.collect()` |
| **Backtest (advanced)** | `Next<&Bar, Output = StrategySignal>` drives streaming simulation |

## Related traits

- **`SmoothingAlgorithm`** — swappable MA inside composite indicators (SuperTrend, Keltner, …)
- **`IndicatorConfig`** — builds a fresh `Next` state machine from parameters

## Steel-thread reference

[SuperTrend](../indicators/native/supertrend.md) is the canonical end-to-end indicator: core `Next` impl, Polars `.ta()`, gold-standard JSON, and documentation.

## Further reading

- [Rust guides index](index.md)
- [Crate map](crate-map.md)
- [docs.rs — quantwave_core::traits](https://docs.rs/quantwave-core/latest/quantwave_core/traits/index.html)