# Backtest Engine Benchmarks

Criterion benchmarks comparing **quantwave-backtest** against a **naive row-loop** baseline (the pattern most Polars/Python notebooks use before a dedicated engine).

Aligned with `planning/BACKTEST_ENGINE_RESEARCH.md` §9 (cr6v.13).

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
| `single_symbol_flip` | 10K / 100K / 1M rows | `quantwave_backtest` vs `naive_row_loop` |
| `multi_symbol_long` | 100 symbols × 5K bars (500K rows) | per-symbol naive loop vs engine grouping |

### Baselines

- **quantwave_backtest** — `BacktestEngine::run` on a long-format Polars `LazyFrame` (zero commission/slippage). Produces a full **trade blotter**, **equity curve DataFrame**, and **summary stats** every iteration.
- **naive_row_loop** — Rust row iterator over collected `close` + `signal` columns; same-bar long-only flip semantics, no costs, returns **final cash only** (no trade records). Serves as a native-code lower bound; real Python/Polars `.iter_rows()` loops are slower due to interpreter overhead.

Synthetic data uses a fixed RNG seed (`0xC26E0013`) and alternating exposure blocks so every run produces trades.

## Interpreting results

Criterion reports **time per iteration** and **throughput (elements/s)**. Compare groups at the same row count.

**Important:** quantwave-backtest is not optimized to beat a bare `for` loop that returns one `f64`. The engine pays for rich Polars outputs (trades, equity, multi-symbol portfolio rows, metadata-ready paths). Use these benches to track regressions and to quantify that overhead — not as a claim that the sim core is faster than a minimal loop.

### Sample (Apple Silicon, `cargo bench -- --sample-size 10`)

| Case | Rows | quantwave | naive row-loop | Notes |
|------|------|-----------|----------------|-------|
| single_symbol_flip | 10,000 | ~234 µs | ~46 µs | Engine ~5× slower (rich output) |
| single_symbol_flip | 100,000 | ~2.16 ms | ~457 µs | Same ratio band |
| single_symbol_flip | 1,000,000 | ~23.4 ms | ~4.57 ms | ~5× overhead at scale |
| multi_symbol_long | 500,000 | ~811 ms | ~22.5 ms | 100 symbols; portfolio + per-symbol equity |

Re-run on your machine and replace the table above when publishing.

## Source

- Benchmark harness: [`quantwave-backtest/benches/backtest_vs_naive.rs`](https://github.com/lavs9/quantwave/blob/main/quantwave-backtest/benches/backtest_vs_naive.rs)

## Related

- [Strategy Backtesting](strategy_backtest.md) — notebook using `.bt.backtest()`
- [Project benchmarks page](../../benchmarks.md) — indicator throughput (separate from backtest sim)