# Regime Detection

Regime detection tools help identify different market states, such as Bull/Bear markets, High/Low volatility periods, or structural breaks in time series data.

QuantWave provides several state-of-the-art algorithms for regime detection:

## Available Algorithms

### 1. Volatility Clustering
Inspired by **Prakash et al. (2021)**, this tool uses rolling ATR and online K-Means clustering to identify discrete volatility regimes (e.g., Stable, Crisis).

### 2. Hidden Markov Models (HMM)
Based on **Hamilton (1989)**, this implements a regime-switching HMM with Gaussian emissions. It uses the Viterbi algorithm for real-time state decoding.

### 3. Gaussian Mixture Models (GMM)
Inspired by **Two Sigma (2021)**, this uses multi-variate clustering on factor data to identify latent market states.

### 4. Changepoint Detection (PELT)
Implementation of the **Pruned Exact Linear Time (PELT)** algorithm from **Killick et al. (2012)**. It provides exact segmentation of historical data based on statistical shifts.

---

## Usage Examples

### Polars (Batch)

```python
import polars as pl
import quantwave as qw

df = pl.read_csv("market_data.csv")

# Identify 3 volatility regimes (Low, Medium, High)
df = df.lazy().ta().volatility_clusterer(
    high="high", 
    low="low", 
    close="close", 
    atr_period=14, 
    window_size=100, 
    k=3
).collect()

# Bull/Bear HMM
df = df.lazy().ta().hmm_bull_bear("returns").collect()
```

### Rust (Streaming)

```rust
use quantwave_core::regimes::hmm::HMM;
use quantwave_core::traits::Next;

let mut hmm = HMM::bull_bear();
for price in prices {
    let regime = hmm.next(price);
    println!("Current Regime: {:?}", regime);
}
```
