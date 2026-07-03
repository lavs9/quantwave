# Rust Backtest Engine

`quantwave-backtest` is the vectorized simulation core behind Python's `LazyFrame.bt` API. Use it directly when building Rust-native research tools or services without the Python wheel.

!!! tip "Short answer"
    Build a long-format Polars `DataFrame` (timestamp, close, signal), configure `BacktestConfig`, run `BacktestEngine::run`, then read `trades`, `equity_curve`, `stats`, and `metrics()`.

## Python vs Rust

| | Python | Rust |
|---|--------|------|
| Entry | `df.lazy().bt.backtest_with_report(...)` | `BacktestEngine::new(config).run(lf)` |
| Output | `BacktestReport` PyO3 wrapper | `BacktestReport { result, metrics }` |
| Docs | [Backtest quickstart](../backtest/quickstart.md) | [docs.rs/quantwave-backtest](https://docs.rs/quantwave-backtest) |

Both paths share the same execution semantics (costs, fills, stops, portfolio modes).

## Minimal Rust sketch

```rust
use polars::prelude::*;
use quantwave_backtest::{BacktestConfig, BacktestEngine};

fn main() -> PolarsResult<()> {
    let df = df!(
        "timestamp" => &[1i64, 2, 3, 4, 5],
        "close" => &[100.0, 101.0, 99.0, 102.0, 103.0],
        "signal" => &[0.0, 1.0, 1.0, 0.0, 0.0],
    )?;

    let config = BacktestConfig::default();
    let engine = BacktestEngine::new(config);
    let report = engine.run(df.lazy())?;

    let metrics = report.metrics;
    println!("trades: {}", metrics.num_trades);
    println!("sharpe: {}", metrics.sharpe_ratio);
    Ok(())
}
```

## Output surfaces

| Accessor | Type | Notes |
|----------|------|-------|
| `result.trades` | Polars `DataFrame` | Trade blotter (`trade_id`, `side`, `entry_ts`, …) |
| `result.equity_curve` | Polars `DataFrame` | Per-bar `equity`, `cash`, `position`, `close` |
| `result.stats` | `HashMap<String, f64>` | `initial_cash`, `final_equity`, `net_pnl`, … |
| `report.metrics` | `PerformanceMetrics` | Sharpe, drawdown, win rate, CAGR, … |

!!! warning "Units"
    Return-like metrics (`total_return`, `cagr`, `max_drawdown_pct`) are **fractions**, not percent.
    `max_drawdown_pct` is a **positive** fraction (0.10 = 10% drawdown).
    A typed output contract is tracked in epic `quantwave-f2nn`.

## Streaming parity

`BacktestEngine::run_streaming_simulation` accepts any `Next<&Bar, Output = StrategySignal>` generator — useful for live-style loops with rich PA metadata. Batch vs streaming parity is tested in `quantwave-backtest/tests/`.

## Related

- [Backtest quickstart (Python)](../backtest/quickstart.md)
- [Capability matrix](../backtest/capability_matrix.md)
- [Crate map](crate-map.md)
- [docs.rs — quantwave-backtest](https://docs.rs/quantwave-backtest)