# QuantWave benchmark harness

Reproducible performance measurements for docs and CI. Results land in `results/latest.json`; `docs/benchmarks.md` is rendered from that file only.

## Layout

- `data.py` — deterministic synthetic OHLCV generator (fixed seed).
- `harness.py` — orchestrates Rust criterion benches and Python comparisons.
- `results/latest.json` — machine-readable output (hardware, versions, timings).

## Running locally

Dependencies (`harness.py` checks these at startup and exits with an install
hint if any is missing):

```bash
pip install polars pandas pyarrow numpy psutil
```

`pyarrow` is not optional — the memory benchmark compares against a genuine
pandas frame via polars `.to_pandas()`, which delegates to pyarrow.

```bash
# Full suite (1M rows) + render docs
python benchmarks/harness.py
python scripts/render_benchmarks.py

# Dev smoke (100k rows)
python benchmarks/harness.py --quick
python scripts/render_benchmarks.py

# Criterion HTML reports (100k rows per case)
cargo bench -p quantwave-core --bench indicator_throughput
```

## CI

- **PR / sanity:** `scripts/check_benchmark_claims.py` — no orphan perf numbers in docs.
- **Nightly:** `.github/workflows/benchmarks-nightly.yml` — full harness, render, commit JSON + `docs/benchmarks.md`.
- **Criterion smoke:** same workflow runs `cargo bench` with reduced sample size.