use crate::traits::Next;
use crate::indicators::volatility::ATR;
use crate::indicators::metadata::{IndicatorMetadata, ParamDef};

pub const METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "SuperTrend",
    description: "Trend-following indicator that combines ATR for volatility bands to identify the primary market direction.",
    params: &[
        ParamDef { name: "period", default: "10", description: "ATR length" },
        ParamDef { name: "multiplier", default: "3.0", description: "ATR multiplier" },
    ],
    formula_source: "https://www.tradingview.com/script/7zF0a4f8-SuperTrend-by-Mobius/",
    formula_latex: r#"
\[
\text{SuperTrend} = \begin{cases}
\text{LowerBand} & \text{if trend is up} \\
\text{UpperBand} & \text{if trend is down}
\end{cases}
\]
"#,
    gold_standard_file: "supertrend_10_3.json",
    category: "Classic",
};

/// SuperTrend Indicator
#[derive(Debug, Clone)]
pub struct SuperTrend {
    atr: ATR,
    multiplier: f64,
    prev_close: Option<f64>,
    prev_upper_band: Option<f64>,
    prev_lower_band: Option<f64>,
    direction: i8, // 1 for up, -1 for down
}

impl SuperTrend {
    pub fn new(period: usize, multiplier: f64) -> Self {
        Self {
            atr: ATR::new(period),
            multiplier,
            prev_close: None,
            prev_upper_band: None,
            prev_lower_band: None,
            direction: 1,
        }
    }
}

impl Next<(f64, f64, f64)> for SuperTrend {
    type Output = (f64, i8);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let atr = self.atr.next((high, low, close));
        let mid = (high + low) / 2.0;
        
        let basic_upper = mid + self.multiplier * atr;
        let basic_lower = mid - self.multiplier * atr;

        let upper_band = match self.prev_upper_band {
            Some(prev_upper) => {
                if basic_upper < prev_upper || self.prev_close.unwrap_or(0.0) > prev_upper {
                    basic_upper
                } else {
                    prev_upper
                }
            }
            None => basic_upper,
        };

        let lower_band = match self.prev_lower_band {
            Some(prev_lower) => {
                if basic_lower > prev_lower || self.prev_close.unwrap_or(0.0) < prev_lower {
                    basic_lower
                } else {
                    prev_lower
                }
            }
            None => basic_lower,
        };

        if self.direction == -1 && close > upper_band {
            self.direction = 1;
        } else if self.direction == 1 && close < lower_band {
            self.direction = -1;
        }

        let supertrend = if self.direction == 1 {
            lower_band
        } else {
            upper_band
        };

        self.prev_close = Some(close);
        self.prev_upper_band = Some(upper_band);
        self.prev_lower_band = Some(lower_band);

        (supertrend, self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;
    use proptest::prelude::*;

    #[derive(Debug, Deserialize)]
    struct SuperTrendCase {
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_st: Vec<f64>,
        expected_dir: Vec<i8>,
    }

    #[test]
    fn test_supertrend_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/supertrend_10_3.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/supertrend_10_3.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: SuperTrendCase = serde_json::from_str(&content).unwrap();

        let mut st = SuperTrend::new(10, 3.0);
        for i in 0..case.high.len() {
            let (val, dir) = st.next((case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(val, case.expected_st[i], epsilon = 1e-6);
            assert_eq!(dir, case.expected_dir[i]);
        }
    }

    fn supertrend_batch(data: Vec<(f64, f64, f64)>, period: usize, multiplier: f64) -> Vec<(f64, i8)> {
        let mut st = SuperTrend::new(period, multiplier);
        data.into_iter().map(|x| st.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_supertrend_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
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
            let multiplier = 3.0;
            let mut st = SuperTrend::new(period, multiplier);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(st.next(val));
            }

            let batch_results = supertrend_batch(adj_input, period, multiplier);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                assert_eq!(s.1, b.1);
            }
        }
    }
}
