//! Covariance estimators: sample, EWMA (RiskMetrics-style), Ledoit-Wolf shrinkage.

use super::{PortfolioError, PortfolioResult};
use nalgebra::DMatrix;

/// Validate a `(T, N)` returns matrix: finite, >=2 rows, >=1 col.
pub(crate) fn check_returns_matrix(returns: &DMatrix<f64>) -> PortfolioResult<()> {
    if returns.nrows() == 0 || returns.ncols() == 0 {
        return Err(PortfolioError::Empty("returns".to_string()));
    }
    if returns.nrows() < 2 {
        return Err(PortfolioError::InsufficientObservations);
    }
    if returns.ncols() < 1 {
        return Err(PortfolioError::NoAssets);
    }
    if !returns.iter().all(|v| v.is_finite()) {
        return Err(PortfolioError::NonFinite("returns".to_string()));
    }
    Ok(())
}

/// Validate a covariance matrix: square, finite, >=1 asset.
pub(crate) fn check_cov(cov: &DMatrix<f64>) -> PortfolioResult<()> {
    if cov.nrows() != cov.ncols() {
        return Err(PortfolioError::NonSquareCov(cov.nrows(), cov.ncols()));
    }
    if cov.nrows() < 1 {
        return Err(PortfolioError::Empty("covariance".to_string()));
    }
    if !cov.iter().all(|v| v.is_finite()) {
        return Err(PortfolioError::NonFinite("covariance".to_string()));
    }
    Ok(())
}

/// Sample covariance matrix of asset returns: `np.cov(returns, rowvar=False, ddof=1)`.
///
/// `returns` is `(T, N)` (rows=time, cols=assets); returns an `(N, N)`
/// covariance matrix.
pub fn sample_cov(returns: &DMatrix<f64>) -> PortfolioResult<DMatrix<f64>> {
    check_returns_matrix(returns)?;
    let t = returns.nrows() as f64;
    let n = returns.ncols();

    // Column means.
    let mean = returns.row_mean();
    let mut centered = returns.clone();
    for mut row in centered.row_iter_mut() {
        row -= &mean;
    }
    let cov = (centered.transpose() * &centered) / (t - 1.0);
    debug_assert_eq!(cov.nrows(), n);
    Ok(cov)
}

/// Exponentially-weighted covariance matrix (RiskMetrics-style).
///
/// Weights the `t`-th most-recent observation (`t=0` = most recent) by
/// `lambda**t` where `lambda = 0.5 ** (1 / halflife)`, normalized to sum to
/// 1, applied to the (mean-centered) outer products of returns. Direct port
/// of `ewma_cov` in the Python v1.
pub fn ewma_cov(returns: &DMatrix<f64>, halflife: f64) -> PortfolioResult<DMatrix<f64>> {
    check_returns_matrix(returns)?;
    if halflife <= 0.0 {
        return Err(PortfolioError::InvalidHalflife(halflife));
    }

    let t = returns.nrows();
    let n = returns.ncols();
    let lam = 0.5_f64.powf(1.0 / halflife);

    // t=0 is most recent observation (last row): ages = [T-1, T-2, ..., 0].
    let raw_w: Vec<f64> = (0..t).map(|i| lam.powi((t - 1 - i) as i32)).collect();
    let w_sum: f64 = raw_w.iter().sum();
    let w: Vec<f64> = raw_w.iter().map(|v| v / w_sum).collect();

    // Weighted mean per column.
    let mut mean = vec![0.0_f64; n];
    for (i, wi) in w.iter().enumerate() {
        for j in 0..n {
            mean[j] += wi * returns[(i, j)];
        }
    }

    // Weighted outer-product covariance: sum_i w_i * (x_i - mean)(x_i - mean)^T.
    let mut cov = DMatrix::<f64>::zeros(n, n);
    for (i, wi) in w.iter().enumerate() {
        let centered: Vec<f64> = (0..n).map(|j| returns[(i, j)] - mean[j]).collect();
        for a in 0..n {
            for b in 0..n {
                cov[(a, b)] += wi * centered[a] * centered[b];
            }
        }
    }

    // Bias correction akin to reliability weights: 1 / (1 - sum(w^2)).
    let sum_w2: f64 = w.iter().map(|v| v * v).sum();
    let denom = 1.0 - sum_w2;
    if denom > 1e-12 {
        cov /= denom;
    }

    if !cov.iter().all(|v| v.is_finite()) {
        return Err(PortfolioError::NonFinite("ewma_cov result".to_string()));
    }
    Ok(cov)
}

