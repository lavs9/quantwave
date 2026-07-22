# Tear Sheets

QuantWave includes built-in HTML tear sheets for analyzing backtest results. These are standalone, zero-dependency HTML files containing interactive equity curves, drawdowns, performance metrics, and a paginated trades table.

## Usage

Once you generate a `BacktestReport` using `backtest_with_report()`, you can either get the raw HTML string or save it directly to a file:

```python
import polars as pl
from quantwave import tearsheet

# 1. Run your backtest
df = pl.DataFrame({
    "timestamp": list(range(20)),
    "close": [100.0 + i * 0.5 for i in range(20)],
    "signal": [0.0, 1.0, 1.0, 1.0, 1.0, 0.0] + [0.0] * 14,
})

report = (
    df.lazy()
    .bt.backtest_with_report(
        signal="signal",
        commission_bps=0.0,
        slippage_bps=0.0,
    )
)

# 2. Save as a standalone HTML tear sheet
tearsheet.save_html(report, "report.html", title="My Strategy Backtest")

# Or get the HTML string directly
html_content = tearsheet.render_html(report, title="My Strategy Backtest")
```

The resulting HTML file can be opened in any web browser and contains:
- **Summary Metrics**: Sharpe ratio, CAGR, Max Drawdown, Win Rate.
- **Equity Curve**: Interactive chart of portfolio equity over time.
- **Drawdown Curve**: Interactive chart of portfolio drawdown percentage over time.
- **Rolling Sharpe / Rolling Volatility**: Trailing-window (default 20-bar) annualized Sharpe and volatility charts.
- **Monthly Returns Heatmap**: Year × month grid, green/red shaded by return magnitude.
- **Trade Blotter**: Full per-trade table (entry/exit time, side, prices, quantity, net PnL) — in addition to the existing aggregate Trade Summary card.
- **Run Metadata**: Title, generated-at (UTC) timestamp, and — when supplied — a reproducibility `seed` and arbitrary `run_metadata` config key/values.

## Reproducible run metadata & benchmark-relative section

`to_html()` / `save_html()` (and the `tearsheet.render_html()` / `tearsheet.save_html()` helpers) accept optional keyword arguments, all additive and backward compatible:

```python
report.to_html(
    title="My Strategy Backtest",
    seed=42,                                   # shown in the Run Metadata card
    run_metadata={"commission_bps": "5", "execution_delay": "same_bar"},
    benchmark_returns=benchmark_daily_returns,  # per-bar simple returns, aligned by index
    rolling_window=20,                          # rolling Sharpe/vol window, in bars
)
```

Passing `benchmark_returns` adds a **Benchmark-Relative** card (alpha, beta, strategy vs. benchmark cumulative return) computed from [`PerformanceMetrics.benchmark`](output-contract.md#benchmark-relative-analytics-additive). Omit it and the section is left out entirely — no empty card, no broken layout.
