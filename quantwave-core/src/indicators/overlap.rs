use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
talib_1_in_1_out!(DEMA, talib_rs::overlap::dema, timeperiod: usize);
impl From<usize> for DEMA {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(TRIMA, talib_rs::overlap::trima, timeperiod: usize);
impl From<usize> for TRIMA {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(KAMA, talib_rs::overlap::kama, timeperiod: usize);
impl From<usize> for KAMA {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(T3, talib_rs::overlap::t3, timeperiod: usize, v_factor: f64);
talib_1_in_2_out!(MAMA, talib_rs::overlap::mama, fastlimit: f64, slowlimit: f64);
talib_1_in_3_out!(BBANDS, talib_rs::overlap::bbands, timeperiod: usize, nbdevup: f64, nbdevdn: f64, matype: talib_rs::MaType);
talib_2_in_1_out!(SAR, talib_rs::overlap::sar, acceleration: f64, maximum: f64);
talib_2_in_1_out!(SAREXT, talib_rs::overlap::sar_ext, startvalue: f64, offsetonreverse: f64, accelerationinitlong: f64, accelerationlong: f64, accelerationmaxlong: f64, accelerationinitshort: f64, accelerationshort: f64, accelerationmaxshort: f64);
talib_1_in_1_out!(MIDPOINT, talib_rs::overlap::midpoint, timeperiod: usize);
impl From<usize> for MIDPOINT {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_2_in_1_out!(MIDPRICE, talib_rs::overlap::midprice, timeperiod: usize);
impl From<usize> for MIDPRICE {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_2_in_1_out!(MAVP, talib_rs::overlap::mavp, minperiod: usize, maxperiod: usize, matype: talib_rs::MaType);
talib_1_in_1_out!(HT_TRENDLINE, talib_rs::overlap::ht_trendline);
impl Default for HT_TRENDLINE {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_dema_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut dema = DEMA::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| dema.next(x)).collect();
            let batch_results = talib_rs::overlap::dema(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_trima_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut trima = TRIMA::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| trima.next(x)).collect();
            let batch_results = talib_rs::overlap::trima(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_kama_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut kama = KAMA::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| kama.next(x)).collect();
            let batch_results = talib_rs::overlap::kama(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_t3_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let v_factor = 0.7;
            let mut t3 = T3::new(period, v_factor);
            let streaming_results: Vec<f64> = input.iter().map(|&x| t3.next(x)).collect();
            let batch_results = talib_rs::overlap::t3(&input, period, v_factor).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_bbands_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let nbdevup = 2.0;
            let nbdevdn = 2.0;
            let matype = talib_rs::MaType::Sma;
            let mut bbands = BBANDS::new(period, nbdevup, nbdevdn, matype);
            let streaming_results: Vec<(f64, f64, f64)> = input.iter().map(|&x| bbands.next(x)).collect();
            let (b_upper, b_middle, b_lower) = talib_rs::overlap::bbands(&input, period, nbdevup, nbdevdn, matype).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });

