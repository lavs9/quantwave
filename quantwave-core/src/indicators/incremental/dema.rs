//! Native DEMA — TA-Lib parity via double TalibEma.

use crate::indicators::incremental::talib_ema::TalibEma;
use crate::traits::Next;

/// Double Exponential Moving Average.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DEMA {
    pub timeperiod: usize,
    ema1: TalibEma,
    ema2: TalibEma,
}

impl DEMA {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            ema1: TalibEma::new(timeperiod),
            ema2: TalibEma::new(timeperiod),
        }
    }
}

impl Next<f64> for DEMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let e1 = self.ema1.next(input);
        if e1.is_nan() {
            return f64::NAN;
        }
        let e2 = self.ema2.next(e1);
        if e2.is_nan() {
            return f64::NAN;
        }
        2.0 * e1 - e2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_dema_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut dema = DEMA::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| dema.next(x)).collect();
            let batch = talib_rs::overlap::dema(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }
}
