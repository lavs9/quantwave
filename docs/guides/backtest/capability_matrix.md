# Backtest Engine — Capability Matrix

QuantWave ships a **Polars-native, clean-room backtest engine** (`quantwave-backtest`) with Python `.bt` namespace ergonomics. It is **vectorbt-inspired** research UX on top of QuantWave's **batch ↔ streaming parity** moat.

## Executive summary

**What it is:**

- Long-format LazyFrame input (single- and multi-symbol)
- Realistic costs, execution delay, stops, shorts, sizing filters
- Research analytics: sweeps, walk-forward optimization, Monte Carlo, cross-sectional panels
- Shared-capital portfolio simulation across symbols
- HTML tear sheets and rich `PerformanceMetrics`
- Rich PA/ML metadata preserved into trades

**What it is not (yet):**

- Live order routing (Nautilus bridge deferred)
- Wide-format matrix / portfolio optimization engine

---

## The moat

> **Same strategy logic → identical equity/trades in batch (precomputed DF) and streaming (`Next<T>`) modes.**

| Artifact | Location |
|----------|----------|
| Parity integration test | `quantwave-backtest/src/lib.rs` — `test_batch_vs_streaming_parity_*` |
| Batch/streaming guide | [examples/batch-streaming.md](../../examples/batch-streaming.md) |
| ML features E2E parity notebook | [ml_feature_backtest_parity.md](../../examples/notebooks/ml_feature_backtest_parity.md) |

---

## Feature matrix

Legend: ✅ Shipped · ⏸ Deferred · ❌ Out of scope

### Core user-facing

| # | Requirement | Status | API / module | Proof |
|---|-------------|--------|--------------|-------|
| 1 | Python `BacktestEngine` + config (PyO3) | ✅ | `quantwave.backtest.BacktestEngine` | `test_backtest_engine_run_single_trade` |
| 2 | `.bt.backtest()` / `.bt.backtest_with_report()` | ✅ | `quantwave/bt_polars.py` | `test_bt_backtest_with_report` |
| 3 | `PerformanceMetrics` (Sharpe, Sortino, max DD, CAGR, win rate, PF) | ✅ | `quantwave-backtest/src/metrics.rs` | `test_backtest_metrics_dict_keys` |
| 4 | Multi-symbol long-format grouping | ✅ | `BacktestEngine::run` | `test_backtest_multi_symbol_*` |
| 5 | `entry_filter_col` + `size_multiplier_col` | ✅ | `BacktestConfig` | `test_backtest_entry_filter_*` |
| 6 | ML feature → backtest E2E notebook | ✅ | notebook | [ml_feature_backtest_parity.md](../../examples/notebooks/ml_feature_backtest_parity.md) |
| 7 | Strategy backtest notebook | ✅ | notebook | [strategy_backtest.md](../../examples/notebooks/strategy_backtest.md) |

### Execution depth

| # | Requirement | Status | API | Proof |
|---|-------------|--------|-----|-------|
| 8 | T+1 execution (`execution_delay`) | ✅ | `BacktestConfig.execution_delay` | nextest `execution_delay` |
| 9 | Stop-loss / take-profit / trailing | ✅ | `StopConfig` | nextest `stop_*` |
| 10 | Short positions (signed exposure) | ✅ | signal f64 negative | nextest `short_*` |
| 11 | Struct signal column auto-parse + pole sizing | ✅ | `signal_col` Struct | nextest struct signal tests |
| 12 | Param sweep helper | ✅ | `.bt.sweep()` | `test_bt_sweep_*` |
| 13 | Criterion benches vs naive loop | ✅ | `benches/backtest_vs_naive.rs` | [backtest_benchmark.md](../../examples/notebooks/backtest_benchmark.md) |

### Research robustness

| # | Requirement | Status | API | Proof |
|---|-------------|--------|-----|-------|
| 14 | Walk-forward OOS | ✅ | `.bt.walk_forward()` | `test_bt_walk_forward_returns_folds` |
| 14b | Walk-forward with in-fold optimization | ✅ | `.bt.walk_forward_optimize()` | `test_wfo_opt_*` |
| 15 | Monte Carlo (trade bootstrap) | ✅ | `monte_carlo_trade_bootstrap` | `quantwave-backtest/tests/p2_features.rs` |
| 15b | Monte Carlo (return-path VaR/CVaR) | ✅ | `monte_carlo_return_paths` | `monte_carlo.rs` tests |
| 16 | Cross-sectional factor panel | ✅ | `.bt.cross_sectional_backtest()` | `test_cross_sectional_*` |
| 16b | Factor transforms (neutralize, zscore, winsorize) | ✅ | `transform=` kwarg | `test_bt_cross_sectional_*` |
| 17 | Nautilus live bridge | ⏸ | `LiveBridge` trait stub | planning ADR |
| 18 | HTML tear sheets | ✅ | `tearsheet.render_html` | `test_tearsheet.py` |
| 19 | Shared-capital portfolio backtest | ✅ | `.bt.portfolio_backtest()` | `test_portfolio_backtest.py`, [portfolio notebook](../../examples/notebooks/portfolio_shared_capital_backtest.md) |

### Additional shipped features

| Feature | Status | API | Proof |
|---------|--------|-----|-------|
| PA flag → `.bt` E2E | ✅ | PA notebook + tests | `test_pa_flag_backtest_*`, [pa_flag_breakout_strategy.md](../../examples/notebooks/pa_flag_breakout_strategy.md) |
| Fast metrics-only path | ✅ | `.bt.backtest_metrics()` | `test_metrics_only_*` |
| Sweep with signal rebuild callback | ✅ | `.bt.sweep_callback()` | `test_sweep_callback.py` |

---

## Python `.bt` API surface (complete)

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
| `lf.bt.portfolio_backtest()` | Shared-capital multi-symbol simulation |

Rust-only helpers (no thin Python wrapper): `monte_carlo_return_paths`, factor transform primitives — Python uses equivalent `.bt` paths where noted above.

---

## Runnable showcase artifacts

| Artifact | Path | Audience |
|----------|------|----------|
| Overview | [index.md](index.md) | Landing page |
| Quickstart (5 min) | [quickstart.md](quickstart.md) | New evaluators |
| Full `.bt` tour | [backtest_showcase.md](../../examples/notebooks/backtest_showcase.md) | Demo / sales |
| Tear Sheets | [tear_sheets.md](tear_sheets.md) | HTML reports |
| Portfolio shared capital | [portfolio_shared_capital_backtest.md](../../examples/notebooks/portfolio_shared_capital_backtest.md) | Multi-symbol books |
| PA canonical strategy | [pa_flag_breakout_strategy.md](../../examples/notebooks/pa_flag_breakout_strategy.md) | PA moat |
| Benchmarks | [backtest_benchmark.md](../../examples/notebooks/backtest_benchmark.md) | Performance story |
| ML → backtest E2E | [ml_feature_backtest_parity.md](../../examples/notebooks/ml_feature_backtest_parity.md) | ML pipeline |

---

## Verification gates

```bash
cargo nextest run -p quantwave-backtest
pytest quantwave-python/tests/test_backtest.py quantwave-python/tests/test_pa_flag_backtest.py quantwave-python/tests/test_sweep_callback.py quantwave-python/tests/test_portfolio_backtest.py -q
cargo clippy -p quantwave-backtest -- -D warnings
```