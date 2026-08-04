//! TA-Lib compatible streaming TEMA — O(1) per bar.
//!
//! `TEMA = 3*EMA1 - 3*EMA2 + EMA3`, with each layer seeded by the SMA of the
//! previous layer's first `period` outputs (exactly what [`TalibEma`] does), so
//! the lookback is `3 * (period - 1)` as in `talib_rs::overlap::tema`.
//!
//! Distinct from [`crate::indicators::tema::TEMA`], which uses the general
//! `EMA` (seeded from the first sample, no NaN warmup) and therefore does not
//! reproduce TA-Lib's values.

use crate::indicators::incremental::talib_ema::TalibEma;
use crate::traits::Next;

/// Triple exponential moving average matching `talib_rs::overlap::tema`.
#[derive(Debug, Clone)]
pub struct TalibTema {
    ema1: TalibEma,
    ema2: TalibEma,
    ema3: TalibEma,
}

impl TalibTema {
    pub fn new(period: usize) -> Self {
        Self {
            ema1: TalibEma::new(period),
            ema2: TalibEma::new(period),
            ema3: TalibEma::new(period),
        }
    }
}

impl Next<f64> for TalibTema {
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
        let e3 = self.ema3.next(e2);
        if e3.is_nan() {
            return f64::NAN;
        }
        3.0 * e1 - 3.0 * e2 + e3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_talib_tema_parity(input in prop::collection::vec(0.1..100.0, 1..120)) {
            let period = 10;
            let mut tema = TalibTema::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| tema.next(x)).collect();
            let batch = talib_rs::overlap::tema(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-9); }
            }
        }
    }
}
