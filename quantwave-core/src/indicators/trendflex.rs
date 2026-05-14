use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::super_smoother::SuperSmoother;
use std::collections::VecDeque;

/// Ehlers Trendflex
///
/// Based on John Ehlers' "Reflex: A New Zero-Lag Indicator" (2020).
/// It is a zero-lag averaging indicator that retains the trend component.
#[derive(Debug, Clone)]
pub struct Trendflex {
    length: usize,
    smoother: SuperSmoother,
    filt_history: VecDeque<f64>,
    ms: f64,
}

impl Trendflex {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            // SuperSmoother period is half the cycle length as per paper
            smoother: SuperSmoother::new(length / 2),
            filt_history: VecDeque::with_capacity(length + 1),
            ms: 0.0,
        }
    }
}

impl Next<f64> for Trendflex {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let filt = self.smoother.next(input);
        self.filt_history.push_front(filt);
        
        if self.filt_history.len() <= self.length {
            return 0.0;
        }
        
        if self.filt_history.len() > self.length + 1 {
            self.filt_history.pop_back();
        }

        let mut sum = 0.0;
        for count in 1..=self.length {
            let val = self.filt_history[count];
            sum += filt - val;
        }
        sum /= self.length as f64;
        
        self.ms = 0.04 * sum * sum + 0.96 * self.ms;
        
        if self.ms > 0.0 {
            sum / self.ms.sqrt()
        } else {
            0.0
        }
    }
}

pub const TRENDFLEX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Trendflex",
    description: "A zero-lag averaging indicator designed to retain the trend component while reducing lag.",
    usage: "Use to recognize enduring trends with minimal lag. It is better at identifying the start of a new trend than standard moving averages.",
    keywords: &["zero-lag", "trend", "ehlers", "dsp", "oscillator"],
    ehlers_summary: "Trendflex is the companion to Reflex. While Reflex focuses on the cyclic component by removing the trend slope, Trendflex retains the trend information by measuring the cumulative difference between the current smoothed value and its history without slope adjustment.",
    params: &[
        ParamDef { name: "length", default: "20", description: "Assumed cycle period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/implemented/TRADERS’ TIPS - FEBRUARY 2020.html",
    formula_latex: r#"
\[
Filt = \text{SuperSmoother}(Price, Length/2)
\]
\[
Sum = \frac{1}{Length} \sum_{n=1}^{Length} (Filt_t - Filt_{t-n})
\]
\[
MS = 0.04 \cdot Sum^2 + 0.96 \cdot MS_{t-1}
\]
\[
Trendflex = \frac{Sum}{\sqrt{MS}}
\]
"#,
    gold_standard_file: "trendflex.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_trendflex_basic() {
        let mut tf = Trendflex::new(20);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];
        for input in inputs {
            let res = tf.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_trendflex_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 20;
            let mut tf = Trendflex::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| tf.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut smoother = SuperSmoother::new(length / 2);
            let mut filt_vals = Vec::new();
            let mut ms = 0.0;

            for (i, &input) in inputs.iter().enumerate() {
                let filt = smoother.next(input);
                filt_vals.push(filt);
                
                if filt_vals.len() <= length {
                    batch_results.push(0.0);
                    continue;
                }
                
                let filt_now = filt_vals[i];
                let mut sum = 0.0;
                for count in 1..=length {
                    let val = filt_vals[i - count];
                    sum += filt_now - val;
                }
                sum /= length as f64;
                
                ms = 0.04 * sum * sum + 0.96 * ms;
                let res = if ms > 0.0 { sum / ms.sqrt() } else { 0.0 };
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
