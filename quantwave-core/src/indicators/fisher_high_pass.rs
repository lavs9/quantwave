use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use std::collections::VecDeque;

/// Fisher HighPass Indicator
///
/// Based on John Ehlers' "Inferring Trading Strategies from Probability Distribution Functions".
/// Applies a HighPass filter, normalizes the result to [-1, 1], smooths it with a 3-tap FIR,
/// and then applies the Fisher Transform.
#[derive(Debug, Clone)]
pub struct FisherHighPass {
    hp: HighPass,
    period: usize,
    hp_window: VecDeque<f64>,
    smooth_history: [f64; 2],
    count: usize,
}

impl FisherHighPass {
    pub fn new(hp_len: usize, norm_len: usize) -> Self {
        Self {
            hp: HighPass::new(hp_len),
            period: norm_len,
            hp_window: VecDeque::with_capacity(norm_len),
            smooth_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Default for FisherHighPass {
    fn default() -> Self {
        Self::new(20, 20)
    }
}

impl Next<f64> for FisherHighPass {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let hp_val = self.hp.next(input);

        self.hp_window.push_front(hp_val);
        if self.hp_window.len() > self.period {
            self.hp_window.pop_back();
        }

        if self.hp_window.len() < self.period {
            return 0.0;
        }

        let mut high = f64::MIN;
        let mut low = f64::MAX;
        for &v in &self.hp_window {
            if v > high { high = v; }
            if v < low { low = v; }
        }

        let normalized = if high != low {
            2.0 * (hp_val - low) / (high - low) - 1.0
        } else {
            0.0
        };

        // 3-tap FIR smoothing: (N + N[1] + N[2]) / 3
        let smoothed = (normalized + self.smooth_history[0] + self.smooth_history[1]) / 3.0;
        
        self.smooth_history[1] = self.smooth_history[0];
        self.smooth_history[0] = normalized;

        // Fisher Transform
        // y = 0.5 * ln((1+x)/(1-x))
        // Clip to avoid log(0)
        let x = smoothed.clamp(-0.999, 0.999);
        0.5 * ((1.0 + x) / (1.0 - x)).ln()
    }
}

pub const FISHER_HIGH_PASS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "FisherHighPass",
    description: "Fisher Transform applied to normalized HighPass filtered prices.",
    usage: "Use to isolate high-frequency momentum from the cyclical component of price after trend removal. Provides a purer momentum signal than standard Fisher Transform applied to raw price.",
    keywords: &["oscillator", "ehlers", "dsp", "high-pass", "momentum"],
    ehlers_summary: "FisherHighPass applies the Fisher Transform to the high-pass filtered price rather than raw price. By first removing the low-frequency trend component with a high-pass filter, the resulting Fisher output captures only the cycle-domain momentum, producing an oscillator that is unaffected by the prevailing trend direction.",
    params: &[
        ParamDef {
            name: "hp_len",
            default: "20",
            description: "HighPass filter length",
        },
        ParamDef {
            name: "norm_len",
            default: "20",
            description: "Normalization lookback period",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/InferringTradingStrategies.pdf",
    formula_latex: r#"
\[
HP = \text{HighPass}(Price, hp\_len)
\]
\[
N = 2 \cdot \frac{HP - Low(HP, norm\_len)}{High(HP, norm\_len) - Low(HP, norm\_len)} - 1
\]
\[
S = \frac{N + N_{t-1} + N_{t-2}}{3}
\]
\[
Fisher = 0.5 \cdot \ln\left(\frac{1+S}{1-S}\right)
\]
"#,
    gold_standard_file: "fisher_high_pass.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_fisher_hp_basic() {
        let mut fhp = FisherHighPass::new(20, 20);
        for i in 0..100 {
            let val = fhp.next(100.0 + (i as f64 * 0.1).sin());
            assert!(!val.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_fisher_hp_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..200),
        ) {
            let hp_len = 20;
            let norm_len = 20;
            let mut fhp = FisherHighPass::new(hp_len, norm_len);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| fhp.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut hp = HighPass::new(hp_len);
            let hp_vals: Vec<f64> = inputs.iter().map(|&x| hp.next(x)).collect();

            let mut norm_vals = Vec::new();
            for i in 0..hp_vals.len() {
                let start = if i >= norm_len - 1 { i + 1 - norm_len } else { 0 };
                let window = &hp_vals[start..i + 1];
                
                if window.len() < norm_len {
                    batch_results.push(0.0);
                    norm_vals.push(0.0);
                    continue;
                }

                let mut high = f64::MIN;
                let mut low = f64::MAX;
                for &v in window {
                    if v > high { high = v; }
                    if v < low { low = v; }
                }

                let n = if high != low {
                    2.0 * (hp_vals[i] - low) / (high - low) - 1.0
                } else {
                    0.0
                };
                norm_vals.push(n);

                let s = (norm_vals[i] + (if i > 0 { norm_vals[i-1] } else { 0.0 }) + (if i > 1 { norm_vals[i-2] } else { 0.0 })) / 3.0;
                let x = s.clamp(-0.999, 0.999);
                batch_results.push(0.5 * ((1.0 + x) / (1.0 - x)).ln());
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
