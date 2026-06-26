# QuantWave — Platform State Assessment

**Date:** 2026-06-26  
**Version:** `0.5.2` on `main`  
**Purpose:** Single reference for indicator, backtest, and Rust/Python bridge coverage — expectations, gaps, beads, and SOA recommendations.  
**Audience:** Product/strategy (idea brain) + implementation agents.

---

## Executive summary

QuantWave is past the “can we build this?” phase. The Rust engine is deep; the Polars backtester is research-complete (v1 + v2 + productization). **June 2026 close-out** shipped PA foundation end-to-end, full plugin parity, metadata codegen, and streaming readiness. Remaining gaps are **docs polish**, **Python DX niceties**, and **conscious deferrals** (live bridge, tearsheets) — not core math.

| Pillar | Grade | One-line |
|--------|-------|----------|
| **Indicators** | A engine, B+ packaging | PA + Ehlers + 216 metadata; doc long-tail still uneven |
| **Backtest** | A research, B production | v1+v2+prod done; live bridge + tearsheets deferred |
| **Rust/Python bridge** | A- | Codegen metadata + plugin parity + readiness shipped |
| **Issue tracking** | **1 open bead** + v0.6 epic (`motd`) | Tier 1–3 beads filed; Tier 1 in flight |

---

## Architecture snapshot

```text
quantwave-core          Next<T>, indicators, PA, regimes, metadata (216 registered)
        ↓
quantwave-polars        lf.ta.*()  (~205 methods) + lf.ta.features.* (8)
quantwave-plugins       Polars expression plugins (~219 .ta methods, full parity)
quantwave-backtest      sim, metrics, sweep, WFO, MC, cross-sectional
        ↓
quantwave-python        PyO3, qw.*, lf.bt.*, _metadata_generated.py
        ↓
docs + notebooks + mkdocs + capability_matrix
```

**Moat (all pillars):** batch ↔ streaming parity — same logic, identical results in precomputed DF and `Next<T>` streaming.

---

## 1. Indicators

### 1.1 Coverage (shipped)

| Layer | Scale | Notes |
|-------|-------|-------|
| Rust core (`Next<T>`) | **216** `_METADATA` constants (`metadata_registry.rs`) | Single source of mathematical truth |
| Batch Polars (`.ta()`) | **~205** `pub fn` in `quantwave-polars` + **8** `.ta.features.*` | Includes `sr_monitor`, `market_structure`, `geometric_patterns` |
| Expression plugins | **~219** `.ta` methods in `quantwave-plugins` | Full parity per closed `quantwave-3f7g` |
| Python metadata | **217** codegen entries + hand overrides → **545** `qw.indicators()` | `scripts/generate_indicator_metadata.py` |
| Native doc pages | ~227 under `docs/guides/indicators/native/` | Quality uneven on long tail |
| PA suite | Market Structure, Flags/H&S (**neckline breakout**), S/R (ATR-relative + Polars), confluence | MQL5 Parts 21/66/67/69 — `quantwave-cu03` closed |
| Ehlers DSP | 30+ indicators | Deep niche |
| Regimes | HMM, GMM, PELT, vol clustering | Implemented; thin docs (1 index page) |
| Options India | BS, IV, chain analytics | `quantwave.options` namespace (`quantwave-05q7` closed) |
| Testing | **510** core + **548** polars nextest; proptest + gold_standard | Parity enforced |

### 1.2 Expectations (project standard)

From `AGENTS.md`, `DOCUMENTATION_STANDARDS.md`, gallery:

- Every indicator: **batch ↔ streaming parity**, gold-standard or proptest validation.
- `IndicatorMetadata` (`*_METADATA`) — source of truth for docs + Python (`quantwave-i9dn` rule).
- PA: rich structs (`pole_length_atr`, `PAEvent`, confluence helpers) for sizing and ML.
- Polars-native: `.ta()` on LazyFrame; plugins for performance-critical paths.

### 1.3 Gaps — status (2026-06-26)

