#!/usr/bin/env python3
"""
Export marimo notebooks to self-contained HTML for embedding in the static docs site.

This script is run during the documentation build (see .github/workflows/docs.yml).

Why we do this instead of relying on mkdocs-marimo plugin at runtime:
- Notebooks that depend on the native `quantwave` Rust package cannot execute
  in the browser Pyodide/WASM environment.
- Pre-exporting during CI (where we have the full Rust + Python environment)
  lets us capture real outputs and provide a much better "showcase" experience
  on GitHub Pages.
- Users get nice embedded notebook UIs without raw .py downloads or 404s.

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

    # Output as .html (self-contained where possible)
    # html-wasm is generally preferred for GitHub Pages (runs in browser via Pyodide)
    # but falls back gracefully for cells that can't execute.
    dst = RENDERED_DIR / (src.stem + ".html")

    print(f"Exporting {notebook_name} -> {dst.relative_to(ROOT)} ...")

    # We use html-wasm because it produces a single file that works well on static hosting.
    # If a notebook has heavy native dependencies, some cells may show as non-executable
    # in the browser, but the code + any captured outputs will still be visible.
    cmd = [
        "marimo",
        "export",
        "html-wasm",
        str(src),
        "--output",
        str(dst),
        "--mode",
        "edit",  # or "run" — edit is usually better for documentation
    ]

    try:
        subprocess.run(cmd, check=True, capture_output=True, text=True)
        print(f"  ✓ Exported successfully")
    except subprocess.CalledProcessError as e:
        print(f"  ✗ Export failed for {notebook_name}")
        print(e.stdout)
        print(e.stderr, file=sys.stderr)
        # Don't fail the whole docs build for one notebook
        # (we can still have the landing page with "run locally" instructions)


def main():
    ensure_rendered_dir()

    print("Exporting marimo notebooks for documentation site...\n")

    for nb in NOTEBOOKS:
        export_notebook(nb)

    print("\nDone. Exported notebooks are in docs/examples/notebooks/rendered/")
    print("These can now be embedded in the MkDocs site.")


if __name__ == "__main__":
    main()