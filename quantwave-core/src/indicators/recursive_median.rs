use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Recursive Median Filter
///
/// Based on John Ehlers' "Recursive Median Filters" (TASC March 2018).
/// It applies an EMA smoothing to a 5-bar median filter.
#[derive(Debug, Clone)]
pub struct RecursiveMedian {
    _lp_period: usize,
    alpha1: f64,
    window: VecDeque<f64>,
    prev_rm: f64,
}

impl RecursiveMedian {
    pub fn new(lp_period: usize) -> Self {
        let p = lp_period as f64;
        let deg_to_rad = std::f64::consts::PI / 180.0;
        let alpha1 = ((360.0 / p * deg_to_rad).cos() + (360.0 / p * deg_to_rad).sin() - 1.0)
            / (360.0 / p * deg_to_rad).cos();

        Self {
            _lp_period: lp_period,
            alpha1,
            window: VecDeque::with_capacity(5),
            prev_rm: 0.0,
        }
    }
}

impl Default for RecursiveMedian {
    fn default() -> Self {
        Self::new(12)
    }
}

impl Next<f64> for RecursiveMedian {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_front(input);
        if self.window.len() > 5 {
            self.window.pop_back();
        }

        if self.window.len() < 5 {
            self.prev_rm = input;
            return input;
        }

        let mut sorted: Vec<f64> = self.window.iter().copied().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted[2]; // Median of 5 elements

        let rm = self.alpha1 * median + (1.0 - self.alpha1) * self.prev_rm;
        self.prev_rm = rm;
        rm
    }
}

/// Recursive Median Oscillator
///
/// Based on John Ehlers' "Recursive Median Filters" (TASC March 2018).
/// It applies a 2nd-order Highpass filter to the Recursive Median Filter output.
#[derive(Debug, Clone)]
pub struct RecursiveMedianOscillator {
    _lp_period: usize,
    _hp_period: usize,
    rm_filter: RecursiveMedian,
    alpha2: f64,
    prev_rm: [f64; 2],
    prev_rmo: [f64; 2],
}

impl RecursiveMedianOscillator {
    pub fn new(lp_period: usize, hp_period: usize) -> Self {
        let p = hp_period as f64;
        let deg_to_rad = std::f64::consts::PI / 180.0;
        let angle = 0.707 * 360.0 / p * deg_to_rad;
        let alpha2 = (angle.cos() + angle.sin() - 1.0) / angle.cos();

        Self {
            _lp_period: lp_period,
            _hp_period: hp_period,
            rm_filter: RecursiveMedian::new(lp_period),
            alpha2,
            prev_rm: [0.0; 2],
            prev_rmo: [0.0; 2],
        }
    }
}

impl Default for RecursiveMedianOscillator {
    fn default() -> Self {
        Self::new(12, 30)
    }
}

impl Next<f64> for RecursiveMedianOscillator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let rm = self.rm_filter.next(input);

        // Highpass filter calculation:
        // RMO = (1-alpha2/2)*(1-alpha2/2)*(RM-2*RM[1]+RM[2])+2*(1-alpha2)*RMO[1]-(1-alpha2)*(1-alpha2)*RMO[2]
        let c1 = (1.0 - self.alpha2 / 2.0) * (1.0 - self.alpha2 / 2.0);
        let c2 = 2.0 * (1.0 - self.alpha2);
        let c3 = (1.0 - self.alpha2) * (1.0 - self.alpha2);

        let rmo = c1 * (rm - 2.0 * self.prev_rm[0] + self.prev_rm[1])
            + c2 * self.prev_rmo[0]
            - c3 * self.prev_rmo[1];

        self.prev_rm[1] = self.prev_rm[0];
        self.prev_rm[0] = rm;

        self.prev_rmo[1] = self.prev_rmo[0];
        self.prev_rmo[0] = rmo;

        rmo
    }
}

pub const RECURSIVE_MEDIAN_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "RecursiveMedian",
    description: "EMA of a 5-bar median filter for smooth tracking with minimal jitter.",
    usage: "Use to filter out extreme outliers and noise while maintaining trend sensitivity. Excellent as a baseline for other oscillators.",
    keywords: &["filter", "ehlers", "dsp", "median", "robust", "smoothing"],
    ehlers_summary: "Standard filters like SMA or EMA are distorted by price spikes. The recursive median filter uses the median to reject outliers and an EMA to provide smoothness, offering a cleaner trend representation than standard moving averages.",
    params: &[
        ParamDef {
            name: "lp_period",
            default: "12",
            description: "Low-pass smoothing period",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2018/03/TradersTips.html",
    formula_latex: r#"
\[
\alpha = \frac{\cos(360/P) + \sin(360/P) - 1}{\cos(360/P)}
\]
\[
RM_t = \alpha \cdot \text{Median}(Price, 5)_t + (1 - \alpha) \cdot RM_{t-1}
\]
"#,
    gold_standard_file: "recursive_median.json",
    category: "Ehlers DSP",
};

pub const RECURSIVE_MEDIAN_OSCILLATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "RecursiveMedianOscillator",
    description: "Oscillator derived from the Recursive Median filter using a 2nd-order Highpass filter.",
    usage: "Identify cyclic turning points with reduced lag and noise. The high-pass component removes the trend, leaving the cycle.",
    keywords: &["oscillator", "ehlers", "dsp", "median", "cycle", "highpass"],
    ehlers_summary: "By applying a 2nd-order Highpass filter to the Recursive Median output, we create an oscillator that is specifically tuned to the dominant cycle while remaining immune to the outlier spikes that would otherwise create false signals.",
    params: &[
        ParamDef {
            name: "lp_period",
            default: "12",
            description: "Low-pass smoothing period",
        },
        ParamDef {
            name: "hp_period",
            default: "30",
            description: "High-pass cutoff period",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2018/03/TradersTips.html",
    formula_latex: r#"
\[
\alpha_2 = \frac{\cos(0.707 \cdot 360/HP) + \sin(0.707 \cdot 360/HP) - 1}{\cos(0.707 \cdot 360/HP)}
\]
\[
RMO_t = (1-\alpha_2/2)^2(RM_t - 2RM_{t-1} + RM_{t-2}) + 2(1-\alpha_2)RMO_{t-1} - (1-\alpha_2)^2RMO_{t-2}
\]
"#,
    gold_standard_file: "recursive_median_oscillator.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_recursive_median_basic() {
        let mut rm = RecursiveMedian::new(12);
        for _ in 0..50 {
            let val = rm.next(100.0);
            approx::assert_relative_eq!(val, 100.0, epsilon = 1e-10);
        }
    }

    proptest! {
        #[test]
        fn test_recursive_median_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let lp_period = 12;
            let mut rm = RecursiveMedian::new(lp_period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| rm.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let p = lp_period as f64;
            let deg_to_rad = std::f64::consts::PI / 180.0;
            let alpha1 = ((360.0 / p * deg_to_rad).cos() + (360.0 / p * deg_to_rad).sin() - 1.0)
                / (360.0 / p * deg_to_rad).cos();

            let mut prev_rm = 0.0;
            for i in 0..inputs.len() {
                if i < 4 {
                    prev_rm = inputs[i];
                    batch_results.push(inputs[i]);
                    continue;
                }
                let mut window = vec![
                    inputs[i], inputs[i-1], inputs[i-2], inputs[i-3], inputs[i-4]
                ];
                window.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let median = window[2];
                let val = alpha1 * median + (1.0 - alpha1) * prev_rm;
                batch_results.push(val);
                prev_rm = val;
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
