use crate::traits::Next;
use crate::indicators::smoothing::EMA;

talib_3_in_1_out!(TaATR, talib_rs::volatility::atr, timeperiod: usize);
talib_3_in_1_out!(TaNATR, talib_rs::volatility::natr, timeperiod: usize);
talib_3_in_1_out!(TaTRANGE, talib_rs::volatility::trange);

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
