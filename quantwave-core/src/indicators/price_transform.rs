use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

talib_4_in_1_out!(AVGPRICE, talib_rs::price_transform::avgprice);
impl Default for AVGPRICE {
    fn default() -> Self {
        Self::new()
    }
}
talib_2_in_1_out!(MEDPRICE, talib_rs::price_transform::medprice);
impl Default for MEDPRICE {
    fn default() -> Self {
        Self::new()
    }
}
talib_3_in_1_out!(TYPPRICE, talib_rs::price_transform::typprice);
impl Default for TYPPRICE {
    fn default() -> Self {
        Self::new()
    }
}
talib_3_in_1_out!(WCLPRICE, talib_rs::price_transform::wclprice);
impl Default for WCLPRICE {
    fn default() -> Self {
        Self::new()
    }
}

/// (Open + Close) / 2
/// 
/// Based on John Ehlers' "Every Little Bit Helps" (2023).
/// Used to reduce noise in technical indicators by averaging the open and close.
#[derive(Debug, Clone, Default)]
pub struct OC2;

impl OC2 {
    pub fn new() -> Self {
        Self
    }
}

impl Next<(f64, f64)> for OC2 {
    type Output = f64;
    fn next(&mut self, input: (f64, f64)) -> Self::Output {
        (input.0 + input.1) / 2.0
    }
}

pub const AVGPRICE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Average Price (AVGPRICE)",
    description: "The simple average of the Open, High, Low, and Close prices for a given period.",
    usage: "Use as a smoothed price input for other indicators. It provides a more balanced view of the period's price action than the Close price alone.",
    keywords: &["price-transform", "classic", "smoothing"],
    ehlers_summary: "Average Price is the arithmetic mean of the four key price points in a bar. In technical analysis, using Average Price instead of Close can help filter out erratic price spikes and provide a more stable foundation for trend-following algorithms. — TA-Lib Documentation",
    params: &[],
    formula_source: "https://www.tradingview.com/support/solutions/43000502588-average-price-avgprice/",
    formula_latex: r#"
\[
AVGPRICE = \frac{Open + High + Low + Close}{4}
\]
"#,
    gold_standard_file: "avgprice.json",
    category: "Classic",
};

pub const MEDPRICE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Median Price (MEDPRICE)",
    description: "The midpoint between the High and Low prices for a given period.",
    usage: "Use to identify the central tendency of a bar's range. It is the basis for many oscillators and trend-following indicators like the Bill Williams Alligator.",
    keywords: &["price-transform", "classic", "midpoint"],
    ehlers_summary: "Median Price represents the 50% retracement level of the current period's range. By focusing on the High-Low midpoint, it removes the 'bias' of the closing price, which can often be manipulated by end-of-day positioning. — TA-Lib Documentation",
    params: &[],
    formula_source: "https://www.tradingview.com/support/solutions/43000502589-median-price-medprice/",
    formula_latex: r#"
\[
MEDPRICE = \frac{High + Low}{2}
\]
"#,
    gold_standard_file: "medprice.json",
    category: "Classic",
};

pub const TYPPRICE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Typical Price (TYPPRICE)",
    description: "An average of the High, Low, and Close prices.",
    usage: "Use as the primary price input for the Money Flow Index (MFI) and Commodity Channel Index (CCI). It provides a representative price level for the entire bar.",
    keywords: &["price-transform", "classic"],
    ehlers_summary: "Typical Price is a simple average of the High, Low, and Close. It is widely used in indicators that measure the relationship between price and volume, as it offers a more comprehensive view of the day's activity than the Close price alone. — StockCharts ChartSchool",
    params: &[],
    formula_source: "https://www.investopedia.com/terms/t/typicalprice.asp",
    formula_latex: r#"
\[
TYPPRICE = \frac{High + Low + Close}{3}
\]
"#,
    gold_standard_file: "typprice.json",
    category: "Classic",
};

pub const WCLPRICE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Weighted Close Price (WCLPRICE)",
    description: "An average of the High, Low, and Close prices, with double weight given to the Close price.",
    usage: "Use to emphasize the importance of the closing price while still accounting for the total range of the bar.",
    keywords: &["price-transform", "classic", "weighted"],
    ehlers_summary: "Weighted Close Price gives additional significance to the Close, reflecting the widely held belief that the closing price is the most important data point in a trading session. It provides a more nuanced input for smoothing algorithms. — TA-Lib Documentation",
    params: &[],
    formula_source: "https://www.tradingview.com/support/solutions/43000502590-weighted-close-wclprice/",
    formula_latex: r#"
\[
WCLPRICE = \frac{High + Low + 2 \times Close}{4}
\]
"#,
    gold_standard_file: "wclprice.json",
    category: "Classic",
};

pub const OC2_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Open-Close Average (OC2)",
    description: "A simple average of the Open and Close prices.",
    usage: "Use to reduce noise in technical indicators. Based on John Ehlers' recent research, averaging the open and close can significantly improve signal-to-noise ratios in DSP-based indicators.",
    keywords: &["price-transform", "ehlers", "smoothing", "dsp"],
    ehlers_summary: "In his 2023 paper 'Every Little Bit Helps', John Ehlers demonstrates that using the average of the Open and Close as an input can enhance the performance of various filters and oscillators by providing a cleaner signal with reduced aliasing. — John Ehlers",
    params: &[],
    formula_source: "Every Little Bit Helps (John Ehlers, 2023)",
    formula_latex: r#"
\[
OC2 = \frac{Open + Close}{2}
\]
"#,
    gold_standard_file: "oc2.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_avgprice_parity(
            o in prop::collection::vec(0.1..100.0, 1..100),
            h in prop::collection::vec(0.1..100.0, 1..100),
            l in prop::collection::vec(0.1..100.0, 1..100),
            c in prop::collection::vec(0.1..100.0, 1..100)
        ) {
            let len = o.len().min(h.len()).min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }

            let mut avgprice = AVGPRICE::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| avgprice.next((o[i], h[i], l[i], c[i]))).collect();
            let batch_results = talib_rs::price_transform::avgprice(&o[..len], &h[..len], &l[..len], &c[..len]).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_medprice_parity(
            h in prop::collection::vec(0.1..100.0, 1..100),
            l in prop::collection::vec(0.1..100.0, 1..100)
        ) {
            let len = h.len().min(l.len());
            if len == 0 { return Ok(()); }

            let mut medprice = MEDPRICE::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| medprice.next((h[i], l[i]))).collect();
            let batch_results = talib_rs::price_transform::medprice(&h[..len], &l[..len]).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_oc2_parity(
            o in prop::collection::vec(0.1..100.0, 1..100),
            c in prop::collection::vec(0.1..100.0, 1..100)
        ) {
            let len = o.len().min(c.len());
            if len == 0 { return Ok(()); }

            let mut oc2 = OC2::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| oc2.next((o[i], c[i]))).collect();
            let batch_results: Vec<f64> = (0..len).map(|i| (o[i] + c[i]) / 2.0).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
