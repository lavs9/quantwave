#!/usr/bin/env python3
"""Fail if docs/validation.md drifts from generated validation_stats.json."""

from __future__ import annotations

import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VALIDATION_MD = ROOT / "docs" / "validation.md"
STATS_JSON = ROOT / "docs" / "generated" / "validation_stats.json"

_LAST_UPDATED_RE = re.compile(r"^\*\*Last updated:\*\* [^\n]+\n", re.MULTILINE)


def _normalize_validation_md(text: str) -> str:
    """Drop volatile timestamp line from the generated stats block."""
    return _LAST_UPDATED_RE.sub("", text)


def _normalize_stats(data: dict) -> dict:
    normalized = dict(data)
    normalized.pop("generated_at", None)
    return normalized


def main() -> int:
    if not VALIDATION_MD.exists():
        print("check_validation_docs: missing docs/validation.md", file=sys.stderr)
        return 1
    if not STATS_JSON.exists():
        print("check_validation_docs: missing docs/generated/validation_stats.json", file=sys.stderr)
        return 1

    with tempfile.TemporaryDirectory(prefix="qw-validation-") as td:
        tmp = Path(td)
        shutil.copy2(VALIDATION_MD, tmp / "validation.md")
        shutil.copy2(STATS_JSON, tmp / "validation_stats.json")

        subprocess.check_call([sys.executable, str(ROOT / "scripts" / "collect_validation_stats.py")], cwd=ROOT)
        subprocess.check_call([sys.executable, str(ROOT / "scripts" / "render_validation_docs.py")], cwd=ROOT)

        md_ok = _normalize_validation_md(VALIDATION_MD.read_text(encoding="utf-8")) == _normalize_validation_md(
            (tmp / "validation.md").read_text(encoding="utf-8")
        )
        stats_ok = _normalize_stats(json.loads(STATS_JSON.read_text(encoding="utf-8"))) == _normalize_stats(
            json.loads((tmp / "validation_stats.json").read_text(encoding="utf-8"))
        )

        if not md_ok or not stats_ok:
            print(
                "check_validation_docs: validation docs are out of date.\n"
                "  Run: python scripts/collect_validation_stats.py && python scripts/render_validation_docs.py",
                file=sys.stderr,
            )
            return 1

    print("check_validation_docs: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())