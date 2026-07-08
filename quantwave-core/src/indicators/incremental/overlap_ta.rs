//! Native O(1) overlap indicators: TRIMA, KAMA, T3, MIDPOINT, MIDPRICE.

use crate::indicators::incremental::rolling::{MAX, MIN};
use crate::indicators::incremental::talib_ema::TalibEma;
use crate::indicators::incremental::talib_sma::TalibSma;
use crate::traits::Next;

#[inline]
fn trima_periods(period: usize) -> (usize, usize) {
    if period.is_multiple_of(2) {
        (period / 2, period / 2 + 1)
    } else {
        let n = period.div_ceil(2);
        (n, n)
    }
}

/// Triangular Moving Average.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TRIMA {
    pub timeperiod: usize,
    sma1: TalibSma,
    sma2: TalibSma,
}

impl TRIMA {
    pub fn new(timeperiod: usize) -> Self {
        let (n1, n2) = trima_periods(timeperiod);
        Self {
            timeperiod,
            sma1: TalibSma::new(n1),
            sma2: TalibSma::new(n2),
        }
    }
}

impl Next<f64> for TRIMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let mid = self.sma1.next(input);
        if mid.is_nan() {
            f64::NAN
        } else {
            self.sma2.next(mid)
        }
    }
}

/// Midpoint over `timeperiod`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MIDPOINT {
    pub timeperiod: usize,
    max: MAX,
    min: MIN,
}

impl MIDPOINT {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            max: MAX::new(timeperiod),
            min: MIN::new(timeperiod),
        }
    }
}

impl Next<f64> for MIDPOINT {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let hi = self.max.next(input);
        let lo = self.min.next(input);
        if hi.is_nan() || lo.is_nan() {
            f64::NAN
        } else {
            (hi + lo) / 2.0
        }
    }
}

/// Midprice over `timeperiod`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MIDPRICE {
    pub timeperiod: usize,
    max: MAX,
    min: MIN,
}

impl MIDPRICE {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            max: MAX::new(timeperiod),
            min: MIN::new(timeperiod),
        }
    }
}

impl Next<(f64, f64)> for MIDPRICE {
    type Output = f64;

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        let hi = self.max.next(high);
        let lo = self.min.next(low);
        if hi.is_nan() || lo.is_nan() {
            f64::NAN
        } else {
            (hi + lo) / 2.0
        }
    }
}

/// Kaufman Adaptive Moving Average (TA-Lib `overlap::kama`).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct KAMA {
    pub timeperiod: usize,
    fast_sc: f64,
    slow_sc: f64,
    history: Vec<f64>,
    prev_kama: f64,
}

impl KAMA {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            fast_sc: 2.0 / (2.0 + 1.0),
            slow_sc: 2.0 / (30.0 + 1.0),
            history: Vec::new(),
            prev_kama: 0.0,
        }
    }
}

impl Next<f64> for KAMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.history.push(input);
        let p = self.timeperiod;
        let n = self.history.len();

        if n <= p {
            return f64::NAN;
        }
        if n == p + 1 {
            self.prev_kama = self.history[p - 1];
        }

        let today = n - 1;
        let trailing = today - p;
        let mut sum_roc1 = 0.0;
        for i in 1..=p {
            let idx_cur = today - i + 1;
            let idx_prev = today - i;
            sum_roc1 += (self.history[idx_cur] - self.history[idx_prev]).abs();
        }
        let sum_roc2 = (self.history[today] - self.history[trailing]).abs();
        let er = if sum_roc1 != 0.0 {
            sum_roc2 / sum_roc1
        } else {
            0.0
        };
        let sc = (er * (self.fast_sc - self.slow_sc) + self.slow_sc).powi(2);
        self.prev_kama += sc * (input - self.prev_kama);
        self.prev_kama
    }
}

/// Tilson T3 — six cascaded TA-Lib EMAs with Tilson coefficients.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct T3 {
    pub timeperiod: usize,
    pub v_factor: f64,
    e1: TalibEma,
    e2: TalibEma,
    e3: TalibEma,
    e4: TalibEma,
    e5: TalibEma,
    e6: TalibEma,
}

impl T3 {
    pub fn new(timeperiod: usize, v_factor: f64) -> Self {
        Self {
            timeperiod,
            v_factor,
            e1: TalibEma::new(timeperiod),
            e2: TalibEma::new(timeperiod),
            e3: TalibEma::new(timeperiod),
            e4: TalibEma::new(timeperiod),
            e5: TalibEma::new(timeperiod),
            e6: TalibEma::new(timeperiod),
        }
    }
}

impl Next<f64> for T3 {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let v1 = self.e1.next(input);
        let v2 = self.e2.next(v1);
        let v3 = self.e3.next(v2);
        let v4 = self.e4.next(v3);
        let v5 = self.e5.next(v4);
        let v6 = self.e6.next(v5);
        if v6.is_nan() {
            return f64::NAN;
        }
        let v = self.v_factor;
        let c1 = -v.powi(3);
        let c2 = 3.0 * v * v + 3.0 * v.powi(3);
        let c3 = -6.0 * v * v - 3.0 * v - 3.0 * v.powi(3);
        let c4 = 1.0 + 3.0 * v + v.powi(3) + 3.0 * v * v;
        c1 * v6 + c2 * v5 + c3 * v4 + c4 * v3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_trima_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut trima = TRIMA::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| trima.next(x)).collect();
            let batch = talib_rs::overlap::trima(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_midpoint_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut mid = MIDPOINT::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| mid.next(x)).collect();
            let batch = talib_rs::overlap::midpoint(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_kama_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut kama = KAMA::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| kama.next(x)).collect();
            let batch = talib_rs::overlap::kama(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_t3_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let vf = 0.7;
            let mut t3 = T3::new(period, vf);
            let streaming: Vec<f64> = input.iter().map(|&x| t3.next(x)).collect();
            let batch = talib_rs::overlap::t3(&input, period, vf)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
