use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::ultimate_smoother::UltimateSmoother;
use std::collections::VecDeque;

/// Ultimate Bands
/// 
/// Based on John Ehlers' "Ultimate Channel and Ultimate Bands" (S&C 2024).
/// Replaces the SMA in Bollinger Bands with UltimateSmoother and calculates 
/// Standard Deviation relative to the smoothed center line.
#[derive(Debug, Clone)]
pub struct UltimateBands {
    smoother: UltimateSmoother,
    num_sds: f64,
    length: usize,
    diff_sq_history: VecDeque<f64>,
    sum_diff_sq: f64,
}

impl UltimateBands {
    pub fn new(length: usize, num_sds: f64) -> Self {
        Self {
            smoother: UltimateSmoother::new(length),
            num_sds,
            length,
            diff_sq_history: VecDeque::with_capacity(length),
            sum_diff_sq: 0.0,
        }
    }
}

impl Next<f64> for UltimateBands {
    type Output = (f64, f64, f64); // (Upper, Center, Lower)

    fn next(&mut self, input: f64) -> Self::Output {
        let center = self.smoother.next(input);
        
        let diff = input - center;
        let diff_sq = diff * diff;
        
        self.sum_diff_sq += diff_sq;
        self.diff_sq_history.push_back(diff_sq);
        
        if self.diff_sq_history.len() > self.length {
            if let Some(old) = self.diff_sq_history.pop_front() {
                self.sum_diff_sq -= old;
            }
        }
        
        let sd = if self.sum_diff_sq > 0.0 {
            (self.sum_diff_sq / self.diff_sq_history.len() as f64).sqrt()
        } else {
            0.0
        };

        let upper = center + self.num_sds * sd;
        let lower = center - self.num_sds * sd;

        (upper, center, lower)
    }
}

pub const ULTIMATE_BANDS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ultimate Bands",
    description: "A Bollinger-style band using UltimateSmoother for the center line and standard deviation of the price-smooth difference for width.",
    params: &[
        ParamDef { name: "length", default: "20", description: "Smoothing and SD period" },
        ParamDef { name: "num_sds", default: "1.0", description: "Standard Deviation multiplier" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UltimateChannel.pdf",
    formula_latex: r#"
\[
Smooth = UltimateSmoother(Close, Length)
\]
\[
SD = \sqrt{\frac{1}{n}\sum_{i=0}^{n-1} (Close_{t-i} - Smooth_{t-i})^2}
\]
\[
Upper = Smooth + NumSDs \times SD
\]
\[
Lower = Smooth - NumSDs \times SD
\]
"#,
    gold_standard_file: "ultimate_bands.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_ultimate_bands_basic() {
        let mut ub = UltimateBands::new(20, 1.0);
        let inputs = vec![10.0, 11.0, 12.0, 11.0, 10.0];
        for input in inputs {
            let (u, c, l) = ub.next(input);
            assert!(!u.is_nan());
            assert!(!c.is_nan());
            assert!(!l.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_ultimate_bands_parity(
            inputs in prop::collection::vec(1.0..100.0, 30..100),
        ) {
            let length = 20;
            let num_sds = 1.0;
            let mut ub = UltimateBands::new(length, num_sds);
            let streaming_results: Vec<(f64, f64, f64)> = inputs.iter().map(|&x| ub.next(x)).collect();
            
            // Reference implementation
            let mut sm = UltimateSmoother::new(length);
            let mut diff_sqs = Vec::with_capacity(inputs.len());
            let mut batch_results = Vec::with_capacity(inputs.len());
            
            for &input in &inputs {
                let center = sm.next(input);
                let diff = input - center;
                diff_sqs.push(diff * diff);
                
                let start = if diff_sqs.len() > length { diff_sqs.len() - length } else { 0 };
                let window = &diff_sqs[start..];
                let sd = (window.iter().sum::<f64>() / window.len() as f64).sqrt();
                
                batch_results.push((center + num_sds * sd, center, center - num_sds * sd));
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-10);
            }
        }
    }
}
