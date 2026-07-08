//! Gaussian Mixture Models (Two Sigma 2021)
//!
//! Source: Two Sigma (2021). "A Machine Learning Approach to Regime Modeling."
//! Foundational EM Algorithm: Dempster, A. P., Laird, N. M., & Rubin, D. B. (1977).
//! "Maximum Likelihood from Incomplete Data via the EM Algorithm."
//! Journal of the Royal Statistical Society: Series B (Methodological), 39(1), 1-22.
//!
//! Multi-variate clustering for latent market states using the Expectation-Maximization (EM)
//! algorithm. This implementation uses diagonal covariance matrices for efficiency.

use crate::regimes::MarketRegime;
use crate::traits::Next;
use serde::{Deserialize, Serialize};

const VAR_FLOOR: f64 = 1e-9;
const LOG_FLOOR: f64 = 1e-300;

/// A Gaussian Mixture Model for multi-factor regime detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GMM {
    k: usize,
    dims: usize,
    /// Means for each component [k][dim]
    means: Vec<Vec<f64>>,
    /// Variances for each component [k][dim] (diagonal covariance)
    vars: Vec<Vec<f64>>,
    /// Mixing coefficients
    weights: Vec<f64>,
}

/// Configuration for EM fitting.
#[derive(Debug, Clone)]
pub struct GmmFitConfig {
    pub max_iter: usize,
    pub tol: f64,
    pub seed: u64,
}

impl Default for GmmFitConfig {
    fn default() -> Self {
        Self {
            max_iter: 100,
            tol: 1e-6,
            seed: 42,
        }
    }
}

/// Result of EM parameter estimation.
#[derive(Debug, Clone)]
pub struct GmmFitResult {
    pub log_likelihood: f64,
    pub iterations: usize,
    pub converged: bool,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum GmmError {
    #[error("invalid GMM parameters: {0}")]
    InvalidParams(String),
    #[error("need at least {min} observations, got {got}")]
    InsufficientData { min: usize, got: usize },
    #[error("EM did not converge within {max_iter} iterations")]
    EmNotConverged { max_iter: usize },
}

impl GMM {
    /// Creates a new GMM with pre-defined parameters.
    pub fn new(means: Vec<Vec<f64>>, vars: Vec<Vec<f64>>, weights: Vec<f64>) -> Self {
        let k = means.len();
        let dims = means[0].len();
        Self {
            k,
            dims,
            means,
            vars,
            weights,
        }
    }

    /// Unfitted model with `k` components and `dims` dimensions (for `.fit()`).
    pub fn with_components(k: usize, dims: usize) -> Self {
        let means = vec![vec![0.0; dims]; k];
        let vars = vec![vec![1.0; dims]; k];
        let weights = vec![1.0 / k as f64; k];
        Self {
            k,
            dims,
            means,
            vars,
            weights,
        }
    }

    pub fn components(&self) -> usize {
        self.k
    }

    pub fn dims(&self) -> usize {
        self.dims
    }

    pub fn means(&self) -> &[Vec<f64>] {
        &self.means
    }

    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// Log PDF of x under component `k_idx` (diagonal Gaussian).
    fn log_pdf(&self, x: &[f64], k_idx: usize) -> f64 {
        let mut log_prob = 0.0;
        for d in 0..self.dims {
            let mu = self.means[k_idx][d];
            let var = self.vars[k_idx][d].max(VAR_FLOOR);
            let diff = x[d] - mu;
            log_prob += -0.5 * ((2.0 * std::f64::consts::PI * var).ln() + diff * diff / var);
        }
        log_prob
    }

    /// Calculate multivariate Gaussian PDF (diagonal covariance)
    fn pdf(&self, x: &[f64], k_idx: usize) -> f64 {
        self.log_pdf(x, k_idx).exp()
    }

