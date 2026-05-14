use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::volatility::ATR;
use crate::traits::Next;

/// ATR Trailing Stop
/// A volatility-based trailing stop.
/// Long Stop = Close - Multiplier * ATR
/// Short Stop = Close + Multiplier * ATR
#[derive(Debug, Clone)]
pub struct ATRTrailingStop {
    atr: ATR,
    multiplier: f64,
    prev_stop: Option<f64>,
    direction: i8, // 1 for Long, -1 for Short
}

impl ATRTrailingStop {
    pub fn new(period: usize, multiplier: f64) -> Self {
        Self {
            atr: ATR::new(period),
            multiplier,
            prev_stop: None,
            direction: 1,
        }
    }
}

impl Next<(f64, f64, f64)> for ATRTrailingStop {
    type Output = (f64, i8); // (Stop Level, Direction)

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let current_atr = self.atr.next((high, low, close));
        let long_stop = close - self.multiplier * current_atr;
        let short_stop = close + self.multiplier * current_atr;

        let prev_stop = match self.prev_stop {
            Some(stop) => stop,
            None => {
                self.prev_stop = Some(long_stop);
                self.direction = 1;
                return (long_stop, 1);
            }
        };

        if self.direction == 1 {
            if close < prev_stop {
                self.direction = -1;
                self.prev_stop = Some(short_stop);
                (short_stop, -1)
            } else {
                let new_stop = prev_stop.max(long_stop);
                self.prev_stop = Some(new_stop);
                (new_stop, 1)
            }
        } else {
            if close > prev_stop {
                self.direction = 1;
                self.prev_stop = Some(long_stop);
                (long_stop, 1)
            } else {
                let new_stop = prev_stop.min(short_stop);
                self.prev_stop = Some(new_stop);
                (new_stop, -1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct ATRTSCase {
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_stop: Vec<f64>,
        expected_dir: Vec<i8>,
    }

    #[test]
    fn test_atr_ts_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/atr_ts_14_25.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path
                .parent()
                .unwrap()
                .join("tests/gold_standard/atr_ts_14_25.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: ATRTSCase = serde_json::from_str(&content).unwrap();

        let mut atr_ts = ATRTrailingStop::new(14, 2.5);
        for i in 0..case.high.len() {
            let (stop, dir) = atr_ts.next((case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(stop, case.expected_stop[i], epsilon = 1e-6);
            assert_eq!(dir, case.expected_dir[i]);
        }
    }

    fn atr_ts_batch(data: Vec<(f64, f64, f64)>, period: usize, multiplier: f64) -> Vec<(f64, i8)> {
        let mut atr_ts = ATRTrailingStop::new(period, multiplier);
        data.into_iter().map(|x| atr_ts.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_atr_ts_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }

            let period = 14;
            let multiplier = 2.5;
            let mut atr_ts = ATRTrailingStop::new(period, multiplier);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(atr_ts.next(val));
            }

            let batch_results = atr_ts_batch(adj_input, period, multiplier);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                assert_eq!(s.1, b.1);
            }
        }
    }

    #[test]
    fn test_atr_ts_basic() {
        let mut atr_ts = ATRTrailingStop::new(14, 2.5);

        let (stop1, dir1) = atr_ts.next((10.0, 8.0, 9.0));
        assert!(stop1 < 9.0);
        assert_eq!(dir1, 1);
    }
}

pub const ATR_TS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "ATR Trailing Stop",
    description: "A trailing stop based on Average True Range to keep trades in a trend.",
    usage: "Use as a dynamic trailing stop that widens in volatile markets and tightens in calm ones, automatically adjusting stop distance to current market conditions.",
    keywords: &["volatility", "trend", "stop-loss", "atr", "classic"],
    ehlers_summary: "ATR Trailing Stop uses Average True Range to set a stop distance that scales with market volatility. During high-volatility regimes the stop moves further from price to avoid premature exit; during low-volatility regimes it tightens to lock in more profit. It is one of the most robust mechanical stop methods in systematic trading.",
    params: &[
        ParamDef {
            name: "period",
            default: "10",
            description: "ATR period",
        },
        ParamDef {
            name: "multiplier",
            default: "3.0",
            description: "ATR Multiplier",
        },
    ],
    formula_source: "https://www.tradingview.com/support/solutions/43000589105-average-true-range-atr/",
    formula_latex: r#"
\[
Stop = P_{high} - (Multiplier \times ATR)
\]
"#,
    gold_standard_file: "atr_ts.json",
    category: "Classic",
};
