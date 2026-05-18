//! Threshold Autoregressive (TAR / SETAR) Models
//! 
//! Source: Tong (1983) "Non-Linear Time Series: A Dynamical System Approach"
//! 
//! TAR models allow regime shifts to be triggered by an observable signal 
//! (e.g., volatility or price relative to a threshold).

use crate::traits::Next;
use crate::regimes::MarketRegime;
use serde::{Deserialize, Serialize};

/// A Threshold Autoregressive model with multiple thresholds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TAR {
    /// Sorted thresholds [t1, t2, ..., tn]
    pub thresholds: Vec<f64>,
}

impl TAR {
    /// Creates a new TAR model with a single threshold.
    pub fn new(threshold: f64) -> Self {
        Self { thresholds: vec![threshold] }
    }

    /// Creates a multi-threshold TAR model (SETAR).
    pub fn multi(thresholds: Vec<f64>) -> Self {
        let mut t = thresholds;
        t.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self { thresholds: t }
    }
}

impl Next<f64> for TAR {
    type Output = MarketRegime;

    fn next(&mut self, signal: f64) -> Self::Output {
        // Find which threshold bin the signal falls into
        match self.thresholds.binary_search_by(|t| t.partial_cmp(&signal).unwrap()) {
            Ok(idx) => MarketRegime::Cluster(idx as u8 + 1), // Exact match
            Err(idx) => {
                if idx == 0 {
                    MarketRegime::Steady // Below first threshold
                } else if idx >= self.thresholds.len() {
                    MarketRegime::Crisis // Above last threshold
                } else {
                    MarketRegime::Cluster(idx as u8)
                }
            }
        }
    }
}
