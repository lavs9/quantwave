# Marimo Notebooks

Explore interactive examples of QuantWave in action using [Marimo](https://marimo.io/).

These pages are landing pages for the notebooks. For the best experience (full interactivity + native Rust performance), run them locally with `marimo edit`.

### Available Notebooks

- **[Strategy Backtesting](strategy_backtest.md)**  
  Steel-thread example using indicators inside the vectorized backtester with rich signal metadata.

- **[Backtest Engine Benchmarks](backtest_benchmark.md)**  
  Criterion harness comparing `quantwave-backtest` vs naive row-loop baselines (10K–1M rows, multi-symbol).

- **[Multi-Indicator Analysis](multi_indicator_analysis.md)**  
  Clean chaining of multiple indicators (SMA, EMA, Momentum, SuperTrend, etc.) in one lazy Polars expression.

- **[ML Feature Stability & Tiny Model](ml_feature_stability.md)** — *Canonical example*  
  Builds feature matrices from the new toolkit, proves batch/streaming parity + no-lookahead, trains a tiny regime+direction model with per-regime metrics.

- **[ML Features → Realistic Backtest (E2E)](ml_feature_backtest_parity.md)** — *Primary cross-epic reference* (closed epics 4ps + gwx)  
  End-to-end demonstration of the locked features surface feeding the backtester. Shows batch vs streaming parity with rich metadata preserved all the way into trades.

- **[PA Foundation Strategy (MarketStructure + Flags/H&S)](pa_foundation_strategy.py)**  
  Production-ready surface for the MQL5 PA toolkit foundation (Parts 21/66/69: swings/bias/flips + geometric). Realistic strategy: bull Flag breakout only on confirmed bullish MarketStructure + regime + ML (hurst) filter, dynamically sized from `pole_length_atr` rich metadata. Python streaming + Polars Rust paths + backtester sketch. Synthetic + notes for real data. See also the four dedicated PA guides under Native Indicators.

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
