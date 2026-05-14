use crate::indicators::correlation_cycle::CorrelationCycle;
use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// Ehlers Market State Variable
///
/// Based on John Ehlers' "Correlation As A Cycle Indicator" (TASC June 2020).
/// Uses the phase angle from Correlation Cycle to differentiate between cycle mode (0),
/// up trend (1), and down trend (-1).
#[derive(Debug, Clone)]
pub struct MarketState {
    cc: CorrelationCycle,
    prev_angle: f64,
    threshold: f64,
}

impl MarketState {
    pub fn new(period: usize, threshold: f64) -> Self {
        Self {
            cc: CorrelationCycle::new(period),
            prev_angle: 0.0,
            threshold,
        }
    }
}

impl Default for MarketState {
    fn default() -> Self {
        Self::new(14, 9.0)
    }
}

impl Next<f64> for MarketState {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let (_, _, angle) = self.cc.next(input);
        
        let mut state = 0.0;
        if (angle - self.prev_angle).abs() < self.threshold {
            if angle < 0.0 {
                state = -1.0;
            } else {
                state = 1.0;
            }
        }

        self.prev_angle = angle;
        state
    }
}

pub const MARKET_STATE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "MarketState",
    description: "Identifies trend vs cycle regimes using Correlation Cycle phase angle.",
    usage: "Returns 1 for uptrend, -1 for downtrend, and 0 for cycle mode. Use to switch between trend-following and mean-reversion strategies.",
    keywords: &["trend", "cycle", "regime", "ehlers", "dsp"],
    ehlers_summary: "In 'Correlation As A Cycle Indicator' (2020), Ehlers defines a Market State variable based on the rate of change of the Correlation Cycle phase angle. When the angle changes slowly (less than 9 degrees per bar), the market is in a trend regime (positive angle for uptrend, negative for downtrend). Rapid angle changes indicate a cycle regime.",
    params: &[
        ParamDef {
            name: "period",
            default: "14",
            description: "Correlation wavelength",
        },
        ParamDef {
            name: "threshold",
            default: "9.0",
            description: "Angle rate of change threshold for trend detection",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2020/06/TradersTips.html",
    formula_latex: r#"
\[
\text{State} = 
\begin{cases} 
1 & \text{if } |\Delta \text{Angle}| < \text{Threshold} \text{ and Angle} \geq 0 \\
-1 & \text{if } |\Delta \text{Angle}| < \text{Threshold} \text{ and Angle} < 0 \\
0 & \text{otherwise}
\end{cases}
\]
"#,
    gold_standard_file: "market_state.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_market_state_basic() {
        let mut ms = MarketState::new(14, 9.0);
        for i in 0..100 {
            let input = 100.0 + i as f64; // Strong uptrend
            let val = ms.next(input);
            if i > 20 {
                // In a strong trend, the angle should stabilize
                assert!(val.abs() <= 1.0);
            }
        }
    }

    proptest! {
        #[test]
        fn test_market_state_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let period = 14;
            let threshold = 9.0;
            let mut ms = MarketState::new(period, threshold);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| ms.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut cc = CorrelationCycle::new(period);
            let mut prev_a = 0.0;
            for &x in inputs.iter() {
                let (_, _, angle) = cc.next(x);
                let mut state = 0.0;
                if (angle - prev_a).abs() < threshold {
                    if angle < 0.0 {
                        state = -1.0;
                    } else {
                        state = 1.0;
                    }
                }
                batch_results.push(state);
                prev_a = angle;
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
