#!/usr/bin/env python3
"""Collect machine-readable validation/correctness stats (quantwave-ruh0.2)."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
GOLD_DIR = ROOT / "quantwave-core" / "tests" / "gold_standard"
METADATA = ROOT / "metadata_export.json"
OUT = ROOT / "docs" / "generated" / "validation_stats.json"

RUST_TEST_PACKAGES = (
    "quantwave-core",
    "quantwave-polars",
    "quantwave-backtest",
)


def _rg_count(pattern: str, paths: list[Path]) -> int:
    cmd = ["rg", "-c", pattern, *[str(p) for p in paths if p.exists()]]
    try:
        out = subprocess.check_output(cmd, text=True, cwd=ROOT, stderr=subprocess.DEVNULL)
    except subprocess.CalledProcessError as exc:
        if exc.returncode == 1:
            return 0
        raise
    return sum(int(line.split(":")[-1]) for line in out.splitlines() if ":" in line)


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
    }


def main() -> int:
    stats = collect()
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(stats, indent=2) + "\n", encoding="utf-8")
    print(f"collect_validation_stats: wrote {OUT}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())