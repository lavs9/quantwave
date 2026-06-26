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
use crate::regimes::hmm::HMM;
use crate::traits::Next;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegimeProbFeatures {
    /// 5-dimensional soft vector (Bull, Bear, Crisis, Steady, Other/Cluster)
    pub probs: [f64; 5],
    pub hard_label: MarketRegime,
}

pub fn regime_to_prob_features(regime: MarketRegime) -> RegimeProbFeatures {
    let mut probs = [0.05; 5];
    let idx = match regime {
        MarketRegime::Bull => 0,
        MarketRegime::Bear => 1,
        MarketRegime::Crisis => 2,
        MarketRegime::Steady => 3,
        MarketRegime::Cluster(c) => 4.min(c as usize),
    };
    probs[idx] = 0.80;
    let sum: f64 = probs.iter().sum();
    for p in &mut probs {
        *p /= sum;
    }
    RegimeProbFeatures {
        probs,
        hard_label: regime,
    }
}

/// Streaming extractor: HMM hard label + forward state probabilities as soft features.
#[derive(Debug, Clone)]
pub struct RegimeProbFeatureExtractor {
    hmm: HMM,
}

impl RegimeProbFeatureExtractor {
    pub fn bull_bear() -> Self {
        Self {
            hmm: HMM::bull_bear(),
        }
    }
}

impl Next<f64> for RegimeProbFeatureExtractor {
    type Output = RegimeProbFeatures;

    fn next(&mut self, input: f64) -> Self::Output {
        let label = self.hmm.next(input);
        let state_probs = self.hmm.state_probabilities();
        let mut probs = [0.05; 5];
        if state_probs.len() >= 2 {
            probs[0] = state_probs[0];
            probs[1] = state_probs[1];
        }
        let sum: f64 = probs.iter().sum();
        if sum > 0.0 {
            for p in &mut probs {
                *p /= sum;
            }
        }
        RegimeProbFeatures {
            probs,
            hard_label: label,
        }
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
