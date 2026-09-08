# Backtest Quickstart

Get from zero to a first backtest with trades and metrics in under 5 minutes.

---

## 1. Install

```bash
pip install "quantwave[polars]"
```

From source (contributors):

```bash
maturin develop -p quantwave-python --release
pip install polars
```

---

## 2. Minimal script (copy-paste)

```python
import polars as pl
from quantwave.backtest import BacktestEngine, BacktestConfig

# Synthetic OHLCV + signal (long when close rises)
df = pl.DataFrame({
    "timestamp": list(range(20)),
    "close": [100.0 + i * 0.5 for i in range(20)],
    "signal": [0.0, 1.0, 1.0, 1.0, 1.0, 0.0] + [0.0] * 14,
})

config = BacktestConfig(commission_bps=0.0, slippage_bps=0.0)
report = BacktestEngine(config).backtest_with_report(df)

print("Trades:", report.result.trades.height)
print("Sharpe:", report.metrics()["sharpe_ratio"])
print(report.result.trades.head())
```

Expected: `num_trades >= 1`, finite Sharpe, one row in trades DataFrame.

---

## 3. Polars `.bt` namespace (preferred DX)

```python
import polars as pl

df = pl.DataFrame({...})  # same as above
report = (
    df.lazy()
    .bt.backtest_with_report(
        signal="signal",
        commission_bps=0.0,
        slippage_bps=0.0,
    )
)
metrics = report.metrics()
```

---

## 4. Signal conventions: get `signal_type` right

!!! danger "This is the single most important thing to get right in a portfolio backtest"

    A signal column's magnitude means something different depending on
    `signal_type`. Pick the wrong one and the backtest still runs, still
    produces trades, and still prints a plausible-looking (but silently
    wrong) report.

