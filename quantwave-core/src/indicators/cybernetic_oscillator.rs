use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use crate::indicators::super_smoother::SuperSmoother;
use std::collections::VecDeque;

/// Cybernetic Oscillator
///
/// Based on John Ehlers' "Making A Better Oscillator" (TASC June 2025).
/// Combines a HighPass filter and a SuperSmoother, then normalizes by RMS.
#[derive(Debug, Clone)]
pub struct CyberneticOscillator {
    hp: HighPass,
    ss: SuperSmoother,
    rms_window: VecDeque<f64>,
    rms_len: usize,
    sum_sq: f64,
}

impl CyberneticOscillator {
    pub fn new(hp_length: usize, lp_length: usize, rms_len: usize) -> Self {
        Self {
            hp: HighPass::new(hp_length),
            ss: SuperSmoother::new(lp_length),
            rms_window: VecDeque::with_capacity(rms_len),
            rms_len,
            sum_sq: 0.0,
        }
    }
}

impl Default for CyberneticOscillator {
    fn default() -> Self {
        Self::new(30, 20, 100)
    }
}

impl Next<f64> for CyberneticOscillator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let hp_val = self.hp.next(input);
        let lp_val = self.ss.next(hp_val);

        // Update RMS
        let val_sq = lp_val * lp_val;
        self.rms_window.push_back(lp_val);
        self.sum_sq += val_sq;

        if self.rms_window.len() > self.rms_len {
            let oldest = self.rms_window.pop_front().unwrap();
            self.sum_sq -= oldest * oldest;
        }

        // Avoid precision issues resulting in small negative sum_sq
        if self.sum_sq < 0.0 {
            self.sum_sq = 0.0;
        }

        let rms = (self.sum_sq / self.rms_len as f64).sqrt();

        if rms != 0.0 {
            lp_val / rms
        } else {
            0.0
        }
    }
}

pub const CYBERNETIC_OSCILLATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "CyberneticOscillator",
    description: "Combined HighPass and SuperSmoother filters normalized by RMS.",
    usage: "Use as a generalized Ehlers cycle oscillator when you need a configurable bandpass response tuned to a specific dominant cycle period.",
    keywords: &["oscillator", "ehlers", "dsp", "cycle", "momentum"],
    ehlers_summary: "The Cybernetic Oscillator is derived from the bandpass filter framework in Ehlers Cybernetic Analysis for Stocks and Futures (2004). By tuning the filter center frequency to the measured dominant cycle period, it extracts only the cyclical component and presents it as an oscillator ranging above and below zero.",
    params: &[
        ParamDef {
            name: "hp_length",
            default: "30",
            description: "HighPass filter length",
        },
        ParamDef {
            name: "lp_length",
            default: "20",
            description: "LowPass (SuperSmoother) length",
        },
        ParamDef {
            name: "rms_len",
            default: "100",
            description: "RMS normalization length",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JUNE%202025.html",
    formula_latex: r#"
\[
HP = HighPass(Price, HPLen)
\]
\[
LP = SuperSmoother(HP, LPLen)
\]
\[
RMS = \sqrt{\frac{1}{N} \sum_{i=0}^{N-1} LP_{t-i}^2}
\]
\[
CO = \frac{LP}{RMS}
\]
"#,
    gold_standard_file: "cybernetic_oscillator.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_cybernetic_oscillator_basic() {
        let mut co = CyberneticOscillator::new(30, 20, 100);
        for i in 0..150 {
            let val = co.next(100.0 + (i as f64).sin());
            assert!(!val.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_cybernetic_oscillator_parity(
            inputs in prop::collection::vec(1.0..100.0, 150..250),
        ) {
            let hp_len = 30;
            let lp_len = 20;
            let rms_len = 100;
            let mut co = CyberneticOscillator::new(hp_len, lp_len, rms_len);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| co.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            
            let mut hp = HighPass::new(hp_len);
            let mut ss = SuperSmoother::new(lp_len);
            let lp_vals: Vec<f64> = inputs.iter().map(|&x| ss.next(hp.next(x))).collect();

            for i in 0..lp_vals.len() {
                let start = if i >= rms_len - 1 { i + 1 - rms_len } else { 0 };
                let window = &lp_vals[start..i + 1];
                
                let mut sum_sq = 0.0;
                for &v in window {
                    sum_sq += v * v;
                }
                
                // Note: The denominator in Ehlers' EasyLanguage is constant (Length)
                // whereas the window may be smaller initially. 
                // But Ehlers' code: $RMS = SquareRoot(SumSq / Length)
                // So we always divide by rms_len.
                let rms = (sum_sq / rms_len as f64).sqrt();
                
                if rms != 0.0 {
                    batch_results.push(lp_vals[i] / rms);
                } else {
                    batch_results.push(0.0);
                }
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
