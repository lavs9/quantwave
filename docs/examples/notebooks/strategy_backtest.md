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

## Documentation site limitation

Because this is a static site (GitHub Pages), notebooks that require the native `quantwave` package show fallback content. For the full interactive marimo experience with real computations, run the notebook locally.

---

See the [Multi-Indicator Analysis notebook](multi_indicator_analysis.md) for more examples of chaining indicators.