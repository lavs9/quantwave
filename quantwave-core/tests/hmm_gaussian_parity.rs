//! Streaming vs batch parity and EM fitting behavior for generic Gaussian HMM.

use approx::assert_relative_eq;
use proptest::prelude::*;
use quantwave_core::regimes::gaussian_hmm::{
    fit_em, EmissionFamily, GaussianHmmFitConfig, GaussianHmmParams,
};
use quantwave_core::regimes::hmm::HMM;
use quantwave_core::traits::Next;

fn synthetic_two_state_series() -> Vec<f64> {
    vec![
        0.01, -0.008, 0.012, -0.015, 0.009, -0.011, 0.007, -0.013, 0.011, -0.009, 0.008,
        -0.014, 0.006, -0.01, 0.013, -0.012, 0.005, -0.007, 0.01, -0.016,
    ]
}

#[test]
fn streaming_forward_filter_matches_batch_forward_filter() {
    let params = GaussianHmmParams::new(
        vec![0.6, 0.4],
        vec![vec![0.92, 0.08], vec![0.15, 0.85]],
        vec![0.008, -0.012],
        vec![0.018, 0.028],
    )
    .expect("params");

    let x = synthetic_two_state_series();
    let batch = params.decode(&x).expect("decode");
    let mut stream = params.filter();
    for (t, &obs) in x.iter().enumerate() {
        let probs = stream.next(obs);
        for s in 0..params.n_states {
            assert_relative_eq!(probs[s], batch.forward_filter[s][t], epsilon = 1e-12);
        }
    }
}

#[test]
fn em_fit_improves_mllk_on_generic_series() {
    let x = synthetic_two_state_series();
    let init = GaussianHmmParams::new(
        vec![0.5, 0.5],
        vec![vec![0.8, 0.2], vec![0.2, 0.8]],
        vec![0.0, 0.0],
        vec![0.02, 0.02],
    )
    .expect("init");
    let init_mllk = init.mllk(&x).expect("init mllk");

    let fit = fit_em(
        &x,
        &GaussianHmmFitConfig {
            n_states: 2,
            max_iter: 80,
            tol: 1e-7,
            ..Default::default()
        },
    )
    .expect("fit");

    let fit_mllk = fit.params.mllk(&x).expect("fit mllk");
    assert!(fit_mllk <= init_mllk + 1e-6, "EM should not increase MLLK");
    assert!(fit.log_likelihood.is_finite());
    assert!(fit.aic.is_finite());
    assert!(fit.bic.is_finite());
    assert!(fit.bic >= fit.aic);
}

#[test]
fn bull_bear_preset_streaming_unchanged() {
    let mut hmm = HMM::bull_bear();
    let returns = [0.001, -0.002, 0.003, -0.004, 0.001];
    let labels: Vec<_> = returns
        .iter()
        .map(|&r| {
            use quantwave_core::regimes::MarketRegime;
            match hmm.next(r) {
                MarketRegime::Bull => 1u32,
                MarketRegime::Bear => 2,
                other => panic!("unexpected regime {other:?}"),
            }
        })
        .collect();
    // Regression lock: preset bull/bear labels for this fixed return path.
    assert_eq!(labels, vec![1, 1, 1, 1, 1]);
}

/// Synthetic leptokurtic log-return path: mostly small moves with occasional fat-tail spikes.
fn leptokurtic_return_series() -> Vec<f64> {
    vec![
        0.002, -0.003, 0.001, -0.002, 0.003, -0.001, 0.002, -0.004, 0.001, -0.002, 0.035,
        -0.041, 0.002, -0.003, 0.001, 0.038, -0.002, 0.001, -0.036, 0.003, -0.001, 0.002,
        -0.039, 0.004, -0.002, 0.001, -0.003, 0.042, -0.001, 0.002, -0.004, 0.001, -0.037,
        0.003, -0.002, 0.001, 0.040, -0.003, 0.002, -0.001,
    ]
}

