use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::traits::Next;

/// Moving Average Difference (MAD) Indicator
///
/// Based on John Ehlers' "Cycle/Trend Analytics And The MAD Indicator" (2021).
/// It computes the percentage difference between a short-term SMA and a long-term SMA.
#[derive(Debug, Clone)]
pub struct MAD {
    short_sma: SMA,
    long_sma: SMA,
}

impl MAD {
    pub fn new(short_period: usize, long_period: usize) -> Self {
        Self {
            short_sma: SMA::new(short_period),
            long_sma: SMA::new(long_period),
        }
    }
}

impl Next<f64> for MAD {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let s = self.short_sma.next(input);
        let l = self.long_sma.next(input);
        if l != 0.0 {
            100.0 * (s - l) / l
        } else {
            0.0
        }
    }
}

pub const MAD_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "MAD",
    description: "Moving Average Difference: 100 * (SMA(short) - SMA(long)) / SMA(long)",
    params: &[
        ParamDef {
            name: "short_period",
            default: "8",
            description: "Short-term SMA period",
        },
        ParamDef {
            name: "long_period",
            default: "23",
            description: "Long-term SMA period",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - OCTOBER 2021.html",
    formula_latex: r#"
\[
MAD = 100 \times \frac{SMA(short) - SMA(long)}{SMA(long)}
\]
"#,
    gold_standard_file: "mad.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use crate::test_utils::{load_gold_standard, assert_indicator_parity};
    use proptest::prelude::*;

    #[test]
    fn test_mad_gold_standard() {
        let case = load_gold_standard("mad");
        let mad = MAD::new(8, 23);
        assert_indicator_parity(mad, &case.input, &case.expected);
    }

    #[test]
    fn test_mad_basic() {
        let mut mad = MAD::new(5, 10);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = mad.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_mad_parity(
            inputs in prop::collection::vec(1.0..100.0, 20..100),
        ) {
            let short = 8;
            let long = 23;
            let mut mad = MAD::new(short, long);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| mad.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            for i in 0..inputs.len() {
                let s_sum: f64 = inputs[(i.saturating_sub(short - 1))..=i].iter().sum();
                let l_sum: f64 = inputs[(i.saturating_sub(long - 1))..=i].iter().sum();
                
                let s_count = (i + 1).min(short);
                let l_count = (i + 1).min(long);
                
                let s = s_sum / s_count as f64;
                let l = l_sum / l_count as f64;
                
                let res = if l != 0.0 {
                    100.0 * (s - l) / l
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
