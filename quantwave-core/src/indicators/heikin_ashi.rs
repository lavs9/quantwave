use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;

/// Heikin-Ashi Candlesticks
/// HA_Close = (Open + High + Low + Close) / 4
/// HA_Open = (prev_HA_Open + prev_HA_Close) / 2
/// HA_High = max(High, HA_Open, HA_Close)
/// HA_Low = min(Low, HA_Open, HA_Close)
#[derive(Debug, Clone)]
pub struct HeikinAshi {
    prev_ha_open: Option<f64>,
    prev_ha_close: Option<f64>,
}

impl HeikinAshi {
    pub fn new() -> Self {
        Self {
            prev_ha_open: None,
            prev_ha_close: None,
        }
    }
}

impl Next<(f64, f64, f64, f64)> for HeikinAshi {
    type Output = (f64, f64, f64, f64); // (HA_Open, HA_High, HA_Low, HA_Close)

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        let ha_close = (open + high + low + close) / 4.0;

        let ha_open = match (self.prev_ha_open, self.prev_ha_close) {
            (Some(prev_open), Some(prev_close)) => (prev_open + prev_close) / 2.0,
            _ => (open + close) / 2.0,
        };

        let ha_high = high.max(ha_open).max(ha_close);
        let ha_low = low.min(ha_open).min(ha_close);

        self.prev_ha_open = Some(ha_open);
        self.prev_ha_close = Some(ha_close);

        (ha_open, ha_high, ha_low, ha_close)
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
    struct HACase {
        open: Vec<f64>,
        high: Vec<f64>,
        low: Vec<f64>,
        close: Vec<f64>,
        expected_open: Vec<f64>,
        expected_high: Vec<f64>,
        expected_low: Vec<f64>,
        expected_close: Vec<f64>,
    }

    #[test]
    fn test_heikin_ashi_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/heikin_ashi.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path
                .parent()
                .unwrap()
                .join("tests/gold_standard/heikin_ashi.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: HACase = serde_json::from_str(&content).unwrap();

        let mut ha = HeikinAshi::new();
        for i in 0..case.open.len() {
            let (o, h, l, c) = ha.next((case.open[i], case.high[i], case.low[i], case.close[i]));
            approx::assert_relative_eq!(o, case.expected_open[i], epsilon = 1e-6);
            approx::assert_relative_eq!(h, case.expected_high[i], epsilon = 1e-6);
            approx::assert_relative_eq!(l, case.expected_low[i], epsilon = 1e-6);
            approx::assert_relative_eq!(c, case.expected_close[i], epsilon = 1e-6);
        }
    }

    fn heikin_ashi_batch(data: Vec<(f64, f64, f64, f64)>) -> Vec<(f64, f64, f64, f64)> {
        let mut ha = HeikinAshi::new();
        data.into_iter().map(|x| ha.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_heikin_ashi_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0, 0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (o, h, l, c) in input {
                let o_f: f64 = o;
                let h_f: f64 = h;
                let l_f: f64 = l;
                let c_f: f64 = c;
                let high = h_f.max(o_f).max(l_f).max(c_f);
                let low = l_f.min(o_f).min(h_f).min(c_f);
                adj_input.push((o_f, high, low, c_f));
            }

            let mut ha = HeikinAshi::new();
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(ha.next(val));
            }

            let batch_results = heikin_ashi_batch(adj_input);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-6);
                approx::assert_relative_eq!(s.3, b.3, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_heikin_ashi_basic() {
        let mut ha = HeikinAshi::new();

        // Bar 1: O=10, H=12, L=8, C=11
        // HA_Close = (10+12+8+11)/4 = 41/4 = 10.25
        // HA_Open = (10+11)/2 = 10.5
        // HA_High = max(12, 10.5, 10.25) = 12
        // HA_Low = min(8, 10.5, 10.25) = 8
        let (o1, h1, l1, c1) = ha.next((10.0, 12.0, 8.0, 11.0));
        assert_eq!(o1, 10.5);
        assert_eq!(h1, 12.0);
        assert_eq!(l1, 8.0);
        assert_eq!(c1, 10.25);

        // Bar 2: O=11, H=13, L=10, C=12
        // HA_Close = (11+13+10+12)/4 = 46/4 = 11.5
        // HA_Open = (10.5 + 10.25)/2 = 20.75 / 2 = 10.375
        // HA_High = max(13, 10.375, 11.5) = 13
        // HA_Low = min(10, 10.375, 11.5) = 10
        let (o2, h2, l2, c2) = ha.next((11.0, 13.0, 10.0, 12.0));
        assert_eq!(o2, 10.375);
        assert_eq!(h2, 13.0);
        assert_eq!(l2, 10.0);
        assert_eq!(c2, 11.5);
    }
}

pub const HEIKIN_ASHI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Heikin-Ashi",
    description: "Heikin-Ashi candles filter market noise to reveal the underlying trend.",
    usage: "Use to smooth candlestick charts and reduce noise for trend identification. Two or more consecutive same-colored HA candles with no lower/upper wicks confirm a strong trend.",
    keywords: &["trend", "candlestick", "smoothing", "classic", "visualization"],
    ehlers_summary: "Heikin-Ashi candles, developed by Munehisa Homma in the 18th century, use averaged OHLC values to produce smoother candles that better represent the prevailing trend. Each HA bar open equals the midpoint of the previous HA bar, while close equals the OHLC average, creating continuity that raw candles lack. — StockCharts ChartSchool",
    params: &[],
    formula_source: "https://www.investopedia.com/trading/heikin-ashi-better-candlestick/",
    formula_latex: r#"
\[
HA_{Close} = \frac{O + H + L + C}{4} \\ HA_{Open} = \frac{HA_{Open, t-1} + HA_{Close, t-1}}{2}
\]
"#,
    gold_standard_file: "heikin_ashi.json",
    category: "Classic",
};
