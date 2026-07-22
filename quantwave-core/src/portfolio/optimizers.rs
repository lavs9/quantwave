//! Closed-form / fixed-point portfolio optimizers: min-variance, max-Sharpe,
//! risk parity (Spinu).

use super::covariance::check_cov;
use super::projection::apply_bounds_and_budget;
use super::{Bounds, PortfolioError, PortfolioResult};
use nalgebra::{DMatrix, DVector};

/// Solve `cov @ x = rhs`, returning [`PortfolioError`] on singularity.
///
/// Uses the condition number purely to *detect* near-singularity before
/// attempting the exact solve (`nalgebra`'s LU decomposition), mirroring
/// `_safe_solve_spd` in the Python v1.
fn safe_solve_spd(cov: &DMatrix<f64>, rhs: &DVector<f64>) -> PortfolioResult<DVector<f64>> {
    let n = cov.nrows();
    if n == 1 {
        let val = cov[(0, 0)];
        if val <= 0.0 || !val.is_finite() {
            return Err(PortfolioError::InvalidSingleAssetCov);
        }
        return Ok(DVector::from_vec(vec![rhs[0] / val]));
    }

    // Condition number via SVD (largest / smallest singular value), same
    // diagnostic role as `np.linalg.cond`.
    let svd = cov.clone().svd(false, false);
    let sv = svd.singular_values;
    let max_sv = sv.max();
    let min_sv = sv.min();
    let cond = if min_sv > 0.0 {
        max_sv / min_sv
    } else {
        f64::INFINITY
    };
    if !cond.is_finite() || cond > 1e12 {
        return Err(PortfolioError::IllConditionedCov(cond));
    }

    let lu = cov.clone().lu();
    let x = lu
        .solve(rhs)
        .ok_or_else(|| PortfolioError::DegenerateCov("LU solve failed".to_string()))?;
    if !x.iter().all(|v| v.is_finite()) {
        return Err(PortfolioError::NonFiniteSolve);
    }
    Ok(x)
}

/// Minimum-variance portfolio weights.
///
/// Closed form (unconstrained): `w ∝ Σ⁻¹ · 1`, normalized to sum to 1.
/// Bounds/long-only constraints are then applied via capped-simplex
/// projection (see [`apply_bounds_and_budget`]).
pub fn min_variance(
    cov: &DMatrix<f64>,
    bounds: Option<&[Bounds]>,
) -> PortfolioResult<DVector<f64>> {
    check_cov(cov)?;
    let n = cov.nrows();
    let ones = DVector::from_element(n, 1.0);
    let raw = safe_solve_spd(cov, &ones)?;
    let total = raw.sum();
    if total.abs() < 1e-12 {
        return Err(PortfolioError::MinVarianceDegenerate);
    }
    let w = raw / total;
    apply_bounds_and_budget(&w, bounds, true)
}

/// Maximum-Sharpe (tangency) portfolio weights.
///
/// Closed form (unconstrained, zero risk-free rate): `w ∝ Σ⁻¹ · μ`,
/// normalized to sum to 1. Bounds/long-only constraints are then applied
/// via capped-simplex projection.
pub fn max_sharpe(
    mean: &DVector<f64>,
    cov: &DMatrix<f64>,
    bounds: Option<&[Bounds]>,
) -> PortfolioResult<DVector<f64>> {
    check_cov(cov)?;
    if mean.len() != cov.nrows() {
        return Err(PortfolioError::MeanLengthMismatch(mean.len(), cov.nrows()));
    }
    if !mean.iter().all(|v| v.is_finite()) {
        return Err(PortfolioError::NonFinite("mean".to_string()));
    }
    let raw = safe_solve_spd(cov, mean)?;
    let total = raw.sum();
    if total.abs() < 1e-12 {
        return Err(PortfolioError::MaxSharpeDegenerate);
    }
    let w = raw / total;
    apply_bounds_and_budget(&w, bounds, true)
}

