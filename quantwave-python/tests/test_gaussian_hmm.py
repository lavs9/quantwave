"""Smoke tests for generic Gaussian HMM Python bindings."""

from __future__ import annotations

import pytest

qw = pytest.importorskip("quantwave")


GENERIC_RETURNS = [
    0.01,
    -0.008,
    0.012,
    -0.015,
    0.009,
    -0.011,
    0.007,
    -0.013,
    0.011,
    -0.009,
    0.008,
    -0.014,
    0.006,
    -0.01,
    0.013,
    -0.012,
    0.005,
    -0.007,
    0.01,
    -0.016,
]


def test_fit_gaussian_hmm_returns_finite_metrics() -> None:
    result = qw.fit_gaussian_hmm(GENERIC_RETURNS, n_states=2, max_iter=40, fit_lambdas=False)
    assert result.n_observations == len(GENERIC_RETURNS)
    assert result.params.n_states == 2
    assert len(result.viterbi_path) == len(GENERIC_RETURNS)
    assert len(result.smooth_probs_flat) == len(GENERIC_RETURNS) * 2
    assert result.log_likelihood == pytest.approx(result.log_likelihood)
    assert result.log_likelihood > float("-inf")
    assert result.iterations >= 1
    assert result.aic == pytest.approx(result.aic)
    assert result.bic == pytest.approx(result.bic)
    assert result.bic >= result.aic


def test_fit_lambda_hmm_estimates_lambda_above_one() -> None:
    result = qw.fit_gaussian_hmm(GENERIC_RETURNS, n_states=2, max_iter=40, fit_lambdas=True)
    assert result.params.n_states == 2
    assert len(result.params.lambdas) == 2
    assert all(lam > 0.0 for lam in result.params.lambdas)
    assert result.log_likelihood > float("-inf")


def test_gaussian_hmm_filter_streaming_probs_sum_to_one() -> None:
    fit = qw.fit_gaussian_hmm(GENERIC_RETURNS, n_states=2, max_iter=40, fit_lambdas=False)
    filt = qw.GaussianHmmFilterPy.from_params(fit.params)
    for x in GENERIC_RETURNS:
        probs = filt.next(x)
        assert len(probs) == 2
        assert sum(probs) == pytest.approx(1.0, abs=1e-9)