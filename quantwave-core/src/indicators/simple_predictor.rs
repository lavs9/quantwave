use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use crate::indicators::super_smoother::SuperSmoother;

/// Simple 2-Pole Predictor
///
/// Based on John Ehlers' "Linear Predictive Filters And Instantaneous Frequency" (TASC January 2025).
/// A non-adaptive 2-pole linear predictive filter using a fixed Q factor.
#[derive(Debug, Clone)]
pub struct SimplePredictor {
    hp: HighPass,
    ss: SuperSmoother,
    q: f64,
    signal_history: [f64; 2],
    count: usize,
}

impl SimplePredictor {
    pub fn new(hp_len: usize, lp_len: usize, q: f64) -> Self {
        Self {
            hp: HighPass::new(hp_len),
            ss: SuperSmoother::new(lp_len),
            q,
            signal_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Default for SimplePredictor {
    fn default() -> Self {
        Self::new(15, 30, 0.35)
    }
}

impl Next<f64> for SimplePredictor {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let signal = self.ss.next(self.hp.next(input));

        let c1 = 1.8 * self.q;
        let c2 = -self.q * self.q;
        let sum = 1.0 - c1 - c2;

        let res = if self.count < 3 {
            signal
        } else {
            // Predict = (Signal - c1*Signal[1] - c2*Signal[2]) / sum
            // Note: Pine script: 
            // c0 = (1.0 / sum) * Signal
            // c1_ = (c1 / sum) * Signal[1]
            // c2_ = (c2 / sum) * Signal[2]
            // Predict = c0 - c1_ - c2_
            (signal - c1 * self.signal_history[0] - c2 * self.signal_history[1]) / sum
        };

        self.signal_history[1] = self.signal_history[0];
        self.signal_history[0] = signal;
        
        res
    }
}

pub const SIMPLE_PREDICTOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "SimplePredictor",
    description: "A fixed-coefficient 2-pole linear predictive filter.",
    usage: "Use as a lightweight one-bar-ahead price predictor for cycle-mode markets. Its low computational cost makes it suitable for real-time streaming at high frequency.",
    keywords: &["prediction", "cycle", "ehlers", "dsp"],
    ehlers_summary: "Ehlers derives a Simple Predictor that extrapolates price one bar forward using only the current and prior bars weighted by the dominant cycle coefficient. Despite its simplicity it provides useful one-bar forecasts in cycling markets, demonstrating the predictive value of cycle measurement.",
    params: &[
        ParamDef {
            name: "hp_len",
            default: "15",
            description: "HighPass filter length",
        },
        ParamDef {
            name: "lp_len",
            default: "30",
            description: "LowPass (SuperSmoother) length",
        },
        ParamDef {
            name: "q",
            default: "0.35",
            description: "Damping/Predictor coefficient",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html",
    formula_latex: r#"
\[
Predict = \frac{Signal - 1.8Q \cdot Signal_{t-1} + Q^2 \cdot Signal_{t-2}}{1 - 1.8Q + Q^2}
\]
"#,
    gold_standard_file: "simple_predictor.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_simple_predictor_basic() {
        let mut sp = SimplePredictor::new(15, 30, 0.35);
        for i in 0..50 {
            let val = sp.next(100.0 + i as f64);
            assert!(!val.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_simple_predictor_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let hp_len = 15;
            let lp_len = 30;
            let q = 0.35;
            let mut sp = SimplePredictor::new(hp_len, lp_len, q);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| sp.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut hp = HighPass::new(hp_len);
            let mut ss = SuperSmoother::new(lp_len);
            let signal_vals: Vec<f64> = inputs.iter().map(|&x| ss.next(hp.next(x))).collect();

            let c1 = 1.8 * q;
            let c2 = -q * q;
            let sum = 1.0 - c1 - c2;

            for (i, &signal) in signal_vals.iter().enumerate() {
                let bar = i + 1;
                let res = if bar < 3 {
                    signal
                } else {
                    (signal - c1 * signal_vals[i-1] - c2 * signal_vals[i-2]) / sum
                };
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
