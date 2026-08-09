#!/usr/bin/env python3
"""Run benchmark harness and write results/latest.json."""

from __future__ import annotations

import argparse
import importlib.util
import json
import platform
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from benchmarks.data import OhlcvConfig, frame_hash, generate_ohlcv  # noqa: E402

RESULTS_DIR = Path(__file__).resolve().parent / "results"
RESULTS_JSON = RESULTS_DIR / "latest.json"
HARNESS_VERSION = 2

# (import name, pip name, why the harness needs it, needed even for --dry-run).
# Checked up front so a missing dependency fails immediately instead of
# half-way through a 1M-row run.
REQUIRED_MODULES: tuple[tuple[str, str, str, bool], ...] = (
    ("polars", "polars", "the frames under test", True),
    ("numpy", "numpy", "array conversion for the comparison timings", True),
    ("pandas", "pandas", "the pandas side of the memory-footprint comparison", False),
    (
        "pyarrow",
        "pyarrow",
        "the pandas conversion in the memory benchmark (polars .to_pandas() "
        "delegates to pyarrow)",
        False,
    ),
)


def check_dependencies(dry_run: bool = False) -> None:
    """Fail fast with an actionable message if a harness dependency is missing."""
    missing = [
        (mod, pkg, why)
        for mod, pkg, why, needed_for_dry_run in REQUIRED_MODULES
        if (needed_for_dry_run or not dry_run)
        and importlib.util.find_spec(mod) is None
    ]
    if not missing:
        return

    lines = ["benchmarks/harness.py is missing required dependencies:"]
    lines += [f"  - {mod} — needed for {why}" for mod, _pkg, why in missing]
    lines.append("")
    lines.append("Install them with:")
    lines.append("    pip install " + " ".join(pkg for _mod, pkg, _why in missing))
    raise SystemExit("\n".join(lines))


def hardware_info() -> dict[str, str]:
    try:
        import psutil

        ram_gb = f"{psutil.virtual_memory().total / (1024**3):.1f} GB"
    except ImportError:
        ram_gb = "unknown"
    return {
        "cpu": platform.processor() or platform.machine(),
        "ram": ram_gb,
        "os": f"{platform.system()} {platform.release()}",
        "python": platform.python_version(),
    }


def memory_benchmark(df) -> dict[str, float]:
    import pandas as pd

    pdf = df.select(["open", "high", "low", "close", "volume", "symbol"]).to_pandas()
    pl_mb = df.estimated_size() / (1024 * 1024)
    pd_mb = pdf.memory_usage(deep=True).sum() / (1024 * 1024)

    sym_df = df.select("symbol")
    sym_pl_mb = sym_df.estimated_size() / (1024 * 1024)
    sym_pd_mb = sym_df.to_pandas().memory_usage(deep=True).sum() / (1024 * 1024)

    return {
        "ohlcv_symbol_quantwave_mb": round(pl_mb, 2),
        "ohlcv_symbol_pandas_mb": round(pd_mb, 2),
        "ohlcv_symbol_ratio": round(pd_mb / pl_mb, 2) if pl_mb else 0.0,
        "strings_quantwave_mb": round(sym_pl_mb, 2),
        "strings_pandas_mb": round(sym_pd_mb, 2),
        "strings_ratio": round(sym_pd_mb / sym_pl_mb, 2) if sym_pl_mb else 0.0,
    }


def rust_benchmarks(quick: bool) -> dict:
    from benchmarks.rust_timing import run_rust_benchmarks

    return run_rust_benchmarks(quick=quick)


def python_benchmarks(quick: bool) -> dict:
    from benchmarks.python_comparisons import run_comparisons

    return run_comparisons(quick=quick)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--quick", action="store_true", help="Use 100k rows for dev smoke")
    parser.add_argument("--dry-run", action="store_true", help="Validate data only")
    parser.add_argument("--skip-python", action="store_true", help="Skip Python comparisons")
    parser.add_argument("--skip-rust", action="store_true", help="Skip Rust benchmarks")
    args = parser.parse_args()

    check_dependencies(dry_run=args.dry_run)

    rows = 100_000 if args.quick else 1_000_000
    cfg = OhlcvConfig(rows=rows)
    df = generate_ohlcv(cfg)
    digest = frame_hash(df)

    if args.dry_run:
        print(f"frame_hash={digest} rows={rows}")
        return 0

    t0 = time.perf_counter()
    rust: dict = {"throughput": {}, "latency": {}, "criterion": {}}
    if not args.skip_rust:
        try:
            rust = rust_benchmarks(args.quick)
        except Exception as exc:  # noqa: BLE001
            rust = {"error": str(exc)}

    comparisons: dict = {}
    if not args.skip_python:
        try:
            comparisons = python_benchmarks(args.quick)
        except Exception as exc:  # noqa: BLE001
            comparisons = {"error": str(exc)}

    payload = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "harness_version": HARNESS_VERSION,
        "dataset": {"rows": rows, "seed": cfg.seed, "frame_hash": digest},
        "hardware": hardware_info(),
        "memory": memory_benchmark(df),
        "throughput": rust.get("throughput", {}),
        "latency": rust.get("latency", {}),
        "criterion": rust.get("criterion", {}),
        "comparisons": comparisons,
        "notes": [
            "All numbers generated by benchmarks/harness.py — do not hand-edit docs tables.",
            "Latency uses per-tick instrumentation (benchmark_export), not batch ms relabeled.",
        ],
    }
    elapsed = time.perf_counter() - t0
    payload["harness_runtime_s"] = round(elapsed, 3)

    RESULTS_DIR.mkdir(parents=True, exist_ok=True)
    RESULTS_JSON.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
    print(f"Wrote {RESULTS_JSON}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())