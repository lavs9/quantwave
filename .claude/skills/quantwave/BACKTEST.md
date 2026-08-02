# The `.bt` Backtest Contract

Read [PITFALLS.md](PITFALLS.md) §1, §6 and §10 before trusting any output from this API.

## Input requirements

| Requirement | Detail |
|---|---|
| Frame shape | Long format, one row per (timestamp, symbol) |
| Sort order | Single-symbol: by `timestamp`. Multi-symbol: `["timestamp", "symbol"]` — that order |
| `signal` column | Numeric position signal: `-1` short, `0` flat, `1` long |
| `timestamp_col` | Monotonic bar index **or** datetime |
| `close_col` | Mark-to-market and fill price |
| Warmup | Sliced off before the call (`drop_nulls()` will not do it — warmup is NaN) |
| `high_col` / `low_col` | Required when `touched_exit=True` |
| `size_multiplier_col` | Must be `Float64` — `Int64` is rejected |

## Choosing an entry point

```python
lf.bt.backtest_with_report(...)   # recommended: metrics + trades + equity + tear sheet
lf.bt.backtest(...)               # raw result
lf.bt.backtest_metrics(...)       # metrics only
lf.bt.portfolio_backtest(...)     # shared-capital, multi-symbol
lf.bt.order_backtest(orders, ...) # explicit orders: market/limit/stop/stop-limit/bracket
lf.bt.sweep(...)                  # parameter sweep
lf.bt.walk_forward_optimize(...)  # WFO (grid + Bayesian TPE)
lf.bt.monte_carlo(...)            # resampled equity paths
```

## Defaults you should almost always override

```python
report = df.lazy().bt.backtest_with_report(
    signal="signal",
    execution_delay="next_bar",       # default "same_bar" fills on the signal bar's close
    size_multiplier_col="size_mult",  # default is 1 unit, not 1 unit of capital
    commission_bps=5.0,               # default 5.0 — set to your venue's real cost
    slippage_bps=2.0,                 # default 2.0
    initial_cash=100_000.0,
)
```

`stop_loss_pct`, `take_profit_pct`, `trailing_stop_pct` are **fractions** (`0.02` = 2%).
Set `touched_exit=True` (plus `high_col`/`low_col`) for intrabar OHLC stop detection;
without it, stops are evaluated on closes only and your drawdowns are understated.

## Output contract

`.metrics()` returns exactly **10 keys** — a stable, test-enforced contract that will
never gain or lose keys:

`total_return`, `cagr`, `sharpe_ratio`, `sortino_ratio`, `max_drawdown_pct`, `win_rate`,
`profit_factor`, `num_trades`, `avg_trade_pnl`, `final_equity`

All return-like values are **fractions**. `max_drawdown_pct` is a **positive** fraction.
`profit_factor` and `sortino_ratio` can be `inf`.

Additive, opt-in surfaces (they do not change `.metrics()`):

- `.extended_metrics()` — the 10 above plus `calmar_ratio`, `var_95`, `cvar_95`, `benchmark`
- `.metrics_with_benchmark(benchmark_returns)` — populates `benchmark` with `alpha`,
  `beta`, `cumulative_return`, `benchmark_cumulative_return`, `excess_cumulative_return`.
  Returns `benchmark=None` if fewer than 2 aligned observations or the benchmark has
  ~zero variance.
- `.stats()` — `initial_cash`, `final_equity`, `net_pnl`, `num_trades`, `total_return`
- `.save_html(path)` / `.to_html()` — self-contained tear sheet

Metrics objects are typed (`PerformanceMetrics`, `BacktestStats`) with dict-style access
retained for compatibility: `report.metrics()["sharpe_ratio"]` works.

### DataFrame schemas

`report.trades` — `trade_id` (u32), `side` (i8: `1` long / `-1` short), `entry_ts` /
`exit_ts` (i64 **epoch seconds**, `exit_ts` null if still open at series end),
`entry_price` / `exit_price` (raw signal price), `entry_fill_price` / `exit_fill_price`
(after slippage), `quantity` (f64), `pnl_net` (f64, after commission and slippage).

`report.equity_curve` — `ts` (i64 epoch seconds), `equity`, `cash`, `position` (signed
units), `close`.

## `order_backtest` position model

Flat-or-single-position, **no pyramiding**:

- an order filling while flat opens a position
- an opposite-side fill closes it and records a trade
- a same-side fill while already in a position is **silently ignored**
- any open position is force-flattened at the final bar's close

Orders frame columns: `bar_index` (0-based row index into `df`), `side` (`"buy"`/`"sell"`),
`type` (`"market"`/`"limit"`/`"stop"`/`"stop_limit"`), `qty`, `price` (limit leg),
`trigger` (stop level). Optional `take_profit` + `stop_loss` attach an OCO bracket — set
**both or neither**; one leg alone raises. Same-bar double-touch resolves
stop-before-target (pessimistic).

Returns `(trades_df, equity_df)` — a tuple, not a `BacktestReport`.

## Risk overlays and rebalancing

`risk_model=` accepts `vol_target`, `inverse_vol`, `position_limit`, `pre_trade`.
Overlays size a position **at entry only** — there is no intra-trade resizing.

```python
risk_model={"vol_target": {"target_annual_vol": 0.15, "lookback": 20},
            "position_limit": {"max_abs_exposure": 50.0}}
```

`portfolio_backtest(rebalance_policy=...)` takes exactly one top-level key: `calendar`,
`drift`, `signal`, or `turnover`. Default `None` rebalances every bar. Stop-loss,
take-profit and trailing-stop exits are **always** evaluated regardless of the policy.
