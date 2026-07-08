#!/usr/bin/env python3
"""Render docs/benchmarks.md from benchmarks/results/latest.json (single source of truth)."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RESULTS = ROOT / "benchmarks" / "results" / "latest.json"
BENCH_MD = ROOT / "docs" / "benchmarks.md"


def _replace_block(text: str, start: str, end: str, body: str) -> str:
    if start not in text:
        raise ValueError(f"missing marker {start}")
    return re.sub(
        rf"{re.escape(start)}.*?{re.escape(end)}",
        f"{start}\n{body.rstrip()}\n{end}",
        text,
        flags=re.DOTALL,
    )


def render_meta(data: dict) -> str:
    hw = data.get("hardware", {})
    ds = data.get("dataset", {})
    lines = [
        f"**Last harness run:** {data.get('generated_at', 'unknown')}",
        f"**Harness version:** {data.get('harness_version', '?')}",
        f"**Dataset:** {ds.get('rows', '?'):,} rows, seed `{ds.get('seed', '?')}`"
        if isinstance(ds.get("rows"), int)
        else f"**Dataset:** {ds.get('rows', '?')} rows",
        f"**Host CPU:** {hw.get('cpu', 'unknown')} · **RAM:** {hw.get('ram', '?')} · **OS:** {hw.get('os', '?')}",
        "",
        "Regenerate: `python benchmarks/harness.py && python scripts/render_benchmarks.py`",
    ]
    return "\n".join(lines)


def render_memory(mem: dict, rows: int) -> str:
    lines = [
        f"Measured on **{rows:,}** synthetic OHLCV rows (+ symbol column where noted).",
        "",
        "### OHLCV + Symbol",
        "",
        "| Framework | Memory Usage | Footprint |",
        "|-----------|----------------|-----------|",
        f"| **QuantWave (Polars)** | **{mem.get('ohlcv_symbol_quantwave_mb', '?')} MB** | **1.0x** |",
        f"| Pandas | {mem.get('ohlcv_symbol_pandas_mb', '?')} MB | {mem.get('ohlcv_symbol_ratio', '?')}x |",
        "",
        "### High-cardinality strings (Symbol column only)",
        "",
        "| Framework | Memory | Footprint |",
        "|-----------|--------|-----------|",
        f"| **QuantWave (Polars)** | **{mem.get('strings_quantwave_mb', '?')} MB** | **1.0x** |",
        f"| Pandas | {mem.get('strings_pandas_mb', '?')} MB | {mem.get('strings_ratio', '?')}x |",
    ]
    return "\n".join(lines)


def render_throughput(data: dict) -> str:
    hw = data.get("hardware", {})
    tp = data.get("throughput", {})
    rows = tp.get("rows", data.get("dataset", {}).get("rows", "?"))
    lines = [
        "### Rust streaming throughput",
        "",
        f"- **Rows:** {rows:,}" if isinstance(rows, int) else f"- **Rows:** {rows}",
        f"- **Source:** `{tp.get('source', 'benchmark_export')}`",
        "",
        "| Indicator | Mode | Time (ms) |",
        "|-----------|------|-----------|",
    ]
    for label, key in [
        ("SMA (20)", "sma_20_streaming_ms"),
        ("RSI (14)", "rsi_14_streaming_ms"),
        ("SuperTrend (10,3)", "supertrend_10_3_streaming_ms"),
    ]:
        if key in tp and isinstance(tp[key], (int, float)):
            lines.append(f"| {label} | streaming | {tp[key]:.4f} |")
    crit = data.get("criterion", {})
    if crit:
        lines.extend(
            [
                "",
                f"> Criterion HTML reports: `cargo bench -p quantwave-core --bench indicator_throughput` "
                f"({crit.get('bench_rows', 100_000):,} rows per case).",
            ]
        )
    return "\n".join(lines)


def render_comparisons(comp: dict) -> str:
    if not comp or "error" in comp:
        return f"Python comparisons unavailable: `{comp.get('error', 'skipped')}`"
    timings = comp.get("timings_ms", {})
    libs = comp.get("libraries", {})
    lines = [
        f"### SMA batch throughput ({comp.get('indicator', 'SMA')}, {comp.get('rows', '?')} rows)",
        "",
        "Correctness pre-check on 1k rows passed before timing.",
        "",
        "| Library | Time (ms) |",
        "|---------|-----------|",
    ]
    labels = {
        "quantwave_polars_ta": "QuantWave (.ta)",
        "polars_rolling_mean": "Polars rolling_mean",
        "pandas_rolling": "Pandas rolling",
        "talib_batch": "TA-Lib",
    }
    for key, label in labels.items():
        val = timings.get(key)
        if isinstance(val, (int, float)):
            lines.append(f"| {label} | {val:.4f} |")
        elif val is None:
            lines.append(f"| {label} | _not installed_ |")
    if libs:
        lib_line = ", ".join(f"{k} {v}" for k, v in sorted(libs.items()))
        lines.extend(["", f"**Library versions:** {lib_line}"])
    return "\n".join(lines)


def render_latency(lat: dict) -> str:
    if not lat or "source" not in lat:
        return "_Per-tick latency not available (run full harness with Rust benchmarks)._"
    samples = lat.get("samples", "?")
    lines = [
        f"### Per-tick streaming latency ({samples:,} samples)"
        if isinstance(samples, int)
        else f"### Per-tick streaming latency ({samples} samples)",
        "",
        f"Source: `{lat.get('source')}` — real per-tick instrumentation, not batch ms relabeled.",
        "",
        "| Indicator | Mean (ns) | P99 (ns) |",
        "|-----------|-----------|----------|",
    ]
    pairs = [
        ("SMA (20)", "sma_20_mean_ns", "sma_20_p99_ns"),
        ("RSI (14)", "rsi_14_mean_ns", "rsi_14_p99_ns"),
    ]
    for label, mean_k, p99_k in pairs:
        if mean_k in lat and p99_k in lat:
            lines.append(f"| {label} | {lat[mean_k]:.1f} | {lat[p99_k]:.1f} |")
    return "\n".join(lines)


def main() -> int:
    if not RESULTS.exists():
        print(f"render_benchmarks: missing {RESULTS}", file=sys.stderr)
        return 1
    data = json.loads(RESULTS.read_text(encoding="utf-8"))
    rows = data.get("dataset", {}).get("rows", 1_000_000)

    text = BENCH_MD.read_text(encoding="utf-8")
    text = _replace_block(text, "<!-- bench:meta:start -->", "<!-- bench:meta:end -->", render_meta(data))
    text = _replace_block(
        text, "<!-- bench:memory:start -->", "<!-- bench:memory:end -->", render_memory(data.get("memory", {}), rows)
    )
    text = _replace_block(
        text, "<!-- bench:throughput:start -->", "<!-- bench:throughput:end -->", render_throughput(data)
    )
    text = _replace_block(
        text,
        "<!-- bench:comparisons:start -->",
        "<!-- bench:comparisons:end -->",
        render_comparisons(data.get("comparisons", {})),
    )
    text = _replace_block(
        text, "<!-- bench:latency:start -->", "<!-- bench:latency:end -->", render_latency(data.get("latency", {}))
    )
    BENCH_MD.write_text(text, encoding="utf-8")
    print(f"render_benchmarks: updated {BENCH_MD}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())