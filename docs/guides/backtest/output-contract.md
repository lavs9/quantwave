# Output Contract

!!! tip "Short answer"
    QuantWave backtest metrics are typed as `PerformanceMetrics` and `BacktestStats` objects (with dict-like access for backward compatibility).
    - All return-like values are **fractions**, not percents (e.g. `0.05` = 5%).
    - `max_drawdown_pct` is a **positive fraction** (e.g. `0.10` = 10% decline).
    - Ratios are **`NaN` when undefined** — `sortino_ratio` (no downside), `profit_factor` (no losing trades), `calmar_ratio` (no drawdown), `sharpe_ratio` (zero dispersion with a non-zero mean). Never `inf`. Test with `math.isnan()`, not `==`.
    - Ratio metrics mean nothing below **30 trades**. Check `.diagnostics()`.
    - DataFrames have stable schemas: `entry_ts` and `exit_ts` are epoch seconds; sides are `1` (long) and `-1` (short).

This document outlines the strict schema and semantic contract for the backtest engine outputs in QuantWave 0.6.0+.

## Typed Metric Outputs

When you call `.metrics()` on a `BacktestReport` (or dictionary result), it returns a `PerformanceMetrics` object.

| Key | Definition | Units / Sign |
|-----|------------|--------------|
| `total_return` | final/initial − 1 | fraction |
| `cagr` | annualized return | fraction |
| `sharpe_ratio` | annualized, risk-free = 0 | ratio (**`NaN` if the return series has zero dispersion but a non-zero mean** — undefined) |
| `sortino_ratio` | annualized downside | ratio (**`NaN` if no negative returns** — undefined) |
| `max_drawdown_pct` | peak-to-trough decline | **positive fraction** (0.10 = 10% drawdown) |
| `win_rate` | winners / closed trades | fraction (0–1) |
| `profit_factor` | gross profit / gross loss | ratio (**`NaN` if no losing trades** — undefined) |
| `num_trades` | closed trade count | count |
| `avg_trade_pnl` | mean net PnL per trade | currency |
| `final_equity` | ending portfolio value | currency |

*Note: For backward compatibility, `metrics()["sharpe_ratio"]` will continue to work exactly like dictionary access.*

*This 10-key set is a **stable contract** enforced by tests — `.metrics()` will never gain or lose keys. New/extra analytics (below) live on separate, opt-in methods.*

### Undefined ratios are `NaN`, not `inf`

Every ratio QuantWave reports divides by a risk-like quantity. When that
denominator is **empty** — no negative bar returns, no losing trades, no
drawdown — the ratio is not "infinitely good", it is **undefined**. QuantWave
returns `NaN` for these cases rather than `inf`, because `inf` reads like a
measurement and `NaN` reads like the absence of one.

**The convention is uniform across the whole bundle** — one condition, one
answer. A single run never mixes `NaN` and `inf` for the same situation:

| Metric | Undefined when | Value |
|---|---|---|
| `profit_factor` | no losing trades | `NaN` |
| `sortino_ratio` | no negative returns, or zero downside deviation | `NaN` |
| `sharpe_ratio` | zero return dispersion with a non-zero mean | `NaN` |
| `calmar_ratio` (extended) | zero max drawdown with positive CAGR | `NaN` |

Consequences for your code:

- Test with `math.isnan(x)` / `x != x` (Python) or `.is_nan()` (Rust).
  **`NaN == NaN` is `False`** — an equality check will silently do the wrong thing.
- `NaN` propagates through arithmetic. If you rank or sort strategies on
  `profit_factor`, filter the undefined ones out first; otherwise comparisons
  against `NaN` are all `False` and the ordering is not what you expect.
- `0.0` still means "no activity at all" (no trades, all trades exactly flat, or
  a genuinely flat equity curve), which is distinct from "undefined".

!!! note "Why this matters for optimizers"
    Walk-forward and sweep selection pick the argmax of an objective metric with
    a `v > best_val` comparison. `inf > anything` is `True`, so a degenerate
    variant that simply never lost — one trade, no drawdown — would win the fold
    and be carried into the out-of-sample window. `NaN > anything` is `False`, so
    an undefined variant is skipped exactly like a null. If every candidate is
    undefined, the fold reports `-inf` as its `train_metric`, which is visible
    rather than silently plausible.

