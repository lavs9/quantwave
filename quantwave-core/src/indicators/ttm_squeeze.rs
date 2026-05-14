use crate::indicators::donchian::DonchianChannels;
use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::indicators::statistics::{LinearRegression, StandardDeviation};
use crate::indicators::volatility::ATR;
use crate::traits::Next;

/// TTM Squeeze Indicator
/// Combines Bollinger Bands and Keltner Channels to identify volatility compression.
/// Includes a momentum histogram based on linear regression.
#[derive(Debug, Clone)]
pub struct TTMSqueeze {
    sma: SMA,
    stdev: StandardDeviation,
    atr: ATR,
    donchian: DonchianChannels,
    linreg: LinearRegression,
    multiplier_bb: f64,
    multiplier_kc: f64,
}

impl TTMSqueeze {
    pub fn new(period: usize, multiplier_bb: f64, multiplier_kc: f64) -> Self {
        Self {
            sma: SMA::new(period),
            stdev: StandardDeviation::new(period),
            atr: ATR::new(period),
            donchian: DonchianChannels::new(period),
            linreg: LinearRegression::new(period),
            multiplier_bb,
            multiplier_kc,
        }
    }
}

/// Output of TTM Squeeze: (Momentum Histogram, Is Squeezed)
impl Next<(f64, f64, f64)> for TTMSqueeze {
    type Output = (f64, bool);

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let sma = self.sma.next(close);
        let stdev = self.stdev.next(close);
        let atr = self.atr.next((high, low, close));
        let (_max_high, donchian_mid, _min_low) = self.donchian.next((high, low));

        // Bollinger Bands
        let bb_upper = sma + self.multiplier_bb * stdev;
        let bb_lower = sma - self.multiplier_bb * stdev;

        // Keltner Channels (using SMA of close as midline for TTM Squeeze standard)
        let kc_upper = sma + self.multiplier_kc * atr;
        let kc_lower = sma - self.multiplier_kc * atr;

        // Squeeze Status
        let is_squeezed = bb_upper < kc_upper && bb_lower > kc_lower;

        // Momentum Histogram
        // Value = Close - (DonchianMidpoint + SMAClose) / 2
        let momentum_base = close - (donchian_mid + sma) / 2.0;
        let histogram = self.linreg.next(momentum_base);

        (histogram, is_squeezed)
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
    struct TTMCase {
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_histogram: Vec<f64>,
        expected_squeezed: Vec<bool>,
    }

    #[test]
    fn test_ttm_squeeze_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/ttm_squeeze_20_2_15.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path
                .parent()
                .unwrap()
                .join("tests/gold_standard/ttm_squeeze_20_2_15.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: TTMCase = serde_json::from_str(&content).unwrap();

        let mut ttm = TTMSqueeze::new(20, 2.0, 1.5);
        for i in 0..case.high.len() {
            let (hist, is_sq) = ttm.next((case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(hist, case.expected_histogram[i], epsilon = 1e-6);
            assert_eq!(is_sq, case.expected_squeezed[i], "Mismatch at index {}", i);
        }
    }

    fn ttm_batch(
        data: Vec<(f64, f64, f64)>,
        period: usize,
        multiplier_bb: f64,
        multiplier_kc: f64,
    ) -> Vec<f64> {
        let mut ttm = TTMSqueeze::new(period, multiplier_bb, multiplier_kc);
        data.into_iter().map(|x| ttm.next(x).0).collect()
    }

    proptest! {
        #[test]
        fn test_ttm_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l, c) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(l_f).max(c_f);
                let low = l_f.min(h_f).min(c_f);
                adj_input.push((high, low, c_f));
            }

            let period = 20;
            let m_bb = 2.0;
            let m_kc = 1.5;
            let mut ttm = TTMSqueeze::new(period, m_bb, m_kc);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(ttm.next(val).0);
            }

            let batch_results = ttm_batch(adj_input, period, m_bb, m_kc);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_ttm_squeeze_basic() {
        let mut ttm = TTMSqueeze::new(20, 2.0, 1.5);

        // Feed some data to warm up
        for i in 0..50 {
            let val = 100.0 + (i as f64).sin() * 5.0;
            let (hist, _squeezed) = ttm.next((val + 1.0, val - 1.0, val));
            if i >= 19 {
                assert!(!hist.is_nan());
            }
        }
    }
}

pub const TTM_SQUEEZE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "TTM Squeeze",
    description: "TTM Squeeze measures the relationship between Bollinger Bands and Keltner Channels to identify volatility consolidations.",
    usage: "Use to identify periods of compressed volatility (Bollinger Bands inside Keltner Channels) followed by high-energy breakouts. The momentum histogram direction at squeeze release indicates trade direction.",
    keywords: &["volatility", "momentum", "breakout", "squeeze", "classic"],
    ehlers_summary: "The TTM Squeeze, developed by John Carter, identifies market consolidation by detecting when Bollinger Bands contract inside Keltner Channels — a squeeze condition indicating coiling energy. When the bands expand back outside the Keltner Channels, the squeeze releases and a momentum histogram shows the expected breakout direction. — Mastering the Trade, John Carter",
    params: &[
        ParamDef {
            name: "bb_period",
            default: "20",
            description: "Bollinger Bands Period",
        },
        ParamDef {
            name: "bb_mult",
            default: "2.0",
            description: "Bollinger Bands Multiplier",
        },
        ParamDef {
            name: "kc_period",
            default: "20",
            description: "Keltner Channel Period",
        },
        ParamDef {
            name: "kc_mult",
            default: "1.5",
            description: "Keltner Channel Multiplier",
        },
    ],
    formula_source: "https://www.investopedia.com/articles/active-trading/110714/intro-ttm-squeeze-indicator.asp",
    formula_latex: r#"
\[
\text{Squeeze} = BB_{width} < KC_{width}
\]
"#,
    gold_standard_file: "ttm_squeeze.json",
    category: "Classic",
};
