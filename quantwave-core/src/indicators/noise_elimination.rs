use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Noise Elimination Technology (NET)
///
/// Based on John Ehlers' "Noise Elimination Technology" paper.
/// Uses Kendall correlation to strip out noise from an indicator in a nonlinear fashion
/// without introducing lag.
#[derive(Debug, Clone)]
pub struct NoiseElimination {
    length: usize,
    window: VecDeque<f64>,
    denom: f64,
}

impl NoiseElimination {
    pub fn new(length: usize) -> Self {
        let denom = 0.5 * (length as f64) * (length as f64 - 1.0);
        Self {
            length,
            window: VecDeque::with_capacity(length),
            denom,
        }
    }
}

impl Default for NoiseElimination {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Next<f64> for NoiseElimination {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_front(input);
        if self.window.len() > self.length {
            self.window.pop_back();
        }

        if self.window.len() < self.length {
            return 0.0;
        }

        let mut num = 0.0;
        // Ehlers' formula: Num = Num - Sign(X[count] - X[K])
        // where count = 2..N, K = 1..count-1 (1-indexed)
        // In my window [0..N-1], count-1 and K-1 are the indices.
        // i = count-1 (1..N-1), j = K-1 (0..i-1)
        for i in 1..self.length {
            for j in 0..i {
                let diff = self.window[i] - self.window[j];
                if diff > 0.0 {
                    num -= 1.0;
                } else if diff < 0.0 {
                    num += 1.0;
                }
            }
        }

        num / self.denom
    }
}

pub const NOISE_ELIMINATION_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Noise Elimination Technology",
    description: "Nonlinear noise removal using Kendall correlation against a straight line.",
    usage: "Use as a pre-filter to remove spike noise from price or intermediate indicator data without introducing lag. Particularly useful when raw tick or 1-minute data is used.",
    keywords: &["filter", "noise", "ehlers", "dsp", "smoothing"],
    ehlers_summary: "Ehlers Noise Elimination Technology (NET) is a nonlinear filter that removes isolated noise spikes while leaving genuine price moves intact. It works by comparing each bar to its neighbors and replacing outliers with interpolated values, achieving noise reduction without the lag of conventional smoothers.",
    params: &[ParamDef {
        name: "length",
        default: "14",
        description: "Correlation length",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Noise%20Elimination%20Technology.pdf",
    formula_latex: r#"
\[
Num = \sum_{i=1}^{N-1} \sum_{j=0}^{i-1} -sgn(X_i - X_j)
\]
\[
Denom = \frac{N(N-1)}{2}
\]
\[
NET = \frac{Num}{Denom}
\]
"#,
    gold_standard_file: "noise_elimination.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_noise_elimination_basic() {
        let mut net = NoiseElimination::new(14);
        let inputs = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0];
        let mut last_net = 0.0;
        for input in inputs {
            last_net = net.next(input);
        }
        // Increasing input should result in positive NET (1.0)
        assert_eq!(last_net, 1.0);
    }

    proptest! {
        #[test]
        fn test_noise_elimination_parity(
            inputs in prop::collection::vec(-1.0..1.0, 20..100),
        ) {
            let length = 14;
            let mut net = NoiseElimination::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| net.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let denom = 0.5 * (length as f64) * (length as f64 - 1.0);

            for i in 0..inputs.len() {
                if i < length - 1 {
                    batch_results.push(0.0);
                    continue;
                }

                let mut num = 0.0;
                for ii in 1..length {
                    for jj in 0..ii {
                        let diff = inputs[i - ii] - inputs[i - jj];
                        if diff > 0.0 {
                            num -= 1.0;
                        } else if diff < 0.0 {
                            num += 1.0;
                        }
                    }
                }
                batch_results.push(num / denom);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
