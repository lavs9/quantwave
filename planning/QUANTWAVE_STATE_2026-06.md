# QuantWave — Platform State Assessment

**Date:** 2026-06-19  
**Version:** `0.5.2` on `main`  
**Purpose:** Single reference for indicator, backtest, and Rust/Python bridge coverage — expectations, gaps, beads, and SOA recommendations.  
**Audience:** Product/strategy (idea brain) + implementation agents.

---

## Executive summary

QuantWave is past the “can we build this?” phase. The Rust engine is deep; the Polars backtester is research-complete (v1 + v2 + productization). The main gaps are **packaging consistency** (docs, metadata pipeline, plugin migration) and **conscious deferrals** (live bridge, tearsheets), not core math.

| Pillar | Grade | One-line |
|--------|-------|----------|
| **Indicators** | A- engine, B- packaging | World-class Rust depth; docs/plugins/metadata sync behind |
| **Backtest** | A research, B production | v1+v2+prod done; live bridge + tearsheets deferred |
| **Rust/Python bridge** | B+ | Works; needs codegen metadata + plugin parity |
| **Issue tracking** | ~27 open beads | Good for engineering; needs epic close-out + v0.6 planning |

---

## Architecture snapshot

```text
quantwave-core          Next<T>, indicators, PA, regimes, metadata
        ↓
quantwave-polars        lf.ta.*()  (~204 methods)
quantwave-plugins       Polars expression plugins (~91 registered)
quantwave-backtest      sim, metrics, sweep, WFO, MC, cross-sectional
        ↓
quantwave-python        PyO3, qw.*, lf.bt.*
        ↓
docs + notebooks + mkdocs + capability_matrix
```

**Moat (all pillars):** batch ↔ streaming parity — same logic, identical results in precomputed DF and `Next<T>` streaming.

---

## 1. Indicators

### 1.1 Coverage (shipped)

| Layer | Scale | Notes |
|-------|-------|-------|
| Rust core (`Next<T>`) | ~218 `_METADATA` constants | Single source of mathematical truth |
| Batch Polars (`.ta()`) | ~204 `pub fn` in `quantwave-polars` | Primary research UX |
| Expression plugins | ~91 in `quantwave-plugins` | Zero-copy vectorized subset |
| Native doc pages | ~227 under `docs/guides/indicators/native/` | Quality uneven |
| PA suite | Market Structure, Flags/H&S, S/R, geometric_patterns | MQL5 Parts 21/66/67/69 |
| Ehlers DSP | 30+ indicators | Deep niche |
| Regimes | HMM, GMM, PELT, vol clustering | Implemented; thin docs (1 index page) |
| Options India | BS, IV, chain analytics | v0.4.0 |
| Testing | proptest, gold_standard in `quantwave-core/tests/` | Parity enforced |

### 1.2 Expectations (project standard)

From `AGENTS.md`, `DOCUMENTATION_STANDARDS.md`, gallery:

- Every indicator: **batch ↔ streaming parity**, gold-standard or proptest validation.
- `IndicatorMetadata` (`*_METADATA`) — source of truth for docs + Python (`quantwave-i9dn` rule).
- PA: rich structs (`pole_length_atr`, `PAEvent`, etc.) for sizing and ML.
- Polars-native: `.ta()` on LazyFrame; plugins for performance-critical paths.

### 1.3 Gaps

| Gap | Severity | Bead(s) |
|-----|----------|---------|
| Doc quality rollout — many pages still thin vs STANDARDS | High (positioning) | `quantwave-6br5`, epic `quantwave-p1k6` |
| Plugin migration — ~91 vs ~200+ indicators | Medium (perf) | `quantwave-jlk6` (epic, open) |
| Metadata sync — Python `_metadata.py` manually synced | Medium (drift) | `quantwave-i9dn` (rule), `quantwave-iqq7` (auto-gen, P2) |
| Warmup / NaN semantics not uniform in Python | Medium | `quantwave-976r` |
| S/R monitor bugs (arity/borrow) | Medium | `quantwave-epqh`, `quantwave-wmd2` |
| Stale PA epics (core shipped) | Low (hygiene) | `quantwave-b7u`, `quantwave-cu03` — reconcile/close |
| Roadmap says “portfolio backtest” as future | Low | No bead — update `docs/roadmap.md` |
| ML feature surface polish | Medium | `quantwave-wlx`, `quantwave-hbtm`, `quantwave-8aht` (P2) |

