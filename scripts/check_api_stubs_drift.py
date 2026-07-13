#!/usr/bin/env python3
"""Fail CI if API registry / .pyi stubs drift from codegen (quantwave-5ipk.3)."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TRACKED = [
    ROOT / "quantwave-py/python/quantwave/_ta_registry_generated.py",
    ROOT / "quantwave-py/python/quantwave/ta.pyi",
    ROOT / "quantwave-py/python/quantwave/py.typed",
]


def main() -> int:
    subprocess.run(
        [sys.executable, str(ROOT / "scripts/generate_api_stubs.py")],
        cwd=ROOT,
        check=True,
    )
    diff = subprocess.run(
        ["git", "diff", "--", *[str(p.relative_to(ROOT)) for p in TRACKED]],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    if diff.stdout.strip():
        print(diff.stdout, file=sys.stderr)
        print(
            "\nERROR: API stub codegen drift detected.\n"
            "Run:\n"
            "  python scripts/generate_api_stubs.py\n"
            "Then commit the updated files.",
            file=sys.stderr,
        )
        return 1
    print("OK: API registry and .pyi stubs are up to date.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())