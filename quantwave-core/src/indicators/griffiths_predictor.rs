use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use crate::indicators::super_smoother::SuperSmoother;
use std::collections::VecDeque;

/// Griffiths Predictor
///
/// Based on John Ehlers' "Linear Predictive Filters And Instantaneous Frequency" (TASC January 2025).
/// Uses an adaptive LMS (Griffiths) algorithm to predict future signal values.
#[derive(Debug, Clone)]
pub struct GriffithsPredictor {
    length: usize,
    bars_fwd: usize,
    mu: f64,
    hp: HighPass,
    ss: SuperSmoother,
    peak: f64,
    signal_window: VecDeque<f64>,
    coef: Vec<f64>,
}

impl GriffithsPredictor {
    pub fn new(lower_bound: usize, upper_bound: usize, length: usize, bars_fwd: usize) -> Self {
        Self {
            length,
            bars_fwd,
            mu: 1.0 / (length as f64),
            hp: HighPass::new(upper_bound),
            ss: SuperSmoother::new(lower_bound),
            peak: 0.1,
            signal_window: VecDeque::with_capacity(length + 1),
            coef: vec![0.0; length + 1], // 1-indexed logic compatibility
        }
    }
}

impl Default for GriffithsPredictor {
    fn default() -> Self {
        Self::new(18, 40, 18, 2)
    }
}

impl Next<f64> for GriffithsPredictor {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let hp_val = self.hp.next(input);
        let lp_val = self.ss.next(hp_val);

        // Peak detection
        self.peak *= 0.991;
        if lp_val.abs() > self.peak {
            self.peak = lp_val.abs();
        }

        let signal = if self.peak != 0.0 {
            lp_val / self.peak
        } else {
            0.0
        };

        self.signal_window.push_front(signal);
        if self.signal_window.len() > self.length {
            self.signal_window.pop_back();
        }

        if self.signal_window.len() < self.length {
            return 0.0;
        }

        // Current signal is at index 0 (latest)
        // Previous signals are at indices 1..Length-1
        // Ehlers' XX[Length] is current signal.
        // XX[Length - count] is previous signals.
        // XX[Length - 1] = window[1]
        // XX[Length - length] = window[length]? Wait.
        
        // Let's use Ehlers' indexing directly by copying to a temp vector
        let mut xx = vec![0.0; self.length + 1];
        for i in 1..=self.length {
            xx[i] = self.signal_window[self.length - i];
        }

        let mut x_bar = 0.0;
        for count in 1..=self.length {
            x_bar += xx[self.length - count] * self.coef[count];
        }

        for count in 1..=self.length {
            self.coef[count] += self.mu * (xx[self.length] - x_bar) * xx[self.length - count];
        }

        // Prediction
        let mut x_pred = 0.0;
        let mut xx_temp = xx.clone();
        for _advance in 1..=self.bars_fwd {
            x_pred = 0.0;
            for count in 1..=self.length {
                x_pred += xx_temp[self.length + 1 - count] * self.coef[count];
            }
            
            // Shift
            for count in 1..self.length {
                xx_temp[count] = xx_temp[count + 1];
            }
            xx_temp[self.length] = x_pred;
        }

        x_pred
    }
}

pub const GRIFFITHS_PREDICTOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "GriffithsPredictor",
    description: "Adaptive LMS linear predictive filter for signal forecasting.",
    params: &[
        ParamDef {
            name: "lower_bound",
            default: "18",
            description: "Lower frequency bound (SS length)",
        },
        ParamDef {
            name: "upper_bound",
            default: "40",
            description: "Upper frequency bound (HP length)",
        },
        ParamDef {
            name: "length",
            default: "18",
            description: "LMS filter length",
        },
        ParamDef {
            name: "bars_fwd",
            default: "2",
            description: "Number of bars to predict forward",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html",
    formula_latex: r#"
\[
\mu = 1/L
\]
\[
\bar{x} = \sum_{i=1}^L xx_{L-i} \cdot coef_i
\]
\[
coef_i = coef_i + \mu(xx_L - \bar{x})xx_{L-i}
\]
"#,
    gold_standard_file: "griffiths_predictor.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_griffiths_predictor_basic() {
        let mut gp = GriffithsPredictor::new(18, 40, 18, 2);
        for i in 0..100 {
            let val = gp.next(100.0 + (i as f64 * 0.1).sin());
            assert!(!val.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_griffiths_predictor_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..200),
        ) {
            let lb = 18;
            let ub = 40;
            let length = 18;
            let bars_fwd = 2;
            let mut gp = GriffithsPredictor::new(lb, ub, length, bars_fwd);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| gp.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut hp = HighPass::new(ub);
            let mut ss = SuperSmoother::new(lb);
            let lp_vals: Vec<f64> = inputs.iter().map(|&x| ss.next(hp.next(x))).collect();

            let mut peak = 0.1;
            let mut signals = Vec::new();
            let mut coef = vec![0.0; length + 1];
            let mu = 1.0 / length as f64;

            for (i, &lp_val) in lp_vals.iter().enumerate() {
                peak *= 0.991;
                if lp_val.abs() > peak {
                    peak = lp_val.abs();
                }
                let signal = if peak != 0.0 { lp_val / peak } else { 0.0 };
                signals.push(signal);

                if signals.len() < length {
                    batch_results.push(0.0);
                    continue;
                }

                let mut xx = vec![0.0; length + 1];
                for j in 1..=length {
                    xx[j] = signals[i - (length - j)];
                }

                let mut x_bar = 0.0;
                for count in 1..=length {
                    x_bar += xx[length - count] * coef[count];
                }

                for count in 1..=length {
                    coef[count] += mu * (xx[length] - x_bar) * xx[length - count];
                }

                let mut x_pred = 0.0;
                let mut xx_temp = xx.clone();
                for _advance in 1..=bars_fwd {
                    x_pred = 0.0;
                    for count in 1..=length {
                        x_pred += xx_temp[length + 1 - count] * coef[count];
                    }
                    for count in 1..length {
                        xx_temp[count] = xx_temp[count + 1];
                    }
                    xx_temp[length] = x_pred;
                }
                batch_results.push(x_pred);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
