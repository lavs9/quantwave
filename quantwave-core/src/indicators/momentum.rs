use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
talib_1_in_1_out!(RSI, talib_rs::momentum::rsi, timeperiod: usize);
impl From<usize> for RSI {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_3_out!(MACD, talib_rs::momentum::macd, fastperiod: usize, slowperiod: usize, signalperiod: usize);
talib_1_in_3_out!(MACDEXT, talib_rs::momentum::macd_ext, fastperiod: usize, fastmatype: talib_rs::MaType, slowperiod: usize, slowmatype: talib_rs::MaType, signalperiod: usize, signalmatype: talib_rs::MaType);
talib_1_in_3_out!(MACDFIX, talib_rs::momentum::macd_fix, signalperiod: usize);

talib_3_in_2_out!(STOCH, talib_rs::momentum::stoch, fastk_period: usize, slowk_period: usize, slowk_matype: talib_rs::MaType, slowd_period: usize, slowd_matype: talib_rs::MaType);
talib_3_in_2_out!(STOCHF, talib_rs::momentum::stochf, fastk_period: usize, fastd_period: usize, fastd_matype: talib_rs::MaType);
talib_1_in_2_out!(STOCHRSI, talib_rs::momentum::stochrsi, timeperiod: usize, fastk_period: usize, fastd_period: usize, fastd_matype: talib_rs::MaType);

talib_3_in_1_out!(ADX, talib_rs::momentum::adx, timeperiod: usize);
impl From<usize> for ADX {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_3_in_1_out!(ADXR, talib_rs::momentum::adxr, timeperiod: usize);
impl From<usize> for ADXR {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_3_in_1_out!(CCI, talib_rs::momentum::cci, timeperiod: usize);
impl From<usize> for CCI {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(MOM, talib_rs::momentum::mom, timeperiod: usize);
impl From<usize> for MOM {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(ROC, talib_rs::momentum::roc, timeperiod: usize);
impl From<usize> for ROC {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(ROCP, talib_rs::momentum::rocp, timeperiod: usize);
impl From<usize> for ROCP {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(ROCR, talib_rs::momentum::rocr, timeperiod: usize);
impl From<usize> for ROCR {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(ROCR100, talib_rs::momentum::rocr100, timeperiod: usize);
impl From<usize> for ROCR100 {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_3_in_1_out!(WILLR, talib_rs::momentum::willr, timeperiod: usize);
impl From<usize> for WILLR {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_1_in_1_out!(APO, talib_rs::momentum::apo, fastperiod: usize, slowperiod: usize, matype: talib_rs::MaType);
talib_1_in_1_out!(PPO, talib_rs::momentum::ppo, fastperiod: usize, slowperiod: usize, matype: talib_rs::MaType);
talib_4_in_1_out!(BOP, talib_rs::momentum::bop);
impl Default for BOP {
    fn default() -> Self {
        Self::new()
    }
}
talib_1_in_1_out!(CMO, talib_rs::momentum::cmo, timeperiod: usize);
impl From<usize> for CMO {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_2_in_2_out!(AROON, talib_rs::momentum::aroon, timeperiod: usize);
talib_2_in_1_out!(AROONOSC, talib_rs::momentum::aroon_osc, timeperiod: usize);
talib_4_in_1_out!(MFI, talib_rs::momentum::mfi, timeperiod: usize);
talib_1_in_1_out!(TRIX, talib_rs::momentum::trix, timeperiod: usize);
impl From<usize> for TRIX {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_3_in_1_out!(ULTOSC, talib_rs::momentum::ultosc, timeperiod1: usize, timeperiod2: usize, timeperiod3: usize);
talib_3_in_1_out!(DX, talib_rs::momentum::dx, timeperiod: usize);
impl From<usize> for DX {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_3_in_1_out!(PLUS_DI, talib_rs::momentum::plus_di, timeperiod: usize);
impl From<usize> for PLUS_DI {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_3_in_1_out!(MINUS_DI, talib_rs::momentum::minus_di, timeperiod: usize);
impl From<usize> for MINUS_DI {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_2_in_1_out!(PLUS_DM, talib_rs::momentum::plus_dm, timeperiod: usize);
talib_2_in_1_out!(MINUS_DM, talib_rs::momentum::minus_dm, timeperiod: usize);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_rsi_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 14;
            let mut rsi = RSI::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| rsi.next(x)).collect();
            let batch_results = talib_rs::momentum::rsi(&input, period).unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_macd_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let fast = 12;
            let slow = 26;
            let signal = 9;
            let mut macd = MACD::new(fast, slow, signal);
            let streaming_results: Vec<(f64, f64, f64)> = input.iter().map(|&x| macd.next(x)).collect();
            let (b_macd, b_signal, b_hist) = talib_rs::momentum::macd(&input, fast, slow, signal).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });

