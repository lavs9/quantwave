//! Hurst Exponent feature extractor wrapper.
//!
//! Wraps the existing `HurstExponent` to provide a clean ML feature interface
//! (persistence value + optional regime classification).
//!
//! This is one of the highest-ROI single features for ML/regime work:
//! - H < 0.5 → mean-reverting (anti-persistent)
//! - H ≈ 0.5 → random walk
//! - H > 0.5 → trending (persistent)
//!
//! Source: quantwave-core/src/indicators/hurst.rs (Rescaled Range implementation)

use crate::indicators::hurst::HurstExponent;
use crate::traits::Next;

/// Rich output for Hurst-based features.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HurstFeatures {
    pub persistence: f64,
    /// Optional discrete regime label for convenience in ML pipelines.
    /// -1 = mean-reverting, 0 = random, +1 = trending (thresholds configurable later).
    pub regime_label: Option<i8>,
}

impl HurstFeatures {
    pub fn new(persistence: f64, regime_label: Option<i8>) -> Self {
        Self {
            persistence,
            regime_label,
        }
    }
}

/// Wrapper that turns the existing HurstExponent into a feature-rich extractor.
#[derive(Debug, Clone)]
pub struct HurstFeatureExtractor {
    inner: HurstExponent,
    /// Thresholds for regime classification (can be made configurable later).
    mean_reverting_threshold: f64,
    trending_threshold: f64,
}

impl HurstFeatureExtractor {
    pub fn new(period: usize) -> Self {
        Self {
            inner: HurstExponent::new(period),
            mean_reverting_threshold: 0.45,
            trending_threshold: 0.55,
        }
    }

    pub fn with_thresholds(mut self, mean_rev: f64, trending: f64) -> Self {
        self.mean_reverting_threshold = mean_rev;
        self.trending_threshold = trending;
        self
    }
}

impl Next<f64> for HurstFeatureExtractor {
    type Output = HurstFeatures;

    fn next(&mut self, input: f64) -> Self::Output {
        let persistence = self.inner.next(input);

        let regime_label = if persistence.is_nan() {
            None
        } else if persistence < self.mean_reverting_threshold {
            Some(-1)
        } else if persistence > self.trending_threshold {
            Some(1)
        } else {
            Some(0)
        };

        HurstFeatures::new(persistence, regime_label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_hurst_feature_basic() {
        let mut extractor = HurstFeatureExtractor::new(20);

        // Feed trending data
        for i in 0..30 {
            let val = 100.0 + (i as f64) * 0.5;
            let f = extractor.next(val);
            if !f.persistence.is_nan() {
                assert!(
                    f.persistence > 0.5,
                    "Expected trending persistence, got {}",
                    f.persistence
                );
            }
        }
    }

    #[test]
    fn test_hurst_feature_regime_labels() {
        let mut extractor = HurstFeatureExtractor::new(10).with_thresholds(0.4, 0.6);

        // Force a high persistence value via dummy (real impl will vary)
        // This is a smoke test for the wrapper logic
        let f = extractor.next(100.0);
        // After warmup it should produce a label
        if !f.persistence.is_nan() {
            assert!(f.regime_label.is_some());
        }
    }
}
