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
- **Trades Table**: Complete log of all simulated trades with their PnL.
