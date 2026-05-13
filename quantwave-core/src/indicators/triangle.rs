use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Triangle Windowed FIR Filter
///
/// Based on John Ehlers' "Windowing" (S&C 2021).
/// A finite impulse response (FIR) filter using a triangle-shaped window for smoothing.
#[derive(Debug, Clone)]
pub struct TriangleFilter {
    length: usize,
    window: VecDeque<f64>,
    coefficients: Vec<f64>,
    coef_sum: f64,
}

impl TriangleFilter {
    pub fn new(length: usize) -> Self {
        let mut coefficients = Vec::with_capacity(length);
        let mut coef_sum = 0.0;
        for count in 1..=length {
            let coef = if (count as f64) < (length as f64 / 2.0) {
                count as f64
            } else if (count as f64) == (length as f64 / 2.0) {
                length as f64 / 2.0
            } else {
                length as f64 + 1.0 - count as f64
            };
            coefficients.push(coef);
            coef_sum += coef;
        }

        Self {
            length,
            window: VecDeque::with_capacity(length),
            coefficients,
            coef_sum,
        }
    }
}

impl Default for TriangleFilter {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Next<f64> for TriangleFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_front(input);
        if self.window.len() > self.length {
            self.window.pop_back();
        }

        if self.window.len() < self.length {
            return input;
        }

        let mut filt = 0.0;
        for (i, &val) in self.window.iter().enumerate() {
            filt += self.coefficients[i] * val;
        }

        if self.coef_sum.abs() > 1e-10 {
            filt / self.coef_sum
        } else {
            input
        }
    }
}

pub const TRIANGLE_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "TriangleFilter",
    description: "Triangle windowed FIR filter.",
    params: &[
        ParamDef {
            name: "length",
            default: "20",
            description: "Filter length",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - SEPTEMBER 2021.html",
    formula_latex: r#"
\[
Coef(n) = \begin{cases} n & n < L/2 \\ L/2 & n = L/2 \\ L + 1 - n & n > L/2 \end{cases}
\]
\[
Filt = \frac{\sum_{n=1}^L Coef(n) \cdot Price_{t-n+1}}{\sum Coef(n)}
\]
"#,
    gold_standard_file: "triangle_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_triangle_basic() {
        let mut tri = TriangleFilter::new(20);
        for _ in 0..50 {
            let val = tri.next(100.0);
            approx::assert_relative_eq!(val, 100.0, epsilon = 1e-10);
        }
    }

    proptest! {
        #[test]
        fn test_triangle_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 20;
            let mut tri = TriangleFilter::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| tri.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut coeffs = Vec::new();
            let mut c_sum = 0.0;
            for count in 1..=length {
                let coef = if (count as f64) < (length as f64 / 2.0) {
                    count as f64
                } else if (count as f64) == (length as f64 / 2.0) {
                    length as f64 / 2.0
                } else {
                    length as f64 + 1.0 - count as f64
                };
                coeffs.push(coef);
                c_sum += coef;
            }

            for i in 0..inputs.len() {
                if i < length - 1 {
                    batch_results.push(inputs[i]);
                    continue;
                }
                let mut f = 0.0;
                for j in 0..length {
                    f += coeffs[j] * inputs[i - j];
                }
                batch_results.push(f / c_sum);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
