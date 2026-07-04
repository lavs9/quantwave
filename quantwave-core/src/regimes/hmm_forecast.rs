//! HMM forecasting and diagnostics (ldhmm parity — slice 3).
//!
//! Implements `forecast_state`, `forecast_volatility`, `forecast_prob`, `pseudo_residuals`,
//! and decode statistics on top of fitted Gaussian/lambda HMM parameters.
//!
//! Sources: references/ldhmm/ldhmm-cran-reference.pdf; references/ldhmm/ssrn-2979516.pdf

use super::ecld::{ecld_cdf, ecld_pdf, ecld_variance};
use super::gaussian_hmm::{GaussianHmmDecode, GaussianHmmError, GaussianHmmParams};
use nalgebra::DMatrix;
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Normal};

/// Per-bar weighted decode statistics (ldhmm `decode_stats_history`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HmmDecodeStatsRow {
    pub weighted_mean: f64,
    pub weighted_vol: f64,
    pub weighted_lambda: f64,
}

/// Per-state summary statistics from weighted observations (ldhmm `calc_stats_from_obs`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HmmStateObsStats {
    pub state: usize,
    pub weight_sum: f64,
    pub mean: f64,
    pub vol: f64,
    pub lambda: f64,
}

/// h-step ahead state probability forecast from a current distribution (ldhmm `forecast_state`).
///
/// Returns `π_{t+h|t} = π_t · Γ^h` as a row vector over states.
pub fn forecast_state(
    params: &GaussianHmmParams,
    current_probs: &[f64],
    horizon: usize,
) -> Result<Vec<f64>, GaussianHmmError> {
    params.validate()?;
    let m = params.n_states;
    if current_probs.len() != m {
        return Err(GaussianHmmError::InvalidParams(
            "current_probs length must match n_states".into(),
        ));
    }
    if horizon == 0 {
        return Ok(current_probs.to_vec());
    }
    let gamma_h = transition_power(&params.gamma, horizon)?;
    let pi = DMatrix::from_row_slice(1, m, current_probs);
    let forecast = &pi * gamma_h;
    Ok(forecast.as_slice().to_vec())
}

/// Mixture volatility forecast h steps ahead (SSRN 2979516 / ldhmm `forecast_volatility`).
///
/// Returns `sqrt(Σ_s π_{t+h|t}(s) · (var_s + (μ_s − μ̄)²))` using lambda-aware emission variances.
pub fn forecast_volatility(
    params: &GaussianHmmParams,
    current_probs: &[f64],
    horizon: usize,
) -> Result<f64, GaussianHmmError> {
    let probs = forecast_state(params, current_probs, horizon)?;
    Ok(mixture_vol(&probs, &params.means, params))
}

/// Mixture mean forecast h steps ahead.
pub fn forecast_observation_mean(
    params: &GaussianHmmParams,
    current_probs: &[f64],
    horizon: usize,
) -> Result<f64, GaussianHmmError> {
    let probs = forecast_state(params, current_probs, horizon)?;
    Ok(mixture_mean(&probs, &params.means))
}

/// Mixture observation density at `x` for horizon `h` (ldhmm `forecast_prob` point evaluation).
pub fn forecast_observation_pdf(
    params: &GaussianHmmParams,
    current_probs: &[f64],
    horizon: usize,
    x: f64,
) -> Result<f64, GaussianHmmError> {
    let probs = forecast_state(params, current_probs, horizon)?;
    Ok(mixture_pdf(x, &probs, params))
}

/// Probability integral transform residuals (ldhmm `pseudo_residuals`).
///
/// Uses the filtered mixture CDF at each time: `u_t = F_t(x_t)`, then `Φ^{-1}(u_t)`.
pub fn pseudo_residuals(
    params: &GaussianHmmParams,
    forward_filter: &[Vec<f64>],
    observations: &[f64],
) -> Result<Vec<f64>, GaussianHmmError> {
    params.validate()?;
    let m = params.n_states;
    let n = observations.len();
    if forward_filter.len() != m {
        return Err(GaussianHmmError::InvalidParams(
            "forward_filter rows must match n_states".into(),
        ));
    }
    if forward_filter.first().map(|r| r.len()).unwrap_or(0) != n {
        return Err(GaussianHmmError::InvalidParams(
            "forward_filter length must match observations".into(),
        ));
    }
    let normal = Normal::new(0.0, 1.0).expect("standard normal");
    let mut out = Vec::with_capacity(n);
    for t in 0..n {
        let probs: Vec<f64> = (0..m).map(|s| forward_filter[s][t]).collect();
        let u = mixture_cdf(observations[t], &probs, params).clamp(1e-12, 1.0 - 1e-12);
        out.push(normal.inverse_cdf(u));
    }
    Ok(out)
}