#[test]
fn lambda_em_fit_runs_on_leptokurtic_series_without_regression() {
    let x = leptokurtic_return_series();

    let gaussian_fit = fit_em(
        &x,
        &GaussianHmmFitConfig {
            n_states: 2,
            max_iter: 80,
            tol: 1e-7,
            emission_family: EmissionFamily::Gaussian,
            fit_lambdas: false,
        },
    )
    .expect("gaussian fit");

    let lambda_fit = fit_em(
        &x,
        &GaussianHmmFitConfig {
            n_states: 2,
            max_iter: 80,
            tol: 1e-7,
            emission_family: EmissionFamily::Lambda,
            fit_lambdas: true,
        },
    )
    .expect("lambda fit");

    // λ-constrained EM should match or beat Gaussian on the same series (ldhmm λ≥1 nest Gaussian).
    assert!(
        lambda_fit.log_likelihood >= gaussian_fit.log_likelihood - 0.05,
        "lambda HMM should not materially worsen log-likelihood: gaussian={}, lambda={}",
        gaussian_fit.log_likelihood,
        lambda_fit.log_likelihood
    );
    assert!(
        lambda_fit.params.lambdas.iter().all(|&l| l >= 1.0 && l <= 5.0),
        "fitted λ must stay in ldhmm leptokurtic range [1, 5]: {:?}",
        lambda_fit.params.lambdas
    );
    assert!(lambda_fit.params.validate().is_ok());
}

#[test]
fn fixed_lambda_params_improve_mllk_vs_gaussian_on_gold_fixture() {
    let gaussian = GaussianHmmParams::new(
        vec![0.6, 0.4],
        vec![vec![0.92, 0.08], vec![0.15, 0.85]],
        vec![0.008, -0.012],
        vec![0.018, 0.028],
    )
    .expect("gaussian params");
    let lambda = GaussianHmmParams::new_with_lambdas(
        vec![0.6, 0.4],
        vec![vec![0.92, 0.08], vec![0.15, 0.85]],
        vec![0.008, -0.012],
        vec![0.018, 0.028],
        vec![1.0, 1.3],
    )
    .expect("lambda params");
    let obs = vec![
        0.005, -0.003, 0.011, -0.021, 0.007, -0.015, 0.002, -0.009, 0.004, -0.018,
    ];
    let gaussian_mllk = gaussian.mllk(&obs).expect("gaussian mllk");
    let lambda_mllk = lambda.mllk(&obs).expect("lambda mllk");
    assert!(
        lambda_mllk <= gaussian_mllk,
        "fixed λ=[1.0,1.3] should improve mllk: gaussian={gaussian_mllk}, lambda={lambda_mllk}"
    );
}

#[test]
fn em_fit_supports_three_state_generic_series() {
    let x: Vec<f64> = (0..30)
        .map(|i| {
            let phase = (i % 3) as f64;
            0.01 * (phase - 1.0) + 0.002 * ((i as f64) * 0.3).sin()
        })
        .collect();

    let fit = fit_em(
        &x,
        &GaussianHmmFitConfig {
            n_states: 3,
            max_iter: 60,
            tol: 1e-6,
            ..Default::default()
        },
    )
    .expect("3-state fit");

    assert_eq!(fit.params.n_states, 3);
    assert!(fit.params.validate().is_ok());
    assert!(fit.log_likelihood.is_finite());
}

proptest! {
    #[test]
    fn forward_filter_rows_sum_to_one(obs in proptest::collection::vec(-0.05f64..0.05, 8..40)) {
        let params = GaussianHmmParams::new(
            vec![0.55, 0.45],
            vec![vec![0.9, 0.1], vec![0.12, 0.88]],
            vec![0.005, -0.005],
            vec![0.015, 0.025],
        ).unwrap();
        let batch = params.decode(&obs).unwrap();
        for t in 0..obs.len() {
            let sum: f64 = (0..params.n_states)
                .map(|s| batch.forward_filter[s][t])
                .sum();
            prop_assert!((sum - 1.0).abs() < 1e-10);
        }
    }
}