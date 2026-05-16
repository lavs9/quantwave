use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Adaptive Exponential Moving Average (AEMA)
/// TASC April 2019, by Vitali Apirine
#[derive(Debug, Clone)]
pub struct AdaptiveEMA {
    _period: usize,
    pds: usize,
    mltp1: f64,
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
    prev_aema: Option<f64>,
}

impl AdaptiveEMA {
    pub fn new(period: usize, pds: usize) -> Self {
        Self {
            _period: period,
            pds,
            mltp1: 2.0 / (period as f64 + 1.0),
            highs: VecDeque::with_capacity(pds),
            lows: VecDeque::with_capacity(pds),
            prev_aema: None,
        }
    }
}

impl Next<(f64, f64, f64)> for AdaptiveEMA {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        self.highs.push_back(high);
        self.lows.push_back(low);

        if self.highs.len() > self.pds {
            self.highs.pop_front();
            self.lows.pop_front();
        }

        let mut max_high = f64::MIN;
        for &h in self.highs.iter() {
            if h > max_high {
                max_high = h;
            }
        }

        let mut min_low = f64::MAX;
        for &l in self.lows.iter() {
            if l < min_low {
                min_low = l;
            }
        }

        let mltp2 = if max_high - min_low == 0.0 {
            0.0
        } else {
            (((close - min_low) - (max_high - close)).abs()) / (max_high - min_low)
        };

        let rate = self.mltp1 * (1.0 + mltp2);

        let aema = match self.prev_aema {
            Some(prev) => prev + rate * (close - prev),
            None => close,
        };

        self.prev_aema = Some(aema);
        aema
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn adaptive_ema_batch(data: Vec<(f64, f64, f64)>, period: usize, pds: usize) -> Vec<f64> {
        let mut aema = AdaptiveEMA::new(period, pds);
        data.into_iter().map(|x| aema.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_adaptive_ema_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }

            let period = 10;
            let pds = 10;
            let mut aema = AdaptiveEMA::new(period, pds);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(aema.next(val));
            }

            let batch_results = adaptive_ema_batch(adj_input, period, pds);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_adaptive_ema_basic() {
        let mut aema = AdaptiveEMA::new(10, 10);
        let val1 = aema.next((10.0, 8.0, 9.0));
        assert_eq!(val1, 9.0); // Starts with close

        let val2 = aema.next((12.0, 7.0, 11.0));
        assert!(val2 > 9.0); // Should move up
    }
}

pub const ADAPTIVE_EMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Adaptive Exponential Moving Average",
    description: "An adaptive moving average that adjusts its smoothing factor based on volatility.",
    usage: "Use to identify overall trends. AEMA reacts faster to large price movements by adapting the smoothing factor using the highest high and lowest low of a lookback period.",
    keywords: &["moving-average", "adaptive", "volatility", "trend"],
    ehlers_summary: "Introduced by Vitali Apirine in TASC April 2019, AEMA alters the EMA's alpha (smoothing factor) by comparing the distance of the close from the lowest low and highest high. This amplifies the smoothing factor during strong price moves while reducing it during sideways chop, yielding a moving average with less lag when it matters most.",
    params: &[
        ParamDef {
            name: "period",
            default: "10",
            description: "Smoothing period",
        },
        ParamDef {
            name: "pds",
            default: "10",
            description: "Lookback period for volatility",
        },
    ],
    formula_source: "Technical Analysis of Stocks & Commodities, April 2019",
    formula_latex: r#"
\[
Rate = \frac{2}{P+1} \times \left(1 + \frac{|(C - L_{min}) - (H_{max} - C)|}{H_{max} - L_{min}}\right) \\ AEMA_t = AEMA_{t-1} + Rate \times (C - AEMA_{t-1})
\]
"#,
    gold_standard_file: "",
    category: "Moving Averages",
};
