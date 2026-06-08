# Backtest Engine Landscape Research (quantwave-wnu)

**Date:** 2026-06-03 IST  
**Task:** quantwave-wnu  
**Parent:** quantwave-b7u  
**Author:** QuantWave agent session  
**Status:** Complete — feeds follow-on epic `quantwave-cr6v` (Polars Backtester Core v1)

---

## Executive Summary

**Question:** Should QuantWave inherit/rewrite vectorbt (or another OSS engine) in Rust for a Polars-native backtester?

**Answer:** **No wholesale fork or dependency.** Build on the existing `quantwave-backtest` crate with a **clean-room** design inspired by the best ideas from polars-backtest (UX + long-format portfolio sim), vectorbt (param-grid research ergonomics), RaptorBT (analytics depth), and QF-Lib (execution/sizing — already partially ported in n1yc).

| Option | Verdict |
|--------|---------|
| Fork / embed **vectorbt** | ❌ Commons Clause blocks selling/redistributing engine value |
| Fork / depend on **polars-backtest** | ❌ PolyForm Noncommercial — incompatible with commercial `quantwave` distribution without separate license |
| Depend on **RaptorBT** as core | ⚠️ MIT-safe but wrong shape (NumPy-wide, duplicate indicators, no Next\<T\> parity) |
| Wrap **sigc** DSL | ⚠️ MIT-safe complement for cross-sectional factor sweeps, not PA event strategies |
| Embed **Nautilus** sim | ❌ LGPL + event-driven production focus, not vectorized research |
| **Extend quantwave-backtest** | ✅ Recommended — preserves unique batch/streaming parity + rich PA/ML metadata |

**Time-to-first-usable (PA flag + regime + feature strategy in Polars):** ~4–6 weeks for v1 (Python `.bt` API + analytics + multi-symbol), assuming current MVP stays stable.

---

## 1. Current QuantWave Baseline (audited 2026-06-03)

### Crate: `quantwave-backtest` (v0.5.2, ~1,300 LOC, single `lib.rs`)

| Capability | Status |
|------------|--------|
| Long-format LazyFrame input | ✅ |
| Causal entry/exit on exposure | ✅ (long-only MVP) |
| Commission + slippage (bps) | ✅ |
| Pluggable `CommissionModel` / `SlippageModel` | ✅ (types exist) |
| `ExecutionModel::HighFidelity` (sqrt impact) | ✅ |
| `InitialRiskPositionSizer` (QF-Lib pattern) | ✅ |
| `StrategySignal` + `Trade.entry_metadata` | ✅ |
| `run_streaming_simulation` + `Next<&Bar>` | ✅ |
| Batch ↔ streaming parity tests | ✅ (6 tests, nextest green) |
| Multi-symbol grouping | ❌ (config stub only) |
| Shorts, stops, T+1, partial fills | ❌ |
| Sharpe / max DD / tearsheet | ❌ (changelog overclaims; not in source) |
| `entry_filter_col` / `size_multiplier_col` in batch `run()` | ❌ (config only) |
| Native Struct column ingestion | ❌ |
| Python / Polars `.bt` namespace | ❌ |
| Parameter sweep / grid | ❌ |

### Integration surface today

- **Rust tests** exercise regime (TAR) + CyberCycle + pole-height sizing parity.
- **`ml_feature_backtest_parity.py`** documents the contract but runs a **Python port** of `run_simulation`, not the Rust engine via `pip install quantwave`.
- **`strategy_backtest.py`** is still indicator-only (SuperTrend); no PnL/trades.

### QuantWave differentiator (keep at all costs)

> **Same strategy logic → identical equity/trades in batch (precomputed DF) and streaming (`Next<T>`) modes.**

None of the evaluated external engines offer this. It is the moat for live-trading parity with research.

---

## 2. Engine-by-Engine Review

### 2.1 vectorbt (OSS) — [polakowo/vectorbt](https://github.com/polakowo/vectorbt)

