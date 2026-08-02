use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::EMA;
use crate::traits::Next;
use serde::{Deserialize, Serialize};

pub use crate::indicators::incremental::ta_atr::TaATR;
impl From<usize> for TaATR {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}
pub use crate::indicators::incremental::trange::{TaNATR, TaTRANGE};
impl From<usize> for TaNATR {
    fn from(p: usize) -> Self {
        Self::new(p)
    }
}

/// True Range (TR)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// Average True Range (ATR) — **EMA-smoothed variant, not Wilder's RMA**.
///
/// This smooths true range with a standard EMA (`alpha = 2/(period+1)`), seeded
/// from the first bar's `high - low` and emitting a value from bar 0 with no NaN
/// warmup. It is **not** the ATR that Wilder defined in 1978 and that TA-Lib,
/// TradingView Pine `ta.atr`, and `pandas.ewm(alpha=1/period, adjust=False)`
/// compute — for that, use [`TaATR`], which is proptest-verified against
/// `talib_rs::volatility::atr`.
///
/// No authoritative source has been recorded for this EMA-smoothed variant; the
/// divergence and its blast radius (SuperTrend, Keltner, ATR trailing stop, TTM
/// Squeeze, VPN, S/R monitor, volatility-clustering regime) are tracked in
/// `quantwave-xnaf` and recorded as data in
/// [`crate::indicators::conventions::CONVENTION_NOTES`].
///
/// Note that the Polars plugin `pl.col(close).ta.atr(high, low)` and
/// `quantwave.talib.ATR` are backed by [`TaATR`] and *are* Wilder; only this
/// struct and the `atr(period, high, low, close)` PyO3 batch function are not.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    description: "ATR represents the average of true ranges over a specified period. CONVENTION: the `Atr` streaming class and the `atr(period, high, low, close)` batch function smooth true range with an EMA (alpha = 2/(period+1)), NOT Wilder's RMA. For TA-Lib/TradingView-identical ATR use `ta_atr`. See the Average True Range indicator guide for the full surface-by-surface breakdown.",
    usage: "Use as the foundational volatility module providing ATR, True Range, and related volatility measures used by higher-level indicators such as SuperTrend and Keltner Channels. Check `quantwave.conventions(\"atr\")` before reconciling values against TA-Lib, TradingView, or pandas.",
    keywords: &[
        "volatility",
        "atr",
        "classic",
        "range",
        "convention-divergence",
    ],
    ehlers_summary: "Average True Range, developed by J. Welles Wilder in New Concepts in Technical Trading Systems (1978), measures the average of the true range over N bars. True Range accounts for overnight gaps by taking the maximum of: current high minus low, current high minus prior close, prior close minus current low. It remains the industry standard raw volatility measure.",
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Smoothing period",
    }],
    // NOTE (quantwave-xnaf): this URL documents Wilder's RMA, which is what `ta_atr`
    // implements. No source has been established for the EMA-smoothed variant that
    // `Atr` / `atr()` actually compute; per AGENTS.md it is recorded as unsourced in
    // `conventions::CONVENTION_NOTES` rather than having a source assumed for it.
    formula_source: "https://www.investopedia.com/terms/a/atr.asp (documents Wilder's RMA — implemented by `ta_atr`; no source recorded for the EMA-smoothed `Atr`)",
    formula_latex: r#"
\[
TR_t = \max(H_t - L_t, |H_t - C_{t-1}|, |L_t - C_{t-1}|)
\]
\[
\text{Atr / atr() (EMA variant): } ATR_t = \alpha TR_t + (1-\alpha) ATR_{t-1}, \quad \alpha = \frac{2}{n+1}
\]
\[
\text{ta\_atr (Wilder RMA, TA-Lib): } ATR_t = \frac{ATR_{t-1} \times (n-1) + TR_t}{n}
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
    params: &[ParamDef {
        name: "timeperiod",
        default: "14",
        description: "Smoothing period",
    }],
    formula_source: "https://www.tradingtechnologies.com/help/x-study/technical-indicator-definitions/normalized-average-true-range-natr/",
    formula_latex: r#"
\[
NATR = \frac{ATR(n)}{Close} \times 100
\]
"#,
    gold_standard_file: "natr.json",
    category: "Classic",
};
