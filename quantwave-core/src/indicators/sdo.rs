use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::EMA;
use crate::traits::Next;
use std::collections::VecDeque;

/// Stochastic Distance Oscillator (SDO)
///
/// Based on Vitali Apirine's article "The Stochastic Distance Oscillator" (TASC June 2023).
/// The SDO is a momentum study that shows the magnitude of the current distance relative
/// to the maximum-minimum distance range over a set period.
#[derive(Debug, Clone)]
pub struct SDO {
    lookback_period: usize,
    period: usize,
    ema: EMA,
    prices: VecDeque<f64>,
    distances: VecDeque<f64>,
}

impl SDO {
    pub fn new(lookback_period: usize, period: usize, ema_pds: usize) -> Self {
        Self {
            lookback_period,
            period,
            ema: EMA::new(ema_pds),
            prices: VecDeque::with_capacity(period + 1),
            distances: VecDeque::with_capacity(lookback_period),
        }
    }
}

impl Default for SDO {
    fn default() -> Self {
        Self::new(200, 12, 3)
    }
}

impl Next<f64> for SDO {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.prices.push_back(input);
        if self.prices.len() > self.period + 1 {
            self.prices.pop_front();
        }

        if self.prices.len() <= self.period {
            return 0.0;
        }

        let prev_price = self.prices[0];
        let dist = (input - prev_price).abs();
        
        self.distances.push_back(dist);
        if self.distances.len() > self.lookback_period {
            self.distances.pop_front();
        }

        let mut max_dist = f64::MIN;
        let mut min_dist = f64::MAX;

        for &d in self.distances.iter() {
            if d > max_dist {
                max_dist = d;
            }
            if d < min_dist {
                min_dist = d;
            }
        }

        let mut ddo = 0.0;
        let range = max_dist - min_dist;
        if range > 0.0 {
            ddo = (dist - min_dist) / range;
        }

        let dd_val = if input > prev_price {
            ddo
        } else if input < prev_price {
            -ddo
        } else {
            0.0
        };

        self.ema.next(dd_val) * 100.0
    }
}

pub const SDO_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Stochastic Distance Oscillator",
    description: "A momentum indicator based on the classic stochastic oscillator applied to price distances.",
    usage: "Identify bull and bear trend changes through overbought (+40) and oversold (-40) levels. Suitable for both trending and ranging markets.",
    keywords: &["momentum", "stochastic", "oscillator", "apirine", "trend"],
    ehlers_summary: "The Stochastic Distance Oscillator (SDO) by Vitali Apirine adapts the stochastic formula to measure the current price distance relative to its historical range. By smoothing this relative distance with an EMA, it provides a cleaner momentum signal that identifies potential trend reversals when crossing extreme thresholds.",
    params: &[
        ParamDef {
            name: "lookback_period",
            default: "200",
            description: "Range lookback for stochastic calculation",
        },
        ParamDef {
            name: "period",
            default: "12",
            description: "Distance calculation period",
        },
        ParamDef {
            name: "ema_pds",
            default: "3",
            description: "Smoothing EMA period",
        },
    ],
    formula_source: "https://traders.com/Documentation/FEEDbk_docs/2023/06/TradersTips.html",
    formula_latex: r#"
\[
Dist = |Price_t - Price_{t-n}|
\]
\[
DVal = \frac{Dist - \min(Dist_{lookback})}{\max(Dist_{lookback}) - \min(Dist_{lookback})}
\]
\[
DDVal = \begin{cases} DVal & \text{if } Price_t > Price_{t-n} \\ -DVal & \text{if } Price_t < Price_{t-n} \\ 0 & \text{otherwise} \end{cases}
\]
\[
SDO = EMA(DDVal, smoothing) \times 100
\]
"#,
    gold_standard_file: "sdo.json",
    category: "Momentum",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_sdo_basic() {
        let mut sdo = SDO::new(200, 12, 3);
        for i in 0..250 {
            let val = sdo.next(100.0 + i as f64);
            assert!(!val.is_nan());
            if i > 12 {
                assert!(val >= 0.0); // Trending up
            }
        }
    }

    proptest! {
        #[test]
        fn test_sdo_parity(
            inputs in prop::collection::vec(1.0..100.0, 250..300),
        ) {
            let lookback = 200;
            let period = 12;
            let ema_pds = 3;
            let mut sdo_obj = SDO::new(lookback, period, ema_pds);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| sdo_obj.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut ema_obj = EMA::new(ema_pds);
            let mut distances = Vec::new();

            for (i, &input) in inputs.iter().enumerate() {
                if i < period {
                    batch_results.push(0.0);
                    continue;
                }

                let prev_price = inputs[i - period];
                let dist = (input - prev_price).abs();
                distances.push(dist);

                let start = if distances.len() > lookback { distances.len() - lookback } else { 0 };
                let current_window = &distances[start..];
                
                let mut max_dist = f64::MIN;
                let mut min_dist = f64::MAX;
                for &d in current_window {
                    if d > max_dist { max_dist = d; }
                    if d < min_dist { min_dist = d; }
                }

                let mut ddo = 0.0;
                let range = max_dist - min_dist;
                if range > 0.0 {
                    ddo = (dist - min_dist) / range;
                }

                let dd_val = if input > prev_price {
                    ddo
                } else if input < prev_price {
                    -ddo
                } else {
                    0.0
                };

                batch_results.push(ema_obj.next(dd_val) * 100.0);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
