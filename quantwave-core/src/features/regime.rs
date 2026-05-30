//! Basic regime feature helpers for ML pipelines.
//!
//! For MVP we provide simple converters from existing regime outputs
//! (e.g. MarketRegime labels or probs) into feature-friendly vectors.
//!
//! This will be expanded when full regime probability outputs are exposed
//! from the regimes module.
//!
//! Sources: regimes/mod.rs (MarketRegime enum), quantwave-4ub research notes.

use crate::regimes::MarketRegime;

/// Simple one-hot or embedding style features from a regime label.
/// For richer work, use actual probability vectors from HMM/GMM etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegimeFeatures {
    /// One-hot style (or embedded) vector. Length depends on regime set.
    pub regime_vector: [f64; 5], // Bull, Bear, Crisis, Steady, Cluster placeholder
    pub regime_label: MarketRegime,
}

pub fn regime_to_features(regime: MarketRegime) -> RegimeFeatures {
    let mut vec = [0.0; 5];
    let idx = match regime {
        MarketRegime::Bull => 0,
        MarketRegime::Bear => 1,
        MarketRegime::Crisis => 2,
        MarketRegime::Steady => 3,
        MarketRegime::Cluster(c) => 4.min(c as usize),
    };
    if idx < 5 {
        vec[idx] = 1.0;
    }
    RegimeFeatures {
        regime_vector: vec,
        regime_label: regime,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regime_to_features_bull() {
        let f = regime_to_features(MarketRegime::Bull);
        assert_eq!(f.regime_vector[0], 1.0);
        assert_eq!(f.regime_label, MarketRegime::Bull);
    }
}
