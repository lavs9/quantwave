use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// BandPass Filter
///
/// Based on John Ehlers' "Empirical Mode Decomposition" and "Fourier Series Model".
/// Isolates cyclic components within a specific frequency band.
#[derive(Debug, Clone)]
pub struct BandPass {
    alpha: f64,
    beta: f64,
    price_prev1: f64,
    price_prev2: f64,
    bp_history: [f64; 2],
    count: usize,
}

impl BandPass {
    pub fn new(period: usize, bandwidth: f64) -> Self {
        let beta = (2.0 * PI / period as f64).cos();
        let gamma = 1.0 / (2.0 * PI * bandwidth / period as f64).cos();
        let alpha = gamma - (gamma * gamma - 1.0).sqrt();

        Self {
            alpha,
            beta,
            price_prev1: 0.0,
            price_prev2: 0.0,
            bp_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Next<f64> for BandPass {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;

        // BP = .5*(1 - alpha)*(Price - Price[2]) + beta*(1 + alpha)*BP[1] - alpha*BP[2];
        let bp = 0.5 * (1.0 - self.alpha) * (input - self.price_prev2)
            + self.beta * (1.0 + self.alpha) * self.bp_history[0]
            - self.alpha * self.bp_history[1];

        self.bp_history[1] = self.bp_history[0];
        self.bp_history[0] = bp;
        self.price_prev2 = self.price_prev1;
        self.price_prev1 = input;

        bp
    }
}

pub const BANDPASS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "BandPass",
    description: "A bandpass filter that isolates cycle components around a center period.",
    params: &[
        ParamDef {
            name: "period",
            default: "20",
            description: "Center period of the passband",
        },
        ParamDef {
            name: "bandwidth",
            default: "0.1",
            description: "Relative bandwidth (delta)",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EmpiricalModeDecomposition.pdf",
    formula_latex: r#"
\[
\beta = \cos(360/P), \gamma = 1/\cos(720\delta/P), \alpha = \gamma - \sqrt{\gamma^2 - 1}
\]
\[
BP = 0.5(1 - \alpha)(Price - Price_{t-2}) + \beta(1 + \alpha)BP_{t-1} - \alpha BP_{t-2}
\]
"#,
    gold_standard_file: "bandpass.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_bandpass_basic() {
        let mut bp = BandPass::new(20, 0.1);
        for i in 0..50 {
            let val = bp.next(100.0 + (i as f64 * 0.1).sin());
            assert!(!val.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_bandpass_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let period = 20;
            let bandwidth = 0.1;
            let mut bp_obj = BandPass::new(period, bandwidth);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| bp_obj.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let beta = (2.0 * PI / period as f64).cos();
            let _gamma = 1.0 / (2.0 * PI * 2.0 * bandwidth / period as f64).cos(); // Ehlers uses 720*delta
            // Wait, Ehlers uses 360/P for Cosine (degrees) which is 2*PI/P for cos (radians).
            // Ehlers uses 720*delta/P for Cosine which is 4*PI*delta/P for cos.
            // My alpha/beta in code:
            // beta = cos(2*PI/P)
            // gamma = 1/cos(4*PI*delta/P)
            
            let alpha = {
                let g = 1.0 / (2.0 * PI * bandwidth / period as f64).cos();
                g - (g * g - 1.0).sqrt()
            };

            let mut p_hist = vec![0.0; inputs.len() + 2];
            let mut b_hist = vec![0.0; inputs.len() + 2];

            for (i, &input) in inputs.iter().enumerate() {
                let idx = i + 2;
                p_hist[idx] = input;
                let bp = 0.5 * (1.0 - alpha) * (p_hist[idx] - p_hist[idx-2])
                    + beta * (1.0 + alpha) * b_hist[idx-1]
                    - alpha * b_hist[idx-2];
                b_hist[idx] = bp;
                batch_results.push(bp);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
