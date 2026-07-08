# QuantWave benchmark harness

Reproducible performance measurements for docs and CI. Results land in `results/latest.json`; `docs/benchmarks.md` is rendered from that file only.

## Layout

- `data.py` — deterministic synthetic OHLCV generator (fixed seed).
- `harness.py` — orchestrates Rust criterion benches and Python comparisons.
- `results/latest.json` — machine-readable output (hardware, versions, timings).

## Running locally

```bash
python benchmarks/harness.py --quick   # smoke / dev (subset)
python benchmarks/harness.py           # full 1M-row suite
```

## CI

- `scripts/check_benchmark_claims.py` fails if docs contain unmeasured performance numbers.
- Nightly / manual workflow will run the harness and commit updated JSON (Phase 3).