### 1.4 Indicators verdict

**Depth > breadth achieved in Rust.** Packaging (docs uniformity, metadata pipeline, plugin parity) lags the engine. External story: **gallery + PA + Ehlers** is strong; long tail of indicator pages is not uniformly brochure-quality.

---

## 2. Backtest engine

### 2.1 Coverage (shipped)

Authoritative checklist: [`docs/guides/backtest/capability_matrix.md`](../docs/guides/backtest/capability_matrix.md)

| Tier | Features |
|------|----------|
| **Core** | `.bt.backtest` / `backtest_with_report`, costs, multi-symbol, `entry_filter_col`, `size_multiplier_col` |
| **Execution** | T+1, stops/trailing, shorts, Struct signal + pole sizing |
| **Research** | `.bt.sweep`, `.bt.sweep_callback`, `.bt.walk_forward`, `.bt.walk_forward_optimize`, `.bt.cross_sectional_backtest` (+ zscore/neutralize) |
| **Robustness** | Trade-bootstrap MC + return-path VaR/CVaR (Rust) |
| **Performance** | `.bt.backtest_metrics` / `run_metrics_only`, criterion benches |
| **Showcase** | quickstart, showcase notebook, PA flag E2E, benchmarks doc |

**Closed epics:** `quantwave-cr6v` (v1), `quantwave-cr6v-v2` (v2), `quantwave-bt-prod` (productization, `8ab92da8`).

### 2.2 Expectations (research doc)

From `planning/BACKTEST_ENGINE_RESEARCH.md`:

- vectorbt-*inspired* UX, polars-backtest-*inspired* long-format, RaptorBT-*inspired* analytics — **no** NC/GPL runtime deps.
- Canonical PA notebook with regime + pole sizing + costs — **met**.
- ug9t batch ↔ streaming parity + nextest — **met**.

### 2.3 Gaps

| Gap | Severity | Bead(s) |
|-----|----------|---------|
| Live execution (Nautilus) | Deferred by design | `quantwave-cr6v-v2.7` (P4, HITL LGPL, +6m defer) |
| Python MC — bootstrap/VaR not on `.bt` | Medium | **No bead** |
| `winsorize_factor` — Rust yes; Python cross_sectional partial | Low | **No bead** |
| Tear sheets / HTML reports | Low | **No bead** |
| Portfolio optimization / wide-format matrix | Future | Roadmap only |
| Partial fills, bar magnifier, liquidity | Future | **No bead** |
| `metrics_only` perf — parity with full, not large speedup | Low | Documented honestly in benchmarks |
| `quantwave-polars` `.bt.walk_forward_optimize` | Low | Python has it; Rust polars namespace does not |

### 2.4 Python `.bt` API (complete)

| Method | Purpose |
|--------|---------|
| `lf.bt.backtest()` | Trades + equity DataFrames |
| `lf.bt.backtest_with_report()` | Above + `PerformanceMetrics` |
| `lf.bt.backtest_metrics()` | Metrics only |
| `lf.bt.sweep()` | Pre-built signal column grid |
| `lf.bt.sweep_callback()` | Rebuild signals per param |
| `lf.bt.walk_forward()` | Rolling OOS folds |
| `lf.bt.walk_forward_optimize()` | Train-window sweep + locked OOS |
| `lf.bt.cross_sectional_backtest()` | Universe rank long/short |

### 2.5 Showcase artifacts

