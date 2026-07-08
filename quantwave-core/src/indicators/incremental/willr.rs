//! Native streaming Williams %R — TA-Lib parity (`talib_rs::momentum::willr`).

use crate::traits::Next;

/// Williams %R — input `(high, low, close)`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct WILLR {
    pub timeperiod: usize,
    high: Vec<f64>,
    low: Vec<f64>,
    highest: f64,
    highest_idx: usize,
    lowest: f64,
    lowest_idx: usize,
    trailing_idx: usize,
}

impl WILLR {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            high: Vec::new(),
            low: Vec::new(),
            highest: f64::NEG_INFINITY,
            highest_idx: 0,
            lowest: f64::INFINITY,
            lowest_idx: 0,
            trailing_idx: 0,
        }
    }

    #[inline]
    fn willr_from_range(highest: f64, lowest: f64, close: f64) -> f64 {
        let range = highest - lowest;
        if range > 0.0 {
            -100.0 * (highest - close) / range
        } else {
            0.0
        }
    }
}

impl Next<(f64, f64, f64)> for WILLR {
    type Output = f64;

    fn next(&mut self, (h, l, c): (f64, f64, f64)) -> Self::Output {
        let timeperiod = self.timeperiod;
        if timeperiod < 2 {
            return f64::NAN;
        }
        let lookback = timeperiod - 1;

        self.high.push(h);
        self.low.push(l);
        let today = self.high.len() - 1;

        if today < lookback {
            if today == 0 {
                self.highest = h;
                self.highest_idx = 0;
                self.lowest = l;
                self.lowest_idx = 0;
            } else {
                if h >= self.highest {
                    self.highest = h;
                    self.highest_idx = today;
                }
                if l <= self.lowest {
                    self.lowest = l;
                    self.lowest_idx = today;
                }
            }
            return f64::NAN;
        }

        if today == lookback {
            if h >= self.highest {
                self.highest = h;
                self.highest_idx = today;
            }
            if l <= self.lowest {
                self.lowest = l;
                self.lowest_idx = today;
            }
            let out = Self::willr_from_range(self.highest, self.lowest, c);
            self.trailing_idx = 1;
            return out;
        }

        let high = &self.high;
        let low = &self.low;

        if self.highest_idx < self.trailing_idx {
            self.highest_idx = self.trailing_idx;
            self.highest = high[self.trailing_idx];
            for (j, &val) in high[self.trailing_idx + 1..=today].iter().enumerate() {
                if val >= self.highest {
                    self.highest = val;
                    self.highest_idx = self.trailing_idx + 1 + j;
                }
            }
        } else if h >= self.highest {
            self.highest_idx = today;
            self.highest = h;
        }

        if self.lowest_idx < self.trailing_idx {
            self.lowest_idx = self.trailing_idx;
            self.lowest = low[self.trailing_idx];
            for (j, &val) in low[self.trailing_idx + 1..=today].iter().enumerate() {
                if val <= self.lowest {
                    self.lowest = val;
                    self.lowest_idx = self.trailing_idx + 1 + j;
                }
            }
        } else if l <= self.lowest {
            self.lowest_idx = today;
            self.lowest = l;
        }

        let out = Self::willr_from_range(self.highest, self.lowest, c);
        self.trailing_idx += 1;
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
        fn test_willr_parity(
            h in prop::collection::vec(1.0..100.0, 10..100),
            l in prop::collection::vec(1.0..100.0, 10..100),
            c in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let (high, low, close) = ordered_hlc(&h, &l, &c);
            let len = high.len();
            if len == 0 { return Ok(()); }
            let period = 14;
            let mut willr = WILLR::new(period);
            let streaming: Vec<f64> =
                (0..len).map(|i| willr.next((high[i], low[i], close[i]))).collect();
            let batch = talib_rs::momentum::willr(&high, &low, &close, period)
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
