//! Trendflex feature extractor wrapper.
//!
//! Simple wrapper around existing Trendflex for ML use.
//! Returns the zero-lag trend component value.
//!
//! Source: quantwave-core/src/indicators/trendflex.rs

use crate::indicators::trendflex::Trendflex;
use crate::traits::Next;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrendflexFeatures {
    pub trendflex: f64,
}

#[derive(Debug, Clone)]
pub struct TrendflexFeatureExtractor {
    inner: Trendflex,
}

impl TrendflexFeatureExtractor {
    pub fn new(length: usize) -> Self {
        Self {
            inner: Trendflex::new(length),
        }
    }
}

impl Next<f64> for TrendflexFeatureExtractor {
    type Output = TrendflexFeatures;

    fn next(&mut self, input: f64) -> Self::Output {
        let val = self.inner.next(input);
        TrendflexFeatures { trendflex: val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trendflex_wrapper_basic() {
        let mut ext = TrendflexFeatureExtractor::new(20);
        for i in 0..50 {
            let val = 100.0 + (i as f64) * 0.1;
            let _f = ext.next(val);
            // After warmup it should be non-zero in trending data
        }
    }
}
