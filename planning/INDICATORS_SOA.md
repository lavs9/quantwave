# Indicators — SOA Status

**Updated:** 2026-06-26  
**Version:** 0.5.2  
**One sentence:** The indicator *engine* is SOA-grade; what’s left is mostly polish and one ML primitive—not missing core math.

---

## What “SOA” means here

A state-of-the-art indicator library lets a user:

1. **Trust the math** — same result in batch (Polars), streaming (live), and plugins.
2. **Find and use indicators** — discover names, params, warmup, categories without reading Rust.
3. **Go beyond TA-Lib** — Ehlers DSP, price action, regimes, ML features in one stack.
4. **Validate easily** — one command or API to check batch vs streaming parity.

If we meet those four, indicators are SOA for research and production signal generation. Live *execution* is a backtest/Nautilus topic, not an indicator topic.

---

## What you get today (real impact)

| Capability | What it means for you |
|------------|------------------------|
| **216 registered indicators** | Breadth at or above TA-Lib++, plus Ehlers and PA |
| **Batch ↔ streaming parity** | Research on historical data matches live bars—no silent drift |
| **`.ta()` + plugins (~219 methods)** | Fast Polars research *or* expression-plugin hot paths |
| **Price action suite** | Market structure, S/R, flags/H&S, confluence—for sizing and filters |
| **ML features** | `build_feature_matrix()`, `.ta.features.*`, E2E notebook |
| **Python DX** | `indicators()`, `metadata()`, `categories()`, `boundary_info()`, `talib`, `assert_parity()` |
| **Metadata pipeline + CI gate** | Rust `*_METADATA` auto-syncs to Python; CI fails on drift |
| **Tests** | 510+ core tests, gold standard + proptest on parity |

**Bottom line:** You can build and ship indicator-driven strategies in Python or Rust today. The moat is parity + depth (Ehlers/PA), not “we need 50 more oscillators.”

---

## SOA checklist

| Requirement | Status | Notes |
|-------------|--------|-------|
| Universal `Next<T>` — one math core | ✅ Done | All indicators share streaming truth |
| Batch Polars `.ta()` | ✅ Done | ~205 methods + 8 feature methods |
| Expression plugins parity | ✅ Done | `quantwave-3f7g` closed |
| `IndicatorMetadata` per indicator | ✅ Done | 216 entries; `quantwave-i9dn` rule |
| Python metadata codegen | ✅ Done | `quantwave-iqq7`, CI gate `quantwave-ttge` |
| Gold standard / proptest parity | ✅ Done | Core + polars test suites |
| PA (MS, S/R, patterns, confluence) | ✅ Done | `quantwave-cu03` and children |
| ML feature surface | ✅ Done | `quantwave-wlx`, `build_feature_matrix` (`quantwave-rdpk`) |
| Regime detection (HMM, GMM, PELT, vol) | ✅ Done | Code + [regime guide](../docs/guides/indicators/regimes/index.md) |
| Discovery & warmup APIs | ✅ Done | `quantwave-p3z9` children closed |
| Boundary / error semantics | ✅ Done | `boundary_info()`, `quantwave-p49i` |
| Every native page at doc STANDARDS | ⚠️ Partial | Long tail uneven—cosmetic, not functional |
| Docs auto-generated from metadata | ❌ Not started | No bead |
| Fractional differencing (Prado) | ❌ Not started | Bead: `quantwave-wnd9` |

**Indicator SOA grade: A engine, A- product.** Engine and APIs are there; long-tail docs and frac-diff are the gaps.

---

## What’s pending (indicators only)

### Has a bead

| Bead | Work | Impact if done | Priority |
|------|------|----------------|----------|
| `quantwave-wnd9` | Fractional differencing indicator + Polars/Python | ML pipelines get stationary features without ad-hoc hacks; aligns with Prado-style workflows | P3 (Tier 3) |

### No bead yet (optional / low urgency)

| Gap | Impact if ignored | Suggested action |
|-----|-------------------|------------------|
| Long-tail indicator pages below STANDARDS | Evaluators hitting obscure indicators see thin docs; math still works | File bead when doing doc sprint, or auto-gen from metadata |
| mkdocs pages from `export_metadata` JSON | Manual doc drift vs registry | File bead under `motd` or doc epic |
| Per-indicator `boundary_info` in generated mkdocs | Boundary rules only in Python API, not every native page | Extend doc generator (xtask) |

---

## Closed work (reference)

Major indicator epics/tasks already shipped—no action needed:

- `quantwave-3f7g`, `quantwave-jlk6` — plugin parity  
- `quantwave-iqq7`, `quantwave-ttge` — metadata codegen + CI  
- `quantwave-h6xe` — streaming readiness  
- `quantwave-cu03`, `quantwave-b7u` — PA foundation  
- `quantwave-8aht`, `quantwave-wlx`, `quantwave-22gw` — confluence + ML Polars  
- `quantwave-hbtm`, `quantwave-p49i`, `quantwave-p3z9` children — Python DX + docs polish  
- `quantwave-6br5`, `quantwave-p1k6` — documentation standards rollout  

---

## How to verify

```bash
./scripts/quantwave_verify.sh
```

```python
import quantwave as qw
qw.assert_parity("rsi", {"period": 14}, closes)
df = qw.build_feature_matrix(ohlcv, features="recommended")
```

---

## Key links

| Doc | Path |
|-----|------|
| Gallery | [docs/guides/indicators/gallery.md](../docs/guides/indicators/gallery.md) |
| ML features | [docs/guides/ml_features.md](../docs/guides/ml_features.md) |
| Plugin vs `.ta` | [docs/guides/plugin_vs_ta.md](../docs/guides/plugin_vs_ta.md) |
| Regimes | [docs/guides/indicators/regimes/index.md](../docs/guides/indicators/regimes/index.md) |
| Doc standards | [docs/DOCUMENTATION_STANDARDS.md](../docs/DOCUMENTATION_STANDARDS.md) |
| Platform index | [QUANTWAVE_STATE_2026-06.md](./QUANTWAVE_STATE_2026-06.md) |
| Backtest SOA | [BACKTEST_SOA.md](./BACKTEST_SOA.md) |

---

*Update when closing indicator beads or changing SOA bar.*