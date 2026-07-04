//! Regime Detection and Market State Tools
//!
//! This module provides algorithms for identifying market regimes, such as volatility clustering,
//! hidden Markov models (HMM), and changepoint detection.

pub mod volatility_clustering;
pub mod hmm;
pub mod ecld;
pub mod gaussian_hmm;
pub mod hmm_forecast;

pub use ecld::{ecld_cdf, ecld_log_pdf, ecld_pdf, ecld_variance, natural_to_work, work_to_natural};
pub use gaussian_hmm::{
    fit_em, EmissionFamily, GaussianHmmDecode, GaussianHmmError, GaussianHmmFilter,
    GaussianHmmFitConfig, GaussianHmmFitResult, GaussianHmmParams,
};
pub use hmm_forecast::{
    calc_stats_from_obs, decode_stats_history, forecast_observation_mean, forecast_observation_pdf,
    forecast_state, forecast_volatility, pseudo_residuals, HmmDecodeStatsRow, HmmDiagnostics,
    HmmStateObsStats,
};
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
