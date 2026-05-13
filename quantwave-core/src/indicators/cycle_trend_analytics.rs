use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::traits::Next;

/// Cycle/Trend Analytics Indicator
///
/// Based on John Ehlers' "Cycle/Trend Analytics And The MAD Indicator" (2021).
/// It computes a series of oscillators: Price - SMA(Price, Length) for Length 5 to 30.
#[derive(Debug, Clone)]
pub struct CycleTrendAnalytics {
    smas: Vec<SMA>,
}

impl CycleTrendAnalytics {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        let smas = (min_length..=max_length).map(SMA::new).collect();
        Self {
            smas,
        }
    }
}

impl Next<f64> for CycleTrendAnalytics {
    type Output = Vec<f64>; // Price - SMA for each length from min to max

    fn next(&mut self, input: f64) -> Self::Output {
        self.smas.iter_mut().map(|sma| input - sma.next(input)).collect()
    }
}

pub const CYCLE_TREND_ANALYTICS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Cycle/Trend Analytics",
    description: "A set of oscillators (Price - SMA) with lengths from 5 to 30 used to visualize cycles and trends.",
    params: &[
        ParamDef {
            name: "min_length",
            default: "5",
            description: "Minimum SMA length",
        },
        ParamDef {
            name: "max_length",
            default: "30",
            description: "Maximum SMA length",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - OCTOBER 2021.html",
    formula_latex: r#"
\[
Osc(L) = Price - SMA(Price, L) \quad \text{for } L \in [min, max]
\]
"#,
    gold_standard_file: "cycle_trend_analytics.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use crate::test_utils::{load_gold_standard_vec, assert_indicator_parity_vec};
    use proptest::prelude::*;

    #[test]
    fn test_cycle_trend_analytics_gold_standard() {
        let case = load_gold_standard_vec("cycle_trend_analytics");
        let cta = CycleTrendAnalytics::new(5, 15);
        assert_indicator_parity_vec(cta, &case.input, &case.expected);
    }

    #[test]
    fn test_cycle_trend_analytics_basic() {
        let mut cta = CycleTrendAnalytics::new(5, 10);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = cta.next(input);
            assert_eq!(res.len(), 6);
        }
    }

    proptest! {
        #[test]
        fn test_cycle_trend_analytics_parity(
            inputs in prop::collection::vec(1.0..100.0, 30..100),
        ) {
            let min = 5;
            let max = 15;
            let mut cta = CycleTrendAnalytics::new(min, max);
            let streaming_results: Vec<Vec<f64>> = inputs.iter().map(|&x| cta.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            for i in 0..inputs.len() {
                let mut bar_results = Vec::with_capacity(max - min + 1);
                for length in min..=max {
                    let sum: f64 = inputs[(i.saturating_sub(length - 1))..=i].iter().sum();
                    let count = (i + 1).min(length);
                    let sma = sum / count as f64;
                    bar_results.push(inputs[i] - sma);
                }
                batch_results.push(bar_results);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                for (sv, bv) in s.iter().zip(b.iter()) {
                    approx::assert_relative_eq!(sv, bv, epsilon = 1e-10);
                }
            }
        }
    }
}
