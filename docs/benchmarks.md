# Benchmarks QuantWave publishes **only measured numbers** from the reproducible harness in `benchmarks/`. No hand-written throughput claims. <!-- bench:meta:start -->
**Last harness run:** 2026-07-08T07:38:14.970046+00:00
**Harness version:** 2
**Dataset:** 100,000 rows, seed `1364656129`
**Host CPU:** arm · **RAM:** 24.0 GB · **OS:** Darwin 25.5.0 Regenerate: `python benchmarks/harness.py && python scripts/render_benchmarks.py`
<!-- bench:meta:end --> ## Memory Usage <!-- bench:memory:start -->
Measured on **100,000** synthetic OHLCV rows (+ symbol column where noted). ### OHLCV + Symbol | Framework | Memory Usage | Footprint |
|-----------|----------------|-----------|
| **QuantWave (Polars)** | **4.48 MB** | **1.0x** |
| Pandas | 9.16 MB | 2.04x | ### High-cardinality strings (Symbol column only) | Framework | Memory | Footprint |
|-----------|--------|-----------|
| **QuantWave (Polars)** | **0.67 MB** | **1.0x** |
| Pandas | 5.34 MB | 8.0x |
<!-- bench:memory:end --> ## Speed & Throughput <!-- bench:throughput:start -->
### Rust streaming throughput - **Rows:** 100,000
- **Source:** `/benchmark_export` | Indicator | Mode | Time (ms) |
|-----------|------|-----------|
| SMA (20) | streaming | 0.2305 |
| RSI (14) | streaming | 0.0000 |
| SuperTrend (10,3) | streaming | 0.3457 | > Criterion HTML reports: `cargo bench -p --bench indicator_throughput` (100,000 rows per case).
<!-- bench:throughput:end --> ## Python Comparisons <!-- bench:comparisons:start -->
### SMA batch throughput (SMA(20), 100000 rows) Correctness pre-check on 1k rows passed before timing. | Library | Time (ms) |
|---------|-----------|
| QuantWave (.ta) | 0.4340 |
| Polars rolling_mean | 0.8796 |
| Pandas rolling | 0.4815 |
| TA-Lib | _not installed_ | **Library versions:** numpy 1.26.4, pandas 2.2.3, pandas_ta not_installed, polars 1.40.1, quantwave 0.6.0, talib not_installed
<!-- bench:comparisons:end --> ## Streaming Latency <!-- bench:latency:start -->
### Per-tick streaming latency (10,000 samples) Source: `per_tick_instrumented` — real per-tick instrumentation, not batch ms relabeled. | Indicator | Mean (ns) | P99 (ns) |
|-----------|-----------|----------|
| SMA (20) | 16.0 | 42.0 |
| RSI (14) | 15.0 | 42.0 |
<!-- bench:latency:end --> ## Methodology - **Data:** deterministic synthetic OHLCV from `benchmarks/data.py` (fixed seed, committed generator).
- **Rust:** `cargo run -p --release --bin benchmark_export` + Criterion benches (`cargo bench -p --bench indicator_throughput`).
- **Python:** `benchmarks/python_comparisons.py` — correctness pre-check on 1k rows, then `time.perf_counter` timings.
- **Docs:** `scripts/render_benchmarks.py` renders this page from `benchmarks/results/latest.json` only.
- **CI:** `scripts/check_benchmark_claims.py` fails on orphan performance numbers in README/docs. --- *Page rendered from JSON — edit `benchmarks/results/latest.json` via the harness, not tables here.*