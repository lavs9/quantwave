use crate::indicators::math::RMS;
use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::ultimate_smoother::UltimateSmoother;
use crate::traits::Next;

/// Laguerre Oscillator
///
/// Based on John Ehlers' "Laguerre Filters" (TASC July 2025).
/// The Laguerre Oscillator is a low-lag trend indicator where values above zero
/// generally correspond to upward movement and values below zero to downward movement.
#[derive(Debug, Clone)]
pub struct LaguerreOscillator {
    us: UltimateSmoother,
    rms: RMS,
    gamma: f64,
    l1: f64,
    prev_l0: f64,
    count: usize,
}

impl LaguerreOscillator {
    pub fn new(length: usize, gamma: f64, rms_period: usize) -> Self {
        Self {
            us: UltimateSmoother::new(length),
            rms: RMS::new(rms_period),
            gamma,
            l1: 0.0,
            prev_l0: 0.0,
            count: 0,
        }
    }
}

impl Next<f64> for LaguerreOscillator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let l0 = self.us.next(input);
        self.count += 1;

        if self.count == 1 {
            self.prev_l0 = l0;
            self.l1 = l0;
            let _ = self.rms.next(0.0);
            return 0.0;
        }

        // L1 = -Gama * L0 + L0[1] + Gama * L1[1];
        let next_l1 = -self.gamma * l0 + self.prev_l0 + self.gamma * self.l1;

        let diff = l0 - next_l1;
        let rms_val = self.rms.next(diff);

        let res = if rms_val != 0.0 { diff / rms_val } else { 0.0 };

        self.l1 = next_l1;
        self.prev_l0 = l0;

        res
    }
}

pub const LAGUERRE_OSCILLATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Laguerre Oscillator",
    description: "A low-lag trend oscillator derived from Laguerre polynomials and normalized by RMS volatility.",
    params: &[
        ParamDef {
            name: "length",
            default: "30",
            description: "UltimateSmoother period",
        },
        ParamDef {
            name: "gamma",
            default: "0.5",
            description: "Smoothing factor",
        },
        ParamDef {
            name: "rms_period",
            default: "100",
            description: "RMS normalization period",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JULY%202025.html",
    formula_latex: r#"
\[
L_0 = UltimateSmoother(Close, Length)
\]
\[
L_1 = -\gamma L_0 + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
RMS = \sqrt{\frac{1}{n}\sum (L_0 - L_1)^2}
\]
\[
Osc = (L_0 - L_1) / RMS
\]
"#,
    gold_standard_file: "laguerre_oscillator.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_laguerre_oscillator_basic() {
        let mut lo = LaguerreOscillator::new(30, 0.5, 100);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        for input in inputs {
            let res = lo.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_laguerre_oscillator_parity(
            inputs in prop::collection::vec(1.0..100.0, 110..200),
        ) {
            let length = 30;
            let gamma = 0.5;
            let rms_period = 100;
            let mut lo = LaguerreOscillator::new(length, gamma, rms_period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| lo.next(x)).collect();

            // Reference implementation
            let mut us = UltimateSmoother::new(length);
            let l0_vals: Vec<f64> = inputs.iter().map(|&x| us.next(x)).collect();

            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut l1 = 0.0;
            let mut diffs = Vec::new();

            for (i, &l0) in l0_vals.iter().enumerate() {
                if i == 0 {
                    l1 = l0;
                    diffs.push(0.0);
                    batch_results.push(0.0);
                } else {
                    let prev_l0 = l0_vals[i-1];
                    l1 = -gamma * l0 + prev_l0 + gamma * l1;
                    let diff = l0 - l1;
                    diffs.push(diff);

                    let start = if diffs.len() > rms_period { diffs.len() - rms_period } else { 0 };
                    let window = &diffs[start..];
                    let sum_sq: f64 = window.iter().map(|&x| x*x).sum();
                    let rms = (sum_sq / window.len() as f64).sqrt();

                    let res = if rms != 0.0 { diff / rms } else { 0.0 };
                    batch_results.push(res);
                }
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