`signal_type` is a `.bt.portfolio_backtest()` parameter — it only applies to
shared-capital, multi-symbol runs (see "`shared_capital` vs
`independent_books`" below). It controls how a signal's *magnitude* becomes
a position size:

| `signal_type` | Magnitude means | Example: `signal = 0.25` |
|----------------|------------------|---------------------------|
| `"weight"` (**default**) | Fraction of **total equity**, independent per symbol — not normalized against other active symbols. Caller is responsible for keeping the sum of active weights sane. Matches zipline `order_target_percent` / backtrader `PercentSizer` / QuantConnect `SetHoldings` / vectorbt `targetpercent`. | Deploy 25% of equity into this symbol. |
| `"target_pct"` | A weight normalized across all symbols with a non-zero signal **this bar**: `\|signal_i\| / Σ\|signal\|`. | Deploy `25% / (sum of all active \|signal\| this bar)` of equity. |
| `"shares"` (opt-in, pre-9wji.1 behavior) | A **literal share count**. | Buy 0.25 shares — rounds to 0. |

`"shares"` is the footgun this section exists to warn you off of. A boolean
`0/1` entry signal — the most natural way to write "I'm in this name" or
"I'm out" — is a **share count** under `"shares"`, not a position size. On a
$1,000,000 book, a `1.0` signal buys **one share**, not "all-in." The
backtest doesn't error; it just quietly returns close to 0% because almost
none of the book's capital was ever deployed, and that number never
compounds with equity as the book grows.

```python
import polars as pl
import quantwave  # registers .bt

df = pl.DataFrame({
    "timestamp": [0, 0, 0, 1, 1, 1, 2, 2, 2],
    "symbol":    ["AAA", "BBB", "CCC"] * 3,
    "close":     [100.0, 50.0, 20.0, 101.0, 50.5, 20.2, 102.0, 51.0, 20.4],
    # Boolean "am I in this name" signal, one column, three symbols.
    "signal":    [0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
})

# BEFORE (footgun): signal_type="shares" — buys ~1 share per name on a
# $1,000,000 book, deploys almost none of the capital, ~0% return.
report_shares = (
    df.lazy()
    .bt.portfolio_backtest(
        signal="signal", symbol_col="symbol",
        initial_cash=1_000_000.0, signal_type="shares",
    )
)

# AFTER (default): same boolean shape, but signal values *are* the target
# weight. [0, 0.25, 0.25] deploys 25% of equity into each active name —
# 75% total, 25% held back in cash.
df_weighted = df.with_columns(
    (pl.col("signal") * 0.25).alias("signal")
)
report_weight = (
    df_weighted.lazy()
    .bt.portfolio_backtest(
        signal="signal", symbol_col="symbol",
        initial_cash=1_000_000.0,  # signal_type="weight" is the default
    )
)
```

The takeaway: decide up front whether your signal column is meant to carry
*share counts* or *position weights*, and set `signal_type` to match. When
in doubt, `"weight"` is almost always what you want for equity-fraction
sizing, and it's the default as of quantwave-9wji.1 (2026-09-08) — the
default used to be `"shares"`, so a backtest re-run after upgrading without
passing `signal_type="shares"` explicitly will produce very different
(usually much bigger, and much more correct) numbers.

### Position sizing on top of `signal_type`: `size_multiplier_col`

`size_multiplier_col` (available on `backtest()`, `backtest_with_report()`,
and `portfolio_backtest()`) names an optional `f64` column that multiplies
the raw signal value *before* `signal_type` is applied — e.g. a
normalized "conviction" score, a regime probability, or an ATR-derived pole
height. A `size_multiplier` of `0.5` on a `signal_type="weight"` row of
`0.25` deploys `0.125` (12.5%) of equity instead of `0.25` (25%); on a
`signal_type="shares"` row of `10` it buys 5 shares instead of 10. It has
no interaction with `entry_filter_col` other than ordering: a `False` entry
filter forces exposure to `0.0` regardless of what the multiplier says.

---

## 5. `shared_capital` vs `independent_books`: one cash pool or one per symbol

`portfolio_mode` (a `.bt.portfolio_backtest()` parameter) decides how
capital is shared across symbols in a multi-symbol run:

- `"shared_capital"` (`portfolio_backtest()`'s default) — **one cash pool**
  for the whole run. Opening a position in `AAA` competes for the same
  capital as opening a position in `BBB`; `signal_type` and
  `portfolio_allocator` govern how that shared pool is split. This is what
  you want for "one book, many names" — the realistic shape of most
  portfolio strategies.
- `"independent_books"` (the historical default, still the default for the
  lower-level `.bt.backtest()` family) — each symbol gets its **own**
  `initial_cash`, run as if it were a separate backtest. There's no
  competition for capital between symbols; a large position in `AAA`
  doesn't reduce what `BBB` can buy. Useful when you genuinely want
  per-symbol P&L in isolation rather than a single blended equity curve.

```python
df.lazy().bt.portfolio_backtest(
    signal="signal", symbol_col="symbol",
    portfolio_mode="shared_capital",     # one pool (default for portfolio_backtest)
    portfolio_allocator="equal_weight",  # equity / N active symbols
)
```

---

## 6. Sort your input before you backtest

Every `.bt` method that touches a `symbol_col` validates that rows are
sorted ascending by `(timestamp, symbol)` — timestamp first, then symbol
within a tied timestamp — and raises if they aren't:

```
Data must be sorted by timestamp (and symbol for multi-symbol runs)
```

Build multi-symbol frames with `df.sort(["timestamp", "symbol"])` (or
however your data pipeline already guarantees that order) before calling
`.bt.portfolio_backtest()` or any other `.bt` method with `symbol_col` set.
A single-symbol frame only needs to be sorted by `timestamp_col`.

---

## 7. Trim indicator warmup before you backtest

!!! danger "Warmup is `NaN`, not `null` — `drop_nulls()` will not remove it"

    If your signal comes from an indicator (it usually does), the first
    `warmup_bars` rows are `NaN`. QuantWave emits warmup as **`NaN`, never
    `null`**, which breaks the reflex everyone brings from pandas/Polars:

    ```python
    df = df.with_columns(pl.col("close").ta.rsi(14).alias("rsi"))
    df["rsi"].null_count()   # 0  -> drop_nulls() / dropna() is a SILENT NO-OP
    df["rsi"].is_nan().sum() # 14
    ```

    And because `NaN < 30` evaluates to `False`, a comparison-derived signal is
    `0.0` for the entire warmup — the backtest cannot tell that apart from a real
    "stay flat" decision. The result is a plausible-looking but wrong report.

Trim first, with `qw.trim_warmup()`. It drops the **maximum** warmup across every
indicator you name, so multi-indicator frames stay row-aligned:

```python
import polars as pl
import quantwave as qw

df = df.with_columns(
    pl.col("close").ta.rsi(14).alias("rsi"),
    pl.col("close").ta.ema(50).alias("ema"),
)
df = df.with_columns(
    pl.when(pl.col("close") > pl.col("ema")).then(1.0).otherwise(0.0).alias("signal")
)

report = (
    df.pipe(qw.trim_warmup, "rsi", ("ema", {"period": 50}))   # drops 50 leading rows
    .lazy()
    .bt.backtest_with_report(signal="signal")
)
```

The `.bt` methods also check for you: if the `signal` or `close` column handed to
a backtest starts with `NaN`/`null` rows, QuantWave emits a `quantwave.WarmupWarning`
naming the column and the row count. It is a **warning, not an error** — the
backtest still runs. Silence it once you have deliberately decided the leading
rows are fine:

```python
import warnings
warnings.filterwarnings("ignore", category=qw.WarmupWarning)
```

See [Warmup and NaN Semantics](../../getting-started/python.md#warmup-and-nan-semantics)
for the full convention and the accepted `trim_warmup` spec forms.

---

## 8. When your trades actually fill

By default QuantWave fills a signal observed on bar `t` at bar **`t+1`**'s close
(`execution_delay="next_bar"`). This is deliberate. Your signal is almost
certainly computed from bar `t`'s close — `(rsi < 30)`, a moving-average cross,
a breakout above bar `t`'s high — so filling *at* bar `t`'s close would execute
on information that only exists once the bar has ended. Live, you cannot do
that; the bar has to close before you can know the signal fired and send the
order.

```python
# Default — honest. Signal on bar t, fill at bar t+1's close.
lf.bt.backtest(signal="signal")

# Opt in to same-bar fills, only if it's true of your execution.
lf.bt.backtest(signal="signal", execution_delay="same_bar")
```

`"same_bar"` is the right call in exactly two situations:

- you genuinely execute in the **closing auction** of bar `t`, or
- your signal is built purely from data through **bar `t-1`**, so bar `t`'s
  close is not an input to it.

Otherwise `"same_bar"` will inflate your results — on a rising series, the same
signal frame enters at `100.5` under `same_bar` and `101.0` under `next_bar`,
and that gap is pure look-ahead.

!!! warning "Changed in the upcoming release"

    The default was previously `"same_bar"`. Backtests re-run after upgrading
    will report different, usually worse, numbers — that is the look-ahead
    being removed. See the [changelog](../../changelog.md).

---

## 9. What you get back

| Output | Contents |
|--------|----------|
| `report.result.trades` | trade_id, entry/exit ts, prices, pnl_net, … |
| `report.result.equity_curve` | ts, equity, cash, position |
| `report.metrics()` | sharpe_ratio, max_drawdown_pct, win_rate, … |

Full key list: see [Capability Matrix](capability_matrix.md#python-bt-api-surface-complete).

---

## 10. Next steps

| Goal | Go to |
|------|-------|
| Full feature tour | [Backtest Showcase](../../examples/notebooks/backtest_showcase.md) |
| Shared-capital portfolio | [Portfolio Shared Capital](../../examples/notebooks/portfolio_shared_capital_backtest.md) |
| PA strategy E2E | [PA Flag Breakout](../../examples/notebooks/pa_flag_breakout_strategy.md) |
| Param sweeps / WFO | [Capability Matrix](capability_matrix.md) |
| Benchmarks | [Backtest Benchmarks](../../examples/notebooks/backtest_benchmark.md) |
| Batch ↔ streaming parity | [Batch & Streaming](../../examples/batch-streaming.md) |