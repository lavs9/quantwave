//! Ehlers Autocorrelation feature extractor wrapper.
//!
//! Wraps the lag correlation vector. For ML, we expose the full vector (or top lags)
//! plus a "dominant lag" summary – excellent cycle feature.
//!
//! Source: quantwave-core/src/indicators/ehlers_autocorrelation.rs (returns Vec<f64> correlations)

use crate::indicators::ehlers_autocorrelation::EhlersAutocorrelation;
use crate::traits::Next;

#[derive(Debug, Clone, PartialEq)]
pub struct EhlersAutocorrelationFeatures {
    pub correlations: Vec<f64>,
    pub dominant_lag: usize,
    pub max_correlation: f64,
}

#[derive(Debug, Clone)]
pub struct EhlersAutocorrelationFeatureExtractor {
    inner: EhlersAutocorrelation,
}

impl EhlersAutocorrelationFeatureExtractor {
    pub fn new(length: usize, num_lags: usize) -> Self {
        Self {
            inner: EhlersAutocorrelation::new(length, num_lags),
        }
    }
}

impl Next<f64> for EhlersAutocorrelationFeatureExtractor {
    type Output = EhlersAutocorrelationFeatures;

    fn next(&mut self, input: f64) -> Self::Output {
        let corrs = self.inner.next(input);
        let (dominant_lag, &max_correlation) = corrs
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((0, &0.0));

        EhlersAutocorrelationFeatures {
            correlations: corrs,
            dominant_lag,
            max_correlation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocorr_wrapper_basic() {
        let mut ext = EhlersAutocorrelationFeatureExtractor::new(30, 10);
        for i in 0..100 {
            let val = 100.0 + 5.0 * (i as f64 * 0.2).sin();
            let f = ext.next(val);
            assert_eq!(f.correlations.len(), 10);
        }
    }
}
