//! Gold-standard tests for generic lambda (ecld) HMM.
#![allow(clippy::expect_used, clippy::unwrap_used, dead_code)]

use approx::assert_relative_eq;
use quantwave_core::regimes::gaussian_hmm::GaussianHmmParams;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct HmmLambdaGoldCase {
    n_states: usize,
    delta: Vec<f64>,
    gamma: Vec<Vec<f64>>,
    means: Vec<f64>,
    stds: Vec<f64>,
    lambdas: Vec<f64>,
    observations: Vec<f64>,
    expected: HmmLambdaGoldExpected,
}

#[derive(Debug, Deserialize)]
struct HmmLambdaGoldExpected {
    mllk: f64,
    smooth_probs: Vec<Vec<f64>>,
    viterbi_path: Vec<usize>,
}

fn load_case(name: &str) -> HmmLambdaGoldCase {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/gold_standard")
        .join(format!("{name}.json"));
    let content = fs::read_to_string(path).expect("read gold standard");
    serde_json::from_str(&content).expect("parse gold standard")
}

#[test]
fn hmm_lambda_mllk_matches_gold_standard() {
    let case = load_case("hmm_lambda_2state");
    let params = GaussianHmmParams::new_with_lambdas(
        case.delta,
        case.gamma,
        case.means,
        case.stds,
        case.lambdas,
    )
    .expect("valid params");

    let mllk = params.mllk(&case.observations).expect("mllk");
    assert_relative_eq!(mllk, case.expected.mllk, epsilon = 1e-9);
}

#[test]
fn hmm_lambda_smooth_probs_match_gold_standard() {
    let case = load_case("hmm_lambda_2state");
    let params = GaussianHmmParams::new_with_lambdas(
        case.delta,
        case.gamma,
        case.means,
        case.stds,
        case.lambdas,
    )
    .expect("valid params");

    let decode = params.decode(&case.observations).expect("decode");
    for (state, expected_row) in case.expected.smooth_probs.iter().enumerate() {
        for (t, &exp) in expected_row.iter().enumerate() {
            assert_relative_eq!(decode.smooth_probs[state][t], exp, epsilon = 1e-9);
        }
    }
}

#[test]
fn hmm_lambda_viterbi_matches_gold_standard() {
    let case = load_case("hmm_lambda_2state");
    let params = GaussianHmmParams::new_with_lambdas(
        case.delta,
        case.gamma,
        case.means,
        case.stds,
        case.lambdas,
    )
    .expect("valid params");

    let decode = params.decode(&case.observations).expect("decode");
    assert_eq!(decode.viterbi_path, case.expected.viterbi_path);
}

#[test]
fn lambda_lambdas_one_matches_gaussian_gold_mllk() {
    let gaussian = fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/gold_standard/hmm_gaussian_2state.json"),
    )
    .expect("read gaussian gold");
    let lambda_case = load_case("hmm_lambda_2state");

    #[derive(Deserialize)]
    struct GaussianExpected {
        expected: GaussianMllk,
    }
    #[derive(Deserialize)]
    struct GaussianMllk {
        mllk: f64,
    }
    let gaussian_case: GaussianExpected = serde_json::from_str(&gaussian).expect("parse");

    let params = GaussianHmmParams::new_with_lambdas(
        lambda_case.delta,
        lambda_case.gamma,
        lambda_case.means,
        lambda_case.stds,
        vec![1.0, 1.0],
    )
    .expect("params");

    let mllk = params.mllk(&lambda_case.observations).expect("mllk");
    assert_relative_eq!(mllk, gaussian_case.expected.mllk, epsilon = 1e-9);
}