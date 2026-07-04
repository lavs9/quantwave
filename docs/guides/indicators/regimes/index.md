# Regime Detection — User Guide

Regime detection identifies **market states** (bull/bear, volatility clusters, structural breaks) for filtering signals, sizing positions, and ML feature engineering.

All algorithms live in `quantwave-core/src/regimes/` and expose batch (Polars) and streaming (`Next<T>`) paths with identical semantics.

**Fittable HMM (ldhmm parity):** see the dedicated [ldhmm-Style HMM Workflow](ldhmm_workflow.md) for fit → decode → forecast → strategy integration.

---

## Algorithm overview

| Algorithm | Core type | Best for | Polars method |
|-----------|-----------|----------|---------------|
| **Volatility clustering** | Online K-means on ATR | Crisis vs stable vol regimes | `.ta().volatility_clusterer(...)` |
| **HMM (Hamilton)** | Gaussian emissions + Viterbi | Bull/bear switching | `.ta().hmm_bull_bear(col)` |
| **Gaussian / lambda HMM (ldhmm)** | EM fit + forward-backward | Fittable regimes, vol forecast | `.ta().hmm_fit(...)`, `.ta().hmm_forecast_vol(...)` |
| **GMM** | Multi-variate clustering | Latent factor states | `.ta().gmm(cols, k)` |
| **PELT** | Exact changepoint segmentation | Historical break dating | `.ta().pelt(col, penalty, min_dist)` |

**Sources:** Hamilton (1989); Killick et al. (2012) PELT; Prakash et al. (2021) vol clustering; Two Sigma (2021) GMM notes.

---

## 1. Volatility clustering

Identifies discrete volatility regimes (e.g. Stable → Crisis) using rolling ATR and online clustering.

### Polars (batch)

```python
import polars as pl

df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .ta()
    .volatility_clusterer(
        high="high",
        low="low",
        close="close",
        atr_period=14,
        window_size=100,
        k=3,
    )
    .collect()
)
# Column: volatility_regime (u32 labels)
```

### Rust (streaming)

```rust
use quantwave_core::regimes::volatility_clustering::VolatilityClusterer;
use quantwave_core::traits::Next;

let mut clusterer = VolatilityClusterer::new(14, 100, 3);
for (h, l, c) in ohlcv {
    let regime = clusterer.next((h, l, c));
}
```

### Edge cases

| Condition | Behavior |
|-----------|----------|
| `window_size` not filled | Partial / default cluster assignment |
| `k=1` | Degenerate — use k ≥ 2 |
| NaN OHLC | Skipped or propagates per bar |

### ML integration

Join `volatility_regime` with PA events or `.ta.features.regime_probs()` for confluence filters. See [ML Features guide](../../ml_features.md).

---

## 2. Hidden Markov Model (bull/bear)

Regime-switching HMM with Gaussian emissions; Viterbi decoding for the most likely state path.

### Polars (batch)

```python
df = (
    df.lazy()
    .with_columns(pl.col("close").pct_change().alias("returns"))
    .ta()
    .hmm_bull_bear("returns")
    .collect()
)
# Column: hmm_regime (1=Bull, 2=Bear, 0=other)
```

### Python (streaming)

```python
import quantwave as qw

hmm = qw.BullBearHMM.bull_bear()
for ret in returns:
    state = hmm.next(ret)
```

### Edge cases

| Condition | Behavior |
|-----------|----------|
| Short series | Unstable state estimates; prefer ≥ 100 bars |
| Zero variance returns | Emission collapse — watch for stuck states |

### Strategy pattern

```python
# Long only in bull regime (label 1)
df = df.with_columns(
    (pl.col("hmm_regime") == 1).cast(pl.Float64).alias("regime_filter")
)
```

Used in [ML Features → Backtest E2E](../../../examples/notebooks/ml_feature_backtest_parity.md).

---

## 2b. Fittable HMM (ldhmm parity)

Baum–Welch EM, lambda (ecld) emissions, forward–backward decode, mixture vol forecasts, and pseudo-residual diagnostics — aligned with [ldhmm](https://cran.r-project.org/package=ldhmm) (SSRN 2979516).

**Full workflow:** [ldhmm-Style HMM Workflow](ldhmm_workflow.md) (fit → decode → live filter → diagnostics → strategy).

Quick batch example:

```python
df = (
    df.lazy()
    .with_columns(pl.col("close").pct_change().alias("returns"))
    .ta()
    .hmm_fit("returns", n_states=2, max_iter=100, fit_lambdas=True)
    .collect()
)
```

API reference: [gaussian_hmm](../native/gaussian_hmm.md), [lambda_hmm](../native/lambda_hmm.md), [hmm_forecast](../native/hmm_forecast.md).

---

## 3. Gaussian Mixture Model (GMM)

Clusters multi-column factor data into latent regimes.

### Polars (batch)

```python
df = (
    df.lazy()
    .ta()
    .gmm(["returns", "volume_z"], k=4)
    .collect()
)
# Column: gmm_regime
```

### Edge cases

| Condition | Behavior |
|-----------|----------|
| `k` > row count | Fit degrades — reduce k |
| Correlated columns | Consider orthogonal factors first |

---

## 4. PELT changepoint detection

Pruned Exact Linear Time segmentation — finds statistically significant level shifts in a series.

### Polars (batch)

```python
df = (
    df.lazy()
    .ta()
    .pelt("close", penalty=1.0, min_dist=10)
    .collect()
)
# Changepoint flags in output column
```

### Edge cases

| Condition | Behavior |
|-----------|----------|
| Low `penalty` | Many changepoints (over-segmentation) |
| High `penalty` | Few or no breaks detected |

---

## ML features shortcut

For backtest-ready regime labels without manual HMM wiring:

```python
df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .ta()
    .features()
    .regime_features()   # HMM bull/bear label column
    .collect()
)
```

Also available: `.ta.features.regime_probs()` for probability struct output.

---

## Parity & validation

Regime modules follow the Universal Indicator pattern where stateful. Use streaming wrappers in live systems:

```python
wrapped = qw.wrap_streaming(hmm, name="market_state")
```

Core tests: `cargo nextest run -p quantwave-core -- regimes`

---

## Related docs

- [ldhmm-Style HMM Workflow](ldhmm_workflow.md)
- [ML Features guide](../../ml_features.md)
- [Indicator Gallery](../gallery.md) — Regime section
- [PA confluence notebook](../../../examples/notebooks/pa_foundation_strategy.py)