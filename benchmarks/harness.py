#!/usr/bin/env python3
"""Run benchmark harness and write results/latest.json."""

from __future__ import annotations

import argparse
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
    return {
        "quantwave_polars_mb": round(pl_mb, 2),
        "pandas_mb": round(pd_mb, 2),
        "footprint_ratio": round(pd_mb / pl_mb, 2) if pl_mb else 0.0,
    }


def rust_throughput(quick: bool) -> dict[str, float | int | str]:
    from benchmarks.rust_timing import run_rust_throughput

    try:
        return run_rust_throughput(quick=quick)
    except Exception as exc:  # noqa: BLE001 — bench optional in partial envs
        return {"error": str(exc), "mode": "rust_streaming"}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--quick", action="store_true", help="Use 100k rows for dev smoke")
    parser.add_argument("--dry-run", action="store_true", help="Validate data only")
    args = parser.parse_args()

    rows = 100_000 if args.quick else 1_000_000
    cfg = OhlcvConfig(rows=rows)
    df = generate_ohlcv(cfg)
    digest = frame_hash(df)

    if args.dry_run:
        print(f"frame_hash={digest} rows={rows}")
        return 0

    t0 = time.perf_counter()
    payload = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "harness_version": 1,
        "dataset": {"rows": rows, "seed": cfg.seed, "frame_hash": digest},
        "hardware": hardware_info(),
        "memory": memory_benchmark(df),
        "throughput": rust_throughput(args.quick),
        "latency": {},
        "comparisons": {},
        "notes": [
            "Rust streaming throughput measured via benchmarks/rust_timing.py. "
            "Python competitor comparisons are Phase 2b.",
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