| Gap | Was | Status | Bead(s) | Remaining work |
|-----|-----|--------|---------|----------------|
| Doc quality rollout — thin vs STANDARDS | High | **Closed** | `quantwave-6br5`, `quantwave-p1k6`, `quantwave-hbtm` | Long-tail indicator pages still uneven (low priority) |
| Plugin migration — ~91 vs ~200+ | Medium | **Closed** | `quantwave-jlk6`, `quantwave-3f7g` | Document plugin vs `.ta` decision tree (no bead) |
| Metadata sync — manual `_metadata.py` | Medium | **Closed** | `quantwave-iqq7`, `quantwave-ttge` | mkdocs from registry (future) |
| Warmup / NaN semantics in Python | Medium | **Closed** | `quantwave-976r` | — |
| S/R monitor bugs (arity/borrow) | Medium | **Closed** | `quantwave-epqh`, `quantwave-wmd2` | — |
| Stale PA epics | Low | **Closed** | `quantwave-b7u`, `quantwave-cu03` | — |
| PA Polars + confluence + ML features | Medium | **Closed** | `quantwave-8aht`, `quantwave-wlx`, `quantwave-22gw` | — |
| ML / PA docs polish | Medium | **Closed** | `quantwave-hbtm` | — |
| Per-indicator boundary docs | Low | **Closed** | `quantwave-p49i` | `boundary_info()` + guides |
| Roadmap stale | Low | **Closed** | `quantwave-q43g` | — |

### 1.4 Indicators verdict

**Depth and packaging both strong in Rust.** External story: **gallery + PA + Ehlers** is demo-ready. Remaining packaging gap is **documentation uniformity** on the indicator long tail, not engine capability.

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
- Canonical PA notebook with regime + pole sizing + costs — **met** (`pa_flag_breakout_strategy.py`, `pa_foundation_strategy.py`).
- ug9t batch ↔ streaming parity + nextest — **met**.

### 2.3 Gaps — status (unchanged; backtest scope stable)

| Gap | Severity | Status | Bead(s) |
|-----|----------|--------|---------|
| Live execution (Nautilus) | Deferred | **Open (deferred)** | `quantwave-cr6v-v2.7` (P4, HITL LGPL) |
| Python MC — bootstrap/VaR not on `.bt` | Medium | **Open** | **No bead** — v0.6 candidate |
| `winsorize_factor` — Rust yes; Python partial | Low | **Open** | **No bead** |
| Tear sheets / HTML reports | Low | **Open** | **No bead** |
| Portfolio optimization / wide-format matrix | Future | Deferred | Roadmap only |
| Partial fills, bar magnifier, liquidity | Future | Deferred | **No bead** |
| `metrics_only` perf — parity, not large speedup | Low | Accepted | Documented in benchmarks |
| `quantwave-polars` `.bt.walk_forward_optimize` | Low | **Open** | Python has it; Rust polars namespace does not |

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
| PA foundation | `docs/examples/notebooks/pa_foundation_strategy.py` |
| Benchmarks | `docs/examples/notebooks/backtest_benchmark.md` |
| ML E2E | `docs/examples/notebooks/ml_feature_backtest_parity.py` |

### 2.6 Backtest verdict

**Research backtester feature-complete for v1/v2 scope.** Demo-ready today. Missing: production trading bridge (intentional) and analyst reporting (plots/tearsheets).

---

## 3. Rust ↔ Python bridge

### 3.1 Surfaces

```text
1. Streaming   qw.wrap_streaming / qw.track_streaming + Rust TrackedNext   — parity + readiness
2. Batch       lf.ta.*()  (quantwave-polars)                              — research default
3. Plugins     col("x").ta.*()  (quantwave-plugins, ~219 methods)         — full parity
4. Backtest    lf.bt.*()  (bt_polars + PyO3)
```

### 3.2 What works

- `import quantwave` registers `.bt` with polars extra.
- Discovery: `qw.indicators()` (**545** names), `qw.metadata()`, `qw.assert_parity()`.
- Metadata codegen: Rust registry → `_metadata_generated.py` (217 entries).
- Streaming readiness: `TrackedNext` / `StreamingReadiness` (Rust) + `wrap_streaming(..., warmup_bars_count=)` (Python).
- Backtest: `BacktestEngine` + full `.bt` namespace.
- Options / regimes / talib namespaces (`quantwave.options` for India helpers).
- Getting started links backtest quickstart.

