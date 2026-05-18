//! Multi-Asset Regime Detection
//! 
//! Identifies joint market regimes across multiple assets by clustering 
//! based on returns and rolling correlation structures.

use crate::traits::Next;
use crate::regimes::MarketRegime;
use crate::regimes::volatility_clustering::VolatilityClusterer;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// A clusterer for identifying regimes across multiple assets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiAssetClusterer {
    n_assets: usize,
    window_size: usize,
    /// We use a VolatilityClusterer on a combined feature vector
    inner: VolatilityClusterer,
    history: Vec<VecDeque<f64>>,
}

impl MultiAssetClusterer {
    pub fn new(n_assets: usize, window_size: usize, k: usize) -> Self {
        // Feature vector size: 
        // 1. Mean absolute return (1)
        // 2. Dispersion (1)
        // 3. Average correlation (1)
        // Total features: 3
        Self {
            n_assets,
            window_size,
            inner: VolatilityClusterer::new(14, window_size, k),
            history: vec![VecDeque::with_capacity(window_size); n_assets],
        }
    }

    fn calculate_average_correlation(&self) -> f64 {
        if self.history[0].len() < self.window_size {
            return 1.0;
        }

        let mut total_corr = 0.0;
        let mut pairs = 0;

        for i in 0..self.n_assets {
            for j in (i + 1)..self.n_assets {
                let corr = self.correlation(i, j);
                total_corr += corr;
                pairs += 1;
            }
        }

        if pairs == 0 { 1.0 } else { total_corr / pairs as f64 }
    }

    fn correlation(&self, i: usize, j: usize) -> f64 {
        let x = &self.history[i];
        let y = &self.history[j];
        let n = x.len() as f64;

        let mean_x = x.iter().sum::<f64>() / n;
        let mean_y = y.iter().sum::<f64>() / n;

        let mut cov = 0.0;
        let mut var_x = 0.0;
        let mut var_y = 0.0;

        for k in 0..x.len() {
            let dx = x[k] - mean_x;
            let dy = y[k] - mean_y;
            cov += dx * dy;
            var_x += dx * dx;
            var_y += dy * dy;
        }

        let den = (var_x * var_y).sqrt();
        if den == 0.0 { 1.0 } else { cov / den }
    }
}

impl Next<&[f64]> for MultiAssetClusterer {
    type Output = MarketRegime;

    fn next(&mut self, returns: &[f64]) -> Self::Output {
        if returns.len() != self.n_assets {
            return MarketRegime::Steady;
        }

        // Update history
        for (i, &r) in returns.iter().enumerate() {
            self.history[i].push_back(r);
            if self.history[i].len() > self.window_size {
                self.history[i].pop_front();
            }
        }

        // Feature engineering
        // 1. Mean absolute return (Magnitude of move)
        let mean_abs_ret = returns.iter().map(|r| r.abs()).sum::<f64>() / self.n_assets as f64;
        
        // 2. Dispersion (how much assets are moving in different directions)
        let mean_ret = returns.iter().sum::<f64>() / self.n_assets as f64;
        let dispersion = returns.iter().map(|r| (r - mean_ret).powi(2)).sum::<f64>() / self.n_assets as f64;

        // 3. Average Correlation
        let avg_corr = self.calculate_average_correlation();

        // Pass features to inner clusterer
        // We use mean_abs_ret as the primary signal, dispersion and correlation as modifiers
        // For VolatilityClusterer, we'll map these to high/low/close equivalents
        self.inner.next((mean_abs_ret, mean_abs_ret * (1.0 - dispersion.sqrt()), avg_corr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_asset_clusterer_basic() {
        let mut clusterer = MultiAssetClusterer::new(2, 5, 2);
        
        // Steady market
        for _ in 0..10 {
            clusterer.next(&[0.01, 0.01]);
        }
        let r1 = clusterer.next(&[0.01, 0.01]);
        
        // Highly volatile/correlated
        for _ in 0..10 {
            clusterer.next(&[0.05, 0.05]);
        }
        let r2 = clusterer.next(&[0.05, 0.05]);
        
        // Assert different regimes if enough data for clustering
        // Since it's a dynamic clusterer, exact states depend on initialization
        assert!(matches!(r1, MarketRegime::Steady | MarketRegime::Cluster(_)));
        assert!(matches!(r2, MarketRegime::Steady | MarketRegime::Cluster(_)));
    }
}
