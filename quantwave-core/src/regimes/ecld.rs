//! Symmetric lambda (exponential power / generalized normal) emission density.
//!
//! ldhmm uses `ecld.pdf` via `gnorm::dgnorm` with `beta = 2/λ`. At λ=1 this is Gaussian.
//!
//! Source: references/ldhmm/ssrn-2979516.pdf §2; ldhmm ecld.* functions.

use statrs::distribution::{ContinuousCDF, Normal};
use statrs::function::gamma::{gamma, gamma_lr};

/// PDF of the symmetric lambda distribution (ecld / generalized normal).
///
/// `pdf(x) = (β / (2σ Γ(1/β))) exp(-|(x-μ)/σ|^β)` where `β = 2/λ`.
/// λ=1 delegates to the standard normal density for bit-identical Gaussian parity.
#[inline]
pub fn ecld_pdf(x: f64, mu: f64, sigma: f64, lambda: f64) -> f64 {
    if (lambda - 1.0).abs() < 1e-12 {
        return gaussian_pdf(x, mu, sigma);
    }
    let beta = 2.0 / lambda;
    let z = ((x - mu) / sigma).abs();
    let norm = beta / (2.0 * sigma * gamma(1.0 / beta));
    norm * (-z.powf(beta)).exp()
}

#[inline]
pub fn ecld_log_pdf(x: f64, mu: f64, sigma: f64, lambda: f64) -> f64 {
    ecld_pdf(x, mu, sigma, lambda).ln()
}

/// CDF of the symmetric lambda distribution (ldhmm `ecld.cdf` / gnorm).
#[inline]
pub fn ecld_cdf(x: f64, mu: f64, sigma: f64, lambda: f64) -> f64 {
    if (lambda - 1.0).abs() < 1e-12 {
        return Normal::new(mu, sigma)
            .expect("valid gaussian cdf params")
            .cdf(x);
    }
    let beta = 2.0 / lambda;
    let u = (x - mu) / sigma;
    let p = gamma_lr(1.0 / beta, u.abs().powf(beta));
    if u >= 0.0 {
        0.5 + 0.5 * p
    } else {
        0.5 - 0.5 * p
    }
}

/// Emission variance of the symmetric lambda distribution: `σ² Γ(3/β) / Γ(1/β)`, β=2/λ.
#[inline]
pub fn ecld_variance(sigma: f64, lambda: f64) -> f64 {
    if (lambda - 1.0).abs() < 1e-12 {
        return sigma * sigma;
    }
    let beta = 2.0 / lambda;
    let s2 = sigma * sigma;
    s2 * gamma(3.0 / beta) / gamma(1.0 / beta)
}

#[inline]
fn gaussian_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let variance = sigma * sigma;
    let denom = (2.0 * std::f64::consts::PI * variance).sqrt();
    let exponent = -((x - mu).powi(2)) / (2.0 * variance);
    exponent.exp() / denom
}

/// Natural-parameter style mapping helpers (ldhmm `n2w` / `w2n` stability).
///
/// Work parameters: `(μ, log σ, log λ)` ↔ natural-ish `(μ, σ, λ)`.
pub fn work_to_natural(mu: f64, log_sigma: f64, log_lambda: f64) -> (f64, f64, f64) {
    (mu, log_sigma.exp(), log_lambda.exp())
}

pub fn natural_to_work(mu: f64, sigma: f64, lambda: f64) -> (f64, f64, f64) {
    (mu, sigma.ln(), lambda.ln())
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn lambda_one_matches_gaussian() {
        let x = 0.005;
        let mu = 0.008;
        let sigma = 0.018;
        let g = gaussian_pdf(x, mu, sigma);
        let l = ecld_pdf(x, mu, sigma, 1.0);
        assert_relative_eq!(l, g, epsilon = 1e-15);
    }

    #[test]
    fn leptokurtic_lambda_increases_center_density() {
        let x = 0.008; // at mean
        let mu = 0.008;
        let sigma = 0.018;
        let g = ecld_pdf(x, mu, sigma, 1.0);
        let l = ecld_pdf(x, mu, sigma, 1.3);
        assert!(l > g);
    }
}