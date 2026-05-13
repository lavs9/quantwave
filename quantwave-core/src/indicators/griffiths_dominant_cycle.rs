use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use crate::indicators::super_smoother::SuperSmoother;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Griffiths Dominant Cycle
///
/// Based on John Ehlers' "Linear Predictive Filters And Instantaneous Frequency" (TASC January 2025).
/// Uses the Griffiths spectral estimation to find the dominant cycle in the data.
#[derive(Debug, Clone)]
pub struct GriffithsDominantCycle {
    lb: usize,
    ub: usize,
    length: usize,
    mu: f64,
    hp: HighPass,
    ss: SuperSmoother,
    peak: f64,
    signal_window: VecDeque<f64>,
    coef: Vec<f64>,
    prev_cycle: f64,
}

impl GriffithsDominantCycle {
    pub fn new(lower_bound: usize, upper_bound: usize, length: usize) -> Self {
        Self {
            lb: lower_bound,
            ub: upper_bound,
            length,
            mu: 1.0 / (length as f64),
            hp: HighPass::new(upper_bound),
            ss: SuperSmoother::new(lower_bound),
            peak: 0.1,
            signal_window: VecDeque::with_capacity(length + 1),
            coef: vec![0.0; length + 1],
            prev_cycle: (lower_bound + upper_bound) as f64 / 2.0,
        }
    }
}

impl Default for GriffithsDominantCycle {
    fn default() -> Self {
        Self::new(18, 40, 40)
    }
}

impl Next<f64> for GriffithsDominantCycle {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let hp_val = self.hp.next(input);
        let lp_val = self.ss.next(hp_val);

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
            return self.prev_cycle;
        }

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

        // Spectral scan
        let mut max_pwr = 0.0;
        let mut cycle = self.prev_cycle;

        for period_idx in self.lb..=self.ub {
            let period = period_idx as f64;
            let mut real = 0.0;
            let mut imag = 0.0;

            for count in 1..=self.length {
                let angle = 2.0 * PI * (count as f64) / period;
                real += self.coef[count] * angle.cos();
                imag += self.coef[count] * angle.sin();
            }

            let denom = (1.0 - real).powi(2) + imag.powi(2);
            let pwr = 0.1 / denom;

            if pwr > max_pwr {
                max_pwr = pwr;
                cycle = period;
            }
        }

        // Slew rate limiter
        if cycle > self.prev_cycle + 2.0 {
            cycle = self.prev_cycle + 2.0;
        } else if cycle < self.prev_cycle - 2.0 {
            cycle = self.prev_cycle - 2.0;
        }

        self.prev_cycle = cycle;
        cycle
    }
}

pub const GRIFFITHS_DOMINANT_CYCLE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "GriffithsDominantCycle",
    description: "Dominant cycle estimation using Griffiths adaptive spectral analysis.",
    params: &[
        ParamDef {
            name: "lower_bound",
            default: "18",
            description: "Lower period bound",
        },
        ParamDef {
            name: "upper_bound",
            default: "40",
            description: "Upper period bound",
        },
        ParamDef {
            name: "length",
            default: "40",
            description: "LMS filter length",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html",
    formula_latex: r#"
\[
Pwr(Period) = \frac{0.1}{(1-Real)^2 + Imag^2}
\]
\[
Real = \sum coef_i \cos(2\pi i / Period)
\]
\[
Imag = \sum coef_i \sin(2\pi i / Period)
\]
"#,
    gold_standard_file: "griffiths_dominant_cycle.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_griffiths_dc_basic() {
        let mut gdc = GriffithsDominantCycle::new(18, 40, 40);
        for i in 0..200 {
            // Sine wave with period 30
            let val = gdc.next((2.0 * PI * i as f64 / 30.0).sin());
            if i > 150 {
                // Should converge towards 30
                assert!(val > 25.0 && val < 35.0);
            }
        }
    }

    proptest! {
        #[test]
        fn test_griffiths_dc_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..200),
        ) {
            let lb = 18;
            let ub = 40;
            let length = 40;
            let mut gdc = GriffithsDominantCycle::new(lb, ub, length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| gdc.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut hp = HighPass::new(ub);
            let mut ss = SuperSmoother::new(lb);
            let lp_vals: Vec<f64> = inputs.iter().map(|&x| ss.next(hp.next(x))).collect();

            let mut peak = 0.1;
            let mut signals = Vec::new();
            let mut coef = vec![0.0; length + 1];
            let mu = 1.0 / length as f64;
            let mut prev_cycle = (lb + ub) as f64 / 2.0;

            for (i, &lp_val) in lp_vals.iter().enumerate() {
                peak *= 0.991;
                if lp_val.abs() > peak {
                    peak = lp_val.abs();
                }
                let signal = if peak != 0.0 { lp_val / peak } else { 0.0 };
                signals.push(signal);

                if signals.len() < length {
                    batch_results.push(prev_cycle);
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

                let mut max_pwr = 0.0;
                let mut cycle = prev_cycle;

                for period_idx in lb..=ub {
                    let period = period_idx as f64;
                    let mut real = 0.0;
                    let mut imag = 0.0;
                    for count in 1..=length {
                        let angle = 2.0 * PI * (count as f64) / period;
                        real += coef[count] * angle.cos();
                        imag += coef[count] * angle.sin();
                    }
                    let denom = (1.0 - real).powi(2) + imag.powi(2);
                    let pwr = 0.1 / denom;
                    if pwr > max_pwr {
                        max_pwr = pwr;
                        cycle = period;
                    }
                }

                if cycle > prev_cycle + 2.0 {
                    cycle = prev_cycle + 2.0;
                } else if cycle < prev_cycle - 2.0 {
                    cycle = prev_cycle - 2.0;
                }
                
                prev_cycle = cycle;
                batch_results.push(cycle);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
