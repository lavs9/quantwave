use crate::indicators::high_pass::HighPass;
use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// Precision Trend Analysis
///
/// Based on John Ehlers' "Precision Trend Analysis" (TASC September 2024).
/// Uses the difference between two HighPass filters to identify the trend
/// and its Rate of Change (ROC) to pinpoint reversals.
/// Returns (Trend, ROC).
#[derive(Debug, Clone)]
pub struct PrecisionTrendAnalysis {
    hp1: HighPass,
    hp2: HighPass,
    prev_trend: f64,
    length2: f64,
    count: usize,
}

impl PrecisionTrendAnalysis {
    pub fn new(length1: usize, length2: usize) -> Self {
        Self {
            hp1: HighPass::new(length1),
            hp2: HighPass::new(length2),
            prev_trend: 0.0,
            length2: length2 as f64,
            count: 0,
        }
    }
}

impl Default for PrecisionTrendAnalysis {
    fn default() -> Self {
        Self::new(250, 40)
    }
}

impl Next<f64> for PrecisionTrendAnalysis {
    type Output = (f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let v_hp1 = self.hp1.next(input);
        let v_hp2 = self.hp2.next(input);
        let trend = v_hp1 - v_hp2;
        
        let roc = (self.length2 / 6.28) * (trend - self.prev_trend);
        
        self.prev_trend = trend;
        (trend, roc)
    }
}

pub const PRECISION_TREND_ANALYSIS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Precision Trend Analysis",
    description: "Trend identification using the difference between two high-pass filters.",
    params: &[
        ParamDef {
            name: "length1",
            default: "250",
            description: "First HighPass filter period",
        },
        ParamDef {
            name: "length2",
            default: "40",
            description: "Second HighPass filter period",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20SEPTEMBER%202024.html",
    formula_latex: r#"
\[
HP1 = HighPass(Price, Length1)
\]
\[
HP2 = HighPass(Price, Length2)
\]
\[
Trend = HP1 - HP2
\]
\[
ROC = \frac{Length2}{6.28} \cdot (Trend - Trend_{t-1})
\]
"#,
    gold_standard_file: "precision_trend.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_precision_trend_basic() {
        let mut pt = PrecisionTrendAnalysis::new(250, 40);
        let inputs = vec![10.0; 10];
        for input in inputs {
            let (trend, roc) = pt.next(input);
            assert!(!trend.is_nan());
            assert!(!roc.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_precision_trend_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let l1 = 250;
            let l2 = 40;
            let mut pt = PrecisionTrendAnalysis::new(l1, l2);
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| pt.next(x)).collect();

            // Batch implementation
            let mut hp1 = HighPass::new(l1);
            let mut hp2 = HighPass::new(l2);
            let mut prev_trend = 0.0;
            let mut batch_results = Vec::with_capacity(inputs.len());

            for &input in inputs.iter() {
                let v_hp1 = hp1.next(input);
                let v_hp2 = hp2.next(input);
                let trend = v_hp1 - v_hp2;
                let roc = (l2 as f64 / 6.28) * (trend - prev_trend);
                prev_trend = trend;
                batch_results.push((trend, roc));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
