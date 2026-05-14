use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::hann::HannFilter;
use crate::traits::Next;

/// Moving Average Difference with Hann Windowing (MADH)
///
/// Based on John Ehlers' "The MADH: The MAD Indicator, Enhanced" (S&C 2021).
/// It computes the percentage difference between a short-term Hann-windowed FIR filter
/// and a long-term Hann-windowed FIR filter.
#[derive(Debug, Clone)]
pub struct MADH {
    filter1: HannFilter,
    filter2: HannFilter,
    _short_length: usize,
    _dominant_cycle: usize,
}

impl MADH {
    pub fn new(short_length: usize, dominant_cycle: usize) -> Self {
        let long_length = short_length + dominant_cycle / 2;
        Self {
            filter1: HannFilter::new(short_length),
            filter2: HannFilter::new(long_length),
            _short_length: short_length,
            _dominant_cycle: dominant_cycle,
        }
    }
}

impl Default for MADH {
    fn default() -> Self {
        Self::new(8, 27)
    }
}

impl Next<f64> for MADH {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let f1 = self.filter1.next(input);
        let f2 = self.filter2.next(input);
        if f2.abs() > 1e-10 {
            100.0 * (f1 - f2) / f2
        } else {
            0.0
        }
    }
}

pub const MADH_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "MADH",
    description: "Moving Average Difference with Hann Windowing: 100 * (Hann(short) - Hann(long)) / Hann(long)",
    usage: "Use to measure the volatility of the cyclical price component only, filtering out trend-driven amplitude changes that inflate standard volatility measures in trending markets.",
    keywords: &["volatility", "statistics", "ehlers", "high-pass"],
    ehlers_summary: "MADH applies Mean Absolute Deviation to the high-pass filtered price series rather than raw price. By isolating the cyclical component before measuring dispersion, it quantifies the noise level within the current market cycle rather than conflating it with trend amplitude.",
    params: &[
        ParamDef {
            name: "short_length",
            default: "8",
            description: "Short-term filter length",
        },
        ParamDef {
            name: "dominant_cycle",
            default: "27",
            description: "Dominant cycle for calculating long length",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - NOVEMBER 2021.html",
    formula_latex: r#"
\[
LongLength = \lfloor ShortLength + DominantCycle / 2 \rfloor
\]
\[
Filt1 = HannWindow(Price, ShortLength)
\]
\[
Filt2 = HannWindow(Price, LongLength)
\]
\[
MADH = 100 \times \frac{Filt1 - Filt2}{Filt2}
\]
"#,
    gold_standard_file: "madh.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;
    use std::f64::consts::PI;

    #[test]
    fn test_madh_basic() {
        let mut madh = MADH::new(8, 27);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = madh.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_madh_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let short = 8;
            let dc = 27;
            let long = short + dc / 2;
            let mut madh = MADH::new(short, dc);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| madh.next(x)).collect();

            // Reference implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            
            let mut coeffs1 = Vec::new();
            let mut sum1 = 0.0;
            for count in 1..=short {
                let c = 1.0 - (2.0 * PI * count as f64 / (short as f64 + 1.0)).cos();
                coeffs1.push(c);
                sum1 += c;
            }
            
            let mut coeffs2 = Vec::new();
            let mut sum2 = 0.0;
            for count in 1..=long {
                let c = 1.0 - (2.0 * PI * count as f64 / (long as f64 + 1.0)).cos();
                coeffs2.push(c);
                sum2 += c;
            }

            for i in 0..inputs.len() {
                let f1 = if i < short - 1 {
                    inputs[i]
                } else {
                    let mut sum = 0.0;
                    for j in 0..short {
                        sum += coeffs1[j] * inputs[i - j];
                    }
                    sum / sum1
                };

                let f2 = if i < long - 1 {
                    inputs[i]
                } else {
                    let mut sum = 0.0;
                    for j in 0..long {
                        sum += coeffs2[j] * inputs[i - j];
                    }
                    sum / sum2
                };

                let res = if f2.abs() > 1e-10 {
                    100.0 * (f1 - f2) / f2
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
