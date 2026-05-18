//! Gaussian Mixture Models (Two Sigma 2021)
//!
//! Source: Two Sigma (2021). "A Machine Learning Approach to Regime Modeling."
//! Foundational EM Algorithm: Dempster, A. P., Laird, N. M., & Rubin, D. B. (1977). 
//! "Maximum Likelihood from Incomplete Data via the EM Algorithm." 
//! Journal of the Royal Statistical Society: Series B (Methodological), 39(1), 1-22.
//!
//! Multi-variate clustering for latent market states using the Expectation-Maximization (EM) algorithm.
//! This implementation uses diagonal covariance matrices for efficiency.

use crate::traits::Next;
use crate::regimes::MarketRegime;
use serde::{Deserialize, Serialize};

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

impl GMM {
    /// Creates a new GMM with pre-defined parameters.
    pub fn new(means: Vec<Vec<f64>>, vars: Vec<Vec<f64>>, weights: Vec<f64>) -> Self {
        let k = means.len();
        let dims = means[0].len();
        Self { k, dims, means, vars, weights }
    }

    /// Calculate multivariate Gaussian PDF (diagonal covariance)
    fn pdf(&self, x: &[f64], k_idx: usize) -> f64 {
        let mut prob = 1.0;
        for d in 0..self.dims {
            let mu = self.means[k_idx][d];
            let var = self.vars[k_idx][d].max(1e-9);
            let denom = (2.0 * std::f64::consts::PI * var).sqrt();
            let exponent = -((x[d] - mu).powi(2)) / (2.0 * var);
            prob *= exponent.exp() / denom;
        }
        prob
    }

    /// Batch fit using EM algorithm (simplified placeholder)
    /// In a real implementation, this would iterate until convergence.
    pub fn fit(&mut self, _data: &[Vec<f64>], _max_iter: usize) {
        // TODO: Implement EM algorithm
        // 1. E-step: Responsibilities
        // 2. M-step: Update means, vars, weights
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