/// Weighted mean/vol/lambda per bar from state probabilities (ldhmm `decode_stats_history`).
pub fn decode_stats_history(
    params: &GaussianHmmParams,
    state_probs: &[Vec<f64>],
) -> Result<Vec<HmmDecodeStatsRow>, GaussianHmmError> {
    params.validate()?;
    let m = params.n_states;
    if state_probs.len() != m {
        return Err(GaussianHmmError::InvalidParams(
            "state_probs rows must match n_states".into(),
        ));
    }
    let n = state_probs[0].len();
    if state_probs.iter().any(|row| row.len() != n) {
        return Err(GaussianHmmError::InvalidParams(
            "state_probs rows must have equal length".into(),
        ));
    }
    let mut rows = Vec::with_capacity(n);
    for t in 0..n {
        let probs: Vec<f64> = (0..m).map(|s| state_probs[s][t]).collect();
        let mean = mixture_mean(&probs, &params.means);
        let vol = mixture_vol(&probs, &params.means, params);
        let lambda = mixture_lambda(&probs, params);
        rows.push(HmmDecodeStatsRow {
            weighted_mean: mean,
            weighted_vol: vol,
            weighted_lambda: lambda,
        });
    }
    Ok(rows)
}

/// Per-state weighted summary stats from observations and state probabilities.
pub fn calc_stats_from_obs(
    params: &GaussianHmmParams,
    observations: &[f64],
    state_probs: &[Vec<f64>],
) -> Result<Vec<HmmStateObsStats>, GaussianHmmError> {
    params.validate()?;
    let m = params.n_states;
    let n = observations.len();
    if state_probs.len() != m || state_probs.first().map(|r| r.len()).unwrap_or(0) != n {
        return Err(GaussianHmmError::InvalidParams(
            "state_probs shape must be [n_states][n_obs]".into(),
        ));
    }
    let mut out = Vec::with_capacity(m);
    for s in 0..m {
        let mut w_sum = 0.0;
        let mut mean_acc = 0.0;
        for t in 0..n {
            let w = state_probs[s][t];
            w_sum += w;
            mean_acc += w * observations[t];
        }
        let mean = if w_sum > 0.0 { mean_acc / w_sum } else { 0.0 };
        let mut var_acc = 0.0;
        for t in 0..n {
            let w = state_probs[s][t];
            var_acc += w * (observations[t] - mean).powi(2);
        }
        let vol = if w_sum > 0.0 {
            (var_acc / w_sum).sqrt()
        } else {
            0.0
        };
        out.push(HmmStateObsStats {
            state: s,
            weight_sum: w_sum,
            mean,
            vol,
            lambda: params.lambdas.get(s).copied().unwrap_or(1.0),
        });
    }
    Ok(out)
}

impl GaussianHmmParams {
    /// Forecast state probabilities `h` steps ahead from `current_probs`.
    pub fn forecast_state(
        &self,
        current_probs: &[f64],
        horizon: usize,
    ) -> Result<Vec<f64>, GaussianHmmError> {
        forecast_state(self, current_probs, horizon)
    }

    /// Forecast mixture volatility `h` steps ahead.
    pub fn forecast_volatility(
        &self,
        current_probs: &[f64],
        horizon: usize,
    ) -> Result<f64, GaussianHmmError> {
        forecast_volatility(self, current_probs, horizon)
    }

    /// Full forecast/diagnostic bundle after decode.
    pub fn diagnostics(
        &self,
        decode: &GaussianHmmDecode,
        observations: &[f64],
    ) -> Result<HmmDiagnostics, GaussianHmmError> {
        if observations.len() != decode.forward_filter[0].len() {
            return Err(GaussianHmmError::InvalidParams(
                "observations length must match decode".into(),
            ));
        }
        let m = self.n_states;
        let last_probs: Vec<f64> = (0..m).map(|s| decode.forward_filter[s].last().copied().unwrap_or(0.0)).collect();
        Ok(HmmDiagnostics {
            pseudo_residuals: pseudo_residuals(self, &decode.forward_filter, observations)?,
            decode_stats: decode_stats_history(self, &decode.smooth_probs)?,
            state_obs_stats: calc_stats_from_obs(self, observations, &decode.smooth_probs)?,
            forecast_state_h1: forecast_state(self, &last_probs, 1)?,
            forecast_state_h2: forecast_state(self, &last_probs, 2)?,
            forecast_vol_h1: forecast_volatility(self, &last_probs, 1)?,
            forecast_vol_h2: forecast_volatility(self, &last_probs, 2)?,
            forecast_mean_h1: forecast_observation_mean(self, &last_probs, 1)?,
        })
    }
}