| Artifact | Path |
|----------|------|
| Capability matrix | `docs/guides/backtest/capability_matrix.md` |
| Quickstart | `docs/guides/backtest/quickstart.md` |
| Full tour | `docs/examples/notebooks/backtest_showcase.py` |
| PA canonical | `docs/examples/notebooks/pa_flag_breakout_strategy.py` |
| Benchmarks | `docs/examples/notebooks/backtest_benchmark.md` |
| ML E2E | `docs/examples/notebooks/ml_feature_backtest_parity.py` |

### 2.6 Backtest verdict

**Research backtester feature-complete for v1/v2 scope.** Demo-ready today. Missing: production trading bridge (intentional) and analyst reporting (plots/tearsheets).

---

## 3. Rust ↔ Python bridge

### 3.1 Surfaces

```text
1. Streaming   qw.wrap_streaming(cls) / Next<T>     — parity truth
2. Batch       lf.ta.*()  (quantwave-polars)        — research default
3. Plugins     Polars expressions (quantwave-plugins)
4. Backtest    lf.bt.*()  (bt_polars + PyO3)
```

### 3.2 What works

- `import quantwave` registers `.bt` with polars extra.
- Discovery: `qw.indicators()`, `qw.metadata()`, `qw.assert_parity()`.
- Backtest: `BacktestEngine` + full `.bt` namespace.
- Options / regimes / talib namespaces (guarded imports).
- Getting started links backtest quickstart.

### 3.3 Gaps

| Gap | Impact | Bead(s) |
|-----|--------|---------|
| Metadata dual-write (Rust + `_metadata.py`) | Drift risk | `quantwave-i9dn`, `quantwave-iqq7` |
| Incomplete plugin ↔ `.ta` parity | Some indicators one path only | `quantwave-jlk6` |
| Rust-only analytics (MC return paths, full factor ops) | Python users need Rust or wrappers | **No bead** |
| WFO Python reimplements Rust optimize | Two code paths | Architectural — **no bead** |
| Options namespace cleanup | `quantwave-05q7` | Open P1 |
| Error taxonomy | `quantwave-1x2z` | Open P2 |
| Streaming readiness API | `quantwave-h6xe` | Open P2 |
| Workspace clippy (`quantwave-core` warnings) | CI noise if workspace-wide | Chore — **no bead** |

### 3.4 Bridge verdict

**Usable and demo-ready.** Barrier to “install and forget Rust”: metadata sync and plugin parity.

---

## 4. Issue tracking (beads)

> **Note:** Bead counts from `.beads/backup/issues.jsonl` snapshot. Run `bd ready --json` for live Dolt state.

### 4.1 Closed (major milestones)

| Epic | Status |
|------|--------|
| `quantwave-cr6v` | Closed — backtest v1 |
| `quantwave-cr6v-v2` | Closed — v2.1–v2.6 |
| `quantwave-bt-prod` | Closed — productization artifacts |

### 4.2 Open — by area

| Area | Bead IDs |
|------|----------|
| Docs / visual standards | `quantwave-6br5`, `quantwave-p1k6` |
| Plugin migration | `quantwave-jlk6` |
| Backtest live (deferred) | `quantwave-cr6v-v2.7` |
| PA / ML follow-on | `quantwave-b7u`, `quantwave-cu03`, `quantwave-8aht`, `quantwave-wlx`, `quantwave-hbtm` |
| S/R bugs | `quantwave-epqh`, `quantwave-wmd2` |
| DX polish | `quantwave-iutt`, `quantwave-976r`, `quantwave-05q7`, `quantwave-iqq7` |
| Python errors / readiness | `quantwave-1x2z`, `quantwave-h6xe` |

### 4.3 Gaps without beads (candidates for v0.6 epic)

- Python `.bt.monte_carlo()` wrapper
- `winsorize` on Python `cross_sectional_backtest`
- Tear-sheet / reporting layer
- Portfolio optimization / wide-format engine
- Roadmap refresh (backtest shipped)
- `quantwave-polars` Rust `.bt.walk_forward_optimize`
- Workspace-wide clippy cleanup (or per-crate policy doc)