### 3.3 Gaps — status (2026-06-26)

| Gap | Impact | Status | Bead(s) | Remaining |
|-----|--------|--------|---------|-----------|
| Metadata dual-write | Drift risk | **Closed** | `quantwave-iqq7` | CI gate on `generate_indicator_metadata.py` output |
| Plugin ↔ `.ta` parity | One-path-only indicators | **Closed** | `quantwave-3f7g`, `quantwave-jlk6` | — |
| Options namespace cleanup | DX | **Closed** | `quantwave-05q7` | — |
| Streaming readiness API | Live systems | **Closed** | `quantwave-h6xe` | — |
| Rust-only analytics (MC return paths) | Python users | **Open** | **No bead** | `.bt.monte_carlo()` wrapper |
| WFO Python reimplements Rust optimize | Two code paths | **Open** | **No bead** | Architectural |
| Error taxonomy | DX | **Closed** | `quantwave-1x2z` | `QuantwaveError` hierarchy + `ParityError` |
| PyPI doc links broken | Discovery | **Closed** | `quantwave-l9ha` | `lavs9.github.io/quantwave` in README + pyproject |
| `__version__` missing | DX | **Closed** | `quantwave-2klk` | `importlib.metadata` + test |
| Linux arm64 wheel | Release matrix | **Closed** | `quantwave-7zsb` | `manylinux_2_34_aarch64` on PyPI 0.5.2 |
| Explicit `quantwave.talib` submodule | DX | **Closed** | `quantwave-xwiw` | `list_functions()` + 20 TA-Lib names |
| Categories API | DX | **Closed** | `quantwave-l99s` | `categories()`, `category()`, `indicators_by_category()` |
| Workspace clippy (`quantwave-core` warnings) | CI noise | **Open** | Chore — **no bead** | Per-crate policy |

### 3.4 Bridge verdict

**Production-grade for research workflows.** Metadata codegen and plugin parity removed the main “install and maintain two sources” barrier. Remaining bridge work is **Python wrappers** for Rust-only analytics (MC return paths, etc.).

---

## 4. Issue tracking (beads)

> Run `bd ready --json` for live Dolt state. Snapshot: **2026-06-26**.

### 4.1 Closed (major milestones — June 2026 session)

| Epic / task | Status |
|-------------|--------|
| `quantwave-cr6v`, `cr6v-v2`, `bt-prod` | Backtest v1 + v2 + productization |
| `quantwave-cu03` | PA foundation (MS, S/R, flags/H&S, confluence) |
| `quantwave-b7u` | MQL5 research epic → implementation delivered |
| `quantwave-3f7g`, `quantwave-jlk6` | Plugin migration — full `.ta` parity (~219 methods) |
| `quantwave-iqq7` | Metadata registry + Python codegen pipeline |
| `quantwave-h6xe` | `TrackedNext` + Python `track_streaming` |
| `quantwave-976r`, `quantwave-05q7` | Warmup semantics + options namespace |
| `quantwave-8aht`, `quantwave-wlx`, `quantwave-22gw` | Confluence + ML features Polars surface |
| `quantwave-epqh`, `quantwave-wmd2` | S/R monitor fixes |
| `quantwave-6br5`, `quantwave-p1k6` | Doc standards rollout |

### 4.2 Open — live

| Area | Bead IDs |
|------|----------|
| v0.6 SOA productization (parent) | `quantwave-motd` |
| Tier 2 (research loop) | `quantwave-rdpk`, `quantwave-fsg3`, `quantwave-dk61` |
| Tier 3 (deferred expansion) | `quantwave-wnd9`, `quantwave-0gi1`, `quantwave-8v4s`, `quantwave-cr6v-v2.7` |

### 4.3 Gaps without beads (remaining)

- `winsorize` on Python `cross_sectional_backtest`
- Workspace-wide clippy cleanup (or per-crate policy doc)

