# Multi-Indicator Analysis with QuantWave

In this notebook, we demonstrate how to combine multiple QuantWave indicators (classic moving averages + Ehlers DSP suite) using Polars' high-performance lazy expressions and method chaining.

## What you'll see

- Generating realistic synthetic OHLCV data
- Applying SMA, EMA, Momentum, and SuperTrend in a single vectorized pass via the `.ta` namespace
- Unnesting struct outputs for clean analysis
- Clean, readable Polars code that runs efficiently

## Run the interactive notebook locally

For the full interactive experience with real QuantWave execution:

```bash
# Install dependencies
pip install quantwave marimo polars numpy

# Edit and run the notebook
marimo edit docs/examples/notebooks/multi_indicator_analysis.py
```

The notebook will use the native Rust + Polars backend for fast indicator computation.

## View the source

- [View raw notebook on GitHub](https://github.com/lavs9/quantwave/blob/main/docs/examples/notebooks/multi_indicator_analysis.py)

## Note on the documentation site

This page exists because the live documentation is a static site. Full interactive marimo notebooks that depend on compiled Rust extensions (like QuantWave) cannot run directly in the browser on GitHub Pages.

The best experience is always running the `.py` notebook locally with `marimo edit`.

---

**Related:** See the [Strategy Backtesting notebook](strategy_backtest.md) for an end-to-end example using these indicators inside a backtester.