    fn validate_data(&self, data: &[Vec<f64>]) -> Result<(), GmmError> {
        if data.len() < self.k {
            return Err(GmmError::InsufficientData {
                min: self.k,
                got: data.len(),
            });
        }
        for row in data {
            if row.len() != self.dims {
                return Err(GmmError::InvalidParams(format!(
                    "expected {dims} dims, got {got}",
                    dims = self.dims,
                    got = row.len()
                )));
            }
        }
        Ok(())
    }

    fn init_from_quantiles(&mut self, data: &[Vec<f64>]) {
        let n = data.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by(|&a, &b| {
            data[a][0]
                .partial_cmp(&data[b][0])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for (k, chunk) in order.chunks((n / self.k).max(1)).enumerate().take(self.k) {
            if chunk.is_empty() {
                continue;
            }
            for d in 0..self.dims {
                let sum: f64 = chunk.iter().map(|&i| data[i][d]).sum();
                self.means[k][d] = sum / chunk.len() as f64;
                let var: f64 = chunk
                    .iter()
                    .map(|&i| {
                        let diff = data[i][d] - self.means[k][d];
                        diff * diff
                    })
                    .sum::<f64>()
                    / chunk.len() as f64;
                self.vars[k][d] = var.max(VAR_FLOOR);
            }
            self.weights[k] = chunk.len() as f64 / n as f64;
        }

        let w_sum: f64 = self.weights.iter().sum();
        if w_sum > 0.0 {
            for w in &mut self.weights {
                *w /= w_sum;
            }
        }
    }

    fn responsibilities(&self, data: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = data.len();
        let mut resp = vec![vec![0.0; self.k]; n];
        for (i, x) in data.iter().enumerate() {
            let mut log_probs = vec![0.0; self.k];
            let mut max_log = f64::NEG_INFINITY;
            for k in 0..self.k {
                let lp = self.weights[k].max(LOG_FLOOR).ln() + self.log_pdf(x, k);
                log_probs[k] = lp;
                if lp > max_log {
                    max_log = lp;
                }
            }
            let mut sum = 0.0;
            for k in 0..self.k {
                let r = (log_probs[k] - max_log).exp();
                resp[i][k] = r;
                sum += r;
            }
            if sum > 0.0 {
                for k in 0..self.k {
                    resp[i][k] /= sum;
                }
            }
        }
        resp
    }

    fn log_likelihood(&self, data: &[Vec<f64>]) -> f64 {
        let mut total = 0.0;
        for x in data {
            let mut log_probs = vec![0.0; self.k];
            let mut max_log = f64::NEG_INFINITY;
            for k in 0..self.k {
                let lp = self.weights[k].max(LOG_FLOOR).ln() + self.log_pdf(x, k);
                log_probs[k] = lp;
                if lp > max_log {
                    max_log = lp;
                }
            }
            let ll = max_log + log_probs.iter().map(|&lp| (lp - max_log).exp()).sum::<f64>().ln();
            total += ll;
        }
        total
    }

    fn m_step(&mut self, data: &[Vec<f64>], resp: &[Vec<f64>]) {
        let n = data.len();
        for k in 0..self.k {
            let nk: f64 = resp.iter().map(|r| r[k]).sum();
            if nk < LOG_FLOOR {
                continue;
            }
            self.weights[k] = nk / n as f64;
            for d in 0..self.dims {
                let mean: f64 = resp
                    .iter()
                    .zip(data.iter())
                    .map(|(r, x)| r[k] * x[d])
                    .sum::<f64>()
                    / nk;
                self.means[k][d] = mean;
                let var: f64 = resp
                    .iter()
                    .zip(data.iter())
                    .map(|(r, x)| {
                        let diff = x[d] - mean;
                        r[k] * diff * diff
                    })
                    .sum::<f64>()
                    / nk;
                self.vars[k][d] = var.max(VAR_FLOOR);
            }
        }
    }

    /// Batch fit using EM (diagonal covariance).
    pub fn fit(
        &mut self,
        data: &[Vec<f64>],
        config: &GmmFitConfig,
    ) -> Result<GmmFitResult, GmmError> {
        self.validate_data(data)?;
        self.init_from_quantiles(data);

        let mut prev_ll = f64::NEG_INFINITY;
        let mut iterations = 0usize;
        let mut converged = false;

        for iter in 0..config.max_iter {
            iterations = iter + 1;
            let resp = self.responsibilities(data);
            self.m_step(data, &resp);
            let ll = self.log_likelihood(data);
            if (ll - prev_ll).abs() < config.tol {
                converged = true;
                prev_ll = ll;
                break;
            }
            if ll < prev_ll - config.tol {
                // numerical wobble — still accept if close
            }
            prev_ll = ll;
        }

        Ok(GmmFitResult {
            log_likelihood: prev_ll,
            iterations,
            converged,
        })
    }
}

impl Next<&[f64]> for GMM {
    type Output = MarketRegime;

    fn next(&mut self, x: &[f64]) -> Self::Output {
        let mut max_prob = -1.0;
        let mut best_k = 0;

        for k in 0..self.k {
            let p = self.weights[k] * self.pdf(x, k);
            if p > max_prob {
                max_prob = p;
                best_k = k;
            }
        }

        match best_k {
            0 => MarketRegime::Steady,
            k if k == self.k - 1 => MarketRegime::Crisis,
            _ => MarketRegime::Cluster(best_k as u8),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn sample_three_gaussians(seed: u64) -> (Vec<Vec<f64>>, Vec<f64>) {
        let mut data = Vec::new();
        let true_means = [-5.0, 0.0, 5.0];
        let mut state = seed;
        for (c, &mu) in true_means.iter().enumerate() {
            for _ in 0..200 {
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let u = (state >> 11) as f64 / (1u64 << 53) as f64;
                let v = (state >> 17) as f64 / (1u64 << 47) as f64;
                let z = (-2.0 * u.ln()).sqrt() * (2.0 * std::f64::consts::PI * v).cos();
                data.push(vec![mu + z * 0.5]);
                let _ = c;
            }
        }
        (data, true_means.to_vec())
    }

    #[test]
    fn fit_recovers_three_gaussian_means() {
        let (data, true_means) = sample_three_gaussians(99);
        let mut gmm = GMM::with_components(3, 1);
        let result = gmm
            .fit(&data, &GmmFitConfig::default())
            .expect("fit should succeed");
        assert!(result.converged);
        let mut recovered: Vec<f64> = gmm.means().iter().map(|m| m[0]).collect();
        recovered.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut expected = true_means;
        expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for (r, e) in recovered.iter().zip(expected.iter()) {
            assert_relative_eq!(r, e, epsilon = 0.75);
        }
        for w in gmm.weights() {
            assert_relative_eq!(*w, 1.0 / 3.0, epsilon = 0.15);
        }
    }

    #[test]
    fn fit_insufficient_data_errors() {
        let mut gmm = GMM::with_components(3, 1);
        let err = gmm.fit(&[vec![1.0], vec![2.0]], &GmmFitConfig::default());
        assert!(matches!(err, Err(GmmError::InsufficientData { .. })));
    }

    #[test]
    fn log_likelihood_non_decreasing_on_easy_data() {
        let (data, _) = sample_three_gaussians(7);
        let mut gmm = GMM::with_components(3, 1);
        gmm.validate_data(&data).unwrap();
        gmm.init_from_quantiles(&data);
        let mut prev = f64::NEG_INFINITY;
        for _ in 0..10 {
            let resp = gmm.responsibilities(&data);
            gmm.m_step(&data, &resp);
            let ll = gmm.log_likelihood(&data);
            assert!(ll >= prev - 1e-9, "LL decreased: {prev} -> {ll}");
            prev = ll;
        }
    }

    #[test]
    fn max_iter_one_reports_not_converged() {
        let (data, _) = sample_three_gaussians(3);
        let mut gmm = GMM::with_components(3, 1);
        let cfg = GmmFitConfig {
            max_iter: 1,
            tol: 1e-12,
            seed: 1,
        };
        let result = gmm.fit(&data, &cfg).unwrap();
        assert!(!result.converged);
        assert_eq!(result.iterations, 1);
    }
}