---

## 5. Recommendations — SOA-grade standalone library

*SOA here = modular, contract-driven, standalone excellence (clear crate boundaries + stable public APIs).*

### 5.1 Freeze public API tiers

| Tier | Crates | Stability |
|------|--------|-----------|
| **0** | `quantwave-core` | Semver strict; parity tests mandatory |
| **1** | `quantwave-polars`, `quantwave-backtest` | Documented in capability matrix |
| **2** | `quantwave-python`, plugins | `qw.metadata()` + pytest gates |
| **3** | `quantwave-nautilus` (future) | Separate license; HITL before merge |

### 5.2 One metadata pipeline (highest ROI)

Rust `*_METADATA` → codegen → Python + mkdocs. Kill manual `_metadata.py` sync (`quantwave-iqq7`). CI: new indicator without metadata = fail.

### 5.3 Contract tests as product

- `qw.assert_parity()` in quickstart and README.
- Single `quantwave verify` CLI (nextest + pytest smoke).
- Market batch = streaming as differentiator vs vectorbt/polars-backtest.

### 5.4 Complete the research loop in Python

One path: indicators → signals → `.bt` → sweep/WFO → metrics. Close: `.bt.monte_carlo()` wrapper; fix capability matrix wording where Python already has WFO.

### 5.5 Plugin migration — strategic, not total

Migrate **top ~20** indicators (RSI, EMA, ATR, SuperTrend, market_structure, geometric_patterns). Document “plugin vs `.ta`” decision tree.

### 5.6 GTM / visibility

- GitHub release narrative for 0.5.2 backtest story.
- Demo script from `backtest_showcase` + PA flag notebook.
- Comparison table: QuantWave vs vectorbt vs polars-backtest (license + parity column).

### 5.7 Defer consciously

- Nautilus until LGPL sign-off (`cr6v-v2.7`).
- Portfolio opt / wide-format until user story exists.
- Clippy: per-crate gates (`quantwave-backtest` already clean).

### 5.8 Proposed next epic (not yet created)

**`quantwave-v06` — DX & contract hardening**

Suggested children:

1. Metadata codegen + CI gate (`iqq7`)
2. Plugin migration milestone — top 20 (`jlk6` slice)
3. Python MC + winsorize on `.bt`
4. Close/reconcile stale PA epics + roadmap sync
5. `quantwave verify` CLI or documented one-liner
6. GitHub release + comparison one-pager

---

## 6. Quality gates (current)

```bash
cargo nextest run -p quantwave-backtest    # 73 passed (2026-06-19)
cargo nextest run -p quantwave-core        # parity + gold standard
pytest quantwave-python/tests/test_backtest.py \
       quantwave-python/tests/test_pa_flag_backtest.py \
       quantwave-python/tests/test_sweep_callback.py
cargo clippy -p quantwave-backtest -- -D warnings   # clean
# workspace-wide clippy -D warnings fails on quantwave-core pre-existing noise
```

---

## 7. Key references

| Doc | Path |
|-----|------|
| Backtest capability matrix | `docs/guides/backtest/capability_matrix.md` |
| Backtest research | `planning/BACKTEST_ENGINE_RESEARCH.md` |
| Nautilus ADR | `planning/NAUTILUS_LIVE_BRIDGE_ADR.md` |
| Indicator gallery | `docs/guides/indicators/gallery.md` |
| Documentation standards | `docs/DOCUMENTATION_STANDARDS.md` |
| Roadmap | `docs/roadmap.md` |
| Agents / landing plane | `AGENTS.md` |

---

## 8. Revision history

| Date | Change |
|------|--------|
| 2026-06-19 | Initial capture after cr6v-v2 + bt-prod close; post-showcase/benchmark fixes (`8ab92da8`) |

*Update this file when closing major epics or shifting v0.6 priorities.*