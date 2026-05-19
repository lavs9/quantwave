//! Hidden Semi-Markov Models (HSMM)
//! 
//! Source: Yu (2010) "Hidden Semi-Markov Models"
//! 
//! HSMMs extend HMMs by allowing explicit modeling of the duration of each state, 
//! avoiding the geometric duration distribution inherent in standard HMMs.

use crate::traits::Next;
use crate::regimes::MarketRegime;
use serde::{Deserialize, Serialize};
use statrs::distribution::{Discrete, Poisson};

/// A distribution for the duration of a hidden state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DurationDistribution {
    /// Poisson distribution with mean lambda
    Poisson { lambda: f64 },
    /// Fixed duration
    Fixed { duration: usize },
}

impl DurationDistribution {
    pub fn p(&self, d: usize) -> f64 {
        match self {
            Self::Poisson { lambda } => {
                let dist = Poisson::new(*lambda).unwrap();
                dist.pmf(d as u64)
            }
            Self::Fixed { duration } => {
                if d == *duration { 1.0 } else { 0.0 }
            }
        }
    }
}

/// A Hidden Semi-Markov Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HSMM {
    pub n_states: usize,
    /// Transition matrix (diagonal must be zero)
    pub a: Vec<Vec<f64>>,
    pub means: Vec<f64>,
    pub stds: Vec<f64>,
    pub durations: Vec<DurationDistribution>,
    /// Time spent in current state
    current_duration: usize,
    last_state: usize,
    initialized: bool,
}

impl HSMM {
    pub fn new(
        a: Vec<Vec<f64>>,
        means: Vec<f64>,
        stds: Vec<f64>,
        durations: Vec<DurationDistribution>,
    ) -> Self {
        Self {
            n_states: a.len(),
            a,
            means,
            stds,
            durations,
            current_duration: 0,
            last_state: 0,
            initialized: false,
        }
    }

    fn gaussian_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
        let variance = sigma * sigma;
        let denom = (2.0 * std::f64::consts::PI * variance).sqrt();
        let exponent = -((x - mu).powi(2)) / (2.0 * variance);
        exponent.exp() / denom
    }
}

impl Next<f64> for HSMM {
    type Output = MarketRegime;

    fn next(&mut self, x: f64) -> Self::Output {
        if !self.initialized {
            self.initialized = true;
            // Start in first state by default
            return MarketRegime::Steady;
        }

        self.current_duration += 1;

        // Calculate probability of staying vs switching
        let prob_stay = self.durations[self.last_state].p(self.current_duration);
        
        let mut max_prob;
        let mut best_state = self.last_state;

        // 1. Evaluate staying in current state
        let emission_stay = Self::gaussian_pdf(x, self.means[self.last_state], self.stds[self.last_state]);
        max_prob = prob_stay * emission_stay;

        // 2. Evaluate switching to other states
        for j in 0..self.n_states {
            if j == self.last_state { continue; }
            
            let transition_prob = self.a[self.last_state][j];
            let emission_j = Self::gaussian_pdf(x, self.means[j], self.stds[j]);
            // Probability of starting new state j (duration 1)
            let prob_j = (1.0 - prob_stay) * transition_prob * self.durations[j].p(1) * emission_j;
            
            if prob_j > max_prob {
                max_prob = prob_j;
                best_state = j;
            }
        }

        if best_state != self.last_state {
            self.last_state = best_state;
            self.current_duration = 1;
        }

        match best_state {
            0 => MarketRegime::Steady,
            1 => MarketRegime::Crisis,
            _ => MarketRegime::Cluster(best_state as u8),
        }
    }
}
