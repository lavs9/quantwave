"""Smoke tests for HMM forecast/diagnostics Python bindings."""

from __future__ import annotations

import pytest

qw = pytest.importorskip("quantwave")

GENERIC_RETURNS = [
    0.005,
    -0.003,
    0.011,
    -0.021,
    0.007,
    -0.015,
    0.002,
    -0.009,
    0.004,
    -0.018,
]


def test_gaussian_hmm_diagnostics_matches_gold_fixture() -> None:
    params = qw.GaussianHmmParamsPy(
        n_states=2,
        delta=[0.6, 0.4],
        gamma_flat=[0.92, 0.08, 0.15, 0.85],
        means=[0.008, -0.012],
        stds=[0.018, 0.028],
        lambdas=[1.0, 1.3],
    )
    diag = qw.gaussian_hmm_diagnostics(params, GENERIC_RETURNS)
    assert len(diag.pseudo_residuals) == len(GENERIC_RETURNS)
    assert len(diag.decode_weighted_means) == len(GENERIC_RETURNS)
    assert diag.forecast_vol_h1 == pytest.approx(0.022963708598468312, rel=1e-6)
    assert sum(diag.forecast_state_h1) == pytest.approx(1.0, abs=1e-9)


def test_gaussian_hmm_forecast_state_and_vol() -> None:
    params = qw.GaussianHmmParamsPy(
        n_states=2,
        delta=[0.6, 0.4],
        gamma_flat=[0.92, 0.08, 0.15, 0.85],
        means=[0.008, -0.012],
        stds=[0.018, 0.028],
        lambdas=[1.0, 1.3],
    )
    last_probs = [0.517120403014209, 0.48287959698579097]
    state_h1 = qw.gaussian_hmm_forecast_state(params, last_probs, horizon=1)
    vol_h1 = qw.gaussian_hmm_forecast_vol(params, last_probs, horizon=1)
    assert len(state_h1) == 2
    assert sum(state_h1) == pytest.approx(1.0, abs=1e-9)
    assert vol_h1 == pytest.approx(0.022963708598468312, rel=1e-6)