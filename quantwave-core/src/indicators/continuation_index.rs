use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::ultimate_smoother::UltimateSmoother;
use crate::indicators::generalized_laguerre::GeneralizedLaguerre;
use crate::indicators::smoothing::SMA;

/// Continuation Index
/// 
/// Based on John Ehlers' "The Continuation Index" (TASC September 2025).
/// The indicator is designed to signal both the early onset and potential exhaustion of a trend.
/// It uses a Laguerre filter and UltimateSmoother to reduce lag, then compresses the result 
/// using an Inverse Fisher Transform (tanh).
#[derive(Debug, Clone)]
pub struct ContinuationIndex {
    us: UltimateSmoother,
    lg: GeneralizedLaguerre,
    variance_sma: SMA,
}

impl ContinuationIndex {
    pub fn new(gamma: f64, order: usize, length: usize) -> Self {
        Self {
            us: UltimateSmoother::new(length / 2),
            lg: GeneralizedLaguerre::new(length, gamma, order),
            variance_sma: SMA::new(length),
        }
    }
}

impl Next<f64> for ContinuationIndex {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let us_val = self.us.next(input);
        let lg_val = self.lg.next(input);
        
        let diff = us_val - lg_val;
        let variance = self.variance_sma.next(diff.abs());
        
        let ref_val = if variance != 0.0 {
            2.0 * diff / variance
        } else {
            0.0
        };

        // CI = (exp(2 * ref) - 1) / (exp(2 * ref) + 1) which is tanh(ref)
        ref_val.tanh()
    }
}

pub const CONTINUATION_INDEX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Continuation Index",
    description: "An oscillator that identifies trend onset and exhaustion by comparing a fast UltimateSmoother with a Generalized Laguerre filter.",
    params: &[
        ParamDef { name: "gamma", default: "0.8", description: "Laguerre gamma parameter" },
        ParamDef { name: "order", default: "8", description: "Laguerre filter order" },
        ParamDef { name: "length", default: "40", description: "Base smoothing length" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202025.html",
    formula_latex: r#"
\[
US = UltimateSmoother(Close, Length/2)
\]
\[
LG = Laguerre(Close, \gamma, Order, Length)
\]
\[
Variance = SMA(|US - LG|, Length)
\]
\[
Ref = 2 \times (US - LG) / Variance
\]
\[
CI = \tanh(Ref)
\]
"#,
    gold_standard_file: "continuation_index.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_continuation_index_basic() {
        let mut ci = ContinuationIndex::new(0.8, 8, 40);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        for input in inputs {
            let res = ci.next(input);
            assert!(!res.is_nan());
            assert!(res >= -1.0 && res <= 1.0);
        }
    }

    proptest! {
        #[test]
        fn test_continuation_index_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let gamma = 0.8;
            let order = 8;
            let length = 40;
            let mut ci = ContinuationIndex::new(gamma, order, length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| ci.next(x)).collect();
            
            // Reference implementation
            let mut us = UltimateSmoother::new(length / 2);
            let mut lg = GeneralizedLaguerre::new(length, gamma, order);
            let mut diffs = Vec::new();
            let mut batch_results = Vec::with_capacity(inputs.len());
            
            for &input in &inputs {
                let u = us.next(input);
                let l = lg.next(input);
                let d = u - l;
                diffs.push(d.abs());
                
                let start = if diffs.len() > length { diffs.len() - length } else { 0 };
                let window = &diffs[start..];
                let variance = window.iter().sum::<f64>() / window.len() as f64;
                
                let ref_val = if variance != 0.0 { 2.0 * d / variance } else { 0.0 };
                batch_results.push(ref_val.tanh());
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
