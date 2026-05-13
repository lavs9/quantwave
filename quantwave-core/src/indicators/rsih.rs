use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// RSI with Hann Windowing (RSIH)
///
/// Based on John Ehlers' "(Yet Another) Improved RSI" (January 2022).
/// It applies Hann window coefficients to price changes to create a smoother, 
/// zero-centered oscillator.
#[derive(Debug, Clone)]
pub struct RSIH {
    length: usize,
    price_history: VecDeque<f64>,
    coefficients: Vec<f64>,
}

impl RSIH {
    pub fn new(length: usize) -> Self {
        let mut coefficients = Vec::with_capacity(length);
        for count in 1..=length {
            let coef = 1.0 - (2.0 * PI * count as f64 / (length as f64 + 1.0)).cos();
            coefficients.push(coef);
        }
        Self {
            length,
            price_history: VecDeque::with_capacity(length + 1),
            coefficients,
        }
    }
}

impl Default for RSIH {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Next<f64> for RSIH {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.price_history.push_front(input);
        if self.price_history.len() > self.length + 1 {
            self.price_history.pop_back();
        }

        if self.price_history.len() < self.length + 1 {
            return 0.0;
        }

        let mut cu = 0.0;
        let mut cd = 0.0;

        for count in 1..=self.length {
            let change = self.price_history[count - 1] - self.price_history[count];
            let coef = self.coefficients[count - 1];
            if change > 0.0 {
                cu += coef * change;
            } else if change < 0.0 {
                cd += coef * change.abs();
            }
        }

        if (cu + cd).abs() > 1e-10 {
            (cu - cd) / (cu + cd)
        } else {
            0.0
        }
    }
}

pub const RSIH_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "RSIH",
    description: "RSI enhanced with Hann windowing for superior smoothing and zero-centering.",
    params: &[
        ParamDef { name: "length", default: "14", description: "RSI length" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JANUARY%202022.html",
    formula_latex: r#"
\[
CU = \sum_{n=1}^L (1 - \cos\left(\frac{2\pi n}{L+1}\right)) \cdot \max(0, Close_{t-n+1} - Close_{t-n})
\]
\[
CD = \sum_{n=1}^L (1 - \cos\left(\frac{2\pi n}{L+1}\right)) \cdot \max(0, Close_{t-n} - Close_{t-n+1})
\]
\[
RSIH = \frac{CU - CD}{CU + CD}
\]
"#,
    gold_standard_file: "rsih.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use crate::test_utils::{load_gold_standard, assert_indicator_parity};
    use proptest::prelude::*;

    #[test]
    fn test_rsih_gold_standard() {
        let case = load_gold_standard("rsih");
        let rsih = RSIH::new(14);
        assert_indicator_parity(rsih, &case.input, &case.expected);
    }

    #[test]
    fn test_rsih_basic() {
        let mut rsih = RSIH::default();
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = rsih.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_rsih_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 14;
            let mut rsih = RSIH::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| rsih.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut coeffs = Vec::new();
            for count in 1..=length {
                let c = 1.0 - (2.0 * PI * count as f64 / (length as f64 + 1.0)).cos();
                coeffs.push(c);
            }

            for i in 0..inputs.len() {
                if i < length {
                    batch_results.push(0.0);
                    continue;
                }
                let mut cu = 0.0;
                let mut cd = 0.0;
                for count in 1..=length {
                    let change = inputs[i - count + 1] - inputs[i - count];
                    let coef = coeffs[count - 1];
                    if change > 0.0 {
                        cu += coef * change;
                    } else if change < 0.0 {
                        cd += coef * change.abs();
                    }
                }
                let res = if (cu + cd).abs() > 1e-10 {
                    (cu - cd) / (cu + cd)
                } else {
                    0.0
                };
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
