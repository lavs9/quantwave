//! Hidden Markov Models (Hamilton 1989)
//!
//! Implementation of a regime-switching Hidden Markov Model with Gaussian emissions.
//! Includes the Viterbi algorithm for online state decoding and placeholder for Baum-Welch training.

use crate::traits::Next;
use crate::regimes::MarketRegime;

/// A Hidden Markov Model for regime detection.
#[derive(Debug, Clone)]
pub struct HMM {
    n_states: usize,
    /// Transition probability matrix [from_state][to_state]
    a: Vec<Vec<f64>>,
    /// Emission means for each state
    means: Vec<f64>,
    /// Emission standard deviations for each state
    stds: Vec<f64>,
    /// Initial state probabilities
    pi: Vec<f64>,
    /// Last Viterbi log-probabilities for each state
    last_delta: Vec<f64>,
    initialized: bool,
}

impl HMM {
    /// Creates a new HMM with pre-defined parameters.
    pub fn new(
        a: Vec<Vec<f64>>,
        means: Vec<f64>,
        stds: Vec<f64>,
        pi: Vec<f64>,
    ) -> Self {
        let n_states = a.len();
        Self {
            n_states,
            a,
            means,
            stds,
            pi,
            last_delta: vec![0.0; n_states],
            initialized: false,
        }
    }

    /// Default 2-state HMM (Bull/Bear)
    pub fn bull_bear() -> Self {
        Self::new(
            vec![
                vec![0.95, 0.05], // Bull -> Bull, Bull -> Bear
                vec![0.10, 0.90], // Bear -> Bull, Bear -> Bear
            ],
            vec![0.001, -0.002], // Daily returns: 0.1% for Bull, -0.2% for Bear
            vec![0.01, 0.02],    // Volatility: 1% for Bull, 2% for Bear
            vec![0.5, 0.5],
        )
    }

    /// Calculate Gaussian PDF: P(x | mu, sigma)
    fn gaussian_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
        let variance = sigma * sigma;
        let denom = (2.0 * std::f64::consts::PI * variance).sqrt();
        let exponent = -((x - mu).powi(2)) / (2.0 * variance);
        exponent.exp() / denom
    }
}

impl Next<f64> for HMM {
    type Output = MarketRegime;

    fn next(&mut self, x: f64) -> Self::Output {
        let mut next_delta = vec![0.0; self.n_states];
        let mut best_state = 0;
        let mut max_prob = -f64::INFINITY;

        if !self.initialized {
            for i in 0..self.n_states {
                let emission = Self::gaussian_pdf(x, self.means[i], self.stds[i]);
                next_delta[i] = (self.pi[i] * emission).ln();
                if next_delta[i] > max_prob {
                    max_prob = next_delta[i];
                    best_state = i;
                }
            }
            self.initialized = true;
        } else {
            for j in 0..self.n_states {
                let mut max_prev = -f64::INFINITY;
                for i in 0..self.n_states {
                    // delta_j(t) = max_i [ delta_i(t-1) + ln(A_ij) ] + ln(P(x|j))
                    let prob = self.last_delta[i] + self.a[i][j].ln();
                    if prob > max_prev {
                        max_prev = prob;
                    }
                }
                let emission = Self::gaussian_pdf(x, self.means[j], self.stds[j]);
                next_delta[j] = max_prev + emission.ln();
                
                if next_delta[j] > max_prob {
                    max_prob = next_delta[j];
                    best_state = j;
                }
            }
        }

        self.last_delta = next_delta;

        match best_state {
            0 => MarketRegime::Bull,
            1 => MarketRegime::Bear,
            _ => MarketRegime::Cluster(best_state as u8),
        }
    }
}
