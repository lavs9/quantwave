# Epic: Indicator Docs SOA — Metadata-Driven Generation & Quality Gate

## Problem (plain English)

Structural doc lint passes (`upgrade_to_standards.py --lint`), but three gaps remain:

1. **Long-tail depth** — Bulk-upgraded pages have correct headings but thin/generic Description, Edge Cases, and Polars examples (e.g. RSI Polars line uses wrong `.ta.rsi("close", 14)` signature). Evaluators of obscure Ehlers/pattern indicators see template text, not practitioner-grade depth.
2. **Manual drift** — `quantwave-xtask` still emits the **pre-standards** template (`## Usage`, `## Background`, `## Formula`). New indicators can regress. No CI checks that `docs/guides/indicators/native/*.md` matches `export_metadata` JSON / `ALL_REGISTERED`.
3. **Boundary rules invisible on web** — `quantwave.boundary_info()` exists in Python (`_BOUNDARY_BY_KIND`) but native mkdocs pages do not surface warmup/NaN/empty-data semantics per indicator.

## Success (epic closes when ALL children closed)

- Every registered native indicator slug has exactly one STANDARDS-compliant page generated from registry JSON.
- Depth lint passes (no generic/template violations on any native page).
- Every native page includes a **Boundary Behavior** section derived from `boundary_kind`.
- CI fails on doc drift (like `scripts/check_metadata_drift.py` for Python codegen).
- `quantwave verify` / CI runs the new doc checks.

## Reference files

| File | Role |
|------|------|
| `docs/DOCUMENTATION_STANDARDS.md` | Mandatory template (v1.0) |
| `docs/upgrade_to_standards.py` | Bulk renderer + `--lint` (structural only today) |
| `quantwave-xtask/src/main.rs` | Legacy syn parser — emits wrong template |
| `quantwave-core/src/bin/export_metadata.rs` | JSON export (216 slugs) |
| `scripts/generate_indicator_metadata.py` | Python codegen from export |
| `scripts/check_metadata_drift.py` | Pattern for doc drift CI |
| `quantwave-python/python/quantwave/_metadata.py` | `boundary_info()` + `_boundary_kind()` |
| `docs/guides/indicators/native/relative_strength_index_rsi.md` | Bulk-upgraded (thin) |
| `docs/guides/indicators/native/laguerre_rsi.md` | Hand exemplar (target depth) |

## Counts (baseline 2026-06-26)

- Registered indicators: **216** (`export_metadata`)
- Native md files: **227** (11 orphans or aliases — must reconcile)
- Structural lint: **0 failures** (`python docs/upgrade_to_standards.py --lint`)

## Beads (execution order)

| ID | Title | Start when |
|----|-------|------------|
| `quantwave-frq0` | Parent epic | — |
| `quantwave-frq0.1` | Depth quality lint | **Ready now** |
| `quantwave-frq0.3` | Metadata-driven generator | **Ready now** (parallel with .1) |
| `quantwave-frq0.4` | boundary_info on pages | After .3 |
| `quantwave-frq0.2` | Long-tail enrichment | After .1 + .4 |
| `quantwave-frq0.5` | CI drift + verify | After .2 + .3 |

Claim: `bd update quantwave-frq0.1 --claim --json`

## Verification checklist (run before closing epic)

```bash
# Structural (today — must stay green)
python docs/upgrade_to_standards.py --lint

# Depth (after frq0.1)
python docs/upgrade_to_standards.py --depth-lint

# Registry coverage (after frq0.3 / frq0.5)
python scripts/check_doc_drift.py

# Site build
cd docs && mkdocs build

# Full platform gate
./scripts/quantwave_verify.sh

# Boundary API unchanged
python -c "import quantwave as qw; assert qw.boundary_info('rsi')"

# Spot-check exemplar vs bulk page
diff <(rg -c '^##' docs/guides/indicators/native/laguerre_rsi.md) \
     <(rg -c '^##' docs/guides/indicators/native/griffithsspectrum.md)
```

## Per-bead acceptance (summary)

### frq0.1 — Depth lint
- Implements ≥3 depth rules from epic description
- pytest or small test for lint helpers
- Documents rules in DOCUMENTATION_STANDARDS.md

### frq0.3 — Generator
- `scripts/generate_native_docs.py` or xtask delegates to export_metadata JSON
- No legacy `## Usage` / `## Background` output
- Reconciles 216 slugs vs 227 files

### frq0.4 — Boundary Behavior
- `boundary_kind` in export_metadata JSON
- `## Boundary Behavior` table on every native page
- test_python_dx doc parity for rsi, obv, engulfing, sr_monitor

### frq0.2 — Enrichment
- `--depth-lint` → 0 failures
- RSI Polars: `pl.col("close").ta.rsi(14)` (not `.ta.rsi("close", 14)`)
- mkdocs build green

### frq0.5 — CI
- `check_doc_drift.py` in CI + quantwave_verify.sh
- Fails on missing/orphan native page