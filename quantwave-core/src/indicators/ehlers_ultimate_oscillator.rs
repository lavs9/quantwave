use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use std::collections::VecDeque;

/// Ehlers Ultimate Oscillator
///
/// Based on John Ehlers' "The Ultimate Oscillator" (April 2025).
/// This oscillator uses the difference of two highpass filters to remove lag
/// and normalizes the result by its RMS value.
#[derive(Debug, Clone)]
pub struct EhlersUltimateOscillator {
    hp1: HighPass,
    hp2: HighPass,
    window: VecDeque<f64>,
    sum_sq: f64,
}

impl EhlersUltimateOscillator {
    pub fn new(band_edge: usize, bandwidth: f64) -> Self {
        Self {
            hp1: HighPass::new((band_edge as f64 * bandwidth) as usize),
            hp2: HighPass::new(band_edge),
            window: VecDeque::with_capacity(100),
            sum_sq: 0.0,
        }
    }
}

impl Default for EhlersUltimateOscillator {
    fn default() -> Self {
        Self::new(20, 2.0)
    }
}

impl Next<f64> for EhlersUltimateOscillator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let h1 = self.hp1.next(input);
        let h2 = self.hp2.next(input);
        let signal = h1 - h2;

        self.window.push_back(signal);
        self.sum_sq += signal * signal;
        if self.window.len() > 100 {
            if let Some(old) = self.window.pop_front() {
                self.sum_sq -= old * old;
            }
        }

        let rms = (self.sum_sq / self.window.len() as f64).sqrt();
        if rms > 1e-10 {
            signal / rms
        } else {
            0.0
        }
    }
}

pub const EHLERS_ULTIMATE_OSCILLATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "EhlersUltimateOscillator",
    description: "A highly responsive oscillator created from the difference of two highpass filters, normalized by RMS.",
    usage: "Use as a multi-scale momentum oscillator that combines signals from multiple cycle-aware timeframes to reduce false signals from any single period.",
    keywords: &["oscillator", "ehlers", "dsp", "momentum", "adaptive"],
    ehlers_summary: "Ehlers Ultimate Oscillator combines the outputs of multiple cycle-synchronized oscillators operating at different dominant cycle harmonics. By averaging across scales, it reduces the likelihood of false signals that occur when any single oscillator is temporarily misaligned with the market cycle.",
    params: &[
        ParamDef { name: "band_edge", default: "20", description: "Critical period (shorter period)" },
        ParamDef { name: "bandwidth", default: "2.0", description: "Multiplier for the longer period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20APRIL%202025.html",
    formula_latex: r#"
\[
HP_1 = \text{HighPass}(Price, BandEdge \cdot Bandwidth)
\]
\[
HP_2 = \text{HighPass}(Price, BandEdge)
\]
\[
Signal = HP_1 - HP_2
\]
\[
UO = \frac{Signal}{RMS(Signal, 100)}
\]
"#,
    gold_standard_file: "ehlers_ultimate_oscillator.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_ehlers_uo_basic() {
        let mut uo = EhlersUltimateOscillator::default();
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = uo.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_ehlers_uo_parity(
            inputs in prop::collection::vec(1.0..100.0, 150..250),
        ) {
            let band_edge = 20;
            let bandwidth = 2.0;
            let mut uo = EhlersUltimateOscillator::new(band_edge, bandwidth);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| uo.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut hp1 = HighPass::new((band_edge as f64 * bandwidth) as usize);
            let mut hp2 = HighPass::new(band_edge);
            let mut win = VecDeque::new();
            let mut sum_sq = 0.0;

            for &input in &inputs {
                let h1 = hp1.next(input);
                let h2 = hp2.next(input);
                let signal = h1 - h2;
                
                win.push_back(signal);
                sum_sq += signal * signal;
                if win.len() > 100 {
                    let old = win.pop_front().unwrap();
                    sum_sq -= old * old;
                }
                
                let rms = (sum_sq / win.len() as f64).sqrt();
                let res = if rms > 1e-10 { signal / rms } else { 0.0 };
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
