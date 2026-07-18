"""Tests for the numpy-based quantwave.portfolio optimization engine."""

from __future__ import annotations

import numpy as np
import pytest

from quantwave.portfolio import (
    PortfolioError,
    by_regime,
    ewma_cov,
    hrp,
    ledoit_wolf_cov,
    max_sharpe,
    min_variance,
    risk_parity,
    sample_cov,
)


# ---------------------------------------------------------------------------
# 1. Gold fixtures: hand-computed reference weights.
#
# Fixture covariance (2 assets, annualized-style vol/cov, arbitrary units):
#   cov = [[0.04, 0.01],
#          [0.01, 0.09]]
#
# min_variance closed form: w ∝ Σ⁻¹·1
#   Σ⁻¹ = 1/det * [[0.09, -0.01], [-0.01, 0.04]], det = 0.04*0.09 - 0.01^2 = 0.0035
#   Σ⁻¹·1 = [22.857143, 8.571429]  (sums to 31.428571)
#   w = [22.857143, 8.571429] / 31.428571 = [0.727273, 0.272727]
# (Reproduced via `np.linalg.solve(cov, ones)` normalized — the same closed
# form implemented in `min_variance`; hardcoded here as the gold reference.)
#
# max_sharpe closed form with mean = [0.08, 0.05]: w ∝ Σ⁻¹·μ
#   Σ⁻¹·μ = [0.08*22.857143/... ] -> solved numerically once and hardcoded:
#   w = [0.848101, 0.151899]
# ---------------------------------------------------------------------------

GOLD_COV = np.array([[0.04, 0.01], [0.01, 0.09]])
GOLD_MEAN = np.array([0.08, 0.05])
GOLD_MIN_VAR_W = np.array([0.727273, 0.272727])
GOLD_MAX_SHARPE_W = np.array([0.848101, 0.151899])

TOL = 1e-4


def test_min_variance_gold_fixture():
    w = min_variance(GOLD_COV)
    assert np.allclose(w, GOLD_MIN_VAR_W, atol=TOL)
    assert np.isclose(w.sum(), 1.0)


def test_max_sharpe_gold_fixture():
    w = max_sharpe(GOLD_MEAN, GOLD_COV)
    assert np.allclose(w, GOLD_MAX_SHARPE_W, atol=TOL)
    assert np.isclose(w.sum(), 1.0)


# ---------------------------------------------------------------------------
# 2. Property-based tests.
# ---------------------------------------------------------------------------


def test_risk_parity_equal_risk_contributions():
    cov = np.array(
        [
            [0.10, 0.02, 0.01],
            [0.02, 0.08, 0.015],
            [0.01, 0.015, 0.05],
        ]
    )
    w = risk_parity(cov)
    assert np.isclose(w.sum(), 1.0)
    assert np.all(w >= 0)

    marginal_risk = cov @ w
    risk_contrib = w * marginal_risk
    # All risk contributions should be equal (to total portfolio variance / n).
    assert np.allclose(risk_contrib, risk_contrib[0], atol=1e-6)


def test_hrp_weights_sum_to_one_and_respect_bounds():
    rng = np.random.default_rng(42)
    T, N = 200, 5
    # Correlated returns via a simple factor model for a realistic corr structure.
    factor = rng.normal(size=(T, 1))
    loadings = rng.normal(0.5, 0.2, size=(1, N))
    noise = rng.normal(scale=0.5, size=(T, N))
    returns = factor @ loadings + noise

    w = hrp(returns)
    assert np.isclose(w.sum(), 1.0)
    assert np.all(w >= -1e-9)

    bounds = [(0.05, 0.4)] * N
    w_bounded = hrp(returns, bounds=bounds)
    assert np.isclose(w_bounded.sum(), 1.0)
    for wi in w_bounded:
        assert 0.05 - 1e-6 <= wi <= 0.4 + 1e-6


def test_hrp_accepts_precomputed_cov():
    cov = np.array(
        [
            [0.10, 0.02, 0.01],
            [0.02, 0.08, 0.015],
            [0.01, 0.015, 0.05],
        ]
    )
    w = hrp(cov, is_cov=True)
    assert np.isclose(w.sum(), 1.0)
    assert w.shape == (3,)


def test_min_variance_beats_equal_weight_variance():
    cov = np.array(
        [
            [0.10, 0.02, 0.01],
            [0.02, 0.08, 0.015],
            [0.01, 0.015, 0.05],
        ]
    )
    w_mv = min_variance(cov)
    w_eq = np.ones(3) / 3

    var_mv = w_mv @ cov @ w_mv
    var_eq = w_eq @ cov @ w_eq
    assert var_mv <= var_eq + 1e-12


