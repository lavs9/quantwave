//! Changepoint Detection (Killick et al. 2012)
//!
//! Source: Killick, R., Fearnhead, P., & Eckley, I. A. (2012). 
//! "Optimal Detection of Changepoints with a Linear Computational Cost." 
//! Journal of the American Statistical Association, 107(500), 1590-1598.
//!
//! Implementation of the Pruned Exact Linear Time (PELT) algorithm for exact segmentation.
//! PELT identifies change points by minimizing a cost function over all possible partitions.

/// A PELT changepoint detector.
#[derive(Debug, Clone)]
pub struct PELT {
    penalty: f64,
    min_dist: usize,
}

impl PELT {
    /// Creates a new PELT detector.
    ///
    /// # Arguments
    /// * `penalty` - The penalty (beta) for adding a new changepoint (e.g., ln(n)).
    /// * `min_dist` - Minimum distance between changepoints.
    pub fn new(penalty: f64, min_dist: usize) -> Self {
        Self { penalty, min_dist }
    }

    /// Normal log-likelihood cost function for change in mean.
    /// C(y_s:t) = (t-s) * var(y_s:t)
    fn cost(&self, data: &[f64], start: usize, end: usize) -> f64 {
        if end <= start { return 0.0; }
        let n = (end - start) as f64;
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        for i in start..end {
            sum += data[i];
            sum_sq += data[i] * data[i];
        }
        let mean = sum / n;
        let var = (sum_sq / n) - (mean * mean);
        n * var.max(0.0)
    }

    /// Detect changepoints in a batch of data.
    /// Returns indices of changepoints.
    pub fn detect(&self, data: &[f64]) -> Vec<usize> {
        let n = data.len();
        if n < self.min_dist * 2 { return vec![]; }

        let mut f = vec![0.0; n + 1];
        let mut cp = vec![0; n + 1];
        let mut r = vec![0]; // Potential last changepoints

        f[0] = -self.penalty;

        for t in 1..=n {
            let mut min_val = f64::MAX;
            let mut best_tau = 0;

            for &tau in &r {
                if t - tau < self.min_dist { continue; }
                let val = f[tau] + self.cost(data, tau, t) + self.penalty;
                if val < min_val {
                    min_val = val;
                    best_tau = tau;
                }
            }

            f[t] = min_val;
            cp[t] = best_tau;

            // Pruning step
            let mut next_r = vec![0];
            for &tau in &r {
                if f[tau] + self.cost(data, tau, t) <= f[t] {
                    next_r.push(tau);
                }
            }
            next_r.push(t);
            r = next_r;
        }

        // Backtrack to find changepoints
        let mut changepoints = Vec::new();
        let mut curr = cp[n];
        while curr > 0 {
            changepoints.push(curr);
            curr = cp[curr];
        }
        changepoints.sort();
        changepoints
    }
}
