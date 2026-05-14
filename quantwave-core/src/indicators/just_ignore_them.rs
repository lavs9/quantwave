use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::hann::HannFilter;

/// Undersampled Double Moving Average
///
/// Based on John Ehlers' "Just Ignore Them".
/// Smooths price data by undersampling (e.g., every 5 bars) and then
/// applying fast and slow Hann windowed filters.
#[derive(Debug, Clone)]
pub struct UndersampledDoubleMA {
    fast_hann: HannFilter,
    slow_hann: HannFilter,
    sample: f64,
    sampling_period: usize,
    count: usize,
}

impl UndersampledDoubleMA {
    pub fn new(fast_len: usize, slow_len: usize, sampling_period: usize) -> Self {
        Self {
            fast_hann: HannFilter::new(fast_len),
            slow_hann: HannFilter::new(slow_len),
            sample: 0.0,
            sampling_period,
            count: 0,
        }
    }
}

impl Default for UndersampledDoubleMA {
    fn default() -> Self {
        Self::new(6, 12, 5)
    }
}

impl Next<f64> for UndersampledDoubleMA {
    type Output = (f64, f64); // (Fast, Slow)

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        
        // Ehlers: If CurrentBar / 5 = IntPortion(CurrentBar / 5) Then Sample = Close;
        if self.count % self.sampling_period == 0 {
            self.sample = input;
        } else if self.count == 1 {
            self.sample = input;
        }

        let fast = self.fast_hann.next(self.sample);
        let slow = self.slow_hann.next(self.sample);

        (fast, slow)
    }
}

pub const UNDERSAMPLED_DOUBLE_MA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "UndersampledDoubleMA",
    description: "Undersampled price data smoothed by dual Hann filters to eliminate high frequency noise.",
    usage: "Internal implementation module — not intended as a standalone trading indicator.",
    keywords: &["internal", "utility"],
    ehlers_summary: "This module contains internal utility functions used by other indicators in the library. It is not intended to be used directly as a standalone trading indicator.",
    params: &[
        ParamDef {
            name: "fast_len",
            default: "6",
            description: "Fast Hann filter length",
        },
        ParamDef {
            name: "slow_len",
            default: "12",
            description: "Slow Hann filter length",
        },
        ParamDef {
            name: "sampling_period",
            default: "5",
            description: "Undersampling rate (bars)",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/JustIgnoreThem.pdf",
    formula_latex: r#"
\[
Sample = \begin{cases} Price & \text{if } t \pmod N = 0 \\ Sample_{t-1} & \text{otherwise} \end{cases}
\]
\[
Fast = Hann(Sample, FastLen)
\]
\[
Slow = Hann(Sample, SlowLen)
\]
"#,
    gold_standard_file: "undersampled_double_ma.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_undersampled_double_ma_basic() {
        let mut udma = UndersampledDoubleMA::new(6, 12, 5);
        for _ in 0..50 {
            let (f, s) = udma.next(100.0);
            approx::assert_relative_eq!(f, 100.0, epsilon = 1e-10);
            approx::assert_relative_eq!(s, 100.0, epsilon = 1e-10);
        }
    }

    proptest! {
        #[test]
        fn test_undersampled_double_ma_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..200),
        ) {
            let fast_len = 6;
            let slow_len = 12;
            let samp_per = 5;
            let mut udma = UndersampledDoubleMA::new(fast_len, slow_len, samp_per);
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| udma.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut fast_hann = HannFilter::new(fast_len);
            let mut slow_hann = HannFilter::new(slow_len);
            let mut sample = 0.0;

            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                if bar % samp_per == 0 || bar == 1 {
                    sample = input;
                }
                let f = fast_hann.next(sample);
                let s = slow_hann.next(sample);
                batch_results.push((f, s));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
