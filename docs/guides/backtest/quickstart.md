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

## 4. Trim indicator warmup before you backtest

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

## 5. When your trades actually fill

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

## 6. What you get back

| Output | Contents |
|--------|----------|
| `report.result.trades` | trade_id, entry/exit ts, prices, pnl_net, … |
| `report.result.equity_curve` | ts, equity, cash, position |
| `report.metrics()` | sharpe_ratio, max_drawdown_pct, win_rate, … |

Full key list: see [Capability Matrix](capability_matrix.md#python-bt-api-surface-complete).

---

## 7. Next steps

| Goal | Go to |
|------|-------|
| Full feature tour | [Backtest Showcase](../../examples/notebooks/backtest_showcase.md) |
| Shared-capital portfolio | [Portfolio Shared Capital](../../examples/notebooks/portfolio_shared_capital_backtest.md) |
| PA strategy E2E | [PA Flag Breakout](../../examples/notebooks/pa_flag_breakout_strategy.md) |
| Param sweeps / WFO | [Capability Matrix](capability_matrix.md) |
| Benchmarks | [Backtest Benchmarks](../../examples/notebooks/backtest_benchmark.md) |
| Batch ↔ streaming parity | [Batch & Streaming](../../examples/batch-streaming.md) |