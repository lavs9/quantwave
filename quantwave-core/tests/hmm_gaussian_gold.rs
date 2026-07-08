//! Gold-standard tests for generic Gaussian HMM (not instrument-specific).
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use approx::assert_relative_eq;
use quantwave_core::regimes::gaussian_hmm::GaussianHmmParams;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct HmmGoldCase {
    n_states: usize,
    delta: Vec<f64>,
    gamma: Vec<Vec<f64>>,
    means: Vec<f64>,
    stds: Vec<f64>,
    observations: Vec<f64>,
    expected: HmmGoldExpected,
}

#[derive(Debug, Deserialize)]
struct HmmGoldExpected {
    mllk: f64,
    smooth_probs: Vec<Vec<f64>>,
    viterbi_path: Vec<usize>,
}

fn load_case(name: &str) -> HmmGoldCase {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/gold_standard")
        .join(format!("{name}.json"));
    let content = fs::read_to_string(path).expect("read gold standard");
    serde_json::from_str(&content).expect("parse gold standard")
}

#[test]
fn hmm_gaussian_mllk_matches_gold_standard() {
    let case = load_case("hmm_gaussian_2state");
    let params = GaussianHmmParams::new(
        case.delta,
        case.gamma,
        case.means,
        case.stds,
    )
    .expect("valid params");

    let mllk = params.mllk(&case.observations).expect("mllk");
    assert_relative_eq!(mllk, case.expected.mllk, epsilon = 1e-9);
}

#[test]
fn hmm_gaussian_smooth_probs_match_gold_standard() {
    let case = load_case("hmm_gaussian_2state");
    let params = GaussianHmmParams::new(
        case.delta,
        case.gamma,
        case.means,
        case.stds,
    )
    .expect("valid params");

    let decode = params.decode(&case.observations).expect("decode");
    assert_eq!(decode.smooth_probs.len(), case.n_states);
    for (state, expected_row) in case.expected.smooth_probs.iter().enumerate() {
        for (t, &exp) in expected_row.iter().enumerate() {
            assert_relative_eq!(decode.smooth_probs[state][t], exp, epsilon = 1e-9);
        }
    }
}

#[test]
fn hmm_gaussian_viterbi_matches_gold_standard() {
    let case = load_case("hmm_gaussian_2state");
    let params = GaussianHmmParams::new(
        case.delta,
        case.gamma,
        case.means,
        case.stds,
    )
    .expect("valid params");

    let decode = params.decode(&case.observations).expect("decode");
    assert_eq!(decode.viterbi_path, case.expected.viterbi_path);
}