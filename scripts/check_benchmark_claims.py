#!/usr/bin/env python3
"""Fail CI when docs contain unmeasured performance claims."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RESULTS_JSON = ROOT / "benchmarks" / "results" / "latest.json"

SCAN_FILES = [
    ROOT / "docs" / "benchmarks.md",
    ROOT / "docs" / "comparison.md",
    ROOT / "docs" / "index.md",
    ROOT / "docs" / "guides" / "indicators" / "gallery.md",
    ROOT / "README.md",
]

# Patterns that indicate a numeric performance claim in prose/tables.
CLAIM_PATTERNS = [
    re.compile(r"~\s*\d+\s*×\s*faster", re.IGNORECASE),
    re.compile(r"\b\d+\s*×\s*faster", re.IGNORECASE),
    re.compile(r">\s*\d+\s*ms", re.IGNORECASE),
    re.compile(r"\b\d+\.\d+\s*ms\b"),
    re.compile(r"\b\d+\.\d+\s*ns\b"),
    re.compile(r"\[Placeholder[^\]]*\]"),
    re.compile(r"gen_benchmarks\.py", re.IGNORECASE),
]

# Lines allowed to mention ms/ns/× when documenting the rebuild or linking to harness.
ALLOW_SUBSTRINGS = [
    "being rebuilt",
    "fabricated",
    "relabel",
    "will appear here",
    "preliminary measurement",
    "benchmark_results.md",
    "quantwave-9gek.2",
    "not hand-written",
    "removed",
    "harness",
    "See [benchmarks]",
    "see [benchmarks]",
    "Full benchmarks",
    "memory_usage(deep=True)",
    "estimated_size()",
    "2x to 5x lower memory",
    "2–5× lower memory",
    "2-5x lower memory",
    "10-100x slower",  # README caveat about Python-first libs
    "faster response",
    "faster oscillator",
    "faster trend",
    "faster alternative",
    "faster reaction",
    "reacts faster",
    "faster than a shorter",
    "faster than MACD",
    "faster lower-lag",
    "faster trend identification",
]

# Table rows in comparison.md that had fabricated numbers - we neutralize those separately.
EXEMPT_FILES = set()


def load_provenance_values() -> set[str]:
    if not RESULTS_JSON.exists():
        return set()
    data = json.loads(RESULTS_JSON.read_text(encoding="utf-8"))
    values: set[str] = set()
    for section in ("throughput", "latency", "comparisons", "memory"):
        block = data.get(section)
        if isinstance(block, dict):
            for v in block.values():
                if isinstance(v, (int, float)):
                    values.add(f"{v:.2f}")
                    values.add(str(round(v, 2)))
    return values


def line_allowed(line: str, provenance: set[str]) -> bool:
    if any(sub in line for sub in ALLOW_SUBSTRINGS):
        return True
    if any(
        m in line
        for m in (
            "bench:",
            "_pending_",
            "not installed",
            "not available",
            "Regenerate:",
            "rendered from JSON",
        )
    ):
        return True
    return any(p in line for p in provenance)


def scan_file(path: Path, provenance: set[str]) -> list[str]:
    if not path.exists() or path in EXEMPT_FILES:
        return []
    text = path.read_text(encoding="utf-8")
    issues: list[str] = []
    for lineno, line in enumerate(text.splitlines(), start=1):
        if line_allowed(line, provenance):
            continue
        for pattern in CLAIM_PATTERNS:
            if pattern.search(line):
                rel = path.relative_to(ROOT)
                issues.append(f"{rel}:{lineno}: unmeasured claim: {line.strip()[:120]}")
                break
    return issues


def main() -> int:
    provenance = load_provenance_values()

    issues: list[str] = []
    for path in SCAN_FILES:
        issues.extend(scan_file(path, provenance))

    if issues:
        print("Benchmark claim drift FAILED:", file=sys.stderr)
        for issue in issues:
            print(f"  - {issue}", file=sys.stderr)
        if not RESULTS_JSON.exists():
            print(
                "  Hint: remove or neutralize claims above; publish harness JSON to "
                "benchmarks/results/latest.json before re-adding measured numbers.",
                file=sys.stderr,
            )
        return 1

    print(f"check_benchmark_claims: OK (provenance values: {len(provenance)})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())