| Dimension | Assessment |
|-----------|------------|
| **License** | Apache-2.0 **+ Commons Clause v1.0** — you may **not Sell** the Software (product whose value derives substantially from vectorbt). Internal use + clean-room reimplementation of **concepts** is OK; **forking/shipping the engine** in `quantwave` is legally risky for a commercial library. |
| **Architecture** | Python/NumPy/Numba-first; wide-format matrices; optional `vectorbt-rust` 1.0.0 kernels (`engine="rust"` per call). Not Polars-native. |
| **Data layout** | Wide (time × symbols) primary; excellent broadcasting / Cartesian param grids. |
| **Strengths** | Param grids at scale; `Portfolio.from_signals`; records/mapped arrays; rich stats/plotting; indicator factory; ML labeling; PRO features referenced but OSS still deep. |
| **Performance** | Claims ~1M orders in 70–100ms (M1); rust engine ~20% faster than numba on supported paths. |
| **Gaps for QuantWave** | No Polars `.ta()` integration; no rich Struct PA metadata; no streaming `Next<T>` parity; Commons Clause. |
| **QuantWave fit** | **Concept donor only** (vectorized signal→position→PnL, stats builder, sweep ergonomics). |

**vectorbt-rust** ([PyPI vectorbt-rust 1.0.0](https://pypi.org/project/vectorbt-rust/), Apr 2026): Same license family; accelerates kernels, not a standalone Polars backtester.

---

### 2.2 polars-backtest — [Yvictor/polars_backtest_extension](https://github.com/Yvictor/polars_backtest_extension)

| Dimension | Assessment |
|-----------|------------|
| **License** | **PolyForm Noncommercial 1.0.0** — commercial use requires separate license from author. **Cannot ship as part of MIT/Apache `quantwave` for paid users without negotiation.** |
| **Architecture** | Rust `btcore` + PyO3; Arrow FFI; Python `df.bt.backtest()` namespace. |
| **Data layout** | **Long-format** `(date, symbol, close, weight)` — closest to QuantWave research target. |
| **Strengths** | Native Polars UX; T+1; stop/take-profit/trailing; touched exit (OHLC); `backtest_with_report` (CAGR, Sharpe, trades, benchmark alpha/beta); liquidity metrics; claimed **15×** vs Finlab on 12M rows. |
| **Performance** | 244ms vs 3.7s (author benchmark, 300-day breakout, ~2k stocks × 17y). |
| **Gaps for QuantWave** | No streaming parity; no rich PA Struct metadata; NC license; young project (10★). |
| **QuantWave fit** | **Primary UX and execution-model reference** — clean-room the API shape and long-format sim semantics, do not copy code. |

---

### 2.3 RaptorBT — [alphabench/raptorbt](https://github.com/alphabench/raptorbt)

| Dimension | Assessment |
|-----------|------------|
| **License** | **MIT** — fully compatible. |
| **Architecture** | Rust core + PyO3; **NumPy array** inputs (timestamps, OHLCV, bool entries/exits). |
| **Data layout** | Per-instrument arrays; basket via tuple lists; not Polars LazyFrame chain. |
| **Strengths** | 33+ metrics; stops (fixed/ATR/trailing); basket/pairs/options/spreads/tick; Monte Carlo; batch spread parallel (Rayon); actively maintained (v0.4.0 PyPI, Jun 2026). |
| **Performance** | Author claims 0.25ms @ 1K bars, 1.7ms @ 50K bars (M-series). |
| **Gaps for QuantWave** | Duplicates indicator suite (RSI, MACD, …) — conflicts with `quantwave-core`; no Polars; no rich metadata; no batch/streaming parity. |
| **QuantWave fit** | **Analytics + stops reference**; optional dev dependency for metric parity tests, not runtime dependency. |

---

### 2.4 sigc — [Skelf-Research/sigc](https://github.com/Skelf-Research/sigc)

| Dimension | Assessment |
|-----------|------------|
| **License** | **MIT** (crates.io) |
| **Architecture** | DSL compiler + Polars/Arrow runtime; cross-sectional factor portfolios. |
| **Data layout** | Panel / factor matrices; `rank().long_short()` style. |
| **Strengths** | Deterministic caching; fast full-universe backtests (45ms claim, 500 names × 5y); production daemon path. |
| **Gaps for QuantWave** | Wrong abstraction for sparse PA events (flags, BOS flips); learning curve of `.sig` language. |
| **QuantWave fit** | **Future complement** for universe-level factor research (Ehlers feature panels), not core PA backtester. |

---

### 2.5 Nautilus Trader — [nautechsystems/nautilus_trader](https://github.com/nautechsystems/nautilus_trader)

| Dimension | Assessment |
|-----------|------------|
| **License** | **LGPL-3.0** — copyleft; embedding in permissively-licensed `quantwave` is problematic. |
| **Architecture** | Event-driven production engine (23k★). |
| **QuantWave fit** | **Out of scope** for vectorized research MVP; consider only for future live-execution bridge. |

---

### 2.6 Other mentions

| Project | Notes |
|---------|-------|
| **QF-Lib** ([quarkfin/qf-lib](https://github.com/quarkfin/qf-lib), Apache-2.0) | Already studied in quantwave-n1yc; sizing + execution traits partially ported. Reporting still missing. |
| **rusty-backtest / rust_bt** | Small/unmaintained; no Polars story. |
| **Finlab / Zipline lineage** | polars-backtest benchmarks against Finlab; not Rust-native. |

---

## 3. Feature Parity Matrix

Legend: ✅ full · ◐ partial · ❌ missing · N/A not applicable

| Feature | vectorbt OSS | polars-backtest | RaptorBT | sigc | **quantwave-backtest** |
|---------|:------------:|:---------------:|:--------:|:----:|:----------------------:|
| Polars LazyFrame native | ❌ | ✅ | ❌ | ◐ | ◐ (Rust Polars, no Python) |
| Long-format multi-symbol | ◐ | ✅ | ◐ | ✅ | ◐ |
| Wide param grid / sweep | ✅ | ◐ | ◐ | ✅ | ❌ |
| T+1 execution | ◐ | ✅ | ✅ | N/A | ❌ |
| Stops / trailing | ✅ | ✅ | ✅ | ◐ | ❌ |
| Shorts | ✅ | ◐ | ✅ | ✅ | ❌ |
| 20+ performance metrics | ✅ | ✅ | ✅ | ✅ | ❌ |
| Trade blotter + MAE/MFE | ✅ | ✅ | ◐ | ◐ | ◐ |
| Rich signal metadata | ❌ | ❌ | ❌ | ❌ | ✅ |
| Batch ↔ streaming parity | ❌ | ❌ | ❌ | ❌ | ✅ |
| `.ta()` / indicator integration | ❌ | ❌ | ❌ (built-in) | ❌ | ✅ |
| PA Struct (pole_height) sizing | ❌ | ❌ | ❌ | ❌ | ◐ |
| License safe for commercial OSS | ◐ | ❌ | ✅ | ✅ | ✅ |

---

## 4. QuantWave Integration Analysis

### 4.1 How existing surfaces feed a backtester

```text
OHLCV LazyFrame
  └─ .ta.supertrend() / .ta.market_structure() / .ta.geometric_patterns()
  └─ .ta.features.hurst() / .cyber_cycle() / .regime_features()
       └─ unnest Struct fields (pole_length_atr, bias, cycle_momentum, …)
            └─ Polars exprs → exposure + filter cols + metadata cols
                 └─ BacktestEngine::run (batch)
                 └─ OR FeatureToSignal : Next<&Bar> (streaming)
                      └─ run_streaming_simulation (must match batch)
```

### 4.2 Recommended signal contract (v1)

```rust
pub struct StrategySignal {
    pub exposure: f64,                              // units or weight
    pub metadata: Option<HashMap<String, f64>>,     // pole_height_atr, regime_prob, …
}
```

Batch path should accept either:
- scalar `exposure` column (today), or
- Polars **Struct** `signal` column with fields `{exposure, fraction_at_risk, pole_height_atr, …}` (06sz extension).

### 4.3 Parameter sweep UX (vectorbt-class killer feature)

Target pattern (Polars-native, no pandas):

```python
(
    df.lazy()
    .ta.geometric_patterns()
    .ta.features.hurst(15)
    .with_columns(
        pl.when(pl.col("flag_breakout") & (pl.col("hurst_persistence") > 0.52))
        .then(pl.col("pole_length_atr").clip(0.4, 2.0))
        .otherwise(0.0)
        .alias("exposure")
    )
    .bt.backtest(
        signal="exposure",
        sweep={"hurst_period": [10, 15, 20]},  # v1.1: multi-column grid
        costs={"commission_bps": 5, "slippage_bps": 2},
    )
)
```

v1: single run + manual sweep via Polars `concat` of pre-built DFs.  
v1.1: native sweep returning `BacktestReport` per param tuple (RaptorBT `batch_spread` pattern).

---

## 5. Decision Matrix

| Strategy | Effort | Risk | Time-to-usable | Keeps parity moat | License |
|----------|--------|------|----------------|-------------------|---------|
| **A. Extend quantwave-backtest (RECOMMENDED)** | L | Low | 4–6 wk | ✅ | ✅ |
| B. Fork polars-backtest | M | **High (NC)** | 2–3 wk | ❌ | ❌ |
| C. Depend on RaptorBT core | M | Medium | 3–4 wk | ❌ | ✅ |
| D. Wrap sigc for all backtests | L | High (paradigm) | 6+ wk | ❌ | ✅ |
| E. Rewrite vectorbt in Rust | XL | High | 12+ mo | ◐ | ◐ |
| F. Hybrid: quantwave sim + RaptorBT metrics only | S | Low | 2 wk | ✅ | ✅ |

**Selected path: A + F** — own the simulation semantics; clean-room analytics from RaptorBT/vectorbt stats; UX from polars-backtest.

---

## 6. Proposed API Sketch (quantwave-native)

### Rust (`quantwave-backtest`)

- `BacktestEngine::run(lf: LazyFrame) -> BacktestResult`
- `run_streaming_simulation(bars, gen, config) -> BacktestResult`
- `BacktestResult::metrics() -> PerformanceMetrics` (new)
- `BacktestResult::tearsheet_markdown() -> String` (new)

### Polars (`quantwave-polars`)

```rust
impl BtNamespace for LazyFrame {
    fn backtest(self, signal: &str, config: BacktestConfig) -> Result<BacktestResult, _>;
    fn backtest_with_report(self, ...) -> Result<BacktestReport, _>;
}
```

Register as `df.bt.*` via PyO3 / polars plugin pattern used elsewhere in quantwave.

### Python

```python
import quantwave as qw
result = qw.backtest(df, signal_col="exposure", commission_bps=5)
report = result.metrics()  # sharpe, max_dd, win_rate, …
```

---

## 7. Licensing & Clean-Room Rules

1. **Do not copy source** from vectorbt, polars-backtest, or RaptorBT — document algorithms, reimplement.
2. **Record sources** in module docs (already project convention).
3. **Commons Clause**: marketing must not claim "vectorbt replacement"; say "vectorbt-*inspired* research ergonomics".
4. **PolyForm NC**: do not add `polars-backtest` as dependency; API similarity is fine.
5. **MIT dependencies** (RaptorBT, sigc, QF-Lib): OK for dev benchmarks; avoid shipping their code inside `quantwave-backtest`.

---

## 8. Draft Requirements — Polars Backtester Core v1 (follow-on epic)

### P0 (must ship)

1. **Python exposure** of `BacktestEngine` + config via `quantwave-python` (maturin).
2. **`.bt.backtest()` / `.bt.backtest_with_report()`** on Polars LazyFrame (quantwave-polars).
3. **PerformanceMetrics** struct: Sharpe, Sortino, max DD, CAGR, win rate, profit factor, num trades (clean-room).
4. **Multi-symbol** long-format grouping (per-symbol + portfolio equity).
5. **Wire `entry_filter_col` + `size_multiplier_col`** in batch path.
6. **Update `ml_feature_backtest_parity.py`** to call real Rust, not Python port.
7. **Replace `strategy_backtest.py`** with minimal PnL example using `.bt`.

### P1 (next slice)

8. T+1 execution mode (polars-backtest semantics).
9. Stop-loss / take-profit / trailing (RaptorBT semantics).
10. Short positions.
11. Struct signal column auto-parse.
12. Param sweep helper (`bt.sweep` or Polars-friendly grid util).
13. Benchmarks crate: quantwave vs naive Polars loop on 1M rows (target 10×+).

### P2 (later)

14. Walk-forward / Monte Carlo (RaptorBT-inspired).
15. sigc integration for universe factor panels.
16. Nautilus live bridge (separate epic, LGPL review).

### Verification (non-negotiable)

- All P0 paths preserve **batch ↔ streaming parity** tests.
- `cargo nextest` + parity proptest on simulation invariants.
- One canonical PA notebook: flag breakout + regime + Hurst + pole sizing + costs.

---

## 9. Benchmark Plan (not run in wnu — assigned to pbc3)

| Case | Rows | Engines to compare |
|------|------|-------------------|
| Single-symbol flip | 10K–1M | quantwave-backtest, naive Polars, raptorbt (if installed) |
| Multi-symbol long | 100 symbols × 5K bars | quantwave vs polars-backtest (dev machine, NC license OK for internal bench only) |
| Sweep | 50 param cols × 100K rows | quantwave grid vs vectorbt (internal research only) |

Store results in `quantwave-backtest/benches/` (criterion) + `docs/examples/notebooks/backtest_benchmark.md`.

---

## 10. Sources Recorded

| Source | URL | License | Used for |
|--------|-----|---------|----------|
| vectorbt | https://github.com/polakowo/vectorbt | Apache-2.0 + Commons Clause | Param grid concepts, stats builder |
| vectorbt-rust | https://pypi.org/project/vectorbt-rust/ | Apache-2.0 + Commons Clause | Kernel acceleration pattern |
| polars_backtest_extension | https://github.com/Yvictor/polars_backtest_extension | PolyForm Noncommercial 1.0.0 | `.bt` UX, long-format, T+1/stops |
| RaptorBT | https://github.com/alphabench/raptorbt | MIT | Metrics catalog, stops API |
| sigc | https://github.com/Skelf-Research/sigc | MIT | Universe factor DSL reference |
| Nautilus Trader | https://github.com/nautechsystems/nautilus_trader | LGPL-3.0 | Ruled out for core |
| QF-Lib | https://github.com/quarkfin/qf-lib | Apache-2.0 | Sizing/execution (already in n1yc) |
| quantwave-backtest | `quantwave-backtest/src/lib.rs` | Project license | Current baseline |
| quantwave-366 / gwx / n1yc | beads issues | — | Prior requirements |
| strategy_backtest.py | `docs/examples/notebooks/strategy_backtest.py` | — | Thin steel-thread audit |
| ml_feature_backtest_parity.py | `docs/examples/notebooks/ml_feature_backtest_parity.py` | — | Integration contract |

---

## 11. Recommendation (one paragraph)

QuantWave should **not** rewrite or embed vectorbt or polars-backtest due to **Commons Clause** and **PolyForm Noncommercial** restrictions. The correct path is to **extend `quantwave-backtest`** with polars-backtest-*inspired* `.bt` ergonomics and RaptorBT-*inspired* analytics, while preserving the **batch/streaming parity** and **rich PA/ML metadata** stack that no competitor offers. Treat RaptorBT and sigc as **MIT-safe reference implementations** for metrics and universe sweeps, not as runtime dependencies. Ship **Python + Polars namespace first** (biggest gap vs user expectations), then T+1/stops/sweeps to approach vectorbt research throughput.