use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Center of Gravity (CG) Oscillator
///
/// Based on John Ehlers' "The CG Oscillator".
/// The CG Oscillator is a smoothed oscillator with essentially zero lag.
/// It identifies turning points by calculating the balance point of prices over a window.
#[derive(Debug, Clone)]
pub struct CenterOfGravity {
    period: usize,
    window: VecDeque<f64>,
}

impl CenterOfGravity {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            window: VecDeque::with_capacity(period),
        }
    }
}

impl Next<f64> for CenterOfGravity {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_front(input);
        if self.window.len() > self.period {
            self.window.pop_back();
        }

        let mut num = 0.0;
        let mut denom = 0.0;

        for (i, &price) in self.window.iter().enumerate() {
            let count = i + 1;
            num += count as f64 * price;
            denom += price;
        }

        if denom == 0.0 { 0.0 } else { -num / denom }
    }
}

pub const CG_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Center of Gravity Oscillator",
    description: "The CG Oscillator identifies price turning points with essentially zero lag by calculating the balance point of prices.",
    params: &[ParamDef {
        name: "period",
        default: "10",
        description: "Observation window length",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TheCGOscillator.pdf",
    formula_latex: r#"
\[
CG = -\frac{\sum_{i=0}^{N-1} (i+1) \times Price_i}{\sum_{i=0}^{N-1} Price_i}
\]
"#,
    gold_standard_file: "cg.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_cg_basic() {
        let mut cg = CenterOfGravity::new(10);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        for input in inputs {
            let val = cg.next(input);
            assert!(!val.is_nan());
            // Since prices are increasing, Num/Denom should be small (towards the newest price)
            // -Num/Denom should be less negative.
        }
    }

    proptest! {
        #[test]
        fn test_cg_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let period = 10;
            let mut cg = CenterOfGravity::new(period);

            let streaming_results: Vec<f64> = inputs.iter().map(|&x| cg.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            for i in 0..inputs.len() {
                let start = if i >= period { i + 1 - period } else { 0 };
                let window = &inputs[start..=i];

                let mut num = 0.0;
                let mut denom = 0.0;
                for (j, &price) in window.iter().rev().enumerate() {
                    let count = j + 1;
                    num += count as f64 * price;
                    denom += price;
                }
                batch_results.push(if denom == 0.0 { 0.0 } else { -num / denom });
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