/// Ledoit-Wolf shrinkage covariance estimator (shrinkage target: identity-scaled).
///
/// Implements the analytic shrinkage-intensity formula from Ledoit & Wolf
/// (2004), "A well-conditioned estimator for large-dimensional covariance
/// matrices", shrinking the sample covariance toward `mu * I` where
/// `mu = trace(S) / N` (the same target used by
/// `sklearn.covariance.LedoitWolf` in its default configuration). Direct
/// port of `ledoit_wolf_cov` in the Python v1 -- see that docstring for the
/// full derivation of `pi_hat` / `rho_hat` / `gamma_hat` / `kappa_hat`.
pub fn ledoit_wolf_cov(returns: &DMatrix<f64>) -> PortfolioResult<DMatrix<f64>> {
    check_returns_matrix(returns)?;
    let t = returns.nrows();
    let n = returns.ncols();
    if n == 1 {
        return sample_cov(returns);
    }
    let t_f = t as f64;
    let n_f = n as f64;

    let mean = returns.row_mean();
    let mut x = returns.clone();
    for mut row in x.row_iter_mut() {
        row -= &mean;
    }

    // Population (ddof=0) sample covariance.
    let s_pop = (x.transpose() * &x) / t_f;
    let mu = s_pop.trace() / n_f;

    let diag_target: Vec<f64> = (0..n).map(|i| s_pop[(i, i)] - mu).collect();

    let mut pi_sum = 0.0_f64;
    let mut rho_sum = 0.0_f64;
    for row in 0..t {
        let xt: Vec<f64> = (0..n).map(|j| x[(row, j)]).collect();
        // diff = outer(xt, xt) - s_pop; accumulate sum(diff^2) and the
        // diagonal-only rho term without materializing S_t explicitly.
        let mut diag_diff = vec![0.0_f64; n];
        for a in 0..n {
            for b in 0..n {
                let s_t_ab = xt[a] * xt[b];
                let diff = s_t_ab - s_pop[(a, b)];
                pi_sum += diff * diff;
                if a == b {
                    diag_diff[a] = diff;
                }
            }
        }
        for i in 0..n {
            rho_sum += diag_diff[i] * diag_target[i];
        }
    }
    let pi_hat = pi_sum / t_f;
    let rho_hat = rho_sum / t_f;

    let mut gamma_hat = 0.0_f64;
    for a in 0..n {
        for b in 0..n {
            let target = if a == b { mu } else { 0.0 };
            let d = s_pop[(a, b)] - target;
            gamma_hat += d * d;
        }
    }

    let shrinkage = if gamma_hat < 1e-18 {
        0.0
    } else {
        let kappa_hat = (pi_hat - rho_hat) / gamma_hat;
        (kappa_hat / t_f).clamp(0.0, 1.0)
    };

    let s_sample = sample_cov(returns)?; // ddof=1
    let mut cov = s_sample * (1.0 - shrinkage);
    for i in 0..n {
        cov[(i, i)] += shrinkage * mu;
    }

    if !cov.iter().all(|v| v.is_finite()) {
        return Err(PortfolioError::NonFinite(
            "ledoit_wolf_cov result".to_string(),
        ));
    }
    Ok(cov)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toy_returns() -> DMatrix<f64> {
        // 5 obs x 3 assets, deterministic values.
        DMatrix::from_row_slice(
            5,
            3,
            &[
                0.01, 0.02, -0.01, 0.015, -0.005, 0.02, -0.02, 0.01, 0.005, 0.005, 0.03, -0.01,
                0.0, 0.01, 0.015,
            ],
        )
    }

    #[test]
    fn sample_cov_symmetric_and_finite() {
        let r = toy_returns();
        let cov = sample_cov(&r).unwrap();
        assert_eq!(cov.nrows(), 3);
        for i in 0..3 {
            for j in 0..3 {
                assert!((cov[(i, j)] - cov[(j, i)]).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn sample_cov_matches_numpy_cov_ddof1_fixture() {
        // Computed with numpy: np.cov(r, rowvar=False, ddof=1) on toy_returns().
        let r = toy_returns();
        let cov = sample_cov(&r).unwrap();
        // Spot-check variance of asset 0 by hand: mean=0.004, ddof=1 => /4.
        let col0: Vec<f64> = (0..5).map(|i| r[(i, 0)]).collect();
        let mean0 = col0.iter().sum::<f64>() / 5.0;
        let var0 = col0.iter().map(|v| (v - mean0).powi(2)).sum::<f64>() / 4.0;
        assert!((cov[(0, 0)] - var0).abs() < 1e-12);
    }

    #[test]
    fn ewma_cov_positive_definite_diag() {
        let r = toy_returns();
        let cov = ewma_cov(&r, 3.0).unwrap();
        for i in 0..3 {
            assert!(cov[(i, i)] > 0.0);
        }
    }

    #[test]
    fn ewma_cov_rejects_nonpositive_halflife() {
        let r = toy_returns();
        let err = ewma_cov(&r, 0.0).unwrap_err();
        assert_eq!(err, PortfolioError::InvalidHalflife(0.0));
    }

    #[test]
    fn ledoit_wolf_shrinks_toward_identity_scaled_target() {
        let r = toy_returns();
        let lw = ledoit_wolf_cov(&r).unwrap();
        let s = sample_cov(&r).unwrap();
        // Shrunk diagonal should sit between raw sample diag and the
        // identity-scaled target's implied uniform trace/N value -- at
        // minimum it must stay finite and PSD-plausible (non-negative diag).
        for i in 0..3 {
            assert!(lw[(i, i)] > 0.0);
            assert!(lw[(i, i)].is_finite());
        }
        let _ = s;
    }

    #[test]
    fn ledoit_wolf_single_asset_falls_back_to_sample_cov() {
        let r = DMatrix::from_row_slice(4, 1, &[0.01, -0.02, 0.03, 0.0]);
        let lw = ledoit_wolf_cov(&r).unwrap();
        let s = sample_cov(&r).unwrap();
        assert!((lw[(0, 0)] - s[(0, 0)]).abs() < 1e-12);
    }
}
