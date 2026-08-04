# Backtest Engine — Capability Matrix

QuantWave ships a **Polars-native, clean-room backtest engine** (`quantwave-backtest`) with Python `.bt` namespace ergonomics. It is **vectorbt-inspired** research UX on top of QuantWave's **batch ↔ streaming parity** moat.

"Clean-room" now describes the whole stack, not just the engine: the 221 indicators the engine consumes are QuantWave's own Rust as well, including all 61 candlestick patterns. No C TA-Lib and no third-party TA crate ships with the wheel — `talib-rs` is a dev-dependency retained purely as the test parity oracle and benchmark baseline.

## Executive summary

**What it is:**

- Long-format LazyFrame input (single- and multi-symbol)
- Realistic costs, execution delay, stops, shorts, sizing filters
- First-class order types (market / limit / stop / stop-limit) with deterministic OHLC fills
- Risk overlays (vol-target, inverse-vol, position-limit, pre-trade) and portfolio rebalance policies
- Research analytics: sweeps, walk-forward optimization, Monte Carlo, cross-sectional panels
- Shared-capital portfolio simulation across symbols
- HTML tear sheets, rich `PerformanceMetrics`, and benchmark-relative reporting (alpha / beta / Calmar / VaR / CVaR)
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
| 13a | First-class order types (market / limit / stop / stop-limit) | ✅ | `.bt.order_backtest()`, `quantwave-backtest/src/orders.rs` | `orders::tests::*`, `test_order_backtest.py` |
| 13b | Order-mode batch ↔ streaming parity (fold == incremental) | ✅ | `quantwave-backtest/src/order_exec.rs` | `fold_equals_incremental_stepping_parity` |
| 13c | Bracket / OCO exits (pessimistic same-bar convention) | ✅ | `Order::with_bracket`, wired in `order_exec`; `take_profit`/`stop_loss` cols on `.bt.order_backtest()` | `bracket_*` (Rust), `test_bracket_*` (Python) |
| 13d | Risk overlays (vol-target / inverse-vol / position-limit / pre-trade) | ✅ | `risk_model=`, `quantwave-backtest/src/risk.rs` | `risk_model_batch_streaming_parity`, `test_risk_and_rebalance.py` |

### Research robustness

| # | Requirement | Status | API | Proof |
|---|-------------|--------|-----|-------|
| 14 | Walk-forward OOS | ✅ | `.bt.walk_forward()` | `test_bt_walk_forward_returns_folds` |
| 14b | Walk-forward with in-fold optimization | ✅ | `.bt.walk_forward_optimize()` | `test_wfo_opt_*` |
| 14c | Optional Bayesian (TPE) in-fold optimizer | ✅ | `.bt.walk_forward_optimize(optimizer="tpe", n_trials=...)` | `tpe::tests::*`, `test_wfo_tpe_*`, `test_bt_walk_forward_optimize_tpe_python` |
| 15 | Monte Carlo (trade bootstrap) | ✅ | `monte_carlo_trade_bootstrap` | `quantwave-backtest/tests/p2_features.rs` |
| 15b | Monte Carlo (return-path VaR/CVaR) | ✅ | `monte_carlo_return_paths` | `monte_carlo.rs` tests |
| 16 | Cross-sectional factor panel | ✅ | `.bt.cross_sectional_backtest()` | `test_cross_sectional_*` |
| 16b | Factor transforms (neutralize, zscore, winsorize) | ✅ | `transform=` kwarg | `test_bt_cross_sectional_*` |
| 17 | Nautilus live bridge | ⏸ | `LiveBridge` trait stub | planning ADR |
| 18 | HTML tear sheets | ✅ | `tearsheet.render_html` | `test_tearsheet.py` |
| 19 | Shared-capital portfolio backtest | ✅ | `.bt.portfolio_backtest()` | `test_portfolio_backtest.py`, [portfolio notebook](../../examples/notebooks/portfolio_shared_capital_backtest.md) |
| 20 | Portfolio rebalance policies (calendar / drift / signal / turnover) | ✅ | `rebalance_policy=`, `quantwave-backtest/src/portfolio.rs` | `test_risk_and_rebalance.py` |
| 21 | Benchmark-relative reporting (alpha / beta / excess return) | ✅ | `report.metrics_with_benchmark()` | `metrics.rs` benchmark tests |
| 22 | Extended metrics (Calmar / VaR-95 / CVaR-95) | ✅ | `report.extended_metrics()` | `metrics.rs` tests |
| 23 | Thin-sample / undefined-ratio diagnostics | ✅ | `report.diagnostics()` | `metrics.rs` diagnostics tests, `test_backtest_output_contract.py` |

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
| `lf.bt.portfolio_backtest()` | Shared-capital multi-symbol simulation (`rebalance_policy=` optional) |
| `lf.bt.order_backtest()` | Order-driven sim from an explicit long-format order spec |
| `risk_model=` kwarg | Risk overlays on `backtest()` / `backtest_with_report()` / `backtest_metrics()` |
| `report.metrics_with_benchmark()` | Alpha / beta / excess return vs a benchmark series |
| `report.extended_metrics()` | Calmar / VaR-95 / CVaR-95 / diagnostics beyond the stable 10-key contract |
| `report.diagnostics()` | Thin-sample (< 30 trades) and undefined-ratio (`NaN`) warnings |

Rust-only helpers (no thin Python wrapper): `monte_carlo_return_paths`, factor transform primitives — Python uses equivalent `.bt` paths where noted above.

---

## Runnable showcase artifacts

| Artifact | Path | Audience |
|----------|------|----------|
| Overview | [index.md](index.md) | Landing page |
| Quickstart (5 min) | [quickstart.md](quickstart.md) | New evaluators |
| Full `.bt` tour | [backtest_showcase.md](../../examples/notebooks/backtest_showcase.md) | Demo / sales |
| Execution-aware research (orders + overlays + benchmark) | [execution_aware_research.md](../../examples/notebooks/execution_aware_research.md) | Execution realism |
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