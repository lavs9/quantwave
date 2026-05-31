use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// Kinematic Kalman Filter (2D)
///
/// A second-order Kalman filter that tracks both price (position) and momentum (velocity).
/// By modeling the price as a moving object with velocity, this filter significantly
/// reduces lag during trending markets compared to 1D filters.
///
/// It implements the recursive state-space equations from R.E. Kalman's 1960 paper.
#[derive(Debug, Clone)]
pub struct KinematicKalmanFilter {
    q_pos: f64,
    q_vel: f64,
    r: f64,
    // State: [position, velocity]
    x: f64,
    v: f64,
    // Covariance matrix P (symmetric: p01 == p10)
    p00: f64,
    p01: f64,
    p11: f64,
    initialized: bool,
}

impl KinematicKalmanFilter {
    pub fn new(q_pos: f64, q_vel: f64, r: f64) -> Self {
        Self {
            q_pos,
            q_vel,
            r,
            x: 0.0,
            v: 0.0,
            p00: 1.0,
            p01: 0.0,
            p11: 1.0,
            initialized: false,
        }
    }
}

impl Next<f64> for KinematicKalmanFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if !self.initialized {
            self.x = input;
            self.v = 0.0;
            self.initialized = true;
            return self.x;
        }

        // 1. Prediction (Time Update)
        // Transition matrix F = [[1, 1], [0, 1]]
        // x_pred = x + v
        // v_pred = v
        let x_pred = self.x + self.v;
        let v_pred = self.v;

        // P_pred = F * P * F^T + Q
        // Q = [[q_pos, 0], [0, q_vel]]
        let p00_pred = self.p00 + 2.0 * self.p01 + self.p11 + self.q_pos;
        let p01_pred = self.p01 + self.p11;
        let p11_pred = self.p11 + self.q_vel;

        // 2. Correction (Measurement Update)
        // Measurement matrix H = [1, 0]
        // Innovation S = H * P_pred * H^T + R
        let s = p00_pred + self.r;

        // Kalman Gain K = P_pred * H^T * inv(S)
        let k0 = p00_pred / s;
        let k1 = p01_pred / s;

        // Update State: X = X_pred + K * (input - H * X_pred)
        let innovation = input - x_pred;
        self.x = x_pred + k0 * innovation;
        self.v = v_pred + k1 * innovation;

        // Update Covariance: P = (I - K * H) * P_pred
        // p00 = (1 - k0) * p00_pred
        // p01 = (1 - k0) * p01_pred
        // p11 = -k1 * p01_pred + p11_pred
        self.p00 = (1.0 - k0) * p00_pred;
        self.p01 = (1.0 - k0) * p01_pred;
        self.p11 = -k1 * p01_pred + p11_pred;

        self.x
    }
}

pub const KINEMATIC_KALMAN_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Kinematic Kalman Filter",
    description: "A 2D Kalman filter tracking price and velocity to reduce lag in trends.",
    usage: "Optimized for trend-following strategies where lag reduction is critical. q_pos controls price sensitivity, q_vel controls momentum sensitivity, and r controls overall smoothing.",
    keywords: &[
        "kalman",
        "adaptive",
        "kinematic",
        "momentum",
        "lag-reduction",
    ],
    ehlers_summary: "The Kinematic Kalman Filter extends the 1D model by incorporating a velocity state. This allows the filter to 'anticipate' the next price based on current momentum, providing a zero-lag-like response during strong trends while maintaining smoothness via its optimal error-correction logic.",
    params: &[
        ParamDef {
            name: "q_pos",
            default: "0.001",
            description: "Process noise for position (price)",
        },
        ParamDef {
            name: "q_vel",
            default: "0.0001",
            description: "Process noise for velocity (momentum)",
        },
        ParamDef {
            name: "r",
            default: "0.1",
            description: "Measurement noise (smoothing strength)",
        },
    ],
    formula_source: "https://www.cs.unc.edu/~welch/kalman/media/pdf/Kalman1960.pdf",
    formula_latex: r#"
\[
\hat{x}_{k|k-1} = \Phi \hat{x}_{k-1|k-1}
\]
\[
P_{k|k-1} = \Phi P_{k-1|k-1} \Phi^T + Q
\]
\[
K_k = P_{k|k-1} H^T (H P_{k|k-1} H^T + R)^{-1}
\]
\[
\hat{x}_{k|k} = \hat{x}_{k|k-1} + K_k (z_k - H \hat{x}_{k|k-1})
\]
\[
P_{k|k} = (I - K_k H) P_{k|k-1}
\]
"#,
    gold_standard_file: "kinematic_kalman.json",
    category: "ML Features",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_kinematic_kalman_basic() {
        let mut kf = KinematicKalmanFilter::new(0.001, 0.0001, 0.1);
        let res = kf.next(100.0);
        assert_eq!(res, 100.0); // Initialized to first value

        // Trend up: price goes to 101, 102, 103
        let res2 = kf.next(101.0);
        assert!(res2 > 100.0 && res2 < 101.0);

        let res3 = kf.next(102.0);
        assert!(res3 > res2);
    }

    proptest! {
        #[test]
        fn test_kinematic_kalman_parity(
            inputs in prop::collection::vec(10.0..200.0, 50..100),
        ) {
            let q_pos = 0.001;
            let q_vel = 0.0001;
            let r = 0.1;

            let mut kf = KinematicKalmanFilter::new(q_pos, q_vel, r);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| kf.next(x)).collect();

            // Reference implementation (batch)
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut x = inputs[0];
            let mut v = 0.0;
            let mut p00 = 1.0;
            let mut p01 = 0.0;
            let mut p11 = 1.0;
            batch_results.push(x);

            for i in 1..inputs.len() {
                // Predict
                let x_pred = x + v;
                let v_pred = v;
                let p00_pred = p00 + 2.0 * p01 + p11 + q_pos;
                let p01_pred = p01 + p11;
                let p11_pred = p11 + q_vel;

                // Update
                let s = p00_pred + r;
                let k0 = p00_pred / s;
                let k1 = p01_pred / s;

                let innovation = inputs[i] - x_pred;
                x = x_pred + k0 * innovation;
                v = v_pred + k1 * innovation;

                p00 = (1.0 - k0) * p00_pred;
                p01 = (1.0 - k0) * p01_pred;
                p11 = -k1 * p01_pred + p11_pred;

                batch_results.push(x);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
