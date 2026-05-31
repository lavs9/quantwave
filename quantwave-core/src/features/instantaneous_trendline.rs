//! Instantaneous Trendline feature extractor wrapper.
//!
//! Wraps the zero-lag trendline. For richer ML use we expose the trend value
//! + a simple derived "trend strength" (placeholder for phase/power in future).
//!
//! Source: quantwave-core/src/indicators/instantaneous_trendline.rs (returns f64 trend)

use crate::indicators::instantaneous_trendline::InstantaneousTrendline;
use crate::traits::Next;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InstantaneousTrendlineFeatures {
    pub trend: f64,
    /// Placeholder for future phase/power derived features (0.0 for now)
    pub strength: f64,
}

#[derive(Debug, Clone)]
pub struct InstantaneousTrendlineFeatureExtractor {
    inner: InstantaneousTrendline,
}

impl InstantaneousTrendlineFeatureExtractor {
    pub fn new() -> Self {
        Self {
            inner: InstantaneousTrendline::new(),
        }
    }
}

impl Next<f64> for InstantaneousTrendlineFeatureExtractor {
    type Output = InstantaneousTrendlineFeatures;

    fn next(&mut self, input: f64) -> Self::Output {
        let trend = self.inner.next(input);
        // For MVP, strength is 0.0. Real phase/power can be added later from the indicator internals if exposed.
        InstantaneousTrendlineFeatures {
            trend,
            strength: 0.0,
        }
    }
}
