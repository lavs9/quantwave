use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// Kalman Filter
///
/// A 1D Kalman filter used for adaptive smoothing.
/// It models the true price as a hidden state and updates its estimate
/// based on the process noise (Q) and measurement noise (R).
#[derive(Debug, Clone)]
pub struct KalmanFilter {
    q: f64,
    r: f64,
    x: f64,
    p: f64,
    k: f64,
    initialized: bool,
}

impl KalmanFilter {
    pub fn new(q: f64, r: f64) -> Self {
        Self {
            q,
            r,
            x: 0.0,
            p: 1.0,
            k: 0.0,
            initialized: false,
        }
    }
}

impl Next<f64> for KalmanFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if !self.initialized {
            self.x = input;
            self.p = 1.0;
            self.initialized = true;
            return self.x;
        }

        // Prediction
        // x = x (constant position model)
        // p = p + q
        let p_pred = self.p + self.q;

        // Update
        // k = p_pred / (p_pred + r)
        self.k = p_pred / (p_pred + self.r);
        
        // x = x + k * (input - x)
        self.x = self.x + self.k * (input - self.x);
        
        // p = (1 - k) * p_pred
        self.p = (1.0 - self.k) * p_pred;

        self.x
    }
}

pub const KALMAN_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Kalman Filter",
    description: "An adaptive 1D Kalman filter for smoothing price data with minimal lag.",
    usage: "Use as a highly responsive alternative to moving averages. The Q parameter (process noise) controls responsiveness to trend changes, while R (measurement noise) controls smoothness. Higher Q makes it track price faster; higher R increases smoothing.",
    keywords: &["filter", "adaptive", "smoothing", "ml", "kalman"],
    ehlers_summary: "The Kalman Filter is an optimal estimator for linear systems with Gaussian noise. In technical analysis, the 1D version recursively updates the estimate of the 'true' price by balancing the predicted state against new measurements. It is particularly effective for feature engineering in ML models due to its ability to separate signal from noise dynamically.",
    params: &[
        ParamDef {
            name: "q",
            default: "0.01",
            description: "Process noise (responsiveness)",
        },
        ParamDef {
            name: "r",
            default: "0.1",
            description: "Measurement noise (smoothing)",
        },
    ],
    formula_source: "https://en.wikipedia.org/wiki/Kalman_filter",
    formula_latex: r#"
\[
P_{t|t-1} = P_{t-1} + Q
\]
\[
K_t = \frac{P_{t|t-1}}{P_{t|t-1} + R}
\]
\[
X_t = X_{t-1} + K_t(Z_t - X_{t-1})
\]
\[
P_t = (1 - K_t)P_{t|t-1}
\]
"#,
    gold_standard_file: "kalman_filter.json",
    category: "ML Features",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_kalman_basic() {
        let mut kf = KalmanFilter::new(0.01, 0.1);
        let res = kf.next(100.0);
        assert_eq!(res, 100.0); // First value
        let res2 = kf.next(101.0);
        assert!(res2 > 100.0 && res2 < 101.0);
    }

    proptest! {
        #[test]
        fn test_kalman_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let q = 0.01;
            let r = 0.1;
            let mut kf = KalmanFilter::new(q, r);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| kf.next(x)).collect();

            // Reference implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut x = inputs[0];
            let mut p = 1.0;
            batch_results.push(x);

            for i in 1..inputs.len() {
                let p_pred = p + q;
                let k = p_pred / (p_pred + r);
                x = x + k * (inputs[i] - x);
                p = (1.0 - k) * p_pred;
                batch_results.push(x);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
