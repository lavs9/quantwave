#!/usr/bin/env python3
"""Codegen TA-Lib name map for quantwave.talib from harvested parity slugs (quantwave-5ipk.7)."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HARVEST = ROOT / "docs" / "generated" / "talib_map.json"
OUT = ROOT / "quantwave-py" / "python" / "quantwave" / "_talib_map_generated.py"


def main() -> int:
    subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "harvest_talib_slugs.py")],
        check=True,
        cwd=ROOT,
    )
    data = json.loads(HARVEST.read_text(encoding="utf-8"))
    mapping: dict[str, str] = data["map"]

    lines = [
        '"""Auto-generated TA-Lib slug map — do not edit. Run scripts/generate_talib_map.py."""',
        "",
        "from __future__ import annotations",
        "",
        "TALIB_SLUG_TO_NAME: dict[str, str] = {",
    ]
    for slug, name in sorted(mapping.items()):
        lines.append(f'    "{slug}": "{name}",')
    lines.append("}")
    lines.append("")
    lines.append(f"TALIB_MAP_COUNT = {len(mapping)}")
    lines.append("")

    OUT.write_text("\n".join(lines), encoding="utf-8")
    print(f"wrote {OUT} ({len(mapping)} entries)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())