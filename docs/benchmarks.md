# Benchmarks

QuantWave is built for speed. We publish only measurements from a reproducible harness — not hand-written marketing numbers.

## Status

**Performance benchmarks are being rebuilt.** Earlier versions of this page contained unmeasured throughput figures and a fabricated streaming-latency table (batch milliseconds relabeled as nanoseconds). Those have been removed.

We are building a committed harness under `benchmarks/` that will:

- Run Rust criterion benches and Python comparisons on deterministic synthetic OHLCV data (1M+ rows, fixed seed).
- Record hardware metadata, library versions, and machine-readable JSON results.
- Render this page from that JSON only (CI drift gate — no orphan performance claims).

Track progress: [quantwave-9gek.2](https://github.com/lavs9/quantwave/issues) (benchmark teardown/rebuild).

## Memory Usage (measured)

QuantWave leverages Arrow's zero-copy memory model via Polars. While raw numeric columns have similar footprints across frameworks, QuantWave's advantage becomes substantial when dealing with **realistic quantitative datasets** (multi-column OHLCV + high-cardinality String symbols).

These figures come from real `estimated_size()` / `memory_usage(deep=True)` measurements on synthetic data.

### Benchmark: 1M Rows (OHLCV + Symbol)

We compare a dataset containing 5 numeric columns (`float64`) and 1 `Symbol` column with 1,000 unique tickers.

| Framework | Memory Usage | Footprint |
|-----------|----------------|---------|
| **QuantWave (Polars)** | **41.96 MB** | **1.0x** |
| Pandas    | 88.69 MB | 2.1x |

### Benchmark: High-Cardinality Strings

When isolating just the `Symbol` column (1M rows of ticker strings), the Arrow memory layout used by QuantWave is significantly more optimized than Pandas' Python-object based strings.

| Framework | Memory (Strings) | Footprint |
|-----------|------------------|-----------|
| **QuantWave (Polars)** | **11.44 MB** | **1.0x** |
| Pandas    | 58.17 MB | **~5.1x** |

> **Takeaway**: For production-grade pipelines with thousands of tickers and multiple indicators, QuantWave maintains a **2x to 5x lower memory footprint** on realistic string-heavy workloads.

## Speed & Latency

<!-- bench:throughput:start -->
### Rust streaming throughput (measured)

- **CPU**: arm
- **RAM**: 24.0 GB
- **OS**: Darwin 25.5.0
- **Python**: 3.12.8 (harness host)
- **Rows**: 100,000

| Indicator | Mode | Time (ms) |
|-----------|------|-----------|
| SMA (20) | streaming | 0.50 |
| RSI (14) | streaming | 0.00 |
| SuperTrend (10,3) | streaming | 0.73 |

<!-- bench:throughput:end -->

Per-tick latency benchmarks will publish when a dedicated micro-bench runs (not batch ms relabeled as ns). See also [`benchmark_results.md`](https://github.com/lavs9/quantwave/blob/main/benchmark_results.md) for preliminary SMA comparisons.

---

*Last Updated: {{ git_revision_date_localized }}*