#!/usr/bin/env python3
"""Render generated sections of docs/validation.md from validation_stats.json."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STATS = ROOT / "docs" / "generated" / "validation_stats.json"
VALIDATION_MD = ROOT / "docs" / "validation.md"


def _replace_block(text: str, start: str, end: str, body: str) -> str:
    if start not in text:
        raise ValueError(f"missing marker {start}")
    return re.sub(
        rf"{re.escape(start)}.*?{re.escape(end)}",
        f"{start}\n{body.rstrip()}\n{end}",
        text,
        flags=re.DOTALL,
    )


def render_stats(data: dict) -> str:
    pkg = data["rust_tests_by_package"]
    lines = [
        f"**Last updated:** {data['generated_at']} (UTC)",
        "",
        "| Metric | Count | Source |",
        "|--------|------:|--------|",
        f"| Native indicators (`*_METADATA`) | **{data['indicator_count']}** | `metadata_export.json` |",
        f"| Metadata entries with gold file reference | {data['metadata_with_gold_file']} | `metadata_export.json` |",
        f"| Gold-standard JSON fixtures (on disk) | **{data['gold_fixture_count']}** | `quantwave-core/tests/gold_standard/` |",
        f"| Python streaming gold parity cases | **{data['python_gold_parity_count']}** | `tests/python/gold_parity_registry.py` |",
        f"| Python gold parity deferred (HMM) | {data['python_gold_parity_deferred']} | regime fixtures — separate suite |",
        f"| Rust `#[test]` functions (core) | {pkg['quantwave-core']} | `rg '#[test]' quantwave-core` |",
        f"| Rust `#[test]` functions (polars) | {pkg['quantwave-polars']} | `rg '#[test]' quantwave-polars` |",
        f"| Rust `#[test]` functions (backtest) | {pkg['quantwave-backtest']} | `rg '#[test]' quantwave-backtest` |",
        f"| Rust tests (total, 3 crates) | **{data['rust_test_total']}** | sum of above |",
        f"| `proptest!` blocks (core) | **{data['proptest_blocks']}** | `rg 'proptest!\\{{' quantwave-core` |",
        f"| `check_batch_streaming_parity` call sites | {data['batch_streaming_parity_checks']} | indicator modules |",
        f"| TA-Lib parity test functions | {data['talib_parity_test_fns']} | `test_*_talib_parity.rs` |",
        f"| Indicators with proptest parity (CI-enforced) | **{data['parity_proptest_indicators']}** | `check_indicator_parity_coverage.py` |",
        f"| Reviewed parity exemptions | {data['parity_exemptions']} | `parity_exemptions.toml` |",
        "",
        "Regenerate: `python scripts/collect_validation_stats.py && python scripts/render_validation_docs.py`",
    ]
    return "\n".join(lines)


def render_gold_list(data: dict) -> str:
    lines = [
        "Each file below lives in `quantwave-core/tests/gold_standard/` and is consumed by Rust unit tests",
        "and/or the Python gold parity registry.",
        "",
        "| Fixture |",
        "|---------|",
    ]
    for name in data["gold_fixtures"]:
        lines.append(f"| `{name}` |")
    return "\n".join(lines)


def main() -> int:
    if not STATS.exists():
        print(f"render_validation_docs: missing {STATS}; run collect_validation_stats.py first", file=sys.stderr)
        return 1
    data = json.loads(STATS.read_text(encoding="utf-8"))
    if not VALIDATION_MD.exists():
        print(f"render_validation_docs: missing {VALIDATION_MD}", file=sys.stderr)
        return 1

    text = VALIDATION_MD.read_text(encoding="utf-8")
    text = _replace_block(text, "<!-- VALIDATION:STATS:START -->", "<!-- VALIDATION:STATS:END -->", render_stats(data))
    text = _replace_block(text, "<!-- VALIDATION:GOLD:START -->", "<!-- VALIDATION:GOLD:END -->", render_gold_list(data))
    VALIDATION_MD.write_text(text, encoding="utf-8")
    print(f"render_validation_docs: updated {VALIDATION_MD}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())