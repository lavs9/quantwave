//! Regime Detection and Market State Tools
//!
//! This module provides algorithms for identifying market regimes, such as volatility clustering,
//! hidden Markov models (HMM), and changepoint detection.

pub mod volatility_clustering;
pub mod hmm;
pub mod gmm;
pub mod pelt;
pub mod analytics;
pub mod ms_garch;
pub mod ensemble;
pub mod india;
pub mod tar;
pub mod hsmm;
pub mod hmm_gas;
pub mod multi_asset;

use serde::{Deserialize, Serialize};

/// Represents common market regime states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketRegime {
    /// A period of low volatility and generally upward price movement.
    Bull,
    /// A period of high volatility or downward price movement.
    Bear,
    /// A transitional or unstable period.
    Crisis,
    /// A steady state with normal characteristics.
    Steady,
    /// Custom state for user-defined clusters.
    Cluster(u8),
}

impl Default for MarketRegime {
    fn default() -> Self {
        Self::Steady
    }
}