*(Most v0.6 candidates now tracked under `quantwave-motd` children.)*

---

## 5. Recommendations — SOA-grade standalone library

### 5.1 Freeze public API tiers

| Tier | Crates | Stability |
|------|--------|-----------|
| **0** | `quantwave-core` | Semver strict; parity tests mandatory |
| **1** | `quantwave-polars`, `quantwave-backtest` | Documented in capability matrix |
| **2** | `quantwave-python`, plugins | `qw.metadata()` + pytest gates |
| **3** | `quantwave-nautilus` (future) | Separate license; HITL before merge |

### 5.2 One metadata pipeline — **shipped; gate it**

✅ `regenerate_metadata_registry.py` → `generate_indicator_metadata.py` → `_metadata_generated.py`

**Next:** CI fails if generated output drifts; optional mkdocs feed from same JSON export.

### 5.3 Contract tests as product

- `qw.assert_parity()` in quickstart and README.
- Single `quantwave verify` CLI (nextest + pytest smoke).
- Market batch = streaming as differentiator vs vectorbt/polars-backtest.

### 5.4 Complete the research loop in Python

One path: indicators → signals → `.bt` → sweep/WFO → metrics. **Still open:** `.bt.monte_carlo()` wrapper.

### 5.5 Plugin migration — **done**

Full parity achieved (`3f7g`). **Next:** document when to use plugin vs `.ta` lazy map (performance vs ergonomics).

### 5.6 GTM / visibility

- GitHub release narrative for 0.5.2+ (backtest + PA + plugins + metadata codegen).
- Demo script from `backtest_showcase` + PA flag notebook.
- Comparison table: QuantWave vs vectorbt vs polars-backtest (license + parity column).

### 5.7 Defer consciously

- Nautilus until LGPL sign-off (`cr6v-v2.7`).
- Portfolio opt / wide-format until user story exists.
- Clippy: per-crate gates (`quantwave-backtest` already clean).

### 5.8 Proposed v0.6 epic (refined)

**`quantwave-v06` — Polish & Python research loop completion**

| # | Child | Status |
|---|-------|--------|
| 1 | Metadata codegen CI gate | **Done** (`ttge`) |
| 2 | Plugin migration top-N | **Done** (`3f7g`) |
| 3 | Python MC + winsorize on `.bt` | **Open** |
| 4 | Close PA epics + roadmap sync | **Done** |
| 5 | `quantwave verify` CLI | **Done** (`072m`) |
| 6 | GitHub release + comparison one-pager | **Done** (`l7xg`) |
| 7 | `hbtm` docs polish + `p49i` boundaries | **Done** |

---

## 6. Quality gates (current)

```bash
cargo nextest run -p quantwave-core        # 510 passed (2026-06-26)
cargo nextest run -p quantwave-polars      # 548 passed (incl. features + sr_monitor smoke)
cargo nextest run -p quantwave-backtest    # 73 passed
pytest quantwave-python/tests/test_backtest.py \
       quantwave-python/tests/test_pa_flag_backtest.py \
       quantwave-python/tests/test_metadata_codegen.py \
       quantwave-python/tests/test_streaming_readiness.py
cargo clippy -p quantwave-backtest -- -D warnings   # clean
# workspace-wide clippy -D warnings fails on quantwave-core pre-existing noise
```

**Metadata codegen refresh:**

```bash
python scripts/regenerate_metadata_registry.py
python scripts/generate_indicator_metadata.py
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
| Metadata codegen | `scripts/generate_indicator_metadata.py`, `quantwave-core/src/bin/export_metadata.rs` |

---

## 8. Revision history

| Date | Change |
|------|--------|
| 2026-06-19 | Initial capture after cr6v-v2 + bt-prod close; post-showcase/benchmark fixes (`8ab92da8`) |
| 2026-06-26 | Gap reconciliation: PA foundation, plugins (3f7g), metadata codegen (iqq7), streaming readiness (h6xe), warmup/options closed; 9 open beads; updated coverage counts and grades |

*Update this file when closing major epics or shifting v0.6 priorities.*