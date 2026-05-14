use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::super_smoother::SuperSmoother;
use std::collections::VecDeque;

/// Ehlers Reflex
///
/// Based on John Ehlers' "Reflex: A New Zero-Lag Indicator" (2020).
/// It is a zero-lag averaging indicator that synchronizes with the cycle component.
#[derive(Debug, Clone)]
pub struct Reflex {
    length: usize,
    smoother: SuperSmoother,
    filt_history: VecDeque<f64>,
    ms: f64,
}

impl Reflex {
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

impl Next<f64> for Reflex {
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

        let filt_n = self.filt_history[self.length];
        let slope = (filt_n - filt) / self.length as f64;
        
        let mut sum = 0.0;
        for count in 1..=self.length {
            let val = self.filt_history[count];
            sum += (filt + count as f64 * slope) - val;
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

pub const REFLEX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Reflex",
    description: "A zero-lag averaging indicator designed to synchronize with the cycle component in price data.",
    usage: "Use to identify cyclic reversals with minimal lag. It is more sensitive to significant reversals than standard moving averages.",
    keywords: &["zero-lag", "cycle", "ehlers", "dsp", "oscillator"],
    ehlers_summary: "Ehlers introduces Reflex as a way to reduce lag in averaging indicators by measuring the difference between the current SuperSmoother value and its historical values, adjusted for a linear slope. This 'reflexes' the indicator to show reversals as they happen rather than after the fact.",
    params: &[
        ParamDef { name: "length", default: "20", description: "Assumed cycle period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/implemented/TRADERS’ TIPS - FEBRUARY 2020.html",
    formula_latex: r#"
\[
Filt = \text{SuperSmoother}(Price, Length/2)
\]
\[
Slope = \frac{Filt_{t-Length} - Filt_t}{Length}
\]
\[
Sum = \frac{1}{Length} \sum_{n=1}^{Length} (Filt_t + n \cdot Slope - Filt_{t-n})
\]
\[
MS = 0.04 \cdot Sum^2 + 0.96 \cdot MS_{t-1}
\]
\[
Reflex = \frac{Sum}{\sqrt{MS}}
\]
"#,
    gold_standard_file: "reflex.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_reflex_basic() {
        let mut reflex = Reflex::new(20);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0];
        for input in inputs {
            let res = reflex.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_reflex_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 20;
            let mut reflex = Reflex::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| reflex.next(x)).collect();

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
                let filt_n = filt_vals[i - length];
                let slope = (filt_n - filt_now) / length as f64;
                
                let mut sum = 0.0;
                for count in 1..=length {
                    let val = filt_vals[i - count];
                    sum += (filt_now + count as f64 * slope) - val;
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
