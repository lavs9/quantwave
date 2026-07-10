#!/usr/bin/env python3
"""Collect machine-readable validation/correctness stats (quantwave-ruh0.2)."""

from __future__ import annotations

import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GOLD_DIR = ROOT / "quantwave-core" / "tests" / "gold_standard"
METADATA = ROOT / "metadata_export.json"
OUT = ROOT / "docs" / "generated" / "validation_stats.json"
PARITY_REPORT = ROOT / "docs" / "generated" / "parity_coverage.json"

RUST_TEST_PACKAGES = (
    "quantwave-core",
    "quantwave-polars",
    "quantwave-backtest",
)


def _iter_rs_files(paths: list[Path]):
    for p in paths:
        if not p.exists():
            continue
        if p.is_file():
            yield p
        else:
            yield from p.rglob("*.rs")


def _rg_count(pattern: str, paths: list[Path]) -> int:
    """Count matching lines across .rs files (matches ``rg -c`` semantics)."""
    regex = re.compile(pattern)
    total = 0
    for path in _iter_rs_files(paths):
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        total += sum(1 for line in text.splitlines() if regex.search(line))
    return total


def _indicator_count() -> int:
    data = json.loads(METADATA.read_text(encoding="utf-8"))
    return len(data)


def _metadata_with_gold() -> int:
    data = json.loads(METADATA.read_text(encoding="utf-8"))
    return sum(1 for m in data if (m.get("gold_standard_file") or "").strip())


def _python_gold_parity_count() -> int:
    sys.path.insert(0, str(ROOT / "tests" / "python"))
    from gold_parity_registry import GOLD_PARITY_CASES  # noqa: E402

    return len(GOLD_PARITY_CASES)


def collect() -> dict:
    gold_files = sorted(p.name for p in GOLD_DIR.glob("*.json"))
    core_src = ROOT / "quantwave-core" / "src"
    core_tests = ROOT / "quantwave-core" / "tests"

    pkg_tests: dict[str, int] = {}
    for pkg in RUST_TEST_PACKAGES:
        base = ROOT / pkg
        paths = [base / "src", base / "tests"]
        pkg_tests[pkg] = _rg_count(r"#\[test\]", paths)

    proptest_blocks = _rg_count(r"proptest!\s*\{", [core_src, core_tests])
    batch_streaming_checks = _rg_count(r"check_batch_streaming_parity", [core_src])
    talib_parity_fns = _rg_count(
        r"fn test_\w+_parity",
        [core_tests / "test_all_talib_parity.rs", core_tests / "test_missing_talib_parity.rs"],
    )

    parity: dict = {}
    if PARITY_REPORT.exists():
        parity = json.loads(PARITY_REPORT.read_text(encoding="utf-8"))

    return {
        "generated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "indicator_count": _indicator_count(),
        "metadata_with_gold_file": _metadata_with_gold(),
        "gold_fixture_count": len(gold_files),
        "gold_fixtures": gold_files,
        "python_gold_parity_count": _python_gold_parity_count(),
        "python_gold_parity_deferred": 2,
        "rust_tests_by_package": pkg_tests,
        "rust_test_total": sum(pkg_tests.values()),
        "proptest_blocks": proptest_blocks,
        "batch_streaming_parity_checks": batch_streaming_checks,
        "talib_parity_test_fns": talib_parity_fns,
        "parity_proptest_indicators": parity.get("parity_proptest_count", 0),
        "parity_exemptions": parity.get("exemption_count", 0),
        "parity_gaps": parity.get("gap_count", 0),
    }


def main() -> int:
    stats = collect()
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(stats, indent=2) + "\n", encoding="utf-8")
    print(f"collect_validation_stats: wrote {OUT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())