            for (i, (s_macd, s_signal, s_hist)) in streaming_results.into_iter().enumerate() {
                if s_macd.is_nan() {
                    assert!(b_macd[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_macd, b_macd[i], epsilon = 1e-6);
                }
                if s_signal.is_nan() {
                    assert!(b_signal[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_signal, b_signal[i], epsilon = 1e-6);
                }
                if s_hist.is_nan() {
                    assert!(b_hist[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_hist, b_hist[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_stoch_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            for i in 0..len {
                let val_h: f64 = h[i];
                let val_l: f64 = l[i];
                let val_c: f64 = c[i];
                let max: f64 = val_h.max(val_l).max(val_c);
                let min: f64 = val_h.min(val_l).min(val_c);
                high.push(max);
                low.push(min);
                close.push(val_c);
            }

            let fastk = 5;
            let slowk = 3;
            let slowk_ma = talib_rs::MaType::Sma;
            let slowd = 3;
            let slowd_ma = talib_rs::MaType::Sma;

            let mut stoch = STOCH::new(fastk, slowk, slowk_ma, slowd, slowd_ma);
            let streaming_results: Vec<(f64, f64)> = (0..len).map(|i| stoch.next((high[i], low[i], close[i]))).collect();
            let (b_k, b_d) = talib_rs::momentum::stoch(&high, &low, &close, fastk, slowk, slowk_ma, slowd, slowd_ma).unwrap_or_else(|_| {
                (vec![f64::NAN; len], vec![f64::NAN; len])
            });

            for (i, (s_k, s_d)) in streaming_results.into_iter().enumerate() {
                if s_k.is_nan() {
                    assert!(b_k[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_k, b_k[i], epsilon = 1e-6);
                }
                if s_d.is_nan() {
                    assert!(b_d[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_d, b_d[i], epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_adx_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len());
            if len == 0 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            for i in 0..len {
                let val_h: f64 = h[i];
                let val_l: f64 = l[i];
                let val_c: f64 = c[i];
                let max: f64 = val_h.max(val_l).max(val_c);
                let min: f64 = val_h.min(val_l).min(val_c);
                high.push(max);
                low.push(min);
                close.push(val_c);
            }

            let period = 14;
            let mut adx = ADX::new(period);
            let streaming_results: Vec<f64> = (0..len).map(|i| adx.next((high[i], low[i], close[i]))).collect();
            let batch_results = talib_rs::momentum::adx(&high, &low, &close, period).unwrap_or_else(|_| vec![f64::NAN; len]);

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

pub const RSI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Relative Strength Index (RSI)",
    description: "A momentum oscillator that measures the speed and change of price movements.",
    usage: "Use to identify overbought (>70) and oversold (<30) conditions. RSI divergences against price often signal impending trend reversals.",
    keywords: &["momentum", "oscillator", "overbought", "oversold", "classic"],
    ehlers_summary: "Developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), the RSI compares the magnitude of recent gains to recent losses to determine overbought and oversold conditions of an asset. It remains the most widely used momentum oscillator in modern technical analysis.",
    params: &[ParamDef { name: "timeperiod", default: "14", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/r/rsi.asp",
    formula_latex: r#"
\[
RS = \frac{Average Gain}{Average Loss} \\ RSI = 100 - \frac{100}{1 + RS}
\]
"#,
    gold_standard_file: "rsi.json",
    category: "Classic",
};

pub const MACD_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Moving Average Convergence Divergence (MACD)",
    description: "A trend-following momentum indicator that shows the relationship between two moving averages.",
    usage: "Use to identify trend direction and momentum. Crossovers of the MACD line and signal line provide entry and exit signals, while the histogram shows the strength of the trend.",
    keywords: &["trend", "momentum", "moving-average", "classic"],
    ehlers_summary: "Gerald Appel developed the MACD in the late 1970s. It is calculated by subtracting the 26-period EMA from the 12-period EMA. A nine-day EMA of the MACD, called the 'signal line,' is then plotted on top of the MACD line, which can function as a trigger for buy and sell signals. — Investopedia",
    params: &[
        ParamDef { name: "fastperiod", default: "12", description: "Fast EMA period" },
        ParamDef { name: "slowperiod", default: "26", description: "Slow EMA period" },
        ParamDef { name: "signalperiod", default: "9", description: "Signal EMA period" },
    ],
    formula_source: "https://www.investopedia.com/terms/m/macd.asp",
    formula_latex: r#"
\[
MACD = EMA(12) - EMA(26) \\ Signal = EMA(MACD, 9)
\]
"#,
    gold_standard_file: "macd.json",
    category: "Classic",
};

pub const STOCH_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Stochastic Oscillator",
    description: "A momentum indicator comparing a particular closing price of a security to a range of its prices over a certain period of time.",
    usage: "Use to identify trend reversals by looking for crossovers and overbought/oversold levels. The %K and %D lines indicate when the momentum is shifting relative to the recent price range.",
    keywords: &["momentum", "oscillator", "overbought", "oversold", "classic"],
    ehlers_summary: "George Lane developed the Stochastic Oscillator in the 1950s. It is based on the observation that in an uptrend, prices tend to close near their high, and in a downtrend, they tend to close near their low. The sensitivity of the oscillator to market movements is reducible by adjusting the time period or by taking a moving average of the result. — StockCharts ChartSchool",
    params: &[
        ParamDef { name: "fastk_period", default: "5", description: "Fast %K period" },
        ParamDef { name: "slowk_period", default: "3", description: "Slow %K period" },
        ParamDef { name: "slowd_period", default: "3", description: "Slow %D period" },
    ],
    formula_source: "https://www.investopedia.com/terms/s/stochasticoscillator.asp",
    formula_latex: r#"
\[
\%K = 100 \times \frac{C - L14}{H14 - L14} \\ \%D = 3\text{-period SMA of } \%K
\]
"#,
    gold_standard_file: "stoch.json",
    category: "Classic",
};

pub const ADX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Average Directional Index (ADX)",
    description: "An indicator used to quantify trend strength without regard to trend direction.",
    usage: "Use to determine if the market is trending or ranging. ADX values above 25 indicate a strong trend, while values below 20 indicate a weak or non-trending market.",
    keywords: &["trend", "volatility", "classic", "wilder"],
    ehlers_summary: "Developed by J. Welles Wilder, the ADX is derived from two other indicators, also developed by Wilder: the Positive Directional Indicator (+DI) and the Negative Directional Indicator (-DI). While +DI and -DI indicate trend direction, ADX measures the strength of that trend. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "14", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/a/adx.asp",
    formula_latex: r#"
\[
ADX = 100 \times \frac{\text{EMA}(|(+DI) - (-DI)| / |(+DI) + (-DI)|, n)}{n}
\]
"#,
    gold_standard_file: "adx.json",
    category: "Classic",
};

pub const CCI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Commodity Channel Index (CCI)",
    description: "A versatile indicator that can be used to identify a new trend or warn of extreme conditions.",
    usage: "Use to identify cyclical turns in commodities or stocks. Readings above +100 imply a strong uptrend, while readings below -100 imply a strong downtrend.",
    keywords: &["momentum", "oscillator", "classic", "mean-reversion"],
    ehlers_summary: "Developed by Donald Lambert in 1980, the CCI measures the current price level relative to an average price level over a given period. CCI is relatively high when prices are far above their average and relatively low when prices are far below their average. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "14", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/c/commoditychannelindex.asp",
    formula_latex: r#"
\[
CCI = \frac{Price - SMA}{0.015 \times \text{Mean Deviation}}
\]
"#,
    gold_standard_file: "cci.json",
    category: "Classic",
};

pub const WILLR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Williams %R",
    description: "A momentum indicator that measures overbought and oversold levels, similar to a stochastic oscillator.",
    usage: "Use to identify entry and exit points in the market. Readings from 0 to -20 are considered overbought, while readings from -80 to -100 are considered oversold.",
    keywords: &["momentum", "oscillator", "overbought", "oversold", "classic"],
    ehlers_summary: "Developed by Larry Williams, %R compares the closing price of a stock to the high-low range over a specific period, typically 14 days. It is used to determine when a stock might be overbought or oversold and to identify potential trend reversals. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "14", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/w/williamsr.asp",
    formula_latex: r#"
\[
\%R = \frac{\text{Highest High} - \text{Close}}{\text{Highest High} - \text{Lowest Low}} \times -100
\]
"#,
    gold_standard_file: "willr.json",
    category: "Classic",
};

pub const MFI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Money Flow Index (MFI)",
    description: "A technical oscillator that uses price and volume data for identifying overbought or oversold signals.",
    usage: "Use as a volume-weighted RSI. Divergences between MFI and price can signal potential reversals, especially when the MFI is in extreme territory (>80 or <20).",
    keywords: &["momentum", "volume", "oscillator", "classic"],
    ehlers_summary: "The Money Flow Index (MFI) is a momentum indicator that measures the inflow and outflow of money into an asset over a specific period of time. It is related to the RSI but incorporates volume, whereas the RSI only considers price. — Investopedia",
    params: &[ParamDef { name: "timeperiod", default: "14", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/m/mfi.asp",
    formula_latex: r#"
\[
\text{Money Ratio} = \frac{\text{Positive Money Flow}}{\text{Negative Money Flow}} \\ MFI = 100 - \frac{100}{1 + \text{Money Ratio}}
\]
"#,
    gold_standard_file: "mfi.json",
    category: "Classic",
};

pub const AROON_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Aroon Indicator",
    description: "An indicator system that identifies when a new trend is beginning and the strength of the trend.",
    usage: "Use to identify when a security is trending and when it is in a range-bound period. Aroon Up crossing above Aroon Down signals the start of a new uptrend.",
    keywords: &["trend", "classic", "breakout"],
    ehlers_summary: "Developed by Tushar Chande in 1995, the Aroon indicator focuses on the time between highs and the time between lows over a given period. The idea is that strong uptrends will regularly see new highs, and strong downtrends will regularly see new lows. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "25", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/a/aroon.asp",
    formula_latex: r#"
\[
\text{Aroon Up} = \frac{n - \text{Periods since n-period High}}{n} \times 100
\]
"#,
    gold_standard_file: "aroon.json",
    category: "Classic",
};

pub const ULTOSC_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ultimate Oscillator",
    description: "A momentum oscillator designed to capture momentum across three different timeframes.",
    usage: "Use to avoid the pitfalls of oscillators that are limited to a single timeframe. Buy signals are generated when there is bullish divergence between price and the indicator.",
    keywords: &["momentum", "oscillator", "classic", "multi-timeframe"],
    ehlers_summary: "Developed by Larry Williams in 1976, the Ultimate Oscillator uses weighted averages of three different timeframes to reduce the volatility and false signals common in other oscillators. It remains a staple for identifying divergence across short, medium, and long-term price action. — StockCharts ChartSchool",
    params: &[
        ParamDef { name: "timeperiod1", default: "7", description: "Short period" },
        ParamDef { name: "timeperiod2", default: "14", description: "Medium period" },
        ParamDef { name: "timeperiod3", default: "28", description: "Long period" },
    ],
    formula_source: "https://www.investopedia.com/terms/u/ultimateoscillator.asp",
    formula_latex: r#"
\[
\text{BP} = \text{Close} - \min(\text{Low}, \text{PrevClose}) \\ \text{TR} = \max(\text{High}, \text{PrevClose}) - \min(\text{Low}, \text{PrevClose})
\]
"#,
    gold_standard_file: "ultosc.json",
    category: "Classic",
};

pub const TRIX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "TRIX",
    description: "A momentum oscillator that shows the percent rate of change of a triple exponentially smoothed moving average.",
    usage: "Use to filter out market noise and identify trend reversals. TRIX crossings of the zero line or a signal line can provide trade entries.",
    keywords: &["momentum", "oscillator", "smoothing", "classic"],
    ehlers_summary: "Developed by Jack Hutson in the early 1980s, TRIX is a powerful momentum oscillator that effectively filters out minor price fluctuations. By triple-smoothing an EMA, it emphasizes the underlying trend and provides a clear signal when the trend changes direction. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "15", description: "Smoothing period" }],
    formula_source: "https://www.investopedia.com/terms/t/trix.asp",
    formula_latex: r#"
\[
TRIX = \frac{EMA3_t - EMA3_{t-1}}{EMA3_{t-1}} \times 100
\]
"#,
    gold_standard_file: "trix.json",
    category: "Classic",
};

pub const MOM_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Momentum (MOM)",
    description: "A simple indicator that measures the amount that a security's price has changed over a given span of time.",
    usage: "Use to measure the velocity of price changes. Positive values indicate an uptrend, while negative values indicate a downtrend.",
    keywords: &["momentum", "classic", "trend"],
    ehlers_summary: "Momentum is one of the most basic and powerful concepts in technical analysis. It measures the rate of change of an asset's price, providing a clear indication of trend strength and potential exhaustion before the actual price reversal occurs. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "10", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/m/momentum.asp",
    formula_latex: r#"
\[
MOM = Price_t - Price_{t-n}
\]
"#,
    gold_standard_file: "mom.json",
    category: "Classic",
};

pub const ROC_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Rate of Change (ROC)",
    description: "A momentum-based technical indicator that measures the percentage change in price between the current price and the price n periods ago.",
    usage: "Use to measure the speed at which price is changing. It is often used to identify overbought/oversold conditions and trend reversals.",
    keywords: &["momentum", "classic", "oscillator"],
    ehlers_summary: "The Rate of Change (ROC) indicator is a pure momentum oscillator that measures the percentage change in price from one period to the next. It is highly effective at identifying the velocity of a move and anticipating when that velocity is slowing down. — StockCharts ChartSchool",
    params: &[ParamDef { name: "timeperiod", default: "10", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/r/rateofchange.asp",
    formula_latex: r#"
\[
ROC = \frac{Price_t - Price_{t-n}}{Price_{t-n}} \times 100
\]
"#,
    gold_standard_file: "roc.json",
    category: "Classic",
};

pub const CMO_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Chande Momentum Oscillator (CMO)",
    description: "An advanced momentum oscillator developed by Tushar Chande that measures the difference between up and down days.",
    usage: "Use to identify extreme overbought and oversold conditions. CMO is more sensitive to price action than RSI as it uses unsmoothed data in its internal calculations.",
    keywords: &["momentum", "oscillator", "classic", "overbought", "oversold"],
    ehlers_summary: "Developed by Tushar Chande in 1994, the CMO is similar to the RSI but uses the net sum of up and down moves in both the numerator and denominator. This makes it more sensitive to price movements and useful for identifying short-term overextensions in the market. — The New Technical Trader",
    params: &[ParamDef { name: "timeperiod", default: "14", description: "Lookback period" }],
    formula_source: "https://www.investopedia.com/terms/c/chandemomentumoscillator.asp",
    formula_latex: r#"
\[
CMO = 100 \times \frac{S_u - S_d}{S_u + S_d}
\]
"#,
    gold_standard_file: "cmo.json",
    category: "Classic",
};

pub const APO_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Absolute Price Oscillator (APO)",
    description: "Shows the absolute difference between two moving averages of different periods.",
    usage: "Use to identify trend crossovers and momentum. It is essentially a MACD without the signal line, showing the raw distance between fast and slow averages.",
    keywords: &["trend", "momentum", "moving-average", "classic"],
    ehlers_summary: "The Absolute Price Oscillator (APO) is based on the difference between two exponential moving averages. It is a trend-following indicator that signals a change in direction when the fast EMA crosses the slow EMA, providing a clear visual of trend development. — TA-Lib Documentation",
    params: &[
        ParamDef { name: "fastperiod", default: "12", description: "Fast period" },
        ParamDef { name: "slowperiod", default: "26", description: "Slow period" },
    ],
    formula_source: "https://www.tradingview.com/support/solutions/43000501826-absolute-price-oscillator-apo/",
    formula_latex: r#"
\[
APO = EMA(fast) - EMA(slow)
\]
"#,
    gold_standard_file: "apo.json",
    category: "Classic",
};

pub const PPO_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Percentage Price Oscillator (PPO)",
    description: "A momentum oscillator that measures the difference between two moving averages as a percentage of the larger moving average.",
    usage: "Use to compare trend momentum across different securities with varying price levels. PPO is the percentage version of MACD.",
    keywords: &["trend", "momentum", "moving-average", "classic", "normalization"],
    ehlers_summary: "The Percentage Price Oscillator (PPO) is identical to the MACD, except that it measures the difference between two moving averages as a percentage. This allows for comparison across different stocks regardless of their price, making it a superior tool for relative strength analysis. — StockCharts ChartSchool",
    params: &[
        ParamDef { name: "fastperiod", default: "12", description: "Fast period" },
        ParamDef { name: "slowperiod", default: "26", description: "Slow period" },
    ],
    formula_source: "https://www.investopedia.com/terms/p/ppo.asp",
    formula_latex: r#"
\[
PPO = \frac{EMA(12) - EMA(26)}{EMA(26)} \times 100
\]
"#,
    gold_standard_file: "ppo.json",
    category: "Classic",
};
