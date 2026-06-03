//! Native TaTRANGE / TaNATR — TA-Lib parity.

use crate::indicators::incremental::ta_atr::TaATR;
use crate::traits::Next;

/// True Range (TRANGE) — first bar NaN, then standard TR.
#[derive(Debug, Clone, Default)]
#[allow(non_camel_case_types)]
pub struct TaTRANGE {
    prev_close: Option<f64>,
    bars_seen: usize,
}

impl TaTRANGE {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Next<(f64, f64, f64)> for TaTRANGE {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        if self.bars_seen == 0 {
            self.prev_close = Some(close);
            self.bars_seen = 1;
            return f64::NAN;
        }
        let pc = self.prev_close.unwrap();
        let hl = high - low;
        let hc = (high - pc).abs();
        let lc = (low - pc).abs();
        self.prev_close = Some(close);
        self.bars_seen += 1;
        hl.max(hc).max(lc)
    }
}

/// Normalized ATR (NATR) — 100 * ATR / Close.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaNATR {
    pub timeperiod: usize,
    atr: TaATR,
}

impl TaNATR {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            atr: TaATR::new(timeperiod),
        }
    }
}

impl Next<(f64, f64, f64)> for TaNATR {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let atr = self.atr.next((high, low, close));
        if atr.is_nan() {
            return f64::NAN;
        }
        if close == 0.0 {
            0.0
        } else {
            (atr / close) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_ta_trange_parity(
            highs in prop::collection::vec(1.0..100.0, 1..100),
            lows in prop::collection::vec(1.0..100.0, 1..100),
            closes in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = highs.len().min(lows.len()).min(closes.len());
            if len < 3 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            for i in 0..len {
                let hi: f64 = highs[i];
                let lo: f64 = lows[i];
                let cl: f64 = closes[i];
                high.push(hi.max(lo).max(cl));
                low.push(hi.min(lo).min(cl));
                close.push(cl);
            }
            let mut tr = TaTRANGE::new();
            let streaming: Vec<f64> = (0..len).map(|i| tr.next((high[i], low[i], close[i]))).collect();
            let batch = talib_rs::volatility::trange(&high, &low, &close)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_ta_natr_parity(
            highs in prop::collection::vec(1.0..100.0, 1..100),
            lows in prop::collection::vec(1.0..100.0, 1..100),
            closes in prop::collection::vec(1.0..100.0, 1..100)
        ) {
            let len = highs.len().min(lows.len()).min(closes.len());
            if len < 20 { return Ok(()); }
            let mut high = Vec::with_capacity(len);
            let mut low = Vec::with_capacity(len);
            let mut close = Vec::with_capacity(len);
            for i in 0..len {
                let hi: f64 = highs[i];
                let lo: f64 = lows[i];
                let cl: f64 = closes[i];
                high.push(hi.max(lo).max(cl));
                low.push(hi.min(lo).min(cl));
                close.push(cl);
            }
            let period = 14;
            let mut natr = TaNATR::new(period);
            let streaming: Vec<f64> = (0..len).map(|i| natr.next((high[i], low[i], close[i]))).collect();
            let batch = talib_rs::volatility::natr(&high, &low, &close, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}