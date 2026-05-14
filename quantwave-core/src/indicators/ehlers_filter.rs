use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Ehlers Filter (Distance Coefficient)
///
/// Based on John Ehlers' "Ehlers Filters" (2001).
/// A non-linear FIR filter that uses the sum of squared distances between prices
/// as coefficients, allowing it to adapt to sharp transitions while providing
/// smooth output in sideways markets.
#[derive(Debug, Clone)]
pub struct EhlersFilter {
    length: usize,
    window: VecDeque<f64>,
}

impl EhlersFilter {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            window: VecDeque::with_capacity(2 * length),
        }
    }
}

impl Next<f64> for EhlersFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_front(input);
        if self.window.len() > 2 * self.length {
            self.window.pop_back();
        }

        if self.window.len() < 2 * self.length - 1 {
            return input;
        }

        let mut num = 0.0;
        let mut sum_coef = 0.0;

        for count in 0..self.length {
            let mut distance2 = 0.0;
            for lookback in 1..self.length {
                let diff = self.window[count] - self.window[count + lookback];
                distance2 += diff * diff;
            }
            let coef = distance2;
            num += coef * self.window[count];
            sum_coef += coef;
        }

        if sum_coef != 0.0 {
            num / sum_coef
        } else {
            input
        }
    }
}

pub const EHLERS_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ehlers Filter",
    description: "A non-linear FIR filter using distance coefficients to adapt to price transitions while maintaining smoothness.",
    usage: "Use as a configurable digital filter from Ehlers DSP toolkit when you need a specific frequency response not covered by the standard smoother or Butterworth designs.",
    keywords: &["filter", "ehlers", "dsp", "smoothing"],
    ehlers_summary: "The Ehlers Filter is a generalized IIR filter design drawn from Ehlers digital signal processing framework for markets. Its coefficients can be tuned to approximate different filter types (lowpass, highpass, bandpass), making it a flexible building block for custom indicator pipelines.",
    params: &[ParamDef {
        name: "length",
        default: "15",
        description: "Filter window length",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EhlersFilters.pdf",
    formula_latex: r#"
\[
C_i = \sum_{j=1}^{L-1} (Price_{t-i} - Price_{t-i-j})^2
\]
\[
Filt = \frac{\sum_{i=0}^{L-1} C_i Price_{t-i}}{\sum_{i=0}^{L-1} C_i}
\]
"#,
    gold_standard_file: "ehlers_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_ehlers_filter_basic() {
        let mut ef = EhlersFilter::new(15);
        let inputs = vec![10.0; 40];
        for input in inputs {
            let res = ef.next(input);
            assert_eq!(res, 10.0);
        }
    }

    proptest! {
        #[test]
        fn test_ehlers_filter_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 15;
            let mut ef = EhlersFilter::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| ef.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            for i in 0..inputs.len() {
                if i < 2 * length - 2 {
                    batch_results.push(inputs[i]);
                    continue;
                }

                let mut num = 0.0;
                let mut sum_c = 0.0;
                for count in 0..length {
                    let mut d2 = 0.0;
                    for lb in 1..length {
                        let diff = inputs[i-count] - inputs[i-count-lb];
                        d2 += diff * diff;
                    }
                    num += d2 * inputs[i-count];
                    sum_c += d2;
                }

                if sum_c != 0.0 {
                    batch_results.push(num / sum_c);
                } else {
                    batch_results.push(inputs[i]);
                }
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