/// Equal risk-contribution ("risk parity") portfolio weights.
///
/// Uses Spinu's (2013) fixed-point iteration for the risk-parity problem
/// ("An Algorithm for Computing Risk Parity Weights"): solve for `y` such
/// that `Σy = 1/y` (elementwise reciprocal of the risk-budget-scaled
/// weights), via the multiplicative update
///
/// `y_{k+1} = y_k * sqrt(b / (Σ y_k * y_k) * sum(y_k * (Σ y_k)))`
/// (elementwise, `b = 1/n` each budget), starting from equal weight, then
/// normalize `y` to sum to 1. Risk contributions `RC_i = w_i * (Σw)_i` are
/// equal at the fixed point (up to solver tolerance). Direct port of
/// `risk_parity` in the Python v1.
pub fn risk_parity(
    cov: &DMatrix<f64>,
    bounds: Option<&[Bounds]>,
    max_iter: usize,
    tol: f64,
) -> PortfolioResult<DVector<f64>> {
    check_cov(cov)?;
    let n = cov.nrows();
    let b = 1.0 / n as f64; // equal risk budget for every asset

    let mut y = DVector::from_element(n, 1.0 / n as f64);
    for _ in 0..max_iter {
        let sigma_y = cov * &y;
        if sigma_y.iter().any(|v| *v <= 0.0) || !sigma_y.iter().all(|v| v.is_finite()) {
            return Err(PortfolioError::RiskParityNonPositiveMarginalRisk);
        }
        let y_sigma_y: f64 = (0..n).map(|i| y[i] * sigma_y[i]).sum();
        let mut y_new = DVector::from_iterator(
            n,
            (0..n).map(|i| y[i] * (b / (y[i] * sigma_y[i]) * y_sigma_y).sqrt()),
        );
        let sum_new = y_new.sum();
        y_new /= sum_new;

        let max_diff = (0..n).map(|i| (y_new[i] - y[i]).abs()).fold(0.0, f64::max);
        y = y_new;
        if max_diff < tol {
            break;
        }
    }

    if !y.iter().all(|v| v.is_finite()) {
        return Err(PortfolioError::RiskParityDidNotConverge);
    }
    let w = &y / y.sum();
    apply_bounds_and_budget(&w, bounds, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn diag_cov(vals: &[f64]) -> DMatrix<f64> {
        DMatrix::from_diagonal(&DVector::from_row_slice(vals))
    }

    #[test]
    fn min_variance_diagonal_cov_inverse_variance_weighted() {
        // For diagonal Sigma, min-variance weights are inverse-variance,
        // w_i = (1/s_i) / sum(1/s_j) -- a standard closed-form sanity check.
        let cov = diag_cov(&[0.04, 0.01, 0.09]); // vars 0.04, 0.01, 0.09
        let w = min_variance(&cov, None).unwrap();
        let inv = [1.0 / 0.04, 1.0 / 0.01, 1.0 / 0.09];
        let total: f64 = inv.iter().sum();
        for i in 0..3 {
            assert_relative_eq!(w[i], inv[i] / total, epsilon = 1e-9);
        }
        assert_relative_eq!(w.sum(), 1.0, epsilon = 1e-9);
    }

    #[test]
    fn min_variance_equal_variance_equal_weight() {
        let cov = DMatrix::<f64>::identity(4, 4) * 0.02;
        let w = min_variance(&cov, None).unwrap();
        for i in 0..4 {
            assert_relative_eq!(w[i], 0.25, epsilon = 1e-9);
        }
    }

    #[test]
    fn max_sharpe_diagonal_cov_matches_mu_over_var() {
        let cov = diag_cov(&[0.04, 0.01]);
        let mean = DVector::from_row_slice(&[0.08, 0.02]);
        let w = max_sharpe(&mean, &cov, None).unwrap();
        // raw = Sigma^-1 mu = [0.08/0.04, 0.02/0.01] = [2.0, 2.0] -> equal weight.
        assert_relative_eq!(w[0], 0.5, epsilon = 1e-9);
        assert_relative_eq!(w[1], 0.5, epsilon = 1e-9);
    }

    #[test]
    fn risk_parity_equal_variance_uncorrelated_is_equal_weight() {
        let cov = DMatrix::<f64>::identity(3, 3) * 0.05;
        let w = risk_parity(&cov, None, 500, 1e-12).unwrap();
        for i in 0..3 {
            assert_relative_eq!(w[i], 1.0 / 3.0, epsilon = 1e-6);
        }
    }

    #[test]
    fn risk_parity_equalizes_risk_contributions() {
        // Non-trivial diagonal covariance: risk parity should produce
        // inverse-variance-like weights (uncorrelated case has a
        // closed-form risk-parity solution equal to inverse-vol weights).
        let cov = diag_cov(&[0.04, 0.09, 0.01]);
        let w = risk_parity(&cov, None, 1000, 1e-13).unwrap();
        let sigma_w = &cov * &w;
        let rc: Vec<f64> = (0..3).map(|i| w[i] * sigma_w[i]).collect();
        for i in 1..3 {
            assert_relative_eq!(rc[i], rc[0], epsilon = 1e-6);
        }
    }

    #[test]
    fn min_variance_rejects_ill_conditioned_cov() {
        // Near-singular: two identical rows/cols.
        let cov = DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.0, 1.0 + 1e-15]);
        let err = min_variance(&cov, None);
        assert!(err.is_err());
    }

    #[test]
    fn min_variance_with_upper_bound_clips_dominant_asset() {
        let cov = diag_cov(&[0.01, 1.0, 1.0]); // asset 0 wants ~all weight
        let bounds = vec![(0.0, 0.5), (0.0, 1.0), (0.0, 1.0)];
        let w = min_variance(&cov, Some(&bounds)).unwrap();
        assert!(w[0] <= 0.5 + 1e-9);
        assert_relative_eq!(w.sum(), 1.0, epsilon = 1e-9);
    }
}
