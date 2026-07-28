# ML Features → Realistic Backtest with Rich Metadata (E2E)

**Primary canonical artifact** closing the loop between feature engineering and the vectorized backtester.

## What you'll learn

- Using the locked `.ta().features()` surface (Hurst, CyberCycle Struct, Griffiths Dominant Cycle, regime labels)
- Building exposure + metadata columns in Polars (batch path)
- Streaming path using a `Next<&Bar, Output=StrategySignal>` adapter that preserves rich metadata
- Exact batch vs streaming parity verification (equity curves, trade lists, PnL, stats)
- Realistic strategy that uses features + regime for entry filters and conviction sizing

This notebook is the "smoking gun" reference for the integration contract between features and the backtester.

## Run locally

```bash
# After building the Python bindings from source
maturin develop -p quantwave-python --release

pip install marimo polars numpy
marimo edit docs/examples/notebooks/ml_feature_backtest_parity.py
```

All computation is deterministic for reproducible results.

## View source

[Raw notebook on GitHub](https://github.com/lavs9/quantwave/blob/main/docs/examples/notebooks/ml_feature_backtest_parity.py)

## Why this page exists on the docs site

The full notebook relies on native Rust extensions for both feature extractors and the backtester. These cannot run inside the browser-based environment used by the documentation site.

Running it locally gives you the complete interactive experience with real parity checks and rich metadata flowing all the way into trade records.