"""Rust indicator throughput + latency via quantwave-core benchmark_export binary."""

from __future__ import annotations

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ROWS = 1_000_000


def run_rust_benchmarks(quick: bool = False) -> dict:
    rows = 100_000 if quick else ROWS
    proc = subprocess.run(
        [
            "cargo",
            "run",
            "-p",
            "quantwave-core",
            "--release",
            "--bin",
            "benchmark_export",
            "--",
            str(rows),
        ],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    line = proc.stdout.strip().splitlines()[-1]
    payload = json.loads(line)
    return {
        "throughput": payload.get("throughput", {}),
        "latency": payload.get("latency", {}),
        "criterion": payload.get("criterion", {}),
    }