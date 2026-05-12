use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::smoothing::{EMA, SMA};

/// WaveTrend Oscillator
/// Often referred to as "LazyBear's WaveTrend" on TradingView.
/// WT1 = EMA(CI, n2)
/// WT2 = SMA(WT1, n3)
/// where:
/// CI = (AP - ESA) / (0.015 * D)
/// AP = (High + Low + Close) / 3
/// ESA = EMA(AP, n1)
/// D = EMA(|AP - ESA|, n1)
#[derive(Debug, Clone)]
pub struct WaveTrend {
    esa_ema: EMA,
    d_ema: EMA,
    wt1_ema: EMA,
    wt2_sma: SMA,
}

impl WaveTrend {
    pub fn new(n1: usize, n2: usize, n3: usize) -> Self {
        Self {
            esa_ema: EMA::new(n1),
            d_ema: EMA::new(n1),
            wt1_ema: EMA::new(n2),
            wt2_sma: SMA::new(n3),
        }
    }
}

impl Next<(f64, f64, f64)> for WaveTrend {
    type Output = (f64, f64); // (WT1, WT2)

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let ap = (high + low + close) / 3.0;
        let esa = self.esa_ema.next(ap);
        let d_raw = (ap - esa).abs();
        let d = self.d_ema.next(d_raw);

        let ci = if d != 0.0 {
            (ap - esa) / (0.015 * d)
        } else {
            0.0
        };

        let wt1 = self.wt1_ema.next(ci);
        let wt2 = self.wt2_sma.next(wt1);

        (wt1, wt2)
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
    struct WaveTrendCase {
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_wt1: Vec<f64>,
        expected_wt2: Vec<f64>,
    }

    #[test]
    fn test_wavetrend_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/wavetrend_10_21_4.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/wavetrend_10_21_4.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: WaveTrendCase = serde_json::from_str(&content).unwrap();

        let mut wt = WaveTrend::new(10, 21, 4);
        for i in 0..case.high.len() {
            let (wt1, wt2) = wt.next((case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(wt1, case.expected_wt1[i], epsilon = 1e-6);
            approx::assert_relative_eq!(wt2, case.expected_wt2[i], epsilon = 1e-6);
        }
    }

    fn wavetrend_batch(data: Vec<(f64, f64, f64)>, n1: usize, n2: usize, n3: usize) -> Vec<(f64, f64)> {
        let mut wt = WaveTrend::new(n1, n2, n3);
        data.into_iter().map(|x| wt.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_wavetrend_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }
            
            let n1 = 10;
            let n2 = 21;
            let n3 = 4;
            let mut wt = WaveTrend::new(n1, n2, n3);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(wt.next(val));
            }

            let batch_results = wavetrend_batch(adj_input, n1, n2, n3);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_wavetrend_basic() {
        let mut wt = WaveTrend::new(10, 21, 4);
        
        // Feed some dummy data
        for i in 0..50 {
            let val = 100.0 + (i as f64).sin() * 5.0;
            let (wt1, wt2) = wt.next((val + 1.0, val - 1.0, val));
            if i >= 10 {
                assert!(!wt1.is_nan());
                assert!(!wt2.is_nan());
            }
        }
    }
}


pub const WAVETREND_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "WaveTrend Oscillator",
    description: "WaveTrend is an oscillator that helps identify overbought and oversold conditions.",
    params: &[
        ParamDef { name: "n1", default: "10", description: "Channel Length" },
        ParamDef { name: "n2", default: "21", description: "Average Length" },
    ],
    formula_source: "https://www.tradingview.com/script/2KE8wTuF-Indicator-WaveTrend-Oscillator-WT/",
    formula_latex: r#"
\[
WT_1 = EMA(ESA, n_2)
\]
"#,
    gold_standard_file: "wavetrend.json",
};
