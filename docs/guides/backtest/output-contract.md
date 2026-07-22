# Output Contract

!!! tip "Short answer"
    QuantWave backtest metrics are typed as `PerformanceMetrics` and `BacktestStats` objects (with dict-like access for backward compatibility).
    - All return-like values are **fractions**, not percents (e.g. `0.05` = 5%).
    - `max_drawdown_pct` is a **positive fraction** (e.g. `0.10` = 10% decline).
    - DataFrames have stable schemas: `entry_ts` and `exit_ts` are epoch seconds; sides are `1` (long) and `-1` (short).

This document outlines the strict schema and semantic contract for the backtest engine outputs in QuantWave 0.6.0+.

## Typed Metric Outputs

When you call `.metrics()` on a `BacktestReport` (or dictionary result), it returns a `PerformanceMetrics` object.

| Key | Definition | Units / Sign |
|-----|------------|--------------|
| `total_return` | final/initial − 1 | fraction |
| `cagr` | annualized return | fraction |
| `sharpe_ratio` | annualized, risk-free = 0 | ratio |
| `sortino_ratio` | annualized downside | ratio |
| `max_drawdown_pct` | peak-to-trough decline | **positive fraction** (0.10 = 10% drawdown) |
| `win_rate` | winners / closed trades | fraction (0–1) |
| `profit_factor` | gross profit / gross loss | ratio (inf if no losses) |
| `num_trades` | closed trade count | count |
| `avg_trade_pnl` | mean net PnL per trade | currency |
| `final_equity` | ending portfolio value | currency |

*Note: For backward compatibility, `metrics()["sharpe_ratio"]` will continue to work exactly like dictionary access.*

*This 10-key set is a **stable contract** enforced by tests — `.metrics()` will never gain or lose keys. New/extra analytics (below) live on separate, opt-in methods.*

## Extended Metrics & Benchmark-Relative Analytics (additive)

`BacktestReport` / `BacktestResult` also expose an **additive, opt-in** surface that does not change `.metrics()`:

- `.extended_metrics()` — a dict with all 10 keys above plus:

    | Key | Definition | Units / Sign |
    |-----|------------|--------------|
    | `calmar_ratio` | `cagr / max_drawdown_pct` | ratio (`inf` if no drawdown and positive CAGR) |
    | `var_95` | Historical 95% Value-at-Risk on per-bar returns | **positive fraction** (loss magnitude) |
    | `cvar_95` | Historical 95% Conditional VaR (Expected Shortfall) | **positive fraction** (loss magnitude) |
    | `benchmark` | `None`, unless benchmark-relative analytics were attached | dict or `None` |

- `.metrics_with_benchmark(benchmark_returns)` — same as `.extended_metrics()`, but computes benchmark-relative analytics against a supplied per-bar benchmark return series (aligned by index). The `benchmark` key is populated with:

    | Key | Definition | Units / Sign |
    |-----|------------|--------------|
    | `alpha` | Annualized (×252) alpha: `mean(r_s) - beta * mean(r_b)` | fraction |
    | `beta` | `Cov(r_s, r_b) / Var(r_b)` | ratio |
    | `cumulative_return` | Strategy cumulative return over the aligned window | fraction |
    | `benchmark_cumulative_return` | Benchmark cumulative return over the aligned window | fraction |
    | `excess_cumulative_return` | `cumulative_return - benchmark_cumulative_return` | fraction |

`benchmark` is `None` when no benchmark series is supplied, the aligned window has fewer than 2 observations, or the benchmark series has ~zero variance (beta undefined).

## Summary Statistics

Calling `.stats()` returns a `BacktestStats` object which contains a stable subset of summary data:

| Key | Definition |
|-----|------------|
| `initial_cash` | The starting capital |
| `final_equity` | The final portfolio value |
| `net_pnl` | Total net profit (after commissions) |
| `num_trades` | Number of closed trades |
| `total_return` | Overall return as a fraction |

*(Optional keys like `num_symbols` and `portfolio_mode` may appear for multi-asset tests)*

## DataFrame Schemas

### Trades (`.trades`)

The trades blotter is returned as a Polars DataFrame with the following guaranteed columns:

- `trade_id` (u32): Unique identifier for the trade
- `side` (i8): `1` for long, `-1` for short
- `entry_ts` (i64): Unix epoch **seconds** of entry
- `entry_price` (f64): Raw signal price at entry
- `entry_fill_price` (f64): Adjusted price at entry (after slippage)
- `exit_ts` (i64, nullable): Unix epoch **seconds** of exit. Null if the position is still open at the end of the series.
- `exit_price` (f64, nullable): Raw signal price at exit
- `exit_fill_price` (f64, nullable): Adjusted price at exit (after slippage)
- `quantity` (f64): Number of units traded
- `pnl_net` (f64): Net profit after commissions and slippage

### Equity Curve (`.equity_curve`)

The equity curve is returned as a Polars DataFrame:

- `ts` (i64): Unix epoch **seconds**
- `equity` (f64): Total value (cash + position value)
- `cash` (f64): Available cash
- `position` (f64): Signed units held
- `close` (f64): Underlying asset price

## BacktestConfig Conventions

When configuring a backtest via `BacktestConfig`:

- `stop_loss_pct` / `take_profit_pct` / `trailing_stop_pct`: Are always **fractions** (0.05 = 5%).
- Default position size is **1 unit** unless `size_multiplier_col` is set.
- `execution_delay`:
    - `"same_bar"`: Fills trade on the signal bar's close.
    - `"next_bar"`: Fills trade on the next bar's close.
