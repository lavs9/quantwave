#!/usr/bin/env python3
"""Fail CI if tracked repo cruft reappears (quantwave-9gek.5)."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

FORBIDDEN_PATTERNS = [
    "scratch_",
    "fix_tests",
    "fix_code",
    "patch_script",
    "clippy_errors",
    "nextest_output.txt",
    ".traineddata",
    "/dist/",
    "dist/",
]

ALLOWLIST_LARGE = {
    "references/",
    "metadata_export.json",
}


def tracked_files() -> list[str]:
    out = subprocess.run(
        ["git", "ls-files"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return [line.strip() for line in out.stdout.splitlines() if line.strip()]


def main() -> int:
    errors: list[str] = []
    for rel in tracked_files():
        path = ROOT / rel
        for pat in FORBIDDEN_PATTERNS:
            if pat in rel:
                errors.append(f"forbidden tracked path: {rel} (matched {pat!r})")
                break
        if path.is_file() and path.stat().st_size > 2_000_000:
            if not any(rel.startswith(a) for a in ALLOWLIST_LARGE):
                errors.append(f"large tracked file >2MB: {rel}")

    if errors:
        print("Repo hygiene FAILED:", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    print("check_repo_hygiene: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())