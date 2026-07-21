#!/usr/bin/env python3
"""Reject internal beads/ticket IDs in shipped Python package and docs (quantwave-5ipk.9)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Crate / package names — not tracker IDs.
_CRATE_NAMES = frozenset({"core", "plugins", "python", "py", "backtest", "polars", "wasm"})

# Tracker IDs are random base36 — roughly a quarter start with a digit
# (e.g. quantwave-976r, quantwave-5ipk), so the leading char must accept
# digits too. A letter-only first char silently misses those (quantwave-5ipk.9).
BEAD_RE = re.compile(
    r"\bquantwave-(?!(?:" + "|".join(_CRATE_NAMES) + r")\b)[a-z0-9]{2,5}\b"
)

SCAN_ROOTS = (
    ROOT / "quantwave-py" / "python",
    ROOT / "docs",
)

ALLOWLIST_PREFIXES = (
    "planning/",
    ".beads/",
    "scripts/create_",
    "scripts/check_bead_ids.py",
)

SKIP_DOC_SUFFIXES = (
    "/DOCUMENTATION_DECISIONS.md",
    "/changelog.md",
    "/roadmap.md",
)


def _rel(path: Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def _allowed(path: Path) -> bool:
    rel = _rel(path).replace("\\", "/")
    for prefix in ALLOWLIST_PREFIXES:
        if rel.startswith(prefix) or prefix in rel:
            return True
    for suffix in SKIP_DOC_SUFFIXES:
        if rel.endswith(suffix):
            return True
    if rel.startswith("docs/generated/"):
        return True
    # Historical release notes are records of what was announced; like the
    # already-exempt changelog.md / roadmap.md, they may cite tracker IDs.
    if rel.startswith("docs/releases/"):
        return True
    return False


def scan() -> list[tuple[str, int, str]]:
    hits: list[tuple[str, int, str]] = []
    for root in SCAN_ROOTS:
        if not root.exists():
            continue
        for path in sorted(root.rglob("*")):
            if not path.is_file():
                continue
            if path.suffix not in {".py", ".pyi", ".md", ".rs", ".txt", ".json"}:
                continue
            if _allowed(path):
                continue
            try:
                text = path.read_text(encoding="utf-8")
            except (OSError, UnicodeDecodeError):
                continue
            for i, line in enumerate(text.splitlines(), start=1):
                if BEAD_RE.search(line):
                    hits.append((_rel(path), i, line.strip()))
    return hits


def main() -> int:
    hits = scan()
    if hits:
        print("bead ID lint failed — remove tracker IDs from shipped paths:", file=sys.stderr)
        for path, line_no, line in hits[:40]:
            print(f"  {path}:{line_no}: {line[:120]}", file=sys.stderr)
        if len(hits) > 40:
            print(f"  ... and {len(hits) - 40} more", file=sys.stderr)
        return 1
    print("bead ID lint OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())