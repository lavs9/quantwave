#!/usr/bin/env python3
"""Fail CI if metadata codegen outputs drift from committed files (quantwave-ttge)."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TRACKED = [
    ROOT / "quantwave-core/src/indicators/metadata_registry.rs",
    ROOT / "quantwave-python/python/quantwave/_metadata_generated.py",
]


def main() -> int:
    subprocess.run(
        [sys.executable, str(ROOT / "scripts/regenerate_metadata_registry.py")],
        cwd=ROOT,
        check=True,
    )
    subprocess.run(
        [sys.executable, str(ROOT / "scripts/generate_indicator_metadata.py")],
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
            "\nERROR: metadata codegen drift detected.\n"
            "Run:\n"
            "  python scripts/regenerate_metadata_registry.py\n"
            "  python scripts/generate_indicator_metadata.py\n"
            "Then commit the updated files.",
            file=sys.stderr,
        )
        return 1
    print("OK: metadata codegen outputs are up to date.")
    return 0


if __name__ == "__main__":
    sys.exit(main())