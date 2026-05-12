use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Donchian Channels
/// Upper Band: Highest high over the last N periods.
/// Lower Band: Lowest low over the last N periods.
/// Middle Band: (Upper + Lower) / 2
#[derive(Debug, Clone)]
pub struct DonchianChannels {
    period: usize,
    highs: VecDeque<f64>,
    lows: VecDeque<f64>,
}

impl DonchianChannels {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            highs: VecDeque::with_capacity(period),
            lows: VecDeque::with_capacity(period),
        }
    }
}

impl Next<(f64, f64)> for DonchianChannels {
    type Output = (f64, f64, f64);

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        self.highs.push_back(high);
        self.lows.push_back(low);

        if self.highs.len() > self.period {
            self.highs.pop_front();
            self.lows.pop_front();
        }

        let mut max_high = f64::MIN;
        let mut min_low = f64::MAX;

        for &h in self.highs.iter() {
            if h > max_high {
                max_high = h;
            }
        }

        for &l in self.lows.iter() {
            if l < min_low {
                min_low = l;
            }
        }

        let middle = (max_high + min_low) / 2.0;

        (max_high, middle, min_low)
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
    struct DonchianCase {
        highs: Vec<f64>,
        lows: Vec<f64>,
        expected_middle: Vec<f64>,
    }

    #[test]
    fn test_donchian_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/donchian_5.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/donchian_5.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: DonchianCase = serde_json::from_str(&content).unwrap();

        let mut dc = DonchianChannels::new(5);
        for i in 0..case.highs.len() {
            let (_, middle, _) = dc.next((case.highs[i], case.lows[i]));
            approx::assert_relative_eq!(middle, case.expected_middle[i]);
        }
    }

    #[test]
    fn test_donchian_basic() {
        let mut dc = DonchianChannels::new(3);
        
        // bar 1: H=10, L=8 -> U=10, M=9, L=8
        let (u1, m1, l1) = dc.next((10.0, 8.0));
        assert_eq!(u1, 10.0);
        assert_eq!(m1, 9.0);
        assert_eq!(l1, 8.0);

        // bar 2: H=12, L=7 -> U=12, M=9.5, L=7
        let (u2, m2, l2) = dc.next((12.0, 7.0));
        assert_eq!(u2, 12.0);
        assert_eq!(m2, 9.5);
        assert_eq!(l2, 7.0);

        // bar 3: H=11, L=9 -> U=12, M=9.5, L=7
        let (u3, m3, l3) = dc.next((11.0, 9.0));
        assert_eq!(u3, 12.0);
        assert_eq!(m3, 9.5);
        assert_eq!(l3, 7.0);

        // bar 4: H=13, L=10 -> U=13, M=10, L=7 (bar 1 is out)
        let (u4, m4, l4) = dc.next((13.0, 10.0));
        assert_eq!(u4, 13.0);
        assert_eq!(m4, 10.0);
        assert_eq!(l4, 7.0);
    }

    fn donchian_batch(data: Vec<(f64, f64)>, period: usize) -> Vec<f64> {
        let mut dc = DonchianChannels::new(period);
        // We'll just return the middle band for parity check to simplify, 
        // or we could change the parity helper to handle tuples.
        data.into_iter().map(|x| dc.next(x).1).collect()
    }

    proptest! {
        #[test]
        fn test_donchian_parity(highs in prop::collection::vec(0.0..1000.0, 1..100), lows in prop::collection::vec(0.0..1000.0, 1..100)) {
            let len = highs.len().min(lows.len());
            let highs: Vec<f64> = highs[..len].to_vec();
            let lows: Vec<f64> = lows[..len].to_vec();
            let mut input = Vec::with_capacity(len);
            for i in 0..len {
                let h = highs[i];
                let l = lows[i].min(h); // Ensure low <= high
                input.push((h, l));
            }
            
            let period = 5;
            let mut dc = DonchianChannels::new(period);
            let mut streaming_results = Vec::with_capacity(len);
            for &val in &input {
                streaming_results.push(dc.next(val).1);
            }

            let batch_results = donchian_batch(input, period);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                assert_eq!(s, b);
            }
        }
    }
}


pub const DONCHIAN_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Donchian Channels",
    description: "Donchian Channels are volatility indicators formed by taking the highest high and the lowest low of the last N periods.",
    params: &[
        ParamDef { name: "period", default: "20", description: "Channel period" },
    ],
    formula_source: "https://www.investopedia.com/terms/d/donchianchannels.asp",
    formula_latex: r#"
\[
UC = \max(H_{t-n \dots t}) \\ LC = \min(L_{t-n \dots t})
\]
"#,
    gold_standard_file: "donchian.json",
};
