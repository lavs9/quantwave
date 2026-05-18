//! Regime Ensemble and Voting
//! 
//! Combine multiple regime detection models (e.g., HMM, Volatility Clustering, GMM)
//! using weighted voting to increase the robustness of market state identification.

use crate::regimes::MarketRegime;
use serde::{Deserialize, Serialize};

/// A weighted ensemble of market regimes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegimeEnsemble {
    pub weights: Vec<f64>,
}

impl RegimeEnsemble {
    pub fn new(weights: Vec<f64>) -> Self {
        Self { weights }
    }

    /// Combines multiple regimes into a single consensus regime.
    /// 
    /// # Arguments
    /// * `regimes` - A slice of regimes from different models.
    pub fn vote(&self, regimes: &[MarketRegime]) -> MarketRegime {
        if regimes.is_empty() { return MarketRegime::Steady; }
        
        let mut scores: std::collections::HashMap<MarketRegime, f64> = std::collections::HashMap::new();
        
        for (i, &regime) in regimes.iter().enumerate() {
            let weight = self.weights.get(i).unwrap_or(&1.0);
            *scores.entry(regime).or_insert(0.0) += weight;
        }

        // Find regime with highest total weight
        scores.into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(regime, _)| regime)
            .unwrap_or(MarketRegime::Steady)
    }
}
