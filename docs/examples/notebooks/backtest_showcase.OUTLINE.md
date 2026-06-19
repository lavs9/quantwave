# Backtest Showcase Notebook — Section Outline

**Deliverable:** `backtest_showcase.py` (marimo) + exported `backtest_showcase.md`  
**Bead:** `quantwave-bt-prod.2`  
**Depends on:** `capability_matrix.md` (bt-prod.1) for cross-links

Agent: implement each section as a marimo `@app.cell` with synthetic data only (no network).

---

## Cell 0 — Title & install

```markdown
# QuantWave Backtest Engine — Full `.bt` Tour
pip install "quantwave[all]" marimo polars
marimo edit docs/examples/notebooks/backtest_showcase.py
```

Link: [Capability Matrix](../../guides/backtest/capability_matrix.md)

---

## Cell 1 — Shared synthetic data

- 80-bar single-symbol DataFrame: timestamp, close, feature col
- Reused across sections; deterministic seed

---

## Section A — Basic backtest + metrics

**API:** `lf.bt.backtest_with_report()`  
**Show:** trades.head(), metrics dict keys  
**Assert:** num_trades >= 1

---

## Section B — Costs, filters, sizing

**API:** `entry_filter_col`, `size_multiplier_col`, commission_bps  
**Show:** trades differ when filter applied

---

## Section C — Fast metrics path

**API:** `lf.bt.backtest_metrics()` vs full report timing note  
**Show:** metrics match within tolerance (no trades DF)

---

## Section D — Param sweep (pre-built cols)

**API:** `lf.bt.sweep(param_values=..., signal_cols=...)`  
**Show:** 3-row param × metrics DataFrame

---

## Section E — Param sweep (callback rebuild)

**API:** `lf.bt.sweep_callback(param_grid=..., build_fn=...)`  
**Show:** same shape as D; build_fn uses feature threshold

---

## Section F — Walk-forward OOS

**API:** `lf.bt.walk_forward(train_bars=..., test_bars=...)`  
**Show:** fold_id, oos_start_ts, sharpe_ratio columns

---

## Section G — Walk-forward optimize

**API:** `lf.bt.walk_forward_optimize(param_grid=..., build_fn=..., objective=...)`  
**Show:** best_* param cols, train_metric, oos_metric, overfit_flag

---

## Section H — Cross-sectional panel

**API:** `lf.bt.cross_sectional_backtest(factor_col=..., transform="zscore")`  
**Data:** 3 symbols × 10 timestamps panel  
**Show:** report.metrics() num_trades

---

## Section I — Monte Carlo (if Python-exposed)

**Option A:** Document Rust-only `monte_carlo_trade_bootstrap` with code comment + link  
**Option B:** If exposed via Python, run bootstrap on report from Section A  
**Show:** p5/p50/p95 terminal equity or VaR/CVaR from return paths

---

## Section J — PA moat pointer

Markdown only: link to [PA Flag Breakout](pa_flag_breakout_strategy.md) — do not duplicate PA logic here.

---

## Section K — Parity callout

One paragraph + link to [batch-streaming.md](../batch-streaming.md) and ml_feature_backtest_parity.

---

## Acceptance

- [ ] `marimo edit backtest_showcase.py` runs without ImportError
- [ ] Each section A–H produces visible output
- [ ] Export to .md via project notebook pipeline or marimo export
- [ ] Added to `docs/examples/notebooks/index.md` (bt-prod.4)