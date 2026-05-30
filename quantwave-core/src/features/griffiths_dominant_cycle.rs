//! Griffiths Dominant Cycle feature extractor wrapper.
//!
//! Wraps the Griffiths spectral estimation dominant cycle detector.
//! Returns the estimated dominant cycle length — a high-value stationary cycle feature for ML.
//!
//! Source: quantwave-core/src/indicators/griffiths_dominant_cycle.rs (returns f64 dominant cycle estimate)

use crate::indicators::griffiths_dominant_cycle::GriffithsDominantCycle;
use crate::traits::Next;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GriffithsDominantCycleFeatures {
    pub dominant_cycle: f64,
}

#[derive(Debug, Clone)]
pub struct GriffithsDominantCycleFeatureExtractor {
    inner: GriffithsDominantCycle,
}

impl GriffithsDominantCycleFeatureExtractor {
    pub fn new(lower_bound: usize, upper_bound: usize, length: usize) -> Self {
        Self {
            inner: GriffithsDominantCycle::new(lower_bound, upper_bound, length),
        }
    }
}

impl Next<f64> for GriffithsDominantCycleFeatureExtractor {
    type Output = GriffithsDominantCycleFeatures;

    fn next(&mut self, input: f64) -> Self::Output {
        let dc = self.inner.next(input);
        GriffithsDominantCycleFeatures { dominant_cycle: dc }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_griffiths_wrapper_basic() {
        let mut ext = GriffithsDominantCycleFeatureExtractor::new(10, 40, 30);
        for i in 0..100 {
            let val = 100.0 + 5.0 * (i as f64 * 0.25).sin();
            let f = ext.next(val);
            if !f.dominant_cycle.is_nan() {
                assert!(f.dominant_cycle > 5.0 && f.dominant_cycle < 50.0);
            }
        }
    }
}
