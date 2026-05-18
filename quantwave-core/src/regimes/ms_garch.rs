//! Markov-Switching GARCH (MS-GARCH)
//! 
//! Source: Reher (2011) "Markov-switching GARCH models in finance"
//! 
//! MS-GARCH models allow volatility dynamics (GARCH parameters) to vary across 
//! different hidden market regimes.

use crate::traits::Next;
use crate::regimes::MarketRegime;

/// Parameters for a single GARCH(1,1) regime.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct GarchParams {
    pub omega: f64,
    pub alpha: f64,
    pub beta: f64,
}

impl GarchParams {
    pub fn new(omega: f64, alpha: f64, beta: f64) -> Self {
        Self { omega, alpha, beta }
    }
}

/// A Markov-Switching GARCH model.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MSGarch {
    pub n_states: usize,
    /// Transition probability matrix [from_state][to_state]
    pub a: Vec<Vec<f64>>,
    /// GARCH parameters for each state
    pub params: Vec<GarchParams>,
    /// Last estimated variance for each state
    last_variances: Vec<f64>,
    /// Last probability distribution across states
    last_probs: Vec<f64>,
    initialized: bool,
}

impl MSGarch {
    pub fn new(a: Vec<Vec<f64>>, params: Vec<GarchParams>, initial_probs: Vec<f64>) -> Self {
        let n_states = a.len();
        Self {
            n_states,
            a,
            params,
            last_variances: vec![0.0001; n_states], // Small initial variance
            last_probs: initial_probs,
            initialized: false,
        }
    }

    /// Default 2-state MS-GARCH (Low Vol / High Vol)
    pub fn low_high_vol() -> Self {
        Self::new(
            vec![
                vec![0.98, 0.02], // Low -> Low, Low -> High
                vec![0.05, 0.95], // High -> Low, High -> High
            ],
            vec![
                GarchParams::new(1e-6, 0.05, 0.90), // Low Vol regime
                GarchParams::new(1e-4, 0.15, 0.80), // High Vol regime
            ],
            vec![0.9, 0.1],
        )
    }

    /// Calculate Gaussian PDF: P(x | mu=0, sigma)
    fn gaussian_pdf(x: f64, sigma: f64) -> f64 {
        let variance = sigma * sigma;
        let denom = (2.0 * std::f64::consts::PI * variance).sqrt();
        let exponent = -(x.powi(2)) / (2.0 * variance);
        exponent.exp() / denom
    }
}

impl Next<f64> for MSGarch {
    type Output = (MarketRegime, f64); // (Regime, Estimated Combined Volatility)

    fn next(&mut self, returns: f64) -> Self::Output {
        if !self.initialized {
            self.initialized = true;
            return (MarketRegime::Steady, self.last_variances[0].sqrt());
        }

        let mut next_probs = vec![0.0; self.n_states];
        let mut likelihoods = vec![0.0; self.n_states];
        let mut total_likelihood = 0.0;

        // 1. Update Probabilities (Filtering step)
        for j in 0..self.n_states {
            let mut prob_j = 0.0;
            for i in 0..self.n_states {
                prob_j += self.last_probs[i] * self.a[i][j];
            }
            let emission = Self::gaussian_pdf(returns, self.last_variances[j].sqrt());
            likelihoods[j] = prob_j * emission;
            total_likelihood += likelihoods[j];
        }

        if total_likelihood > 0.0 {
            for j in 0..self.n_states {
                next_probs[j] = likelihoods[j] / total_likelihood;
            }
        } else {
            next_probs = self.last_probs.clone();
        }

        // 2. Update GARCH Variances for each state
        let epsilon_sq = returns.powi(2);
        for j in 0..self.n_states {
            let p = &self.params[j];
            // sigma_t^2 = omega + alpha * eps_{t-1}^2 + beta * sigma_{t-1}^2
            self.last_variances[j] = p.omega + p.alpha * epsilon_sq + p.beta * self.last_variances[j];
        }

        self.last_probs = next_probs;

        // 3. Output weighted volatility and most likely regime
        let mut max_p = -1.0;
        let mut best_state = 0;
        let mut combined_var = 0.0;
        for j in 0..self.n_states {
            if self.last_probs[j] > max_p {
                max_p = self.last_probs[j];
                best_state = j;
            }
            combined_var += self.last_probs[j] * self.last_variances[j];
        }

        let regime = match best_state {
            0 => MarketRegime::Steady,
            1 => MarketRegime::Crisis,
            _ => MarketRegime::Cluster(best_state as u8),
        };

        (regime, combined_var.sqrt())
    }
}
