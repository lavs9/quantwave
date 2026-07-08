//! Simple O(1) indicators: BOP, OBV, MFI.

use crate::traits::Next;
use crate::utils::RingBuffer;

/// Balance Of Power.
#[derive(Debug, Clone, Default)]
#[allow(non_camel_case_types)]
pub struct BOP;

impl BOP {
    pub fn new() -> Self {
        Self
    }
}

impl Next<(f64, f64, f64, f64)> for BOP {
    type Output = f64;

    fn next(&mut self, (open, high, low, close): (f64, f64, f64, f64)) -> Self::Output {
        let hl = high - low;
        if hl > 0.0 { (close - open) / hl } else { 0.0 }
    }
}

/// On Balance Volume.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct OBV {
    prev_close: Option<f64>,
    acc: f64,
    started: bool,
}

impl OBV {
    pub fn new() -> Self {
        Self {
            prev_close: None,
            acc: 0.0,
            started: false,
        }
    }
}

impl Next<(f64, f64)> for OBV {
    type Output = f64;

    fn next(&mut self, (close, volume): (f64, f64)) -> Self::Output {
        if !self.started {
            self.acc = volume;
            self.prev_close = Some(close);
            self.started = true;
            return self.acc;
        }
        let pc = self.prev_close.unwrap_or(close);
        if close > pc {
            self.acc += volume;
        } else if close < pc {
            self.acc -= volume;
        }
        self.prev_close = Some(close);
        self.acc
    }
}

/// Money Flow Index.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MFI {
    pub timeperiod: usize,
    prev_tp: Option<f64>,
    flow_window: RingBuffer<(f64, f64)>,
    pos_sum: f64,
    neg_sum: f64,
    comparisons: usize,
}

impl MFI {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            prev_tp: None,
            flow_window: RingBuffer::with_capacity(timeperiod),
            pos_sum: 0.0,
            neg_sum: 0.0,
            comparisons: 0,
        }
    }

    #[inline]
    fn mfi_from(pos: f64, neg: f64) -> f64 {
        if neg > 0.0 {
            100.0 - (100.0 / (1.0 + pos / neg))
        } else {
            100.0
        }
    }
}

impl Next<(f64, f64, f64, f64)> for MFI {
    type Output = f64;

    fn next(&mut self, (high, low, close, volume): (f64, f64, f64, f64)) -> Self::Output {
        let period = self.timeperiod;
        if period < 2 {
            return f64::NAN;
        }
        let tp = (high + low + close) / 3.0;
        let mf = tp * volume;

        let Some(prev_tp) = self.prev_tp else {
            self.prev_tp = Some(tp);
            return f64::NAN;
        };

        let (pos_add, neg_add) = if tp > prev_tp {
            (mf, 0.0)
        } else if tp < prev_tp {
            (0.0, mf)
        } else {
            (0.0, 0.0)
        };
        self.prev_tp = Some(tp);
        self.comparisons += 1;

        if self.flow_window.len() >= period
            && let Some((op, on)) = self.flow_window.pop_front()
        {
            self.pos_sum -= op;
            self.neg_sum -= on;
        }
        self.flow_window.push_back((pos_add, neg_add));
        self.pos_sum += pos_add;
        self.neg_sum += neg_add;

        if self.comparisons < period {
            return f64::NAN;
        }
        Self::mfi_from(self.pos_sum, self.neg_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_mfi_parity(
            h in prop::collection::vec(1.0..100.0, 1..100),
            l in prop::collection::vec(1.0..100.0, 1..100),
            c in prop::collection::vec(1.0..100.0, 1..100),
            v in prop::collection::vec(1.0..1000.0, 1..100)
        ) {
            let len = h.len().min(l.len()).min(c.len()).min(v.len());
            if len < 20 { return Ok(()); }
            let period = 14;
            let mut mfi = MFI::new(period);
            let streaming: Vec<f64> = (0..len)
                .map(|i| mfi.next((h[i], l[i], c[i], v[i])))
                .collect();
            let batch = talib_rs::momentum::mfi(&h[..len], &l[..len], &c[..len], &v[..len], period)
                .unwrap_or_else(|_| vec![f64::NAN; len]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
