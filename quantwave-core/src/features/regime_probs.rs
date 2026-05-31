//! Regime probability features (soft labels) for ML pipelines.
//!
//! Provides a simple way to turn hard regime labels or basic clustering into
//! probability-like vectors that are excellent stationary features.
//!
//! This is a thin wrapper; richer soft outputs (HMM forward probs, GMM responsibilities)
//! can be wired here later when the regimes module exposes them cleanly.
//!
//! Source: regimes/mod.rs (MarketRegime) + quantwave-4ub research notes (regime probs as meta-features).

use crate::regimes::MarketRegime;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegimeProbFeatures {
    /// 5-dimensional soft vector (Bull, Bear, Crisis, Steady, Other/Cluster)
    pub probs: [f64; 5],
    pub hard_label: MarketRegime,
}

pub fn regime_to_prob_features(regime: MarketRegime) -> RegimeProbFeatures {
    let mut probs = [0.05; 5]; // small uniform prior
    let idx = match regime {
        MarketRegime::Bull => 0,
        MarketRegime::Bear => 1,
        MarketRegime::Crisis => 2,
        MarketRegime::Steady => 3,
        MarketRegime::Cluster(c) => 4.min(c as usize),
    };
    probs[idx] = 0.80;
    // normalize roughly
    let sum: f64 = probs.iter().sum();
    for p in &mut probs {
        *p /= sum;
    }
    RegimeProbFeatures {
        probs,
        hard_label: regime,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regime_prob_features_bull() {
        let f = regime_to_prob_features(MarketRegime::Bull);
        assert!(f.probs[0] > 0.7);
        assert_eq!(f.hard_label, MarketRegime::Bull);
    }
}
