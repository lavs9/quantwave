# Benchmarks

QuantWave publishes **only measured numbers** from the reproducible harness in `benchmarks/`. No hand-written throughput claims.

<!-- bench:meta:start -->
**Last harness run:** 2026-11-08T12:51:30.329846+00:00
**Harness version:** 2
**Dataset:** 1,000,000 rows, seed `1364656129`
**Host CPU:** x86_64 · **RAM:** 15.6 GB · **OS:** Linux 7.0.0-1017-azure

Regenerate: `python benchmarks/harness.py && python scripts/render_benchmarks.py`
<!-- bench:meta:end -->

## Memory Usage

<!-- bench:memory:start -->
Measured on **1,000,000** synthetic OHLCV rows (+ symbol column where noted).

### OHLCV + Symbol

| Framework | Memory Usage | Footprint |
|-----------|----------------|-----------|
| **QuantWave (Polars)** | **44.82 MB** | **1.0x** |
| Pandas | 52.45 MB | 1.17x |

### High-cardinality strings (Symbol column only)

| Framework | Memory | Footprint |
|-----------|--------|-----------|
| **QuantWave (Polars)** | **6.68 MB** | **1.0x** |
| Pandas | 14.31 MB | 2.14x |
<!-- bench:memory:end -->

## Speed & Throughput

<!-- bench:throughput:start -->
### Rust streaming throughput

- **Rows:** 1,000,000
- **Source:** `quantwave-core/benchmark_export`

| Indicator | Mode | Time (ms) |
|-----------|------|-----------|
| SMA (20) | streaming | 4.0978 |
| RSI (14) | streaming | 0.0000 |
| SuperTrend (10,3) | streaming | 5.2187 |

> Criterion HTML reports: `cargo bench -p quantwave-core --bench indicator_throughput` (100,000 rows per case).
<!-- bench:throughput:end -->

## Python Comparisons

<!-- bench:comparisons:start -->
### SMA batch throughput (SMA(20), 1000000 rows)

Correctness pre-check on 1k rows passed before timing.

| Library | Time (ms) |
|---------|-----------|
| QuantWave (.ta) | 6.7366 |
| Polars rolling_mean | 7.0553 |
| Pandas rolling | 9.3167 |
| TA-Lib | _not installed_ |

**Library versions:** numpy 2.5.4, pandas 3.1.1, pandas_ta not_installed, polars 1.44.2, quantwave 0.8.1, talib not_installed
<!-- bench:comparisons:end -->

## Streaming Latency

<!-- bench:latency:start -->
### Per-tick streaming latency (10,000 samples)

Source: `per_tick_instrumented` — real per-tick instrumentation, not batch ms relabeled.

| Indicator | Mean (ns) | P99 (ns) |
|-----------|-----------|----------|
| SMA (20) | 25.8 | 31.0 |
| RSI (14) | 24.7 | 31.0 |
<!-- bench:latency:end -->

## Methodology

- **Data:** deterministic synthetic OHLCV from `benchmarks/data.py` (fixed seed, committed generator).
- **Rust:** `cargo run -p quantwave-core --release --bin benchmark_export` + Criterion benches (`cargo bench -p quantwave-core --bench indicator_throughput`). The head-to-head against the `talib-rs` oracle lives in `cargo bench -p quantwave-core --bench talib_comparison` (a bench, not a bin, so `talib-rs` stays out of the shipped dependency graph).
- **Python:** `benchmarks/python_comparisons.py` — correctness pre-check on 1k rows, then `time.perf_counter` timings.
- **Docs:** `scripts/render_benchmarks.py` renders this page from `benchmarks/results/latest.json` only.
- **CI:** `scripts/check_benchmark_claims.py` fails on orphan performance numbers in README/docs.

---

*Page rendered from JSON — edit `benchmarks/results/latest.json` via the harness, not tables here.*