use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
#[allow(unused_imports)]
use crate::traits::Next;

talib_4_in_1_out!(AD, talib_rs::volume::ad);
impl Default for AD {
    fn default() -> Self {
        Self::new()
    }
}
talib_4_in_1_out!(ADOSC, talib_rs::volume::adosc, fastperiod: usize, slowperiod: usize);
talib_2_in_1_out!(OBV, talib_rs::volume::obv);
impl Default for OBV {
    fn default() -> Self {
        Self::new()
    }
}

pub const AD_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Accumulation/Distribution Line (AD)",
    description: "A volume-based indicator designed to measure the cumulative flow of money into and out of a security.",
    usage: "Use to confirm price trends or identify potential reversals through divergences. Rising AD confirms an uptrend; falling AD confirms a downtrend.",
    keywords: &["volume", "momentum", "classic", "accumulation", "distribution"],
    ehlers_summary: "Developed by Marc Chaikin, the AD line uses the relationship between price and volume to determine whether a security is being accumulated or distributed. It is calculated by multiplying the Money Flow Multiplier by the period's volume and adding it to a cumulative total. — StockCharts ChartSchool",
    params: &[],
    formula_source: "https://www.investopedia.com/terms/a/accumulationdistributioncurve.asp",
    formula_latex: r#"
\[
\text{MFM} = \frac{(Close - Low) - (High - Close)}{High - Low} \\ \text{MFV} = \text{MFM} \times Volume \\ AD_t = AD_{t-1} + \text{MFV}
\]
"#,
    gold_standard_file: "ad.json",
    category: "Classic",
};

pub const ADOSC_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Chaikin Oscillator (ADOSC)",
    description: "An indicator that measures the momentum of the Accumulation/Distribution Line using the difference between two exponential moving averages.",
    usage: "Use to anticipate changes in the AD Line. Positive values indicate increasing buying pressure, while negative values indicate increasing selling pressure.",
    keywords: &["volume", "oscillator", "momentum", "classic"],
    ehlers_summary: "Marc Chaikin developed this oscillator to identify momentum shifts in the AD Line. By applying EMAs of different lengths to the AD Line, it highlights changes in money flow before they become apparent in the cumulative total, providing an early warning system for trend exhaustion. — StockCharts ChartSchool",
    params: &[
        ParamDef { name: "fastperiod", default: "3", description: "Fast EMA period" },
        ParamDef { name: "slowperiod", default: "10", description: "Slow EMA period" },
    ],
    formula_source: "https://www.investopedia.com/terms/c/chaikinoscillator.asp",
    formula_latex: r#"
\[
ADOSC = EMA(AD, 3) - EMA(AD, 10)
\]
"#,
    gold_standard_file: "adosc.json",
    category: "Classic",
};

pub const OBV_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "On-Balance Volume (OBV)",
    description: "A momentum indicator that uses volume flow to predict changes in stock price.",
    usage: "Use to identify accumulation by institutions. When price is flat but OBV is rising, a breakout to the upside is likely. Conversely, when price is flat but OBV is falling, a breakdown is likely.",
    keywords: &["volume", "momentum", "classic", "accumulation", "distribution"],
    ehlers_summary: "Introduced by Joe Granville in his 1963 book 'Granville's New Key to Stock Market Profits', OBV is one of the oldest and most respected volume indicators. It operates on the principle that volume precedes price, and that institutional money flow leaves a detectable trail in the volume data before the price move occurs. — StockCharts ChartSchool",
    params: &[],
    formula_source: "https://www.investopedia.com/terms/o/onbalancevolume.asp",
    formula_latex: r#"
\[
OBV_t = OBV_{t-1} + \begin{cases} Volume & \text{if } Close_t > Close_{t-1} \\ 0 & \text{if } Close_t = Close_{t-1} \\ -Volume & \text{if } Close_t < Close_{t-1} \end{cases}
\]
"#,
    gold_standard_file: "obv.json",
    category: "Classic",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ad_parity(
            h in prop::collection::vec(10.0..100.0, 1..100),
            l in prop::collection::vec(10.0..100.0, 1..100),
            c in prop::collection::vec(10.0..100.0, 1..100),
            v in prop::collection::vec(1.0..1000.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len()).min(v.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            let mut volume = Vec::with_capacity(len);
            for i in 0..len {
                let v_h: f64 = h[i];
                let v_l: f64 = l[i];
                let v_c: f64 = c[i];
                let v_v: f64 = v[i];
                high.push(v_h.max(v_l).max(v_c));
                low.push(v_h.min(v_l).min(v_c));
                close.push(v_c);
                volume.push(v_v);
            }

            let mut ad = AD::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| ad.next((high[i], low[i], close[i], volume[i]))).collect();
            let batch_results = talib_rs::volume::ad(&high, &low, &close, &volume).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_obv_parity(
            c in prop::collection::vec(10.0..100.0, 1..100),
            v in prop::collection::vec(1.0..1000.0, 1..100)
        ) {
            let len = c.len().min(v.len());
            if len == 0 { return Ok(()); }
            let close = c[..len].to_vec();
            let volume = v[..len].to_vec();

            let mut obv = OBV::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| obv.next((close[i], volume[i]))).collect();
            let batch_results = talib_rs::volume::obv(&close, &volume).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }
}