            for (i, (s_upper, s_middle, s_lower)) in streaming_results.into_iter().enumerate() {
                if s_upper.is_nan() {
                    assert!(b_upper[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_upper, b_upper[i], epsilon = 1e-6);
                }
                if s_middle.is_nan() {
                    assert!(b_middle[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_middle, b_middle[i], epsilon = 1e-6);
                }
                if s_lower.is_nan() {
                    assert!(b_lower[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_lower, b_lower[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_sar_parity(
            h in prop::collection::vec(10.0..100.0, 1..100),
            l in prop::collection::vec(10.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            for i in 0..len {
                let v_h: f64 = h[i];
                let v_l: f64 = l[i];
                high.push(v_h.max(v_l));
                low.push(v_h.min(v_l));
            }

            let accel = 0.02;
            let max = 0.2;
            let mut sar = SAR::new(accel, max);
            let streaming_results: Vec<f64> = (0..len).map(|i| sar.next((high[i], low[i]))).collect();
            let batch_results = talib_rs::overlap::sar(&high, &low, accel, max).unwrap_or_else(|_| vec![f64::NAN; len]);

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

pub const DEMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Double Exponential Moving Average (DEMA)",
    description: "A fast-acting moving average that reduces lag by using two exponential moving averages.",
    usage: "Use as a replacement for EMA when faster signal generation is required without excessive noise. DEMA reacts more quickly to price changes than a standard EMA.",
    keywords: &["moving-average", "smoothing", "lag-reduction", "classic"],
    ehlers_summary: "Developed by Patrick Mulloy in 1994, DEMA provides a less-laggy alternative to traditional moving averages. It is calculated by taking a single EMA and then subtracting it from a double EMA of the same period. This effectively cancels out some of the lag inherent in the EMA calculation. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "30", description: "Smoothing period" }],
    formula_source: "https://www.investopedia.com/terms/d/double-exponential-moving-average.asp",
    formula_latex: r#"
\[
DEMA = 2 \times EMA - EMA(EMA)
\]
"#,
    gold_standard_file: "dema.json",
    category: "Classic",
};

pub const TRIMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Triangular Moving Average (TRIMA)",
    description: "A double-smoothed simple moving average that gives more weight to the middle of the lookback period.",
    usage: "Use for extremely smooth trend identification. TRIMA is significantly smoother than a standard SMA but introduces more lag; it is ideal for identifying long-term cycles.",
    keywords: &["moving-average", "smoothing", "classic"],
    ehlers_summary: "The Triangular Moving Average is an SMA of an SMA. For a period N, it averages the values over N/2 periods twice. This results in a weight distribution that is triangular, peaking at the center of the window, making it very effective at filtering out high-frequency noise. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "30", description: "Smoothing period" }],
    formula_source: "https://www.tradingview.com/support/solutions/43000591273-triangular-moving-average-tma/",
    formula_latex: r#"
\[
TRIMA = SMA(SMA(Price, n/2), n/2)
\]
"#,
    gold_standard_file: "trima.json",
    category: "Classic",
};

pub const T3_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Tilson T3 Moving Average",
    description: "A smooth, low-lag moving average that uses multiple exponential smoothing.",
    usage: "Use for trend following in noisy markets. T3 offers a superior balance between lag reduction and smoothness compared to DEMA or TEMA.",
    keywords: &["moving-average", "smoothing", "lag-reduction", "classic"],
    ehlers_summary: "Developed by Tim Tilson in 1998, the T3 moving average uses a 'v-factor' (volume factor) to control how much the indicator reacts to price changes. It is essentially a sextuple EMA smoothing process that provides a very smooth curve with remarkably little lag. — Technical Analysis of Stocks & Commodities",
    params: &[
        ParamDef { name: "timeperiod", default: "5", description: "Smoothing period" },
        ParamDef { name: "v_factor", default: "0.7", description: "Volume factor (0.0 to 1.0)" },
    ],
    formula_source: "https://www.tradingview.com/script/667W2a8n-T3-Moving-Average/",
    formula_latex: r#"
\[
e1 = EMA(Price, n) \\ e2 = EMA(e1, n) \\ \dots \\ e6 = EMA(e5, n) \\ T3 = c1 \times e6 + c2 \times e5 + c3 \times e4 + c4 \times e3
\]
"#,
    gold_standard_file: "t3.json",
    category: "Classic",
};

pub const BBANDS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Bollinger Bands",
    description: "A volatility indicator consisting of a middle SMA and two outer bands based on standard deviation.",
    usage: "Use to identify overbought/oversold levels and volatility breakouts. Prices near the upper band suggest overbought conditions, while prices near the lower band suggest oversold conditions. Narrowing bands (The Squeeze) often precede large price moves.",
    keywords: &["volatility", "trend", "classic", "bands"],
    ehlers_summary: "Developed by John Bollinger in the 1980s, Bollinger Bands adapt to volatility by using standard deviation. The middle band is typically a 20-period SMA, and the outer bands are set 2 standard deviations away. This ensures that 95% of price action typically stays within the bands, making escapes highly significant. — BollingerOnBollingerBands.com",
    params: &[
        ParamDef {
            name: "timeperiod",
            default: "20",
            description: "SMA period",
        },
        ParamDef {
            name: "nbdevup",
            default: "2.0",
            description: "Upper deviation multiplier",
        },
        ParamDef {
            name: "nbdevdn",
            default: "2.0",
            description: "Lower deviation multiplier",
        },
    ],
    formula_source: "https://www.investopedia.com/terms/b/bollingerbands.asp",
    formula_latex: r#"
\[
Middle = SMA(n) \\ Upper = Middle + (k \times \sigma) \\ Lower = Middle - (k \times \sigma)
\]
"#,
    gold_standard_file: "bbands.json",
    category: "Classic",
};

pub const SAR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Parabolic SAR",
    description: "A trend-following indicator used to determine price direction and potential reversals.",
    usage: "Use for setting trailing stop losses and identifying trend reversals. Dots below price indicate an uptrend, while dots above price indicate a downtrend.",
    keywords: &["trend", "classic", "stop-loss", "wilder"],
    ehlers_summary: "Developed by J. Welles Wilder, the Parabolic Stop and Reverse (SAR) uses an acceleration factor that increases as the trend persists. This 'parabolic' nature allows the indicator to stay close to price action and provide timely exit signals when a trend exhausts. — StockCharts ChartSchool",
    params: &[
        ParamDef {
            name: "acceleration",
            default: "0.02",
            description: "Acceleration factor",
        },
        ParamDef {
            name: "maximum",
            default: "0.2",
            description: "Maximum acceleration",
        },
    ],
    formula_source: "https://www.investopedia.com/terms/p/parabolicindicator.asp",
    formula_latex: r#"
\[
SAR_{t+1} = SAR_t + AF \times (EP - SAR_t)
\]
"#,
    gold_standard_file: "sar.json",
    category: "Classic",
};

pub const MAMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "MESA Adaptive Moving Average (MAMA)",
    description: "A moving average that adapts to price movement based on the rate of change of phase.",
    usage: "Use as a highly responsive moving average that virtually eliminates overshoot while providing rapid response to price changes. The companion 'FAMA' (Following Adaptive Moving Average) provides a secondary line for crossover signals.",
    keywords: &["moving-average", "adaptive", "ehlers", "dsp", "phase"],
    ehlers_summary: "MAMA adapts to the price movement based on the Hilbert Transform phase rate of change. It provides a unique combination of fast response to price changes while remaining smooth during congested market periods. It is one of the most sophisticated adaptive moving averages available. — Rocket Science for Traders",
    params: &[
        ParamDef {
            name: "fastlimit",
            default: "0.5",
            description: "Fast limit",
        },
        ParamDef {
            name: "slowlimit",
            default: "0.05",
            description: "Slow limit",
        },
    ],
    formula_source: "http://www.mesasoftware.com/Papers/MAMA.pdf",
    formula_latex: r#"
\[
\alpha = \frac{\text{FastLimit}}{\text{PhaseRate}}
\]
"#,
    gold_standard_file: "mama.json",
    category: "Ehlers DSP",
};
