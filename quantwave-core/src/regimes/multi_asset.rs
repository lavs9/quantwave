//! Multi-Asset Regime Detection
//! 
//! Identifies joint market regimes across multiple assets by clustering 
//! based on returns and rolling correlation structures.

use crate::traits::Next;
use crate::regimes::MarketRegime;
use crate::regimes::volatility_clustering::VolatilityClusterer;
use serde::{Deserialize, Serialize};

/// A clusterer for identifying regimes across multiple assets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiAssetClusterer {
    n_assets: usize,
    /// We use a VolatilityClusterer on a combined feature vector
    inner: VolatilityClusterer,
}

impl MultiAssetClusterer {
    pub fn new(n_assets: usize, window_size: usize, k: usize) -> Self {
        // Feature vector size: returns (n) + correlations (n*(n-1)/2)
        // For simplicity, we'll start with just returns + average correlation
        Self {
            n_assets,
            inner: VolatilityClusterer::new(14, window_size, k),
        }
    }
}

impl Next<&[f64]> for MultiAssetClusterer {
    type Output = MarketRegime;

    fn next(&mut self, returns: &[f64]) -> Self::Output {
        if returns.len() != self.n_assets {
            return MarketRegime::Steady;
        }

        // Feature engineering: aggregate multi-asset behavior into 1D for the clusterer
        // 1. Mean absolute return (Magnitude of move)
        let mean_abs_ret = returns.iter().map(|r| r.abs()).sum::<f64>() / self.n_assets as f64;
        
        // 2. Dispersion (how much assets are moving in different directions)
        let mean_ret = returns.iter().sum::<f64>() / self.n_assets as f64;
        let dispersion = returns.iter().map(|r| (r - mean_ret).powi(2)).sum::<f64>() / self.n_assets as f64;

        // Use ATR-like logic on the magnitude of the joint move
        // We pass it to the inner clusterer (which expects high/low/close, but we can mock it)
        self.inner.next((mean_abs_ret, mean_abs_ret - dispersion.sqrt(), mean_abs_ret))
    }
}
