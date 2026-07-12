# Roadmap

QuantWave is a high-performance, Polars-native technical analysis library. The engine phase is complete; v0.6 shipped product guardrails and research-loop polish.

## Current Status (v0.6.0 — released 2026-06-28)

### Shipped

| Area | Status |
|------|--------|
| **Indicators** | 221 Rust `*_METADATA` entries; Polars `.ta()` methods; full expression-plugin parity |
| **Ehlers DSP** | 30+ indicators — deepest open-source cycle toolkit |
| **Price Action** | Market Structure, S/R monitor (ATR-relative), geometric patterns (flags/H&S + neckline breakout), confluence |
| **Regimes** | HMM, GMM, PELT changepoints, volatility clustering |
| **ML features** | `.ta.features.*`, `build_feature_matrix()`, fractional differencing (`FracDiff`) |
| **Options India** | BS Greeks, IV, chain analytics (`quantwave.options`) |
| **Backtest** | Research-complete: sweep, WFO, WFO-optimize, cross-sectional, MC (Rust + Python) |
| **Reporting** | HTML tear sheets (`to_html()` / `save_html()`) |
| **Python DX** | Discovery, metadata codegen, `assert_parity`, `boundary_info`, `categories`, `talib`, arm64 wheels |
| **CI / verify** | `./scripts/quantwave_verify.sh` — metadata drift, doc lint, nextest, pytest |
| **Docs** | 220+ indicator guides (SOA complete), gallery, ML features guide, backtest capability matrix |

### Deferred

- Live execution bridge — Nautilus (`quantwave-cr6v-v2.7`, LGPL HITL)
- Portfolio-wide / wide-format engine (`quantwave-8v4s`)

---

## Architecture (stable)

```text
quantwave-core     Next<T> — single mathematical truth
quantwave-polars   lf.ta.*() + lf.ta.features.*
quantwave-plugins  Polars expression plugins (zero-copy)
quantwave-backtest Backtest engine + tearsheets
quantwave-python   PyO3 (abi3) Python package
```

---

## Version history

| Version | Theme |
|---------|-------|
| **0.6.0** | Product guardrails, research loop, FracDiff, HTML tear sheets, doc SOA |
| **0.5.2** | Research platform — Python DX, plugins parity, arm64 wheels |
| **0.5.1** | Publishing reliability, backtest on crates.io |
| **0.5.0** | Backtest Engine v0.2 — rich metadata sizing, execution models, tearsheet markdown |

See [Changelog](changelog.md) and [Release 0.6.0](releases/0.6.0.md) for details.