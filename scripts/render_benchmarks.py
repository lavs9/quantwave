#!/usr/bin/env python3
"""Render measured throughput table into docs/benchmarks.md from JSON."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RESULTS = ROOT / "benchmarks" / "results" / "latest.json"
BENCH_MD = ROOT / "docs" / "benchmarks.md"

TABLE_START = "<!-- bench:throughput:start -->"
TABLE_END = "<!-- bench:throughput:end -->"


def build_table(data: dict) -> str:
    hw = data.get("hardware", {})
    tp = data.get("throughput", {})
    rows = tp.get("rows", "n/a")
    lines = [
        TABLE_START,
        "### Rust streaming throughput (measured)",
        "",
        f"- **CPU**: {hw.get('cpu', 'unknown')}",
        f"- **RAM**: {hw.get('ram', 'unknown')}",
        f"- **OS**: {hw.get('os', 'unknown')}",
        f"- **Python**: {hw.get('python', 'n/a')} (harness host)",
        f"- **Rows**: {rows:,}" if isinstance(rows, int) else f"- **Rows**: {rows}",
        "",
        "| Indicator | Mode | Time (ms) |",
        "|-----------|------|-----------|",
    ]
    mapping = [
        ("SMA (20)", "sma_20_streaming_ms"),
        ("RSI (14)", "rsi_14_streaming_ms"),
        ("SuperTrend (10,3)", "supertrend_10_3_streaming_ms"),
    ]
    for label, key in mapping:
        if key in tp:
            lines.append(f"| {label} | streaming | {tp[key]:.2f} |")
    lines.extend(["", TABLE_END, ""])
    return "\n".join(lines)


def main() -> int:
    if not RESULTS.exists():
        print(f"render_benchmarks: missing {RESULTS}", file=sys.stderr)
        return 1
    data = json.loads(RESULTS.read_text(encoding="utf-8"))
    if not data.get("throughput"):
        print("render_benchmarks: no throughput data", file=sys.stderr)
        return 1

    table = build_table(data)
    text = BENCH_MD.read_text(encoding="utf-8")
    if TABLE_START in text:
        text = re.sub(
            rf"{re.escape(TABLE_START)}.*?{re.escape(TABLE_END)}",
            table.rstrip(),
            text,
            flags=re.DOTALL,
        )
    else:
        anchor = "## Speed & Latency"
        if anchor not in text:
            print("render_benchmarks: anchor missing in benchmarks.md", file=sys.stderr)
            return 1
        text = text.replace(anchor, f"{anchor}\n\n{table}")
    BENCH_MD.write_text(text, encoding="utf-8")
    print(f"render_benchmarks: updated {BENCH_MD}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())