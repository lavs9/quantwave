#!/usr/bin/env python3
"""Enforce zero-unsafe / zero-panic in quantwave-core production sources (quantwave-ruh0.3)."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CORE_SRC = ROOT / "quantwave-core" / "src"
REPORT = ROOT / "docs" / "generated" / "core_safety.json"

SKIP_FILES = {
    "test_utils.rs",  # cfg(test | test_utils) only
}

SKIP_DIRS = {
    "bin",  # CLI tools; not FFI/streaming hot path
}

FORBIDDEN_PATTERNS = (
    (re.compile(r"\bunsafe\b"), "unsafe block/keyword"),
    (re.compile(r"\bpanic!\s*\("), "panic! macro"),
)

# Production unwrap/expect are tracked (warn via workspace lints); flip to deny after cleanup.
TRACK_PATTERNS = (
    (re.compile(r"\.unwrap\s*\("), "unwrap()"),
    (re.compile(r"\.expect\s*\("), "expect()"),
)


def _strip_cfg_test_modules(text: str) -> str:
    """Remove `#[cfg(test)] mod <name> { ... }` blocks (brace-balanced).

    Matches any test-module name (``tests``, ``parity_tests``, ``unit``, …),
    not just the literal ``tests`` — otherwise test-only ``unwrap()``/``expect()``
    leak into the production tally.
    """
    out: list[str] = []
    i = 0
    n = len(text)
    while i < n:
        m = re.search(r"#\[cfg\(test\)\]\s*mod\s+\w+\s*\{", text[i:])
        if not m:
            out.append(text[i:])
            break
        start = i + m.start()
        out.append(text[i:start])
        brace = i + m.end() - 1
        depth = 0
        j = brace
        while j < n:
            ch = text[j]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    j += 1
                    break
            j += 1
        i = j
    return "".join(out)


def _iter_production_rust_files() -> list[Path]:
    files: list[Path] = []
    for path in sorted(CORE_SRC.rglob("*.rs")):
        if path.name in SKIP_FILES:
            continue
        if any(part in SKIP_DIRS for part in path.relative_to(CORE_SRC).parts):
            continue
        files.append(path)
    return files


def scan() -> dict:
    forbidden_hits: list[dict] = []
    tracked_hits: list[dict] = []

    for path in _iter_production_rust_files():
        rel = str(path.relative_to(CORE_SRC))
        raw = path.read_text(encoding="utf-8")
        prod = _strip_cfg_test_modules(raw)
        for line_no, line in enumerate(prod.splitlines(), start=1):
            for pattern, label in FORBIDDEN_PATTERNS:
                if pattern.search(line):
                    forbidden_hits.append({"file": rel, "line": line_no, "kind": label, "text": line.strip()})
            for pattern, label in TRACK_PATTERNS:
                if pattern.search(line):
                    tracked_hits.append({"file": rel, "line": line_no, "kind": label, "text": line.strip()})

    return {
        "crate": "quantwave-core",
        "files_scanned": len(_iter_production_rust_files()),
        "unsafe_forbidden": True,
        "panic_forbidden": True,
        "forbidden_violations": len(forbidden_hits),
        "tracked_unwrap_expect": len(tracked_hits),
        "forbidden": forbidden_hits,
        "tracked": tracked_hits,
    }


def main() -> int:
    if not CORE_SRC.exists():
        print(f"check_core_safety: missing {CORE_SRC}", file=sys.stderr)
        return 1

    data = scan()
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps({k: v for k, v in data.items() if k not in {"forbidden", "tracked"}}, indent=2) + "\n", encoding="utf-8")

    if data["forbidden_violations"]:
        print("check_core_safety: FAILED — production unsafe/panic detected", file=sys.stderr)
        for hit in data["forbidden"]:
            print(f"  {hit['file']}:{hit['line']}: {hit['kind']}", file=sys.stderr)
        return 1

    print(
        "check_core_safety: OK "
        f"(0 unsafe/panic in {data['files_scanned']} production files; "
        f"{data['tracked_unwrap_expect']} unwrap/expect tracked — workspace lint warn)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())