//! Native O(1) ATR — TA-Lib Wilder smoothing on true range.

use crate::traits::Next;

/// Average True Range — matches `talib_rs::volatility::atr`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TaATR {
    pub timeperiod: usize,
    period_f: f64,
    prev_close: Option<f64>,
    bars_seen: usize,
    warmup_tr_count: usize,
    warmup_sum: f64,
    atr: f64,
}

impl TaATR {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            period_f: timeperiod as f64,
            prev_close: None,
            bars_seen: 0,
            warmup_tr_count: 0,
            warmup_sum: 0.0,
            atr: 0.0,
        }
    }

    #[inline]
    fn true_range(&self, high: f64, low: f64, prev_close: f64) -> f64 {
        let hl = high - low;
        let hc = (high - prev_close).abs();
        let lc = (low - prev_close).abs();
        hl.max(hc).max(lc)
    }
}

impl Next<(f64, f64, f64)> for TaATR {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let period = self.timeperiod;
        if period < 1 {
            return f64::NAN;
        }

        if self.bars_seen == 0 {
            self.prev_close = Some(close);
            self.bars_seen = 1;
            return f64::NAN;
        }

        let pc = self.prev_close.unwrap();
        let tr = self.true_range(high, low, pc);
        self.prev_close = Some(close);
        self.bars_seen += 1;

        if self.warmup_tr_count < period {
            self.warmup_tr_count += 1;
            self.warmup_sum += tr;
            if self.warmup_tr_count < period {
                return f64::NAN;
            }
            self.atr = self.warmup_sum / self.period_f;
            return self.atr;
        }

        self.atr = (self.atr * (self.period_f - 1.0) + tr) / self.period_f;
        self.atr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
            let streaming_results: Vec<f64> =
                (0..len).map(|i| ta_atr.next((high[i], low[i], close[i]))).collect();
            let batch_results = talib_rs::volatility::atr(&high, &low, &close, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);

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