#!/usr/bin/env python3
"""Fail on undelimited LaTeX math in indicator docs (quantwave-5ipk.5).

Block/inline math must be wrapped in arithmatex delimiters (``\\[ ... \\]``,
``$$ ... $$``, or ``$ ... $`` / ``\\( ... \\)``) so it renders instead of
showing as raw ``\\text{...}`` source. Scans real content pages under
``docs/guides/`` (skipping fenced code blocks and the meta standards doc, which
intentionally shows bad examples).
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"
SCAN_DIRS = [DOCS / "guides"]
# Meta docs that intentionally include "bad" LaTeX examples inside prose/fences.
SKIP_FILES = {DOCS / "DOCUMENTATION_STANDARDS.md"}

MATH_CMD = re.compile(
    r"\\(text|frac|sum|prod|sqrt|alpha|beta|gamma|sigma|mu|lambda|le|ge|neq|"
    r"times|cdot|sin|cos|tan|partial|infty|hat|bar|hbar|int|lim|log|ln)\b"
)


def _undelimited_math_lines(text: str) -> list[tuple[int, str]]:
    bad: list[tuple[int, str]] = []
    in_fence = False
    in_block = False
    for i, line in enumerate(text.split("\n"), start=1):
        stripped = line.strip()
        if stripped.startswith("```"):
            in_fence = not in_fence
            continue
        if in_fence:
            continue
        if r"\[" in line:
            in_block = True
        has_math = bool(MATH_CMD.search(line))
        if in_block:
            if r"\]" in line:
                in_block = False
            continue
        if not has_math:
            continue
        # inline-delimited math on this line is fine
        if "$$" in line or line.count("$") >= 2 or r"\(" in line:
            continue
        bad.append((i, stripped[:80]))
    return bad


def main() -> int:
    violations: list[str] = []
    for base in SCAN_DIRS:
        if not base.exists():
            continue
        for md in sorted(base.rglob("*.md")):
            if md in SKIP_FILES:
                continue
            text = md.read_text(encoding="utf-8")
            for lineno, snippet in _undelimited_math_lines(text):
                violations.append(f"{md.relative_to(ROOT)}:{lineno}: {snippet}")

    if violations:
        print(
            f"check_doc_latex: {len(violations)} undelimited LaTeX line(s) "
            "(wrap in \\[ ... \\] or $$ ... $$):",
            file=sys.stderr,
        )
        for v in violations:
            print(f"  - {v}", file=sys.stderr)
        return 1
    print("check_doc_latex: OK (all math delimited)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