Two surfaces stay outside the convention, deliberately:

- `var_95` / `cvar_95` are quantiles, not ratios — no denominator to be empty.
  They are `0.0` on an empty return series.
- `benchmark` is `None` (not a `NaN`-filled dict) when alpha/beta are undefined —
  fewer than 2 aligned observations, or a zero-variance benchmark.

### Ratio metrics are unreliable below 30 trades

**A backtest with a handful of trades produces meaningless ratios.** A single
winning trade worth $2 yields `sharpe_ratio ≈ 7.98`, `win_rate = 1.0`, and
undefined Sortino / profit factor — a screenful of numbers that look like a
world-class strategy and are pure sampling noise.

QuantWave uses **30 closed trades** as the threshold below which
`sharpe_ratio`, `sortino_ratio`, `profit_factor` and `win_rate` should not be
read as evidence of edge. The constant is
`quantwave_backtest::MIN_TRADES_FOR_RELIABLE_RATIOS`.

The metrics are still computed — nothing is suppressed or nulled out, and
`.metrics()` is unchanged. Instead the warning is **additive**, on
`.diagnostics()`:

```python
report = df.lazy().bt.backtest_with_report(...)

report.metrics()          # unchanged: exactly the 10 keys above
diag = report.diagnostics()

if diag["low_sample_size"]:
    for w in diag["warnings"]:
        print("WARNING:", w)
```

`.diagnostics()` returns:

| Key | Definition |
|-----|------------|
| `low_sample_size` | `True` when `num_trades < min_trades_for_reliable_ratios` |
| `num_trades` | closed trade count the diagnostics were derived from |
| `min_trades_for_reliable_ratios` | the threshold (30) |
| `undefined_metrics` | list of the **10 contract** metric names that came back `NaN` (extended metrics like `calmar_ratio` are not scanned — read them directly) |
| `warnings` | human-readable strings; **empty list means nothing looked suspect** |

The same dict is also available as the `diagnostics` key of
`.extended_metrics()`, and in Rust as `PerformanceMetrics::diagnostics()`.

!!! warning "A clean `diagnostics()` is not a validation of your strategy"
    It only means the sample was not obviously too thin and no ratio was
    mathematically undefined. Look-ahead bias, overfitting, and unrealistic fills
    are not detected here.

## Extended Metrics & Benchmark-Relative Analytics (additive)

`BacktestReport` / `BacktestResult` also expose an **additive, opt-in** surface that does not change `.metrics()`:

- `.extended_metrics()` — a dict with all 10 keys above plus:

    | Key | Definition | Units / Sign |
    |-----|------------|--------------|
    | `calmar_ratio` | `cagr / max_drawdown_pct` | ratio (**`NaN` if no drawdown and positive CAGR** — undefined; `0.0` if no drawdown and non-positive CAGR) |
    | `var_95` | Historical 95% Value-at-Risk on per-bar returns | **positive fraction** (loss magnitude) |
    | `cvar_95` | Historical 95% Conditional VaR (Expected Shortfall) | **positive fraction** (loss magnitude) |
    | `diagnostics` | Thin-sample / undefined-metric warnings (see above) | dict |
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
    - `"next_bar"` (**default**): Fills the trade on the *next* bar's close. A
      signal observed on bar `t` fills at bar `t+1`'s close.
    - `"same_bar"`: Fills the trade on the signal bar's own close — bar `t`'s
      signal fills at bar `t`'s close.

!!! warning "`same_bar` is opt-in for a reason"

    Signals are almost always derived from the same bar's close (e.g.
    `(rsi < 30)` computed on bar `t`). Filling that signal at bar `t`'s close
    means executing on information that only exists at the instant the bar
    ends — a look-ahead the live strategy will not have. On a rising series
    this is measurably optimistic: the same signal frame entered at `100.5`
    under `same_bar` versus `101.0` under `next_bar`.

    Only reach for `"same_bar"` when it is genuinely true of your execution:

    - you really do trade the closing auction, or
    - your signal is built purely from data through bar `t-1`, so bar `t`'s
      close is not an input.

    In 0.7.0 and earlier the default was `"same_bar"`. See the
    [changelog](../../changelog.md) for the migration note.
