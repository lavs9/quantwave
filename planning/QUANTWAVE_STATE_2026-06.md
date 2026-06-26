# QuantWave — Platform Index

**Updated:** 2026-06-26  
**Version:** 0.5.2 on `main`

This file is a **short index**. Detailed SOA status lives in dedicated docs—read those for requirements, gaps, beads, and user impact.

---

## Where to look

| Topic | Doc | Grade (plain English) |
|-------|-----|------------------------|
| **Indicators** | [INDICATORS_SOA.md](./INDICATORS_SOA.md) | **A engine, A- product** — math and APIs are SOA; long-tail docs + frac-diff pending |
| **Backtest** | [BACKTEST_SOA.md](./BACKTEST_SOA.md) | **A research, B production** — full research loop; tearsheets/portfolio/live deferred |
| **Roadmap (public)** | [docs/roadmap.md](../docs/roadmap.md) | User-facing priorities |
| **Agents / landing** | [AGENTS.md](../AGENTS.md) | Build, test, push workflow |

---

## Architecture (one picture)

```text
quantwave-core       → 216 indicators, Next<T>, PA, regimes, metadata
quantwave-polars     → lf.ta.*() + lf.ta.features.* + lf.bt()
quantwave-plugins    → pl.col("x").ta.*()  (full parity)
quantwave-backtest   → sim, sweep, WFO, MC, cross-sectional
quantwave-python     → qw.*, lf.bt.*, build_feature_matrix()
```

**Moat:** batch and streaming use the same math—you get identical results in research and live.

---

## Open beads (live)

Run `bd ready --json` for current state. Snapshot:

| Bead | Area | What it unlocks |
|------|------|-----------------|
| `quantwave-motd` | Parent epic | v0.6 SOA productization (Tiers 1–2 done) |
| `quantwave-wnd9` | Indicators | Fractional differencing for ML |
| `quantwave-0gi1` | Backtest | HTML tear sheets |
| `quantwave-8v4s` | Backtest | Portfolio-wide engine (epic) |
| `quantwave-cr6v-v2.7` | Backtest | Nautilus live (deferred, LGPL) |

**Tiers 1–2 closed:** CI metadata gate, `quantwave verify`, docs polish, `build_feature_matrix`, `.bt.monte_carlo()`, Rust `.bt` parity.

---

## Gaps without beads

| Gap | Area | Impact |
|-----|------|--------|
| `winsorize` on Python cross-sectional | Backtest | Manual winsorize in Polars instead |
| Workspace clippy noise | Chore | CI lint not strict workspace-wide |

Indicator doc gaps (long-tail depth, metadata-driven mkdocs, boundary_info on pages) → epic **`quantwave-frq0`** — see [INDICATORS_SOA.md](./INDICATORS_SOA.md).

---

## Verify everything

```bash
./scripts/quantwave_verify.sh
```

---

## Revision history

| Date | Change |
|------|--------|
| 2026-06-19 | Initial monolithic state assessment |
| 2026-06-26 | Gap reconciliation; Tiers 1–2 closed |
| 2026-06-26 | Split into INDICATORS_SOA.md + BACKTEST_SOA.md; this file is index only |

*Update the SOA docs when closing beads; touch this index only for architecture or bead list changes.*