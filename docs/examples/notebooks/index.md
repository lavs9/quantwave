# Notebooks

Runnable [Marimo](https://marimo.io/) notebooks demonstrating QuantWave end-to-end. Each page below is a landing summary; run locally for full interactivity and native Rust performance.

!!! tip "Recommended order"
    1. [Strategy Backtest](strategy_backtest.md) — indicator → signal → `.bt`
    2. [Backtest Showcase](backtest_showcase.md) — sweeps, WFO, Monte Carlo
    3. [ML Features → Backtest E2E](ml_feature_backtest_parity.md) — feature parity into trades

## Backtest & portfolio

### Available Notebooks

- **[Strategy Backtesting](strategy_backtest.md)**  
  Steel-thread example using indicators inside the vectorized backtester with rich signal metadata.

- **[Backtest Engine Showcase](backtest_showcase.md)**  
  Comprehensive tour of the Polars-native backtester: param sweeps, walk-forward optimization, fast metrics, sizing filters, and cross-sectional panels.

- **[Backtest Engine Benchmarks](backtest_benchmark.md)**  
  Criterion harness comparing `quantwave-backtest` vs naive row-loop baselines (10K–1M rows, multi-symbol).

- **[Portfolio Shared Capital](portfolio_shared_capital_backtest.md)**  
  Multi-symbol book simulation with one cash pool via `.bt.portfolio_backtest()`.

- **[Execution-Aware Research](execution_aware_research.md)** — *Canonical execution-realism example*  
  First-class order types (limit / stop / stop-limit) via `.bt.order_backtest()`, risk overlays (`risk_model=`), and benchmark-relative reporting (alpha / beta / Calmar / VaR / CVaR) — all on the batch ↔ streaming parity core.

## Indicators & ML

- **[Multi-Indicator Analysis](multi_indicator_analysis.md)**
  Clean chaining of multiple indicators (SMA, EMA, Momentum, SuperTrend, etc.) in one lazy Polars expression.

- **[ML Feature Stability & Tiny Model](ml_feature_stability.md)** — *Canonical example*  
  Builds feature matrices from the new toolkit, proves batch/streaming parity + no-lookahead, trains a tiny regime+direction model with per-regime metrics.

- **[ML Features → Realistic Backtest (E2E)](ml_feature_backtest_parity.md)** — *Primary cross-epic reference* (closed epics 4ps + gwx)  
  See also the [ML Features guide](../../guides/ml_features.md) for Polars/streaming patterns.  
  End-to-end demonstration of the locked features surface feeding the backtester. Shows batch vs streaming parity with rich metadata preserved all the way into trades.

- **[PA Foundation Strategy (MarketStructure + Flags/H&S)](pa_foundation_strategy.py)**  
  Production-ready surface for the MQL5 PA toolkit foundation (Parts 21/66/69: swings/bias/flips + geometric). Realistic strategy: bull Flag breakout only on confirmed bullish MarketStructure + regime + ML (hurst) filter, dynamically sized from `pole_length_atr` rich metadata. Python streaming + Polars Rust paths + backtester sketch. Synthetic + notes for real data. See also the four dedicated PA guides under Native Indicators.

- **[PA Flag Breakout Canonical E2E](pa_flag_breakout_strategy.md)**  
  Runnable Marimo notebook demonstrating the exact production pattern for the PA geometric tools, complete with sizing via `pole_length_atr` and confirmed market structure filters.

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
