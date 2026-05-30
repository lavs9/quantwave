# Marimo Notebooks

Explore interactive examples of QuantWave in action using [Marimo](https://marimo.io/).

These pages are landing pages for the notebooks. For the best experience (full interactivity + native Rust performance), run them locally with `marimo edit`.

### Available Notebooks

- **[Strategy Backtesting](strategy_backtest.md)**  
  Steel-thread example using indicators inside the vectorized backtester with rich signal metadata.

- **[Multi-Indicator Analysis](multi_indicator_analysis.md)**  
  Clean chaining of multiple indicators (SMA, EMA, Momentum, SuperTrend, etc.) in one lazy Polars expression.

- **[ML Feature Stability & Tiny Model](ml_feature_stability.md)** — *Canonical example* (quantwave-gw7s)  
  Builds feature matrices from the new toolkit, proves batch/streaming parity + no-lookahead, trains a tiny regime+direction model with per-regime metrics.

- **[ML Features → Realistic Backtest (E2E)](ml_feature_backtest_parity.md)** — *Primary cross-epic reference* (quantwave-4ps + quantwave-gwx)  
  End-to-end demonstration of the locked features surface feeding the backtester. Shows batch vs streaming parity with rich metadata preserved all the way into trades.

### How to run any notebook locally

```bash
# Recommended
pip install "quantwave[all]" marimo polars numpy

# Or from source after building the Python bindings
maturin develop -p quantwave-python --release
pip install marimo polars numpy

marimo edit docs/examples/notebooks/<notebook_name>.py
```

**Why some notebooks show limited content here:**  
The live documentation site is static (GitHub Pages). Notebooks that depend on QuantWave's native Rust extensions cannot execute inside the browser. The pages above give you context + the exact commands to run the real interactive versions locally.
