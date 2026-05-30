# Strategy Backtesting with QuantWave

A practical example of building and evaluating a trading strategy using QuantWave's Polars-native indicators and the vectorized backtesting engine.

## Highlights

- Synthetic data generation
- Computing indicators (e.g. SuperTrend) with the real `.ta` extension
- Using the backtester with rich signals and position sizing
- Evaluating performance

## Run locally (recommended)

```bash
pip install quantwave marimo polars numpy pandas
marimo edit docs/examples/notebooks/strategy_backtest.py
```

This notebook demonstrates the high-fidelity execution path and rich metadata support added in the v0.5 backtester improvements.

## View source

- [Raw notebook on GitHub](https://github.com/lavs9/quantwave/blob/main/docs/examples/notebooks/strategy_backtest.py)

## Live Notebook (Exported)

The notebook below is a pre-exported self-contained version generated during the docs build. It shows the structure, code, and any captured outputs.

<iframe src="rendered/strategy_backtest.html" width="100%" height="900px" style="border: 1px solid #ddd; border-radius: 8px;"></iframe>

**Note:** Some cells may have limited interactivity in the embedded view because they depend on the native `quantwave` Rust package. For the best experience, run the notebook locally with the command above.

---

See the [Multi-Indicator Analysis notebook](multi_indicator_analysis.md) for more examples of chaining indicators.