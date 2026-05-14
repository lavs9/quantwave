use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use crate::indicators::super_smoother::SuperSmoother;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Griffiths Spectrum
///
/// Based on John Ehlers' "Linear Predictive Filters And Instantaneous Frequency" (TASC January 2025).
/// It computes the normalized power spectrum using Griffiths adaptive filter coefficients.
#[derive(Debug, Clone)]
pub struct GriffithsSpectrum {
    lb: usize,
    ub: usize,
    length: usize,
    mu: f64,
    hp: HighPass,
    ss: SuperSmoother,
    peak: f64,
    signal_window: VecDeque<f64>,
    coef: Vec<f64>,
}

impl GriffithsSpectrum {
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
        }
    }
}

impl Default for GriffithsSpectrum {
    fn default() -> Self {
        Self::new(18, 40, 40)
    }
}

impl Next<f64> for GriffithsSpectrum {
    type Output = Vec<f64>; // Power for each period from lb to ub

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

        let mut results = vec![0.0; self.ub - self.lb + 1];

        if self.signal_window.len() < self.length {
            return results;
        }

        let mut xx = vec![0.0; self.length + 1];
        for (i, val) in xx.iter_mut().enumerate().skip(1).take(self.length) {
            *val = self.signal_window[self.length - i];
        }

        let mut x_bar = 0.0;
        for count in 1..=self.length {
            x_bar += xx[self.length - count] * self.coef[count];
        }

        for count in 1..=self.length {
            self.coef[count] += self.mu * (xx[self.length] - x_bar) * xx[self.length - count];
        }

        let mut max_pwr = 0.0;
        let mut powers = Vec::with_capacity(self.ub - self.lb + 1);

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
            }
            powers.push(pwr);
        }

        if max_pwr != 0.0 {
            for (i, pwr) in powers.into_iter().enumerate() {
                results[i] = pwr / max_pwr;
            }
        }

        results
    }
}

pub const GRIFFITHS_SPECTRUM_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "GriffithsSpectrum",
    description: "Normalized power spectrum estimation using Griffiths adaptive filters.",
    usage: "Use to generate a high-resolution periodogram for cycle analysis. Best visualized as a heatmap to identify and track multiple market cycles simultaneously.",
    keywords: &["spectrum", "cycle", "ehlers", "dsp", "periodogram"],
    ehlers_summary: "The Griffiths Spectrum is an adaptive spectral estimation method that provides higher resolution than a standard DFT for short data segments. It fits an all-pole model to the signal using an LMS algorithm, allowing for instantaneous frequency measurement without the windowing artifacts of FFT-based methods.",
    params: &[
        ParamDef { name: "lower_bound", default: "18", description: "Lower period bound" },
        ParamDef { name: "upper_bound", default: "40", description: "Upper period bound" },
        ParamDef { name: "length", default: "40", description: "LMS filter length" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202025.html",
    formula_latex: r#"
\[
Pwr(P) = \frac{0.1}{(1 - \sum coef_i \cos(2\pi i/P))^2 + (\sum coef_i \sin(2\pi i/P))^2}
\]
\[
Pwr_{norm}(P) = \frac{Pwr(P)}{\max(Pwr)}
\]
"#,
    gold_standard_file: "griffiths_spectrum.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    // use crate::test_utils;
    // use crate::test_utils::{load_gold_standard_vec, assert_indicator_parity_vec};
    use proptest::prelude::*;

    /*
    #[test]
    fn test_griffiths_spectrum_gold_standard() {
        let case = load_gold_standard_vec("griffiths_spectrum");
        let gs = GriffithsSpectrum::new(18, 40, 40);
        assert_indicator_parity_vec(gs, &case.input, &case.expected);
    }
    */
    // TODO: Restore test once griffiths_spectrum.json is recovered.

    #[test]
    fn test_griffiths_spectrum_basic() {
        let mut gs = GriffithsSpectrum::new(18, 40, 40);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = gs.next(input);
            assert_eq!(res.len(), 40 - 18 + 1);
        }
    }

    proptest! {
        #[test]
        fn test_griffiths_spectrum_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..200),
        ) {
            let lb = 18;
            let ub = 40;
            let length = 40;
            let mut gs = GriffithsSpectrum::new(lb, ub, length);
            let streaming_results: Vec<Vec<f64>> = inputs.iter().map(|&x| gs.next(x)).collect();

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
                    batch_results.push(vec![0.0; ub - lb + 1]);
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

                let mut powers = Vec::new();
                let mut max_pwr = 0.0;
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
                    if pwr > max_pwr { max_pwr = pwr; }
                    powers.push(pwr);
                }
                
                let norm_powers = if max_pwr != 0.0 {
                    powers.into_iter().map(|p| p / max_pwr).collect()
                } else {
                    vec![0.0; ub - lb + 1]
                };
                batch_results.push(norm_powers);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                for (sv, bv) in s.iter().zip(b.iter()) {
                    approx::assert_relative_eq!(sv, bv, epsilon = 1e-10);
                }
            }
        }
    }
}
