//! Fractional differentiation (Prado) — stationary features with memory preservation.
//!
//! **Source**: Marcos López de Prado, *Advances in Financial Machine Learning* (2018), Ch. 5.
//! Weights \(w_k\) for order \(d\): \(w_0=1\), \(w_k = -w_{k-1}\frac{d-k+1}{k}\), truncated when
//! \(|w_k| < \text{threshold}\).

use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::utils::RingBuffer as VecDeque;

/// Fixed-window fractional differencing on a scalar series.
#[derive(Debug, Clone)]
pub struct FracDiff {
    d: f64,
    weights: Vec<f64>,
    window: VecDeque<f64>,
}

/// Compute truncated binomial weights for fractional order `d`.
pub fn frac_diff_weights(d: f64, threshold: f64) -> Vec<f64> {
    let mut w = vec![1.0];
    let mut k = 1usize;
    loop {
        let prev = w[k - 1];
        let w_k = -prev * (d - k as f64 + 1.0) / k as f64;
        if w_k.abs() < threshold {
            break;
        }
        w.push(w_k);
        k += 1;
        if k > 10_000 {
            break;
        }
    }
    w
}

impl FracDiff {
    pub fn new(d: f64, threshold: f64) -> Self {
        let d = d.clamp(0.0, 1.0);
        let threshold = threshold.max(1e-12);
        let weights = frac_diff_weights(d, threshold);
        let cap = weights.len().max(1);
        Self {
            d,
            weights,
            window: VecDeque::with_capacity(cap),
        }
    }

    pub fn window_len(&self) -> usize {
        self.weights.len()
    }

    pub fn d(&self) -> f64 {
        self.d
    }
}

impl Next<f64> for FracDiff {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if input.is_nan() {
            return f64::NAN;
        }

        self.window.push_back(input);
        let w_len = self.weights.len();
        while self.window.len() > w_len {
            self.window.pop_front();
        }

        if self.window.len() < w_len {
            return f64::NAN;
        }

        let mut sum = 0.0;
        for (k, &wk) in self.weights.iter().enumerate() {
            let x = self.window[w_len - 1 - k];
            sum += wk * x;
        }
        sum
    }
}

pub const FRAC_DIFF_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Fractional Differentiation",
    description: "Applies Prado-style fractional differencing to preserve memory while reducing non-stationarity in price series.",
    usage: "Use as an ML feature primitive on log-prices or returns. Lower d (e.g. 0.3–0.5) retains more memory than integer differencing while improving stationarity for tree models and neural nets.",
    keywords: &[
        "ml",
        "stationarity",
        "prado",
        "feature-engineering",
        "fractional",
    ],
    ehlers_summary: "",
    params: &[
        ParamDef {
            name: "d",
            default: "0.4",
            description: "Fractional differentiation order (0 = identity, 1 = full integer diff)",
        },
        ParamDef {
            name: "threshold",
            default: "1e-5",
            description: "Truncate weights when |w_k| falls below this value",
        },
    ],
    formula_source: "https://www.wiley.com/en-us/Advances+in+Financial+Machine+Learning-p-9781119482086",
    formula_latex: r#"
\[
w_0 = 1,\quad w_k = -w_{k-1}\frac{d - k + 1}{k},\quad
\tilde{X}_t = \sum_{k=0}^{K} w_k X_{t-k}
\]
"#,
    gold_standard_file: "frac_diff.json",
    category: "ML Features",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_indicator_parity, load_gold_standard};
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_frac_diff_weights_basic() {
        let w = frac_diff_weights(0.4, 1e-5);
        assert!(w.len() > 2);
        assert!((w[0] - 1.0).abs() < 1e-12);
        assert!(w[1] < 0.0);
    }

    #[test]
    fn test_frac_diff_warmup_nan() {
        let mut fd = FracDiff::new(0.4, 1e-4);
        let w = fd.window_len();
        for i in 0..(w - 1) {
            let out = fd.next(i as f64);
            assert!(out.is_nan(), "expected NaN at bar {i}");
        }
        let first = fd.next(100.0);
        assert!(!first.is_nan());
    }

    #[test]
    fn test_frac_diff_gold_standard() {
        // Gold vector uses d=0.4, threshold=0.05 (compact window for testability).
        let case = load_gold_standard("frac_diff");
        let fd = FracDiff::new(0.4, 0.05);
        assert_indicator_parity(fd, &case.input, &case.expected);
    }

    proptest! {
        #[test]
        fn prop_batch_streaming_parity(
            data in prop::collection::vec(50.0f64..150.0, 20..80),
            d in 0.1f64..0.9,
        ) {
            let mut stream = FracDiff::new(d, 1e-4);
            let streaming: Vec<f64> = data.iter().map(|&x| stream.next(x)).collect();

            let mut batch = FracDiff::new(d, 1e-4);
            let batch_out: Vec<f64> = data.iter().map(|&x| batch.next(x)).collect();

            for (a, b) in streaming.iter().zip(batch_out.iter()) {
                if a.is_nan() && b.is_nan() {
                    continue;
                }
                prop_assert!((a - b).abs() < 1e-12);
            }
        }
    }
}
