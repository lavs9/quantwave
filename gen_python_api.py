import mkdocs_gen_files
from pathlib import Path

"""
gen_python_api.py — QuantWave Python API Reference generator (rbz4 / p1k6)

This script runs via the mkdocs-gen-files plugin during `mkdocs build`.

It does two things:
1. Discovers the small pure-Python surface in quantwave-python/python/quantwave/
   and emits mkdocstrings pages for it (results classes, options helpers, talib
   wrappers, the polars layer, etc.).

2. Produces a high-quality, user-friendly landing page for the entire /api/
   section that explains real usage patterns and — crucially — directs people
   to the authoritative manual documentation (Gallery + per-indicator guides).

The rich indicator implementations live in the compiled Rust extension. We do
not attempt to duplicate the detailed formulas, visuals, edge cases, and
examples that already exist (and are kept up-to-date) in the Guides section.
"""

nav = mkdocs_gen_files.Nav()

# Walk the pure-Python part of the package (the thin wrappers + namespaces)
src = Path("quantwave-python/python/quantwave")

for path in sorted(src.rglob("*.py")):
    module_path = path.relative_to(src.parent).with_suffix("")
    doc_path = path.relative_to(src.parent).with_suffix(".md")
    full_doc_path = Path("api", doc_path)

    parts = list(module_path.parts)

    # Skip private / internal modules
    if any(p.startswith("_") for p in parts):
        continue

    if parts[-1] == "__init__":
        parts.pop()
        doc_path = doc_path.with_name("index.md")
        full_doc_path = full_doc_path.with_name("index.md")
    elif parts[-1] == "__main__":
        continue

    nav[parts] = doc_path.as_posix()

    with mkdocs_gen_files.open(full_doc_path, "w") as fd:
        ident = ".".join(parts)
        fd.write(f"::: {ident}")

    mkdocs_gen_files.set_edit_path(full_doc_path, path)

# ------------------------------------------------------------------
# Professional landing page for the entire API Reference section
# ------------------------------------------------------------------
with mkdocs_gen_files.open("api/index.md", "w") as fd:
    fd.write("""# Python API Reference

QuantWave's Python package provides three primary surfaces:

- **Polars batch** — the `.ta` namespace (`df.lazy().with_columns(pl.col("close").ta.rsi(14), ...)`)
- **Streaming** — lightweight Python wrappers around the same `Next<T>` Rust implementations (`from quantwave import RSI; rsi = RSI(14)`)
- **Backtest** — `df.lazy().bt.backtest_with_report(...)` (Rust `quantwave._backtest`, ships in the wheel)
- **Low-level / advanced** — result structs, options helpers, TA-Lib shim (`quantwave.talib`), discovery (`metadata`, `boundary_info`)

All three surfaces are backed by the same high-performance Rust core and are guaranteed to be bit-identical (validated by property tests against gold-standard vectors).

## Quick Start

```python
import polars as pl
import quantwave as qw

# Polars (recommended for research & feature engineering)
df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .with_columns([
        pl.col("close").ta.rsi(14).alias("rsi"),
        pl.col("high").ta.supertrend(10, 3).alias("supertrend"),
    ])
    .collect()
)

# Streaming (identical math, perfect for live systems / backtesters)
from quantwave import RSI, SuperTrend

rsi = RSI(14)
st  = SuperTrend(10, 3.0)

for row in data:
    r = rsi.next(row.close)
    s = st.next((row.high, row.low, row.close))
```

See the full patterns in [Batch & Streaming Examples](../examples/batch-streaming.md).

## What Lives Where

| Surface                  | Typical Import                              | Best For                          | Detailed Docs |
|--------------------------|---------------------------------------------|-----------------------------------|---------------|
| Polars `.ta` extension   | `pl.col("close").ta.xxx(...)`               | Feature engineering, research     | Guides → Indicators |
| Streaming wrappers       | `from quantwave import RSI, SuperTrend`     | Live systems, custom backtesters  | Per-indicator pages + notebooks |
| Result / Options structs | `from quantwave import results, options`    | Post-processing, risk, India ops  | This reference + docstrings |
| Native indicator catalog | `qw.metadata("rsi")`                        | Discovery + docs by slug          | [Complete catalog](../guides/indicators/native/index.md) |
| Market Structure / PA    | `from quantwave import MarketStructure`     | Event-driven strategies           | Dedicated PA guides + flagship notebook |

## Important Notes

- **Most indicator documentation lives in the Guides**, not here.  
  The manual pages (`guides/indicators/native/...`) contain formulas, parameters, visuals, edge cases, 3-surface code examples, and authoritative sources. They are the primary reference.

- The auto-generated pages below document the **thin Python glue** (result dataclasses, convenience helpers, the polars layer, etc.). They are intentionally lightweight.

- The heavy mathematical implementations (218 indicators, Ehlers DSP, geometric patterns, regimes, etc.) are written in Rust. Python merely exposes them.

- New high-level discovery / introspection helpers (`qw.indicators()`, `qw.metadata(name)`, streaming class wrappers, parity assertions, etc.) are under active development. When they stabilize they will appear in this section with examples.

## Generated Reference

The pages below are produced automatically from the pure-Python portion of the package at build time.

""")

with mkdocs_gen_files.open("api/SUMMARY.md", "w") as nav_file:
    nav_file.write("- [Overview](index.md)\n")
    nav_file.writelines(nav.build_literate_nav())
