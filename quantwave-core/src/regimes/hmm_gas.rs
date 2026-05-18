//! HMM-GAS (Score-Driven Transitions)
//! 
//! Source: Creal, Koopman, and Lucas (2013) 
//! "Generalized Autoregressive Score Models with Applications."
//! 
//! HMM-GAS models allow transition probabilities to be time-varying, 
//! driven by the scaled score of the observation likelihood.

use crate::traits::Next;
use crate::regimes::MarketRegime;
use serde::{Deserialize, Serialize};

/// A 2-state Hidden Markov Model with Score-Driven Transitions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HMMGAS {
    /// Parameters for p11 logit: [omega, alpha, phi]
    pub p11_params: [f64; 3],
    /// Parameters for p22 logit: [omega, alpha, phi]
    pub p22_params: [f64; 3],
    /// Latent logit states
    f11: f64,
    f22: f64,
    pub means: [f64; 2],
    pub stds: [f64; 2],
    last_probs: [f64; 2],
    initialized: bool,
}

impl HMMGAS {
    pub fn new(
        p11_params: [f64; 3],
        p22_params: [f64; 3],
        means: [f64; 2],
        stds: [f64; 2],
    ) -> Self {
        Self {
            p11_params,
            p22_params,
            f11: 2.0, // logit(0.88)
            f22: 2.0,
            means,
            stds,
            last_probs: [0.5, 0.5],
            initialized: false,
        }
    }

    fn logit_inv(f: f64) -> f64 {
        1.0 / (1.0 + (-f).exp())
    }

    fn gaussian_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
        let variance = sigma * sigma;
        let denom = (2.0 * std::f64::consts::PI * variance).sqrt();
        let exponent = -((x - mu).powi(2)) / (2.0 * variance);
        exponent.exp() / denom
    }
}

impl Next<f64> for HMMGAS {
    type Output = MarketRegime;

    fn next(&mut self, x: f64) -> Self::Output {
        // Current transition probabilities
        let p11 = Self::logit_inv(self.f11);
        let p22 = Self::logit_inv(self.f22);
        
        let a = [[p11, 1.0 - p11], [1.0 - p22, p22]];

        let mut likelihoods = [0.0; 2];
        let mut total_likelihood = 0.0;

        // 1. Update Probabilities (Filtering)
        for j in 0..2 {
            let mut prob_j = 0.0;
            for i in 0..2 {
                prob_j += self.last_probs[i] * a[i][j];
            }
            let emission = Self::gaussian_pdf(x, self.means[j], self.stds[j]);
            likelihoods[j] = prob_j * emission;
            total_likelihood += likelihoods[j];
        }

        let next_probs = if total_likelihood > 0.0 {
            [likelihoods[0] / total_likelihood, likelihoods[1] / total_likelihood]
        } else {
            self.last_probs
        };

        // 2. Score-Driven Update of Latent Transition States (Simplified GAS)
        // In a full GAS model, we would use the scaled score of the log-likelihood.
        // Here we use a simplified update based on the state probability shift.
        let score11 = next_probs[0] - self.last_probs[0];
        let score22 = next_probs[1] - self.last_probs[1];

        self.f11 = self.p11_params[0] + self.p11_params[1] * score11 + self.p11_params[2] * self.f11;
        self.f22 = self.p22_params[0] + self.p22_params[1] * score22 + self.p22_params[2] * self.f22;

        self.last_probs = next_probs;
        self.initialized = true;

        if next_probs[0] > next_probs[1] {
            MarketRegime::Steady
        } else {
            MarketRegime::Crisis
        }
    }
}
