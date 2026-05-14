use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::EMA;
use crate::traits::Next;

talib_3_in_1_out!(TaATR, talib_rs::volatility::atr, timeperiod: usize);
impl From<usize> for TaATR {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_3_in_1_out!(TaNATR, talib_rs::volatility::natr, timeperiod: usize);
impl From<usize> for TaNATR {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
talib_3_in_1_out!(TaTRANGE, talib_rs::volatility::trange);
impl Default for TaTRANGE {
    fn default() -> Self {
        Self::new()
    }
}

/// True Range (TR)
#[derive(Debug, Clone, Default)]
pub struct TrueRange {
    prev_close: Option<f64>,
}

impl Next<(f64, f64, f64)> for TrueRange {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let tr = match self.prev_close {
            Some(pc) => {
                let h_l = high - low;
                let h_pc = (high - pc).abs();
                let l_pc = (low - pc).abs();
                h_l.max(h_pc).max(l_pc)
            }
            None => high - low,
        };
        self.prev_close = Some(close);
        tr
    }
}

/// Average True Range (ATR)
#[derive(Debug, Clone)]
pub struct ATR {
    tr: TrueRange,
    smoothing: EMA,
}

impl ATR {
    pub fn new(period: usize) -> Self {
        Self {
            tr: TrueRange::default(),
            smoothing: EMA::new(period),
        }
    }
}

impl Next<(f64, f64, f64)> for ATR {
    type Output = f64;

    fn next(&mut self, input: (f64, f64, f64)) -> Self::Output {
        let tr = self.tr.next(input);
        self.smoothing.next(tr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ta_atr_parity(
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
                let v_h: f64 = h[i];
                let v_l: f64 = l[i];
                let v_c: f64 = c[i];
                high.push(v_h.max(v_l).max(v_c));
                low.push(v_h.min(v_l).min(v_c));
                close.push(v_c);
            }

            let period = 14;
            let mut ta_atr = TaATR::new(period);
            let streaming_results: Vec<f64> = (0..len).map(|i| ta_atr.next((high[i], low[i], close[i]))).collect();
            let batch_results = talib_rs::volatility::atr(&high, &low, &close, period).unwrap_or_else(|_| vec![f64::NAN; len]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }

        #[test]
        fn test_ta_trange_parity(
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
                let v_h: f64 = h[i];
                let v_l: f64 = l[i];
                let v_c: f64 = c[i];
                high.push(v_h.max(v_l).max(v_c));
                low.push(v_h.min(v_l).min(v_c));
                close.push(v_c);
            }

            let mut ta_tr = TaTRANGE::new();
            let streaming_results: Vec<f64> = (0..len).map(|i| ta_tr.next((high[i], low[i], close[i]))).collect();
            let batch_results = talib_rs::volatility::trange(&high, &low, &close).unwrap_or_else(|_| vec![f64::NAN; len]);

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

pub const TRUE_RANGE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "True Range",
    description: "True Range measures daily volatility.",
    usage: "Use as the foundational volatility module providing ATR, True Range, and related volatility measures used by higher-level indicators such as SuperTrend and Keltner Channels.",
    keywords: &["volatility", "atr", "classic", "range"],
    ehlers_summary: "Average True Range, developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), measures the average of the true range over N bars. True Range accounts for overnight gaps by taking the maximum of: current high minus low, current high minus prior close, prior close minus current low. It remains the industry standard raw volatility measure.",
    params: &[],
    formula_source: "https://www.investopedia.com/terms/a/atr.asp",
    formula_latex: r#"
\[
TR = \max(H - L, |H - C_{t-1}|, |L - C_{t-1}|)
\]
"#,
    gold_standard_file: "true_range.json",
    category: "Classic",
};

pub const ATR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Average True Range",
    description: "ATR represents the average of true ranges over a specified period.",
    usage: "Use as the foundational volatility module providing ATR, True Range, and related volatility measures used by higher-level indicators such as SuperTrend and Keltner Channels.",
    keywords: &["volatility", "atr", "classic", "range"],
    ehlers_summary: "Average True Range, developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), measures the average of the true range over N bars. True Range accounts for overnight gaps by taking the maximum of: current high minus low, current high minus prior close, prior close minus current low. It remains the industry standard raw volatility measure.",
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Smoothing period",
    }],
    formula_source: "https://www.investopedia.com/terms/a/atr.asp",
    formula_latex: r#"
\[
ATR = \frac{ATR_{t-1} \times (n-1) + TR_t}{n}
\]
"#,
    gold_standard_file: "atr.json",
    category: "Classic",
};

pub const NATR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Normalized Average True Range (NATR)",
    description: "A normalized version of ATR that represents volatility as a percentage of price.",
    usage: "Use to compare volatility across different securities with varying price levels. NATR allows for normalized risk assessment and position sizing.",
    keywords: &["volatility", "atr", "normalization", "classic"],
    ehlers_summary: "Normalized ATR (NATR) was developed to allow traders to compare the volatility of high-priced stocks with low-priced stocks. By dividing the ATR by the closing price and multiplying by 100, the result is a percentage that can be used consistently across all assets. — TA-Lib Documentation",
    params: &[ParamDef { name: "timeperiod", default: "14", description: "Smoothing period" }],
    formula_source: "https://www.tradingtechnologies.com/help/x-study/technical-indicator-definitions/normalized-average-true-range-natr/",
    formula_latex: r#"
\[
NATR = \frac{ATR(n)}{Close} \times 100
\]
"#,
    gold_standard_file: "natr.json",
    category: "Classic",
};
