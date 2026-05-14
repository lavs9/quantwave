use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Hamming Windowed FIR Filter
///
/// Based on John Ehlers' "Windowing" (S&C 2021).
/// A finite impulse response (FIR) filter using a Hamming-like window with a pedestal for smoothing.
#[derive(Debug, Clone)]
pub struct HammingFilter {
    length: usize,
    _pedestal: f64,
    window: VecDeque<f64>,
    coefficients: Vec<f64>,
    coef_sum: f64,
}

impl HammingFilter {
    pub fn new(length: usize, pedestal_deg: f64) -> Self {
        let mut coefficients = Vec::with_capacity(length);
        let mut coef_sum = 0.0;
        
        // Follows Ehlers' formula: Sine(Pedestal + (180 - 2*Pedestal)*count / (Length - 1))
        // count from 0 to Length - 1
        for count in 0..length {
            let deg = pedestal_deg + (180.0 - 2.0 * pedestal_deg) * count as f64 / (length as f64 - 1.0).max(1.0);
            let coef = (deg * PI / 180.0).sin();
            coefficients.push(coef);
            coef_sum += coef;
        }

        Self {
            length,
            _pedestal: pedestal_deg,
            window: VecDeque::with_capacity(length),
            coefficients,
            coef_sum,
        }
    }
}

impl Default for HammingFilter {
    fn default() -> Self {
        Self::new(20, 10.0)
    }
}

impl Next<f64> for HammingFilter {
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

pub const HAMMING_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "HammingFilter",
    description: "Hamming windowed FIR filter with pedestal.",
    usage: "Apply as a windowing function before DFT-based cycle detection to reduce sidelobe leakage and obtain cleaner dominant cycle estimates.",
    keywords: &["filter", "ehlers", "dsp", "windowing", "spectral"],
    ehlers_summary: "The Hamming window is a raised-cosine weighting function that reduces spectral leakage by tapering the edges of a data block. Ehlers uses it in DFT-based cycle measurement tools to prevent energy in one frequency bin from contaminating adjacent bins, improving cycle period resolution.",
    params: &[
        ParamDef {
            name: "length",
            default: "20",
            description: "Filter length",
        },
        ParamDef {
            name: "pedestal",
            default: "10.0",
            description: "Pedestal in degrees",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - SEPTEMBER 2021.html",
    formula_latex: r#"
\[
Deg(n) = Pedestal + (180 - 2 \times Pedestal) \times \frac{n}{L-1}
\]
\[
Coef(n) = \sin\left(\frac{Deg(n) \times \pi}{180}\right)
\]
\[
Filt = \frac{\sum_{n=0}^{L-1} Coef(n) \cdot Price_{t-n}}{\sum Coef(n)}
\]
"#,
    gold_standard_file: "hamming_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_hamming_basic() {
        let mut ham = HammingFilter::new(20, 10.0);
        for _ in 0..50 {
            let val = ham.next(100.0);
            approx::assert_relative_eq!(val, 100.0, epsilon = 1e-10);
        }
    }

    proptest! {
        #[test]
        fn test_hamming_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 20;
            let pedestal = 10.0;
            let mut ham = HammingFilter::new(length, pedestal);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| ham.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut coeffs = Vec::new();
            let mut c_sum = 0.0;
            for count in 0..length {
                let deg = pedestal + (180.0 - 2.0 * pedestal) * count as f64 / (length as f64 - 1.0).max(1.0);
                let coef = (deg * PI / 180.0).sin();
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
