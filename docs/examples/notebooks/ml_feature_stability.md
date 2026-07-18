# ML Feature Engineering: Stability, Parity & Tiny Model

**Canonical example** for correct usage of QuantWave's ML feature toolkit (part of the ML-features work).

## What this notebook demonstrates

- Building rich feature matrices using `HurstFeatureExtractor`, `CyberCycleFeatureExtractor`, `Trendflex`, `InstantaneousTrendline`, etc.
- Proving **zero-lookahead** and **batch vs streaming parity**
- Regime-conditional stability analysis on synthetic data with known shifts
- End-to-end tiny model training with per-regime metrics

## Run the notebook

```bash
pip install "quantwave[all]" marimo polars numpy
marimo edit docs/examples/notebooks/ml_feature_stability.py
```

(Or use `maturin develop -p quantwave-python --release` if working from source.)

## Sources & Context

This is the living reference for the feature engineering surface and validation methodology.

## View source

[Raw notebook on GitHub](https://github.com/lavs9/quantwave/blob/main/docs/examples/notebooks/ml_feature_stability.py)

---

**Note for the docs site:** Full interactivity requires a local installation because the feature extractors are implemented in Rust. This page provides context and the exact commands to run the real notebook.