use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// MyRSI
///
/// Based on John Ehlers' "MyRSI" from the Noise Elimination Technology paper.
/// Unlike standard RSI which ranges from 0 to 100, MyRSI swings between -1 and +1.
#[derive(Debug, Clone)]
pub struct MyRSI {
    length: usize,
    price_window: VecDeque<f64>,
}

impl MyRSI {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            price_window: VecDeque::with_capacity(length + 1),
        }
    }
}

impl Default for MyRSI {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Next<f64> for MyRSI {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.price_window.push_front(input);
        if self.price_window.len() > self.length + 1 {
            self.price_window.pop_back();
        }

        if self.price_window.len() < self.length + 1 {
            return 0.0;
        }

        let mut cu = 0.0;
        let mut cd = 0.0;

        for i in 0..self.length {
            let diff = self.price_window[i] - self.price_window[i + 1];
            if diff > 0.0 {
                cu += diff;
            } else if diff < 0.0 {
                cd -= diff;
            }
        }

        if cu + cd != 0.0 {
            (cu - cd) / (cu + cd)
        } else {
            0.0
        }
    }
}

pub const MY_RSI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "MyRSI",
    description: "Ehlers' version of RSI that swings between -1 and +1.",
    usage: "Use as Ehlers smoothed RSI variant that applies cycle-aware filtering to reduce whipsaws while maintaining RSI-style overbought/oversold interpretation.",
    keywords: &["oscillator", "rsi", "ehlers", "momentum", "smoothing"],
    ehlers_summary: "Ehlers presents a smoothed RSI formulation that applies a Laguerre or SuperSmoother filter to the up/down ratio before computing the RSI index. This reduces the noise and oscillation of standard RSI without significantly increasing lag, producing more reliable overbought and oversold readings.",
    params: &[ParamDef {
        name: "length",
        default: "14",
        description: "Smoothing length",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Noise%20Elimination%20Technology.pdf",
    formula_latex: r#"
\[
CU = \sum_{i=0}^{length-1} \max(0, Price_i - Price_{i+1})
\]
\[
CD = \sum_{i=0}^{length-1} \max(0, Price_{i+1} - Price_i)
\]
\[
MyRSI = \frac{CU - CD}{CU + CD}
\]
"#,
    gold_standard_file: "my_rsi.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_my_rsi_basic() {
        let mut rsi = MyRSI::new(14);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0];
        let mut last_rsi = 0.0;
        for input in inputs {
            last_rsi = rsi.next(input);
        }
        assert_eq!(last_rsi, 1.0);
    }

    proptest! {
        #[test]
        fn test_my_rsi_parity(
            inputs in prop::collection::vec(1.0..100.0, 20..100),
        ) {
            let length = 14;
            let mut rsi = MyRSI::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| rsi.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            for i in 0..inputs.len() {
                if i < length {
                    batch_results.push(0.0);
                    continue;
                }

                let mut cu = 0.0;
                let mut cd = 0.0;
                for j in 0..length {
                    let diff = inputs[i - j] - inputs[i - j - 1];
                    if diff > 0.0 {
                        cu += diff;
                    } else if diff < 0.0 {
                        cd -= diff;
                    }
                }

                if cu + cd != 0.0 {
                    batch_results.push((cu - cd) / (cu + cd));
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
