# Backtest Engine Benchmarks

Criterion benchmarks comparing **quantwave-backtest** against a **naive row-loop** baseline (the pattern most Polars/Python notebooks use before a dedicated engine).

Aligned with `planning/BACKTEST_ENGINE_RESEARCH.md` §9 (cr6v.13). Updated **2026-06-19**.

## Run locally

```bash
cargo bench -p quantwave-backtest --bench backtest_vs_naive
```

HTML reports are written under `target/criterion/`.

For a quick smoke (fewer samples):

```bash
cargo bench -p quantwave-backtest --bench backtest_vs_naive -- --sample-size 10
```

## Cases

| Group | Shape | Engines |
|-------|-------|---------|
| `single_symbol_flip` | 10K / 100K / 1M rows | `quantwave_backtest`, `quantwave_metrics_only`, `naive_row_loop` |
| `multi_symbol_long` | 100 symbols × 5K bars (500K rows) | `quantwave_backtest`, `quantwave_metrics_only`, `naive_row_loop_per_symbol` |

### Baselines

- **quantwave_backtest** — `BacktestEngine::run` on a long-format Polars `LazyFrame` (zero commission/slippage). Produces a full **trade blotter**, **equity curve DataFrame**, and **summary stats** every iteration.
- **quantwave_metrics_only** — `BacktestEngine::run_metrics_only` (cr6v-v2.5): same simulation path, skips materializing trades/equity DataFrames. Use for param sweeps / WFO grids where only metrics matter.
- **naive_row_loop** — Rust row iterator over collected `close` + `signal` columns; same-bar long-only flip semantics, no costs, returns **final cash only** (no trade records). Serves as a native-code lower bound; real Python/Polars `.iter_rows()` loops are slower due to interpreter overhead.

Synthetic data uses a fixed RNG seed (`0xC26E0013`) and alternating exposure blocks so every run produces trades.

## Interpreting results

Criterion reports **time per iteration** and **throughput (elements/s)**. Compare groups at the same row count.

**Important:** quantwave-backtest is not optimized to beat a bare `for` loop that returns one `f64`. The engine pays for rich Polars outputs (trades, equity, multi-symbol portfolio rows, metadata-ready paths). Use these benches to track regressions and to quantify that overhead — not as a claim that the sim core is faster than a minimal loop.

On current main, `quantwave_metrics_only` is **parity-fast with full report** on single-symbol paths (simulation dominates); the win is **API ergonomics** for sweeps (no large DataFrame allocations) rather than a large constant-factor speedup at 10K–1M rows.

### Results (Apple Silicon, `cargo bench -- --sample-size 10`, 2026-06-19)

| Case | Rows | quantwave_backtest | quantwave_metrics_only | naive row-loop | full/metrics ratio |
|------|------|--------------------|-------------------------|----------------|---------------------|
| single_symbol_flip | 10,000 | 230.6 µs | 228.1 µs | 46.6 µs | 1.01× |
| single_symbol_flip | 100,000 | 2.19 ms | 2.32 ms | 460 µs | 0.94× |
| single_symbol_flip | 1,000,000 | 24.1 ms | 26.1 ms | 4.62 ms | 0.92× |
| multi_symbol_long | 500,000 | 857 ms | 841 ms | 22.5 ms | 1.02× |

Re-run on your machine and replace the table when publishing.

## Source

- Benchmark harness: [`quantwave-backtest/benches/backtest_vs_naive.rs`](https://github.com/lavs9/quantwave/blob/main/quantwave-backtest/benches/backtest_vs_naive.rs)

## Related

- [Backtest Showcase](backtest_showcase.md) — full `.bt` API tour
- [Strategy Backtesting](strategy_backtest.md) — notebook using `.bt.backtest()`
- [Capability Matrix](../../guides/backtest/capability_matrix.md) — shipped feature checklist
- [Project benchmarks page](../../benchmarks.md) — indicator throughput (separate from backtest sim)