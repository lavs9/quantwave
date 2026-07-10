#!/usr/bin/env python3
"""Ensure generated TA-Lib map matches parity-test harvest (quantwave-5ipk.7)."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GENERATED = ROOT / "quantwave-python" / "python" / "quantwave" / "_talib_map_generated.py"
HARVEST = ROOT / "docs" / "generated" / "talib_map.json"


def main() -> int:
    before = GENERATED.read_text(encoding="utf-8") if GENERATED.is_file() else ""
    subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "generate_talib_map.py")],
        check=True,
        cwd=ROOT,
    )
    after = GENERATED.read_text(encoding="utf-8")
    if before != after:
        # Restore committed file if drift detected in dirty tree
        if before:
            GENERATED.write_text(before, encoding="utf-8")
        print("TA-Lib map drift: run python3 scripts/generate_talib_map.py and commit", file=sys.stderr)
        return 1

    data = json.loads(HARVEST.read_text(encoding="utf-8"))
    print(f"TA-Lib map drift OK ({data['count']} entries)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())