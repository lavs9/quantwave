//! Core Analytics and Diagnostics for Market Regimes
//! 
//! This module provides tools to analyze regime persistence, transitions, and stability.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use nalgebra::DMatrix;

/// Statistics regarding the duration of a specific regime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationStats {
    pub regime_id: u32,
    pub mean_duration: f64,
    pub median_duration: f64,
    pub std_duration: f64,
    pub max_duration: usize,
    pub total_observations: usize,
}

/// Core analytical tools for analyzing regime sequences.
pub struct RegimeAnalytics;

impl RegimeAnalytics {
    /// Constructs an empirical transition matrix from a sequence of regime states.
    /// 
    /// The matrix `T[i][j]` represents the probability of transitioning from state `i` to state `j`.
    pub fn transition_matrix(states: &[u32], num_states: usize) -> Vec<Vec<f64>> {
        let mut transitions = vec![vec![0usize; num_states]; num_states];
        let mut row_totals = vec![0usize; num_states];

        for pair in states.windows(2) {
            let from = pair[0] as usize;
            let to = pair[1] as usize;
            if from < num_states && to < num_states {
                transitions[from][to] += 1;
                row_totals[from] += 1;
            }
        }

        let mut matrix = vec![vec![0.0; num_states]; num_states];
        for i in 0..num_states {
            if row_totals[i] > 0 {
                for j in 0..num_states {
                    matrix[i][j] = transitions[i][j] as f64 / row_totals[i] as f64;
                }
            }
        }
        matrix
    }

    /// Forecasts the state probability distribution `n` steps ahead.
    /// 
    /// Uses matrix exponentiation (powering) of the transition matrix.
    pub fn forecast_state(
        transition_matrix: &[Vec<f64>],
        current_state: u32,
        steps: usize,
    ) -> Vec<f64> {
        let n = transition_matrix.len();
        if n == 0 { return vec![]; }
        
        let mut mat_data = Vec::with_capacity(n * n);
        for row in transition_matrix {
            mat_data.extend_from_slice(row);
        }
        
        // nalgebra DMatrix is column-major by default in some constructors, 
        // but from_row_slice makes it clear.
        let m = DMatrix::from_row_slice(n, n, &mat_data);
        let m_n = m.pow(steps as u32);

        let mut initial_dist = vec![0.0; n];
        if (current_state as usize) < n {
            initial_dist[current_state as usize] = 1.0;
        } else {
            return vec![0.0; n];
        }

        let v = nalgebra::DVector::from_vec(initial_dist);
        // Distribution after n steps: v^T * M^n (if using row vectors)
        // Or M^T * v (if using column vectors)
        // nalgebra's pow and vector multiplication
        let result = m_n.transpose() * v;
        result.as_slice().to_vec()
    }

    /// Calculates duration statistics for each regime in the sequence.
    pub fn duration_stats(states: &[u32], num_states: usize) -> Vec<DurationStats> {
        let mut durations: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        
        if states.is_empty() { return vec![]; }

        let mut current_regime = states[0];
        let mut current_duration = 1;

        for &state in &states[1..] {
            if state == current_regime {
                current_duration += 1;
            } else {
                durations.entry(current_regime).or_default().push(current_duration);
                current_regime = state;
                current_duration = 1;
            }
        }
        durations.entry(current_regime).or_default().push(current_duration);

        let mut results = Vec::new();
        for i in 0..num_states as u32 {
            if let Some(d_list) = durations.get(&i) {
                let total_obs: usize = d_list.iter().sum();
                let n = d_list.len() as f64;
                let mean = total_obs as f64 / n;
                
                let mut sorted = d_list.clone();
                sorted.sort_unstable();
                let median = sorted[sorted.len() / 2] as f64;
                let max_dur = *sorted.last().unwrap_or(&0);

                let variance = d_list.iter()
                    .map(|&d| (d as f64 - mean).powi(2))
                    .sum::<f64>() / n;
                let std = variance.sqrt();

                results.push(DurationStats {
                    regime_id: i,
                    mean_duration: mean,
                    median_duration: median,
                    std_duration: std,
                    max_duration: max_dur,
                    total_observations: total_obs,
                });
            }
        }
        results
    }

    /// Calculates a stability score (0.0 to 1.0).
    /// 
    /// Higher scores indicate fewer regime switches relative to the total sequence length.
    pub fn stability_score(states: &[u32]) -> f64 {
        if states.len() < 2 { return 1.0; }
        
        let mut switches = 0;
        for pair in states.windows(2) {
            if pair[0] != pair[1] {
                switches += 1;
            }
        }
        
        1.0 - (switches as f64 / (states.len() - 1) as f64)
    }
}
