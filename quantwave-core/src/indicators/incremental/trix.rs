//! Native O(1) TRIX — TA-Lib parity (`talib_rs::momentum::trix`).

use crate::indicators::incremental::talib_ema::TalibEma;
use crate::traits::Next;

/// TRIX = ROC of triple EMA.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct TRIX {
    pub timeperiod: usize,
    ema1: TalibEma,
    ema2: TalibEma,
    ema3: TalibEma,
    e3_prev: Option<f64>,
}

impl TRIX {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            ema1: TalibEma::new(timeperiod),
            ema2: TalibEma::new(timeperiod),
            ema3: TalibEma::new(timeperiod),
            e3_prev: None,
        }
    }
}

impl Next<f64> for TRIX {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.timeperiod < 2 {
            return f64::NAN;
        }

        let e1 = self.ema1.next(input);
        if e1.is_nan() {
            return f64::NAN;
        }
        let e2 = self.ema2.next(e1);
        if e2.is_nan() {
            return f64::NAN;
        }
        let e3 = self.ema3.next(e2);
        if e3.is_nan() {
            return f64::NAN;
        }

        let Some(prev) = self.e3_prev else {
            self.e3_prev = Some(e3);
            return f64::NAN;
        };

        let out = if prev != 0.0 {
            ((e3 - prev) / prev) * 100.0
        } else {
            0.0
        };
        self.e3_prev = Some(e3);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_trix_parity(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let period = 14;
            let mut trix = TRIX::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| trix.next(x)).collect();
            let batch = talib_rs::momentum::trix(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
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