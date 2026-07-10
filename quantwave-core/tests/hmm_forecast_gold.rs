//! Gold-standard tests for HMM forecasting and diagnostics (slice 3).
#![allow(clippy::expect_used, clippy::unwrap_used)]

use approx::assert_relative_eq;
use quantwave_core::regimes::gaussian_hmm::GaussianHmmParams;
use quantwave_core::regimes::hmm_forecast::{
    calc_stats_from_obs, decode_stats_history, forecast_observation_mean, forecast_state,
    forecast_volatility, pseudo_residuals,
};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct ForecastGoldCase {
    delta: Vec<f64>,
    gamma: Vec<Vec<f64>>,
    means: Vec<f64>,
    stds: Vec<f64>,
    lambdas: Vec<f64>,
    observations: Vec<f64>,
    expected: ForecastGoldExpected,
}

#[derive(Debug, Deserialize)]
struct ForecastGoldExpected {
    last_forward_filter: Vec<f64>,
    forecast_state_h1: Vec<f64>,
    forecast_state_h2: Vec<f64>,
    forecast_vol_h1: f64,
    forecast_vol_h2: f64,
    forecast_mean_h1: f64,
    pseudo_residuals: Vec<f64>,
    decode_stats_first: DecodeStatsGold,
    decode_stats_last: DecodeStatsGold,
}

#[derive(Debug, Deserialize)]
struct DecodeStatsGold {
    weighted_mean: f64,
    weighted_vol: f64,
    weighted_lambda: f64,
}

fn load_case() -> ForecastGoldCase {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/gold_standard/hmm_lambda_2state.json");
    let content = fs::read_to_string(path).expect("read gold standard");
    serde_json::from_str(&content).expect("parse gold standard")
}

fn params_from(case: &ForecastGoldCase) -> GaussianHmmParams {
    GaussianHmmParams::new_with_lambdas(
        case.delta.clone(),
        case.gamma.clone(),
        case.means.clone(),
        case.stds.clone(),
        case.lambdas.clone(),
    )
    .expect("valid params")
}

#[test]
fn forecast_state_matches_gold_standard() {
    let case = load_case();
    let params = params_from(&case);
    let h1 = forecast_state(&params, &case.expected.last_forward_filter, 1).expect("h1");
    let h2 = forecast_state(&params, &case.expected.last_forward_filter, 2).expect("h2");
    for (i, &exp) in case.expected.forecast_state_h1.iter().enumerate() {
        assert_relative_eq!(h1[i], exp, epsilon = 1e-9);
    }
    for (i, &exp) in case.expected.forecast_state_h2.iter().enumerate() {
        assert_relative_eq!(h2[i], exp, epsilon = 1e-9);
    }
}

#[test]
fn forecast_volatility_matches_gold_standard() {
    let case = load_case();
    let params = params_from(&case);
    let last = &case.expected.last_forward_filter;
    let vol1 = forecast_volatility(&params, last, 1).expect("vol1");
    let vol2 = forecast_volatility(&params, last, 2).expect("vol2");
    let mean1 = forecast_observation_mean(&params, last, 1).expect("mean1");
    assert_relative_eq!(vol1, case.expected.forecast_vol_h1, epsilon = 1e-9);
    assert_relative_eq!(vol2, case.expected.forecast_vol_h2, epsilon = 1e-9);
    assert_relative_eq!(mean1, case.expected.forecast_mean_h1, epsilon = 1e-9);
}

#[test]
fn pseudo_residuals_match_gold_standard() {
    let case = load_case();
    let params = params_from(&case);
    let decode = params.decode(&case.observations).expect("decode");
    let residuals =
        pseudo_residuals(&params, &decode.forward_filter, &case.observations).expect("residuals");
    assert_eq!(residuals.len(), case.expected.pseudo_residuals.len());
    for (got, exp) in residuals.iter().zip(case.expected.pseudo_residuals.iter()) {
        assert_relative_eq!(got, exp, epsilon = 1e-6);
    }
}

#[test]
fn decode_stats_history_matches_gold_standard() {
    let case = load_case();
    let params = params_from(&case);
    let decode = params.decode(&case.observations).expect("decode");
    let stats = decode_stats_history(&params, &decode.smooth_probs).expect("stats");
    let first = &stats[0];
    let last = stats.last().expect("last row");
    assert_relative_eq!(
        first.weighted_mean,
        case.expected.decode_stats_first.weighted_mean,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        first.weighted_vol,
        case.expected.decode_stats_first.weighted_vol,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        first.weighted_lambda,
        case.expected.decode_stats_first.weighted_lambda,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        last.weighted_mean,
        case.expected.decode_stats_last.weighted_mean,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        last.weighted_vol,
        case.expected.decode_stats_last.weighted_vol,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        last.weighted_lambda,
        case.expected.decode_stats_last.weighted_lambda,
        epsilon = 1e-9
    );
}

#[test]
fn calc_stats_from_obs_returns_per_state_rows() {
    let case = load_case();
    let params = params_from(&case);
    let decode = params.decode(&case.observations).expect("decode");
    let obs_stats =
        calc_stats_from_obs(&params, &case.observations, &decode.smooth_probs).expect("obs stats");
    assert_eq!(obs_stats.len(), 2);
    assert!(obs_stats.iter().all(|s| s.weight_sum > 0.0));
    assert!(obs_stats.iter().all(|s| s.vol > 0.0));
}

#[test]
fn diagnostics_bundle_matches_forecast_gold() {
    let case = load_case();
    let params = params_from(&case);
    let decode = params.decode(&case.observations).expect("decode");
    let diag = params
        .diagnostics(&decode, &case.observations)
        .expect("diag");
    assert_relative_eq!(
        diag.forecast_vol_h1,
        case.expected.forecast_vol_h1,
        epsilon = 1e-9
    );
    assert_eq!(diag.pseudo_residuals.len(), case.observations.len());
    assert_eq!(diag.decode_stats.len(), case.observations.len());
    assert_eq!(diag.state_obs_stats.len(), 2);
}
