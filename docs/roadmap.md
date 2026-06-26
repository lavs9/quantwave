# Roadmap

QuantWave is a high-performance, Polars-native technical analysis library. The engine phase is complete; v0.6 focuses on product guardrails and research-loop polish.

## Current Status (v0.5.2 — 2026-06)

### Shipped

| Area | Status |
|------|--------|
| **Indicators** | 216 Rust `*_METADATA` entries; ~205 Polars `.ta()` methods; ~219 expression plugins (full parity) |
| **Ehlers DSP** | 30+ indicators — deepest open-source cycle toolkit |
| **Price Action** | Market Structure, S/R monitor (ATR-relative), geometric patterns (flags/H&S + neckline breakout), confluence |
| **Regimes** | HMM, GMM, PELT changepoints, volatility clustering |
| **ML features** | `.ta.features.*` (Hurst, CyberCycle, ITL, trendflex, regime probs, Ehlers autocorr) |
| **Options India** | BS Greeks, IV, chain analytics (`quantwave.options`) |
| **Backtest** | Research-complete: sweep, WFO, WFO-optimize, cross-sectional, MC (Rust) |
| **Python DX** | Discovery, metadata codegen, `assert_parity`, `boundary_info`, `categories`, `talib`, arm64 wheels |
| **Docs** | 220+ indicator guides, gallery, ML features guide, backtest capability matrix |

### In Progress (v0.6 — `quantwave-motd` epic)

- Metadata codegen **CI drift gate** (`quantwave-ttge`)
- **`quantwave verify`** CLI (`quantwave-072m`)
- Research loop: `build_feature_matrix`, Python `.bt.monte_carlo()` (`quantwave-rdpk`, `quantwave-fsg3`)

### Deferred

- Live execution bridge — Nautilus (`quantwave-cr6v-v2.7`, LGPL HITL)
- Portfolio-wide / wide-format engine (`quantwave-8v4s`)
- Tear sheets / HTML reporting (`quantwave-0gi1`)
- Fractional differencing primitive (`quantwave-wnd9`)

---

## Architecture (stable)

```text
quantwave-core     Next<T> — single mathematical truth
quantwave-polars   lf.ta.*() + lf.ta.features.*
quantwave-plugins  pl.col("x").ta.*() expression plugins
quantwave-backtest sim, metrics, sweep, WFO, MC
quantwave-python   PyO3 + qw.* + lf.bt.*
```

**Moat:** batch ↔ streaming bit-identical parity (proptest + gold standard).

See [Plugin vs `.ta`](guides/plugin_vs_ta.md) for integration choices.

---

## Next Priorities

1. **Guardrails** — CI metadata gate + `quantwave verify` (Tier 1)
2. **Research loop** — feature matrix helper + Python MC wrapper (Tier 2)
3. **Visibility** — [comparison one-pager](comparison.md), release narrative
4. **Roadmap hygiene** — keep this file aligned with `planning/QUANTWAVE_STATE_2026-06.md`

---

## Future Horizons

- WASM / browser runtime
- Data-provider integrations and real-time bridges
- Portfolio optimization layer
- Auto-generated docs from `IndicatorMetadata` registry (xtask extension)