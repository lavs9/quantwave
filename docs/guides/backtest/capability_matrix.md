# Backtest Engine — Capability Matrix

**Sources:** [`planning/BACKTEST_ENGINE_RESEARCH.md`](../../../planning/BACKTEST_ENGINE_RESEARCH.md) §8 (v1 requirements) + v2 gap closure (`quantwave-cr6v-v2`).

**Epics:** `quantwave-cr6v` (v1, closed) · `quantwave-cr6v-v2` (v2, closed) · `quantwave-bt-prod` (productization, complete)

---

## Executive summary

QuantWave ships a **Polars-native, clean-room backtest engine** (`quantwave-backtest`) with Python `.bt` namespace ergonomics. It is **not** a vectorbt or polars-backtest fork — it is **vectorbt-*inspired*** research UX on top of QuantWave's unique **batch ↔ streaming parity** moat.

**What it is:**

- Long-format LazyFrame input (single- and multi-symbol)
- Realistic costs, execution delay, stops, shorts, sizing filters
- Research analytics: sweeps, walk-forward optimization, Monte Carlo, cross-sectional panels
- Rich PA/ML metadata preserved into trades

**What it is not (yet):**

- Live order routing (→ deferred `quantwave-cr6v-v2.7`, Nautilus HITL)
- Portfolio optimization / wide-format matrix engine
- Tear-sheet HTML plots (metrics dict + DataFrames only)

---

## The moat

> **Same strategy logic → identical equity/trades in batch (precomputed DF) and streaming (`Next<T>`) modes.**

| Artifact | Location |
|----------|----------|
| Parity integration test | `quantwave-backtest/src/lib.rs` — `test_batch_vs_streaming_parity_*` |
| Batch/streaming guide | [examples/batch-streaming.md](../../examples/batch-streaming.md) |
| ML features E2E parity notebook | [ml_feature_backtest_parity.md](../../examples/notebooks/ml_feature_backtest_parity.md) |

---

## Feature matrix (research §8 → shipped)

Legend: ✅ Shipped · ⏸ Deferred · ❌ Out of scope v1/v2

### P0 — Core user-facing (cr6v v1)

| # | Requirement | Status | Bead | API / module | Proof (test or notebook) |
|---|-------------|--------|------|--------------|--------------------------|
| 1 | Python `BacktestEngine` + config (PyO3) | ✅ | cr6v.4 | `quantwave.backtest.BacktestEngine` | `quantwave-python/tests/test_backtest.py::test_backtest_engine_run_single_trade` |
| 2 | `.bt.backtest()` / `.bt.backtest_with_report()` | ✅ | cr6v.5 | `quantwave-python/python/quantwave/bt_polars.py` | `test_bt_backtest_with_report` |
| 3 | PerformanceMetrics (Sharpe, Sortino, max DD, CAGR, win rate, PF, num trades) | ✅ | cr6v.1 | `quantwave-backtest/src/metrics.rs` | `test_backtest_metrics_dict_keys` |
| 4 | Multi-symbol long-format grouping + portfolio equity | ✅ | cr6v.2 | `BacktestEngine::run` multi-symbol path | `test_backtest_multi_symbol_*` in `test_backtest.py` |
| 5 | `entry_filter_col` + `size_multiplier_col` in batch `run()` | ✅ | cr6v.3 | `BacktestConfig` | `test_backtest_entry_filter_*` |
| 6 | `ml_feature_backtest_parity.py` uses real Rust engine | ✅ | cr6v.6 | notebook | [ml_feature_backtest_parity.md](../../examples/notebooks/ml_feature_backtest_parity.md) |
| 7 | `strategy_backtest.py` shows PnL via `.bt` | ✅ | cr6v.7 | notebook | [strategy_backtest.md](../../examples/notebooks/strategy_backtest.md) |

### P1 — Execution depth (cr6v v1)

| # | Requirement | Status | Bead | API | Proof |
|---|-------------|--------|------|-----|-------|
| 8 | T+1 execution (`execution_delay`) | ✅ | cr6v.8 | `BacktestConfig.execution_delay` | nextest `execution_delay` |
| 9 | Stop-loss / take-profit / trailing | ✅ | cr6v.9 | `StopConfig` | nextest `stop_*` |
| 10 | Short positions (signed exposure) | ✅ | cr6v.10 | signal f64 negative | nextest `short_*` |
| 11 | Struct signal column auto-parse + pole sizing | ✅ | cr6v.11 | `signal_col` Struct | nextest struct signal tests |
| 12 | Param sweep helper | ✅ | cr6v.12 | `.bt.sweep()` / `run_param_sweep` | `test_bt_sweep_*` |
| 13 | Criterion benches vs naive loop | ✅ | cr6v.13 | `benches/backtest_vs_naive.rs` | [backtest_benchmark.md](../../examples/notebooks/backtest_benchmark.md) |

