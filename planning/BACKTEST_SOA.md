# Backtest Engine — SOA Status

**Updated:** 2026-06-26  
**Version:** 0.5.2  
**One sentence:** The backtester is **research-SOA** today; production-SOA needs tearsheets, portfolio engine, and (optionally) live execution.

> This doc is separate from indicators on purpose. See [INDICATORS_SOA.md](./INDICATORS_SOA.md) for the indicator library.

---

## What “SOA” means here

A state-of-the-art **research** backtester lets you:

1. **Run strategies on Polars** — signals in, trades + equity + metrics out.
2. **Research seriously** — param sweep, walk-forward, WFO-optimize, cross-sectional, Monte Carlo.
3. **Trust results** — costs, T+1, stops, multi-symbol, rich metadata on trades (PA/ML).
4. **Reproduce** — same Rust core in Python `.bt` and Rust `lf.bt()`.

A state-of-the-art **production** backtester also adds analyst reporting, portfolio-wide simulation, and optional live bridge. We defer those consciously.

---

## What you get today (real impact)

| Capability | What it means for you |
|------------|------------------------|
| `.bt.backtest` / `backtest_with_report` | Turn a signal column into trades, equity, Sharpe, drawdown, etc. |
| Costs + T+1 + stops/trailing | Results are closer to realistic execution, not fantasy fills |
| `.bt.sweep` / `sweep_callback` | Grid search params without rewriting boilerplate |
| `.bt.walk_forward` / `walk_forward_optimize` | Out-of-sample validation; train-window param pick |
| `.bt.cross_sectional_backtest` | Rank long/short across a universe (factor-style) |
| `.bt.monte_carlo()` | Trade bootstrap + return-path VaR/CVaR (`quantwave-fsg3`) |
| Rust `lf.bt()` parity | Same engine from Rust Polars (`quantwave-dk61` for WFO+MC) |
| PA + ML notebooks | Flag breakout, foundation strategy, ML→backtest E2E with metadata |

**Bottom line:** You can run a full quant research loop (signals → backtest → sweep/WFO → MC) without leaving Polars. You cannot yet get HTML tear sheets or portfolio-opt from one API call.

---

## SOA checklist

| Requirement | Status | Notes |
|-------------|--------|-------|
| Core backtest (long-format) | ✅ Done | `quantwave-cr6v` |
| v2 research (sweep, WFO, CS, MC Rust) | ✅ Done | `quantwave-cr6v-v2` |
| Productization + notebooks | ✅ Done | `quantwave-bt-prod` |
| Python `.bt` namespace | ✅ Done | Full method list below |
| Rust Polars `.bt` (WFO-opt, MC) | ✅ Done | `quantwave-dk61` |
| Python `.bt.monte_carlo()` | ✅ Done | `quantwave-fsg3` |
| Canonical PA + ML E2E notebooks | ✅ Done | parity verified |
| Capability matrix doc | ✅ Done | [capability_matrix.md](../docs/guides/backtest/capability_matrix.md) |
| `winsorize` on Python cross-sectional | ⚠️ Partial | Rust yes; Python path incomplete — **no bead** |
| HTML / PDF tear sheets | ❌ Not started | Bead: `quantwave-0gi1` |
| Portfolio-wide / wide-format engine | ❌ Deferred | Bead: `quantwave-8v4s` (epic) |
| Live execution (Nautilus) | ❌ Deferred | Bead: `quantwave-cr6v-v2.7` (LGPL HITL) |
| Partial fills / bar magnifier | ❌ Deferred | No bead |
| `metrics_only` large speedup | ⚠️ Accepted | Parity path exists; not a big perf win yet |

**Backtest SOA grade: A research, B production.** Research loop is complete; analyst output and portfolio scale are not.

---

## Python `.bt` API (complete)

| Method | Purpose |
|--------|---------|
| `lf.bt.backtest()` | Trades + equity DataFrames |
| `lf.bt.backtest_with_report()` | Above + metrics object |
| `lf.bt.backtest_metrics()` | Metrics only |
| `lf.bt.sweep()` | One backtest per pre-built signal column |
| `lf.bt.sweep_callback()` | Rebuild signals per param combo |
| `lf.bt.walk_forward()` | Rolling OOS folds |
| `lf.bt.walk_forward_optimize()` | Train-window sweep + locked OOS |
| `lf.bt.cross_sectional_backtest()` | Universe rank long/short |
| `lf.bt.monte_carlo()` | Trade bootstrap or return-path MC |

---

## What’s pending (backtest only)

### Has a bead

| Bead | Work | Impact if done | Priority |
|------|------|----------------|----------|
| `quantwave-0gi1` | Tear sheets / HTML reporting | Analysts get shareable reports without Jupyter glue | P3 |
| `quantwave-8v4s` | Portfolio-wide vectorized backtest | Multi-asset book simulation in one pass | P4 (epic) |
| `quantwave-cr6v-v2.7` | Nautilus live bridge | Paper/live execution from same signals; LGPL decision required | P4 (deferred) |

### No bead yet

| Gap | Impact | Suggested action |
|-----|--------|------------------|
| `winsorize_factor` on Python `.bt.cross_sectional_backtest` | Factor research may need manual winsorize in Polars | Small task bead under `motd` |
| Partial fills / liquidity model | Institutional realism gap | Defer until user story |
| WFO Python vs Rust duplicate logic | Two code paths to maintain | Architectural bead if consolidating |

---

## Closed work (reference)

- `quantwave-cr6v` — v1 engine  
- `quantwave-cr6v-v2` — sweep, WFO, cross-sectional, MC (Rust)  
- `quantwave-bt-prod` — productization, showcase notebooks  
- `quantwave-fsg3` — Python MC wrapper  
- `quantwave-dk61` — Rust Polars `.bt` alignment  

Parent epic `quantwave-motd` tracks Tier 3 backtest items above.

---

## Showcase artifacts

| Artifact | Path |
|----------|------|
| Capability matrix | [docs/guides/backtest/capability_matrix.md](../docs/guides/backtest/capability_matrix.md) |
| Quickstart | [docs/guides/backtest/quickstart.md](../docs/guides/backtest/quickstart.md) |
| Full tour | [docs/examples/notebooks/backtest_showcase.py](../docs/examples/notebooks/backtest_showcase.py) |
| PA canonical | [docs/examples/notebooks/pa_flag_breakout_strategy.py](../docs/examples/notebooks/pa_flag_breakout_strategy.py) |
| ML E2E | [docs/examples/notebooks/ml_feature_backtest_parity.py](../docs/examples/notebooks/ml_feature_backtest_parity.py) |

---

## Key links

| Doc | Path |
|-----|------|
| Research spec | [BACKTEST_ENGINE_RESEARCH.md](./BACKTEST_ENGINE_RESEARCH.md) |
| Nautilus ADR | [NAUTILUS_LIVE_BRIDGE_ADR.md](./NAUTILUS_LIVE_BRIDGE_ADR.md) |
| Platform index | [QUANTWAVE_STATE_2026-06.md](./QUANTWAVE_STATE_2026-06.md) |
| Indicators SOA | [INDICATORS_SOA.md](./INDICATORS_SOA.md) |

---

*Update when closing backtest beads or changing SOA bar.*