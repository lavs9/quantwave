#!/usr/bin/env python3
"""
Export marimo notebooks to self-contained HTML for embedding in the static docs site.

This script is run during the documentation build (see .github/workflows/docs.yml).

We install the released `quantwave` package from PyPI before running this script.
This allows the notebooks to actually execute (including `import quantwave` and
using the `.ta` Polars extensions) during export, so real outputs can be captured.

We do this instead of relying on the mkdocs-marimo plugin to execute the raw .py
files at runtime in the browser because:

- Heavy notebooks depend on the native Rust `quantwave` package.
- That package cannot run in Pyodide/WASM on GitHub Pages.
- Pre-exporting during CI gives us beautiful rendered notebooks on the static site.

Usage (called from CI):
    python docs/export_notebooks.py

Outputs go to docs/examples/notebooks/rendered/
"""

import subprocess
import sys
from pathlib import Path

NOTEBOOKS = [
    "strategy_backtest.py",
    "multi_indicator_analysis.py",
    "ml_feature_stability.py",
    "ml_feature_backtest_parity.py",
]

ROOT = Path(__file__).parent.parent
NOTEBOOKS_DIR = ROOT / "docs/examples/notebooks"
RENDERED_DIR = NOTEBOOKS_DIR / "rendered"


def ensure_rendered_dir():
    RENDERED_DIR.mkdir(parents=True, exist_ok=True)


def export_notebook(notebook_name: str):
    src = NOTEBOOKS_DIR / notebook_name
    if not src.exists():
        print(f"WARNING: Notebook not found: {src}", file=sys.stderr)
        return

    dst = RENDERED_DIR / (src.stem + ".html")

    print(f"Exporting {notebook_name} -> {dst.relative_to(ROOT)} ...")

    cmd = [
        "marimo",
        "export",
        "html-wasm",
        str(src),
        "--output",
        str(dst),
        "--mode",
        "edit",
    ]

    try:
        subprocess.run(cmd, check=True, capture_output=True, text=True)
        print(f"  ✓ Exported successfully")
    except subprocess.CalledProcessError as e:
        print(f"  ✗ Export failed for {notebook_name} (common right after a new release when the package isn't on PyPI yet)")
        print(e.stdout)
        print(e.stderr, file=sys.stderr)
        # Never fail the entire docs build because of one notebook.
        # The landing page will still exist with "run locally" instructions.


def main():
    ensure_rendered_dir()

    print("Exporting marimo notebooks for documentation site...\n")

    for nb in NOTEBOOKS:
        export_notebook(nb)

    print("\nDone. Exported notebooks are in docs/examples/notebooks/rendered/")
    print("These can now be embedded in the MkDocs site.")


if __name__ == "__main__":
    main()