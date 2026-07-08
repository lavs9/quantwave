"""Rust indicator throughput timings via `cargo run --release` micro-bench."""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ROWS = 1_000_000


def _bench_source() -> str:
    return f"""
use std::time::Instant;
use quantwave_core::Next;
use quantwave_core::indicators::smoothing::SMA;
use quantwave_core::indicators::momentum::RSI;
use quantwave_core::indicators::supertrend::SuperTrend;

fn bench_sma() -> f64 {{
    let data: Vec<f64> = (0..{ROWS}).map(|i| 100.0 + (i as f64) * 0.001).collect();
    let mut sma = SMA::new(20);
    let t0 = Instant::now();
    for &x in &data {{
        let _ = sma.next(x);
    }}
    t0.elapsed().as_secs_f64() * 1000.0
}}

fn bench_rsi() -> f64 {{
    let data: Vec<f64> = (0..{ROWS}).map(|i| 100.0 + (i as f64) * 0.001).collect();
    let mut rsi = RSI::new(14);
    let t0 = Instant::now();
    for &x in &data {{
        let _ = rsi.next(x);
    }}
    t0.elapsed().as_secs_f64() * 1000.0
}}

fn bench_supertrend() -> f64 {{
    let data: Vec<(f64, f64, f64)> = (0..{ROWS})
        .map(|i| {{
            let c = 100.0 + (i as f64) * 0.001;
            (c + 1.0, c - 1.0, c)
        }})
        .collect();
    let mut st = SuperTrend::new(10, 3.0);
    let t0 = Instant::now();
    for bar in &data {{
        let _ = st.next(*bar);
    }}
    t0.elapsed().as_secs_f64() * 1000.0
}}

fn main() {{
    let out = serde_json::json!({{
        "sma_20_streaming_ms": bench_sma(),
        "rsi_14_streaming_ms": bench_rsi(),
        "supertrend_10_3_streaming_ms": bench_supertrend(),
        "rows": {ROWS},
        "mode": "rust_streaming",
    }});
    println!("{{}}", out);
}}
"""


def run_rust_throughput(quick: bool = False) -> dict[str, float | int | str]:
    rows = 100_000 if quick else ROWS
    source = _bench_source().replace(str(ROWS), str(rows))
    with tempfile.TemporaryDirectory(prefix="qw-bench-") as td:
        manifest = Path(td) / "Cargo.toml"
        manifest.write_text(
            f"""
[package]
name = "qw-bench"
version = "0.0.0"
edition = "2021"

[dependencies]
quantwave-core = {{ path = "{ROOT / "quantwave-core"}" }}
serde_json = "1"
""",
            encoding="utf-8",
        )
        (Path(td) / "src").mkdir()
        (Path(td) / "src" / "main.rs").write_text(source, encoding="utf-8")
        proc = subprocess.run(
            ["cargo", "run", "--release", "--quiet"],
            cwd=td,
            check=True,
            capture_output=True,
            text=True,
        )
        line = proc.stdout.strip().splitlines()[-1]
        return json.loads(line)


if __name__ == "__main__":
    print(json.dumps(run_rust_throughput("--quick" in sys.argv), indent=2))