use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::donchian::DonchianChannels;

/// Ichimoku Cloud (Ichimoku Kinko Hyo)
/// Outputs: (Tenkan-sen, Kijun-sen, Senkou Span A, Senkou Span B)
/// The lagging span (Chikou Span) is simply the close price.
/// Note: Senkou Span A and B are meant to be plotted 26 periods ahead in the future.
/// This implementation returns the values calculated at the current bar, without applying the future offset.
#[derive(Debug, Clone)]
pub struct IchimokuCloud {
    tenkan_dc: DonchianChannels,
    kijun_dc: DonchianChannels,
    senkou_b_dc: DonchianChannels,
}

impl IchimokuCloud {
    pub fn new(tenkan_period: usize, kijun_period: usize, senkou_b_period: usize) -> Self {
        Self {
            tenkan_dc: DonchianChannels::new(tenkan_period),
            kijun_dc: DonchianChannels::new(kijun_period),
            senkou_b_dc: DonchianChannels::new(senkou_b_period),
        }
    }
}

impl Next<(f64, f64)> for IchimokuCloud {
    type Output = (f64, f64, f64, f64); // (Tenkan, Kijun, Senkou A, Senkou B)

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        let (_, tenkan, _) = self.tenkan_dc.next((high, low));
        let (_, kijun, _) = self.kijun_dc.next((high, low));
        let (_, senkou_b, _) = self.senkou_b_dc.next((high, low));

        let senkou_a = (tenkan + kijun) / 2.0;

        (tenkan, kijun, senkou_a, senkou_b)
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
    struct IchimokuCase {
        high: Vec<f64>,
        low: Vec<f64>,
        expected_tenkan: Vec<f64>,
        expected_kijun: Vec<f64>,
        expected_senkou_a: Vec<f64>,
        expected_senkou_b: Vec<f64>,
    }

    #[test]
    fn test_ichimoku_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/ichimoku.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/ichimoku.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: IchimokuCase = serde_json::from_str(&content).unwrap();

        let mut ic = IchimokuCloud::new(9, 26, 52);
        for i in 0..case.high.len() {
            let (t, k, sa, sb) = ic.next((case.high[i], case.low[i]));
            approx::assert_relative_eq!(t, case.expected_tenkan[i], epsilon = 1e-6);
            approx::assert_relative_eq!(k, case.expected_kijun[i], epsilon = 1e-6);
            approx::assert_relative_eq!(sa, case.expected_senkou_a[i], epsilon = 1e-6);
            approx::assert_relative_eq!(sb, case.expected_senkou_b[i], epsilon = 1e-6);
        }
    }

    fn ichimoku_batch(data: Vec<(f64, f64)>, p1: usize, p2: usize, p3: usize) -> Vec<(f64, f64, f64, f64)> {
        let mut ic = IchimokuCloud::new(p1, p2, p3);
        data.into_iter().map(|x| ic.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_ichimoku_parity(input in prop::collection::vec((0.0..100.0, 0.0..100.0), 1..100)) {
            let mut adj_input = Vec::with_capacity(input.len());
            for (h, l) in input {
                let h_f: f64 = h;
                let l_f: f64 = l;
                let high = h_f.max(l_f);
                let low = l_f.min(h_f);
                adj_input.push((high, low));
            }
            
            let p1 = 9;
            let p2 = 26;
            let p3 = 52;
            let mut ic = IchimokuCloud::new(p1, p2, p3);
            let mut streaming_results = Vec::with_capacity(adj_input.len());
            for &val in &adj_input {
                streaming_results.push(ic.next(val));
            }

            let batch_results = ichimoku_batch(adj_input, p1, p2, p3);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-6);
                approx::assert_relative_eq!(s.3, b.3, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_ichimoku_basic() {
        let mut ic = IchimokuCloud::new(3, 5, 10);
        let (t, k, sa, sb) = ic.next((10.0, 8.0));
        assert_eq!(t, 9.0);
        assert_eq!(k, 9.0);
        assert_eq!(sa, 9.0);
        assert_eq!(sb, 9.0);
    }
}


pub const ICHIMOKU_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ichimoku Cloud",
    description: "Ichimoku Kinko Hyo is a comprehensive indicator that defines support and resistance, identifies trend direction, gauges momentum and provides trading signals.",
    params: &[
        ParamDef { name: "tenkan_period", default: "9", description: "Tenkan-sen period" },
        ParamDef { name: "kijun_period", default: "26", description: "Kijun-sen period" },
        ParamDef { name: "senkou_span_b_period", default: "52", description: "Senkou Span B period" },
    ],
    formula_source: "https://www.investopedia.com/terms/i/ichimoku-cloud.asp",
    formula_latex: r#"
\[
\text{Tenkan-sen} = \frac{\text{Highest High} + \text{Lowest Low}}{2} \text{ for past 9 periods}
\]
"#,
    gold_standard_file: "ichimoku.json",
};
