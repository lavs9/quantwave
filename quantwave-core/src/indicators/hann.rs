use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Hann Windowed Lowpass FIR Filter
///
/// Based on John Ehlers' "Just Ignore Them".
/// A finite impulse response (FIR) filter using a Hann window for smoothing.
#[derive(Debug, Clone)]
pub struct HannFilter {
    length: usize,
    window: VecDeque<f64>,
    coefficients: Vec<f64>,
    coef_sum: f64,
}

impl HannFilter {
    pub fn new(length: usize) -> Self {
        let mut coefficients = Vec::with_capacity(length);
        let mut coef_sum = 0.0;
        for count in 1..=length {
            let coef = 1.0 - (2.0 * PI * count as f64 / (length as f64 + 1.0)).cos();
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

impl Default for HannFilter {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Next<f64> for HannFilter {
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

        if self.coef_sum != 0.0 {
            filt / self.coef_sum
        } else {
            input
        }
    }
}

pub const HANN_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "HannFilter",
    description: "Hann windowed lowpass FIR filter.",
    usage: "Use as a windowing function before FFT-based dominant cycle measurement to achieve clean spectral separation between market cycles.",
    keywords: &["filter", "ehlers", "dsp", "windowing", "spectral"],
    ehlers_summary: "The Hann window provides a smooth bell-shaped taper achieving -31.5 dB first sidelobe suppression. Ehlers uses it in Cycle Analytics for Traders as the preferred DFT window because it offers the best trade-off between frequency resolution and leakage rejection.",
    params: &[
        ParamDef {
            name: "length",
            default: "20",
            description: "Filter length",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/JustIgnoreThem.pdf",
    formula_latex: r#"
\[
H(n) = 1 - \cos\left(\frac{2\pi n}{L+1}\right)
\]
\[
Filt = \frac{\sum_{n=1}^L H(n) \cdot Price_{t-n+1}}{\sum H(n)}
\]
"#,
    gold_standard_file: "hann_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_hann_basic() {
        let mut hann = HannFilter::new(20);
        for _ in 0..50 {
            let val = hann.next(100.0);
            approx::assert_relative_eq!(val, 100.0, epsilon = 1e-10);
        }
    }

    proptest! {
        #[test]
        fn test_hann_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 20;
            let mut hann = HannFilter::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| hann.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut coeffs = Vec::new();
            let mut c_sum = 0.0;
            for count in 1..=length {
                let c = 1.0 - (2.0 * PI * count as f64 / (length as f64 + 1.0)).cos();
                coeffs.push(c);
                c_sum += c;
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