def test_min_variance_respects_bounds():
    w = min_variance(GOLD_COV, bounds=[(0.4, 0.6), (0.4, 0.6)])
    assert np.isclose(w.sum(), 1.0)
    for wi in w:
        assert 0.4 - 1e-6 <= wi <= 0.6 + 1e-6


def test_max_sharpe_respects_bounds():
    w = max_sharpe(GOLD_MEAN, GOLD_COV, bounds=[(0.3, 0.7), (0.3, 0.7)])
    assert np.isclose(w.sum(), 1.0)
    for wi in w:
        assert 0.3 - 1e-6 <= wi <= 0.7 + 1e-6


def test_sample_cov_matches_numpy():
    rng = np.random.default_rng(0)
    returns = rng.normal(size=(50, 4))
    cov = sample_cov(returns)
    assert np.allclose(cov, np.cov(returns, rowvar=False, ddof=1))


def test_ewma_cov_weights_recent_observations_more():
    rng = np.random.default_rng(1)
    calm = rng.normal(scale=0.01, size=(100, 2))
    volatile = rng.normal(scale=0.10, size=(20, 2))
    returns = np.vstack([calm, volatile])  # volatility regime shift at the end

    cov_short_hl = ewma_cov(returns, halflife=5)
    cov_long_hl = ewma_cov(returns, halflife=200)

    # A short halflife should pick up the recent volatile regime much more
    # strongly than a long halflife (which averages closer to the full sample).
    assert cov_short_hl[0, 0] > cov_long_hl[0, 0]


def test_by_regime_returns_per_regime_weights():
    rng = np.random.default_rng(2)
    returns = rng.normal(scale=0.02, size=(60, 3))
    labels = ["bull"] * 30 + ["bear"] * 30

    weights_by_regime = by_regime(returns, labels, optimizer=min_variance)
    assert set(weights_by_regime.keys()) == {"bull", "bear"}
    for w in weights_by_regime.values():
        assert np.isclose(w.sum(), 1.0)


# ---------------------------------------------------------------------------
# 3. Ledoit-Wolf vs sklearn (skips cleanly when sklearn is absent).
# ---------------------------------------------------------------------------


def test_ledoit_wolf_matches_sklearn():
    sk = pytest.importorskip("sklearn")
    from sklearn.covariance import LedoitWolf

    rng = np.random.default_rng(7)
    returns = rng.normal(size=(80, 5))

    ours = ledoit_wolf_cov(returns)
    ref = LedoitWolf().fit(returns).covariance_

    assert np.allclose(ours, ref, atol=1e-2, rtol=1e-2)


# ---------------------------------------------------------------------------
# 4. Degenerate inputs raise PortfolioError (no crash, no NaN weights).
# ---------------------------------------------------------------------------


def test_min_variance_singular_cov_raises():
    singular = np.array([[1.0, 1.0], [1.0, 1.0]])
    with pytest.raises(PortfolioError):
        min_variance(singular)


def test_max_sharpe_singular_cov_raises():
    singular = np.array([[1.0, 1.0], [1.0, 1.0]])
    with pytest.raises(PortfolioError):
        max_sharpe(np.array([0.1, 0.1]), singular)


def test_single_asset_min_variance():
    w = min_variance(np.array([[0.04]]))
    assert np.allclose(w, [1.0])


def test_single_asset_zero_variance_raises():
    with pytest.raises(PortfolioError):
        min_variance(np.array([[0.0]]))


def test_nan_returns_raise_in_sample_cov():
    returns = np.array([[0.01, np.nan], [0.02, 0.01], [0.03, 0.02]])
    with pytest.raises(PortfolioError):
        sample_cov(returns)


def test_nan_returns_raise_in_hrp():
    returns = np.array([[0.01, np.nan], [0.02, 0.01], [0.03, 0.02]])
    with pytest.raises(PortfolioError):
        hrp(returns)


def test_empty_returns_raise():
    with pytest.raises(PortfolioError):
        sample_cov(np.empty((0, 3)))


def test_mismatched_bounds_length_raises():
    with pytest.raises(PortfolioError):
        min_variance(GOLD_COV, bounds=[(0.0, 1.0)])


def test_infeasible_bounds_raise():
    with pytest.raises(PortfolioError):
        min_variance(GOLD_COV, bounds=[(0.0, 0.2), (0.0, 0.2)])


def test_no_nan_weights_ever_on_valid_input():
    cov = np.array(
        [
            [0.10, 0.02, 0.01],
            [0.02, 0.08, 0.015],
            [0.01, 0.015, 0.05],
        ]
    )
    for w in (
        min_variance(cov),
        max_sharpe(np.array([0.05, 0.03, 0.02]), cov),
        risk_parity(cov),
        hrp(cov, is_cov=True),
    ):
        assert not np.any(np.isnan(w))
        assert np.isclose(w.sum(), 1.0)