/// Batch diagnostics output.
#[derive(Debug, Clone, PartialEq)]
pub struct HmmDiagnostics {
    pub pseudo_residuals: Vec<f64>,
    pub decode_stats: Vec<HmmDecodeStatsRow>,
    pub state_obs_stats: Vec<HmmStateObsStats>,
    pub forecast_state_h1: Vec<f64>,
    pub forecast_state_h2: Vec<f64>,
    pub forecast_vol_h1: f64,
    pub forecast_vol_h2: f64,
    pub forecast_mean_h1: f64,
}

fn transition_power(gamma: &[Vec<f64>], horizon: usize) -> Result<DMatrix<f64>, GaussianHmmError> {
    let m = gamma.len();
    if m == 0 {
        return Err(GaussianHmmError::InvalidParams("empty transition matrix".into()));
    }
    let flat: Vec<f64> = gamma.iter().flat_map(|row| row.iter().copied()).collect();
    if flat.len() != m * m {
        return Err(GaussianHmmError::InvalidParams(
            "transition matrix must be square".into(),
        ));
    }
    Ok(DMatrix::from_row_slice(m, m, &flat).pow(horizon as u32))
}

fn mixture_mean(probs: &[f64], means: &[f64]) -> f64 {
    probs
        .iter()
        .zip(means.iter())
        .map(|(&p, &mu)| p * mu)
        .sum()
}

fn mixture_vol(probs: &[f64], means: &[f64], params: &GaussianHmmParams) -> f64 {
    let mean = mixture_mean(probs, means);
    let var: f64 = probs
        .iter()
        .enumerate()
        .map(|(s, &p)| {
            let lam = params.lambdas.get(s).copied().unwrap_or(1.0);
            let var_s = ecld_variance(params.stds[s], lam);
            p * (var_s + (params.means[s] - mean).powi(2))
        })
        .sum();
    var.max(0.0).sqrt()
}

fn mixture_lambda(probs: &[f64], params: &GaussianHmmParams) -> f64 {
    probs
        .iter()
        .enumerate()
        .map(|(s, &p)| p * params.lambdas.get(s).copied().unwrap_or(1.0))
        .sum()
}

fn mixture_pdf(x: f64, probs: &[f64], params: &GaussianHmmParams) -> f64 {
    probs
        .iter()
        .enumerate()
        .map(|(s, &p)| {
            let lam = params.lambdas.get(s).copied().unwrap_or(1.0);
            p * ecld_pdf(x, params.means[s], params.stds[s], lam)
        })
        .sum()
}

fn mixture_cdf(x: f64, probs: &[f64], params: &GaussianHmmParams) -> f64 {
    probs
        .iter()
        .enumerate()
        .map(|(s, &p)| {
            let lam = params.lambdas.get(s).copied().unwrap_or(1.0);
            p * ecld_cdf(x, params.means[s], params.stds[s], lam)
        })
        .sum()
}

// --- IndicatorMetadata (quantwave-i9dn) ---

use crate::indicators::metadata::{IndicatorMetadata, ParamDef};

pub const HMM_FORECAST_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "hmm_forecast",
    description:
        "HMM forecasting and diagnostics: state/vol/probability forecasts, pseudo-residuals, decode stats.",
    usage: "After fitting/decoding a Gaussian or lambda HMM, forecast regimes and volatility h steps ahead, \
             evaluate mixture predictive densities, and extract pseudo-residuals for model checking.",
    keywords: &[
        "regime",
        "hmm",
        "forecast",
        "volatility",
        "pseudo_residuals",
        "ldhmm",
        "diagnostics",
    ],
    ehlers_summary: "ldhmm-style post-fit analytics on homogeneous HMMs: π_{t+h|t}=π_t·Γ^h state forecasts, \
                     mixture volatility per SSRN 2979516, filtered pseudo-residuals, and decode_stats_history.",
    params: &[
        ParamDef {
            name: "horizon",
            default: "1",
            description: "Forecast horizon h (bars ahead).",
        },
    ],
    formula_source: "references/ldhmm/ldhmm-cran-reference.pdf; references/ldhmm/ssrn-2979516.pdf",
    formula_latex: "",
    gold_standard_file: "hmm_lambda_2state.json",
    category: "Regime",
};