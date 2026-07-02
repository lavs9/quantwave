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

## 4. What you get back

| Output | Contents |
|--------|----------|
| `report.result.trades` | trade_id, entry/exit ts, prices, pnl_net, … |
| `report.result.equity_curve` | ts, equity, cash, position |
| `report.metrics()` | sharpe_ratio, max_drawdown_pct, win_rate, … |

Full key list: see [Capability Matrix](../capability_matrix.md#python-bt-api-surface-complete).

---

## 5. Next steps

| Goal | Go to |
|------|-------|
| Full feature tour | [Backtest Showcase](../../../examples/notebooks/backtest_showcase.md) |
| Shared-capital portfolio | [Portfolio Shared Capital](../../../examples/notebooks/portfolio_shared_capital_backtest.md) |
| PA strategy E2E | [PA Flag Breakout](../../../examples/notebooks/pa_flag_breakout_strategy.md) |
| Param sweeps / WFO | [Capability Matrix](../capability_matrix.md) |
| Benchmarks | [Backtest Benchmarks](../../../examples/notebooks/backtest_benchmark.md) |
| Batch ↔ streaming parity | [Batch & Streaming](../../../examples/batch-streaming.md) |