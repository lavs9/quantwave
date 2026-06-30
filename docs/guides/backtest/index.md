# Backtest Engine

QuantWave ships a **Polars-native, clean-room backtest engine** (`quantwave-backtest`) with Python `.bt` namespace ergonomics.

## The Moat

QuantWave guarantees **batch ↔ streaming parity**. 
The same strategy logic produces identical equity/trades in batch (precomputed Polars LazyFrame) and streaming (`Next<T>`) modes. The math is guaranteed identical because both Python Polars plugins and the live streaming engine are bound to the identical underlying Rust traits.

## `.bt` API Surface

| Method | Purpose |
|--------|---------|
| `lf.bt.backtest()` | Trades + equity DataFrames |
| `lf.bt.backtest_with_report()` | Above + `PerformanceMetrics` |
| `lf.bt.backtest_metrics()` | Metrics only (no trades/equity DF) |
| `lf.bt.sweep()` | Pre-built signal column grid |
| `lf.bt.sweep_callback()` | Rebuild signals per param via `build_fn` |
| `lf.bt.walk_forward()` | Rolling OOS folds |
| `lf.bt.walk_forward_optimize()` | Train-window sweep + locked OOS param |
| `lf.bt.cross_sectional_backtest()` | Universe rank long/short (`transform=` optional) |

## Quick Links

- [Quickstart](quickstart.md) (5 min)
- [Capability Matrix](capability_matrix.md)
- [Tear Sheets](tear_sheets.md)
- [Backtest Showcase](../../examples/notebooks/backtest_showcase.md)
- [Portfolio Shared Capital](../../examples/notebooks/portfolio_shared_capital_backtest.md)
- [Backtest Benchmarks](../../examples/notebooks/backtest_benchmark.md)
- [Strategy Backtest](../../examples/notebooks/strategy_backtest.md)
- [PA Flag Breakout](../../examples/notebooks/pa_flag_breakout_strategy.md)
- [ML Features → Backtest E2E](../../examples/notebooks/ml_feature_backtest_parity.md)
