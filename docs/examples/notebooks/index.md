# Marimo Notebooks

Explore interactive examples of QuantWave in action using [Marimo](https://marimo.io/).

* [Strategy Backtesting](strategy_backtest.py) - A steel-thread example of backtesting a simple strategy.
* [Multi-Indicator Analysis](multi_indicator_analysis.py) - Combining multiple indicators using Polars expressions.
* [ML Feature Stability & Tiny Model](ml_feature_stability.py) — **Canonical example** (quantwave-gw7s): builds feature matrix from the new toolkit (CyberCycle/Hurst/Trendflex/ITrend features), trains tiny regime+direction model, reports stability + regime-conditional metrics, proves no-lookahead. The living spec for correct ML feature usage.
* [ML Features → Realistic Backtest with Rich Metadata (E2E)](ml_feature_backtest_parity.py) — **Primary cross-epic closure artifact** (quantwave-4ps + quantwave-gwx): full end-to-end using the locked `.ta().features()` surface (Hurst, CyberCycle Struct, Griffiths DC, regime HMM labels) for realistic strategy with feature+regime entry/sizing logic. Demonstrates batch Polars path (exposure + meta columns) vs streaming (FeatureToSignal adapter implementing Next<&Bar> → StrategySignal with rich metadata) with exact parity verification, metadata preservation into trades, and copy-paste Rust adapter. The smoking-gun living reference and contract exercise.

---

*Note: These notebooks are interactive. You can run them locally using `marimo edit`.*
