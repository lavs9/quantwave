use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::smoothing::EMA;
use crate::indicators::volatility::ATR;

#[derive(Debug, Clone)]
pub struct KeltnerChannels {
    ema: EMA,
    atr: ATR,
    multiplier: f64,
}

impl KeltnerChannels {
    pub fn new(ema_period: usize, atr_period: usize, multiplier: f64) -> Self {
        Self {
            ema: EMA::new(ema_period),
            atr: ATR::new(atr_period),
            multiplier,
        }
    }
}

impl Next<(f64, f64, f64)> for KeltnerChannels {
    type Output = (f64, f64, f64);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let typical_price = (high + low + close) / 3.0;
        let middle = self.ema.next(typical_price);
        let atr = self.atr.next((high, low, close));
        
        let upper = middle + self.multiplier * atr;
        let lower = middle - self.multiplier * atr;

        (upper, middle, lower)
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
    struct KeltnerCase {
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_upper: Vec<f64>,
        expected_middle: Vec<f64>,
        expected_lower: Vec<f64>,
    }

    #[test]
    fn test_keltner_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/keltner_20_20_15.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/keltner_20_20_15.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: KeltnerCase = serde_json::from_str(&content).unwrap();

        let mut kc = KeltnerChannels::new(20, 20, 1.5);
        for i in 0..case.high.len() {
            let (u, m, l) = kc.next((case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(u, case.expected_upper[i], epsilon = 1e-6);
            approx::assert_relative_eq!(m, case.expected_middle[i], epsilon = 1e-6);
            approx::assert_relative_eq!(l, case.expected_lower[i], epsilon = 1e-6);
        }
    }

    fn keltner_batch(data: Vec<(f64, f64, f64)>, ema_period: usize, atr_period: usize, multiplier: f64) -> Vec<(f64, f64, f64)> {
        let mut kc = KeltnerChannels::new(ema_period, atr_period, multiplier);
        data.into_iter().map(|x| kc.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_keltner_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }
            
            let ema_period = 20;
            let atr_period = 20;
            let multiplier = 1.5;
            let mut kc = KeltnerChannels::new(ema_period, atr_period, multiplier);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(kc.next(val));
            }

            let batch_results = keltner_batch(adj_input, ema_period, atr_period, multiplier);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_keltner_basic() {
        let mut kc = KeltnerChannels::new(3, 3, 2.0);
        // Typical price = (H+L+C)/3
        // bar 1: H=12, L=8, C=10 -> TP=10. ATR=4 (since TR=4). EMA=10.
        // Upper = 10 + 2*4 = 18. Lower = 10 - 2*4 = 2.
        
        let (upper, middle, lower) = kc.next((12.0, 8.0, 10.0));
        approx::assert_relative_eq!(middle, 10.0);
        approx::assert_relative_eq!(upper, 18.0);
        approx::assert_relative_eq!(lower, 2.0);
    }
}


pub const KELTNER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Keltner Channels",
    description: "Keltner Channels are volatility-based envelopes set above and below an exponential moving average.",
    params: &[
        ParamDef { name: "period", default: "20", description: "EMA Period" },
        ParamDef { name: "multiplier", default: "2.0", description: "ATR Multiplier" },
    ],
    formula_source: "https://www.investopedia.com/terms/k/keltnerchannel.asp",
    formula_latex: r#"
\[
UC = EMA + (Multiplier \times ATR)
\]
"#,
    gold_standard_file: "keltner.json",
};
