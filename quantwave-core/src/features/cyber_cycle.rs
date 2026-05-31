//! Cyber Cycle feature extractor wrapper.
//!
//! Wraps the existing `CyberCycle` to expose both the cycle value and trigger line
//! plus a few derived features useful for ML (momentum, zero-cross signal, etc.).
//!
//! Primary source: quantwave-core/src/indicators/cyber_cycle.rs:35
//! (returns (CyberCycle, Trigger) per Ehlers "Cybernetic Analysis for Stocks and Futures")

use crate::indicators::cyber_cycle::CyberCycle;
use crate::traits::Next;

/// Rich multi-dimensional output for Cyber Cycle features.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CyberCycleFeatures {
    pub cycle: f64,
    pub trigger: f64,
    /// Simple momentum of the cycle (cycle - previous cycle)
    pub cycle_momentum: f64,
    /// Sign of (cycle - trigger) as a basic oscillator signal
    pub trigger_signal: f64,
}

#[derive(Debug, Clone)]
pub struct CyberCycleFeatureExtractor {
    inner: CyberCycle,
    prev_cycle: f64,
}

impl CyberCycleFeatureExtractor {
    pub fn new(length: usize) -> Self {
        Self {
            inner: CyberCycle::new(length),
            prev_cycle: 0.0,
        }
    }
}

impl Next<f64> for CyberCycleFeatureExtractor {
    type Output = CyberCycleFeatures;

    fn next(&mut self, input: f64) -> Self::Output {
        let (cycle, trigger) = self.inner.next(input);

        let momentum = if self.prev_cycle == 0.0 {
            0.0
        } else {
            cycle - self.prev_cycle
        };
        let trigger_signal = (cycle - trigger).signum();

        self.prev_cycle = cycle;

        CyberCycleFeatures {
            cycle,
            trigger,
            cycle_momentum: momentum,
            trigger_signal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_cyber_cycle_features_basic() {
        let mut extractor = CyberCycleFeatureExtractor::new(14);

        // Feed some oscillatory data
        for i in 0..50 {
            let val = 100.0 + 5.0 * (i as f64 * 0.3).sin();
            let f = extractor.next(val);
            if !f.cycle.is_nan() {
                assert!(f.cycle.abs() < 20.0); // sanity
            }
        }
    }
}
