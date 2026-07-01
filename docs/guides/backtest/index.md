# Backtest Engine

QuantWave ships a **Polars-native, clean-room backtest engine** (`quantwave-backtest`) with Python `.bt` namespace ergonomics. It is vectorbt-*inspired* research UX built on QuantWave's unique **batch ↔ streaming parity** — not a fork of vectorbt or polars-backtest.

## Why this engine

| Capability | What you get |
|------------|--------------|
| **Parity moat** | Same signal logic → identical trades/equity in batch (Polars LazyFrame) and streaming (`Next<T>`) |
| **Research depth** | Sweeps, walk-forward optimization, Monte Carlo, cross-sectional panels |
| **Production realism** | Commission, slippage, T+1 execution, stops, shorts, sizing filters |
| **Portfolio mode** | Shared-capital multi-symbol books via `.bt.portfolio_backtest()` |
| **Reporting** | `PerformanceMetrics` dict + standalone HTML tear sheets |

The Rust core owns simulation math; Python exposes ergonomic Polars namespaces. Every shipped feature has nextest or pytest proof — see the [Capability Matrix](capability_matrix.md).

## The parity moat

> **One mathematical truth:** Polars plugins and the live streaming engine bind to the same Rust traits.

| Artifact | Location |
|----------|----------|
| Batch ↔ streaming integration tests | `quantwave-backtest` — `test_batch_vs_streaming_parity_*` |
| Guide | [Batch & Streaming](../../examples/batch-streaming.md) |
| ML → backtest E2E | [ML Features → Backtest](../../examples/notebooks/ml_feature_backtest_parity.md) |

## `.bt` API surface

All methods are on `df.lazy().bt` after `import quantwave` (registers the namespace).

| Method | Purpose |
|--------|---------|
| `backtest()` | Trades + equity DataFrames |
| `backtest_with_report()` | Above + `BacktestReport` with `.metrics()` |
| `backtest_metrics()` | Metrics only — fast path, no trade/equity materialization |
| `sweep()` | Grid over a pre-built signal column |
| `sweep_callback()` | Rebuild signals per parameter via `build_fn` |
| `walk_forward()` | Rolling out-of-sample folds |
| `walk_forward_optimize()` | In-fold param sweep, locked OOS evaluation |
| `cross_sectional_backtest()` | Universe rank long/short (`transform=` optional) |
| `portfolio_backtest()` | **Shared-capital** multi-symbol portfolio simulation |

### Portfolio backtest (shared capital)

When multiple symbols trade from **one cash pool**, use `portfolio_backtest()` instead of independent per-symbol runs:

```python
report = (
    df.lazy()
    .bt.portfolio_backtest(
        signal="signal",
        symbol_col="symbol",
        portfolio_mode="shared_capital",
        portfolio_allocator="equal_weight",
        initial_cash=100_000.0,
    )
)
```

See [Portfolio Shared Capital](../../examples/notebooks/portfolio_shared_capital_backtest.md) for a full walkthrough.

## Quickstart path

1. **[Quickstart](quickstart.md)** — copy-paste script, first trades in 5 minutes
2. **[Capability Matrix](capability_matrix.md)** — feature inventory with proof links
3. **[Tear Sheets](tear_sheets.md)** — HTML reports from `BacktestReport`

## Notebooks

Runnable marimo notebooks (also linked in the site nav):

| Notebook | Focus |
|----------|-------|
| [Backtest Showcase](../../examples/notebooks/backtest_showcase.md) | Full `.bt` tour — sweeps, WFO, Monte Carlo |
| [Portfolio Shared Capital](../../examples/notebooks/portfolio_shared_capital_backtest.md) | Pooled-book multi-symbol simulation |
| [Strategy Backtest](../../examples/notebooks/strategy_backtest.md) | SuperTrend → signal → backtest E2E |
| [PA Flag Breakout](../../examples/notebooks/pa_flag_breakout_strategy.md) | Price-action metadata into trades |
| [Backtest Benchmarks](../../examples/notebooks/backtest_benchmark.md) | Criterion vs naive Python loop |
| [ML Features → Backtest](../../examples/notebooks/ml_feature_backtest_parity.md) | Feature pipeline parity proof |

## Verification

```bash
cargo nextest run -p quantwave-backtest
pytest quantwave-python/tests/test_backtest.py quantwave-python/tests/test_portfolio_backtest.py -q
```