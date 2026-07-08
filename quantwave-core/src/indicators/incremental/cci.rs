//! Native streaming CCI — TA-Lib parity (`talib_rs::momentum::cci`).
//!
//! O(1) sliding sum for typical price SMA; O(period) mean deviation per bar (same as talib-rs).

use crate::traits::Next;

#[inline]
fn cci_value(tp: f64, average: f64, circ_buf: &[f64], timeperiod: usize) -> f64 {
    let tp_f = timeperiod as f64;
    let mut mean_dev_sum = 0.0_f64;
    for j in 0..timeperiod {
        mean_dev_sum += (circ_buf[j] - average).abs();
    }
    let mean_dev = mean_dev_sum / tp_f;
    if mean_dev > 0.0 {
        (tp - average) / (0.015 * mean_dev)
    } else {
        0.0
    }
}

/// Commodity Channel Index — input `(high, low, close)`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CCI {
    pub timeperiod: usize,
    circ_buf: Vec<f64>,
    circ_idx: usize,
    running_sum: f64,
    bars_seen: usize,
}

impl CCI {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            circ_buf: vec![0.0; timeperiod.max(1)],
            circ_idx: 0,
            running_sum: 0.0,
            bars_seen: 0,
        }
    }
}

impl Next<(f64, f64, f64)> for CCI {
    type Output = f64;

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let timeperiod = self.timeperiod;
        if timeperiod < 2 {
            return f64::NAN;
        }
        let lookback = timeperiod - 1;
        let tp = (high + low + close) / 3.0;
        self.bars_seen += 1;
        let i = self.bars_seen - 1;

        if i < timeperiod {
            self.circ_buf[i] = tp;
            self.running_sum += tp;
            if i < lookback {
                return f64::NAN;
            }
            let last_value = self.circ_buf[lookback];
            let the_average = self.running_sum / timeperiod as f64;
            return cci_value(
                last_value,
                the_average,
                &self.circ_buf[..timeperiod],
                timeperiod,
            );
        }

        let new_tp = tp;
        self.running_sum += new_tp - self.circ_buf[self.circ_idx];
        self.circ_buf[self.circ_idx] = new_tp;
        let the_average = self.running_sum / timeperiod as f64;
        let out = cci_value(
            new_tp,
            the_average,
            &self.circ_buf[..timeperiod],
            timeperiod,
        );
        self.circ_idx += 1;
        if self.circ_idx >= timeperiod {
            self.circ_idx = 0;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn ordered_hlc(h: &[f64], l: &[f64], c: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let len = h.len().min(l.len()).min(c.len());
        let mut high = Vec::with_capacity(len);
        let mut low = Vec::with_capacity(len);
        let mut close = Vec::with_capacity(len);
        for i in 0..len {
            let vh = h[i];
            let vl = l[i];
            let vc = c[i];
            high.push(vh.max(vl).max(vc));
            low.push(vh.min(vl).min(vc));
            close.push(vc);
        }
        (high, low, close)
    }

    proptest! {
        #[test]
        fn test_cci_parity(
            h in prop::collection::vec(1.0..100.0, 10..100),
            l in prop::collection::vec(1.0..100.0, 10..100),
            c in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let (high, low, close) = ordered_hlc(&h, &l, &c);
            let len = high.len();
            if len == 0 { return Ok(()); }
            let period = 14;
            let mut cci = CCI::new(period);
            let streaming: Vec<f64> =
                (0..len).map(|i| cci.next((high[i], low[i], close[i]))).collect();
            let batch = talib_rs::momentum::cci(&high, &low, &close, period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else if !b.is_nan() {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }
}