### P2 — Research robustness (cr6v v1 + v2)

| # | Requirement | Status | Bead | API | Proof |
|---|-------------|--------|------|-----|-------|
| 14 | Walk-forward OOS | ✅ | cr6v.14 / xibc | `.bt.walk_forward()` / `run_walk_forward` | `test_bt_walk_forward_returns_folds` |
| 14b | Walk-forward **with in-fold optimization** | ✅ | cr6v-v2.1 | `.bt.walk_forward_optimize()` / `run_walk_forward_optimize` | `test_wfo_opt_*`, `test_bt_walk_forward_optimize_python` |
| 15 | Monte Carlo (trade bootstrap) | ✅ | cr6v.14 | `monte_carlo_trade_bootstrap` | `quantwave-backtest/tests/p2_features.rs` |
| 15b | Monte Carlo (return-path VaR/CVaR) | ✅ | cr6v-v2.2 | `monte_carlo_return_paths` | `monte_carlo.rs` tests |
| 16 | Cross-sectional factor panel (rank long/short) | ✅ | cr6v.15 / 6ypp | `.bt.cross_sectional_backtest()` | `test_cross_sectional_*` |
| 16b | Factor transforms (neutralize, zscore, winsorize) | ✅ | cr6v-v2.3 | `transform=` kwarg + Rust helpers | `test_factor_*`, `test_bt_cross_sectional_zscore_python` |
| 17 | Nautilus live bridge | ⏸ | cr6v-v2.7 | `LiveBridge` trait stub only | `../../../planning/NAUTILUS_LIVE_BRIDGE_ADR.md` |

### v2 additions (not in original §8 table)

| Feature | Status | Bead | API | Proof |
|---------|--------|------|-----|-------|
| Canonical PA flag → `.bt` E2E | ✅ | cr6v-v2.4 | PA notebook + tests | `test_pa_flag_backtest_*`, [pa_flag_breakout_strategy.py](../../examples/notebooks/pa_flag_breakout_strategy.py) |
| Fast metrics-only path | ✅ | cr6v-v2.5 | `.bt.backtest_metrics()` / `run_metrics_only` | `test_metrics_only_*`, bench `quantwave_metrics_only` |
| Sweep with signal rebuild callback | ✅ | cr6v-v2.6 | `.bt.sweep_callback()` | `quantwave-python/tests/test_sweep_callback.py` |

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

Rust-only (no Python wrapper yet): `run_walk_forward_optimize`, `monte_carlo_return_paths`, `neutralize_factor` / `zscore_factor` / `winsorize_factor` — document as Rust API; Python uses equivalent paths where noted above.

---

## Runnable showcase artifacts

| Artifact | Path | Audience |
|----------|------|----------|
| Quickstart (5 min) | `docs/guides/backtest/quickstart.md` (bt-prod.5) | New evaluators |
| Full `.bt` tour | `docs/examples/notebooks/backtest_showcase.py` (bt-prod.2) | Demo / sales |
| PA canonical strategy | [pa_flag_breakout_strategy.md](../../examples/notebooks/pa_flag_breakout_strategy.md) | PA moat |
| Benchmarks | [backtest_benchmark.md](../../examples/notebooks/backtest_benchmark.md) | Performance story |
| ML → backtest E2E | [ml_feature_backtest_parity.md](../../examples/notebooks/ml_feature_backtest_parity.md) | ML pipeline |

---

## Verification gates (global)

```bash
cargo nextest run -p quantwave-backtest
pytest quantwave-python/tests/test_backtest.py quantwave-python/tests/test_pa_flag_backtest.py quantwave-python/tests/test_sweep_callback.py -q
cargo clippy -p quantwave-backtest -- -D warnings
```

---

## Agent completion checklist (bt-prod.1)

- [x] Verify every ✅ row link resolves (fix broken paths)
- [x] Add `docs/guides/backtest/` to `mkdocs.yml` under Guides
- [x] Cross-link from `docs/purpose.md` or `docs/roadmap.md` (one line each)
- [x] Remove "Draft outline" header when done
- [x] `mkdocs build` green