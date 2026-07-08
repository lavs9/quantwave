//! Native O(1) streaming momentum indicators (TA-Lib parity).

use crate::indicators::incremental::utils::RingBuffer;
use crate::traits::Next;

macro_rules! impl_lookback_momentum {
    ($name:ident, $compute:ident) => {
        #[derive(Debug, Clone)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            pub timeperiod: usize,
            history: RingBuffer<f64>,
            bar_count: usize,
        }

        impl $name {
            pub fn new(timeperiod: usize) -> Self {
                let cap = timeperiod.saturating_add(1).max(1);
                Self {
                    timeperiod,
                    history: RingBuffer::with_capacity(cap),
                    bar_count: 0,
                }
            }

            #[inline]
            fn lagged(&self) -> Option<f64> {
                let n = self.bar_count;
                if n <= self.timeperiod {
                    return None;
                }
                let idx = n - 1 - self.timeperiod;
                self.history.get(idx).copied()
            }
        }

        impl Next<f64> for $name {
            type Output = f64;

            fn next(&mut self, input: f64) -> Self::Output {
                self.history.push_back(input);
                self.bar_count += 1;

                let Some(prev) = self.lagged() else {
                    return f64::NAN;
                };
                $compute(input, prev)
            }
        }
    };
}

#[inline]
fn mom_compute(cur: f64, prev: f64) -> f64 {
    cur - prev
}

#[inline]
fn roc_compute(cur: f64, prev: f64) -> f64 {
    if prev != 0.0 {
        ((cur - prev) / prev) * 100.0
    } else {
        0.0
    }
}

#[inline]
fn rocp_compute(cur: f64, prev: f64) -> f64 {
    if prev != 0.0 {
        (cur - prev) / prev
    } else {
        0.0
    }
}

#[inline]
fn rocr_compute(cur: f64, prev: f64) -> f64 {
    if prev != 0.0 { cur / prev } else { 0.0 }
}

#[inline]
fn rocr100_compute(cur: f64, prev: f64) -> f64 {
    if prev != 0.0 {
        (cur / prev) * 100.0
    } else {
        0.0
    }
}

impl_lookback_momentum!(MOM, mom_compute);
impl_lookback_momentum!(ROC, roc_compute);
impl_lookback_momentum!(ROCP, rocp_compute);
impl_lookback_momentum!(ROCR, rocr_compute);
impl_lookback_momentum!(ROCR100, rocr100_compute);

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn assert_momentum_parity<I, F>(mut indicator: I, input: &[f64], batch: F)
    where
        I: Next<f64, Output = f64>,
        F: Fn(&[f64], usize) -> Result<Vec<f64>, talib_rs::error::TaError>,
    {
        let timeperiod = 14;
        let streaming: Vec<f64> = input.iter().map(|&x| indicator.next(x)).collect();
        let batch = batch(input, timeperiod).unwrap_or_else(|_| vec![f64::NAN; input.len()]);
        for (s, b) in streaming.iter().zip(batch.iter()) {
            if s.is_nan() {
                assert!(b.is_nan());
            } else if !b.is_nan() {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }

    proptest! {
        #[test]
        fn mom_parity(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let period = 14;
            assert_momentum_parity(MOM::new(period), &input, talib_rs::momentum::mom);
        }

        #[test]
        fn roc_parity(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let period = 14;
            assert_momentum_parity(ROC::new(period), &input, talib_rs::momentum::roc);
        }

        #[test]
        fn rocp_parity(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let period = 14;
            assert_momentum_parity(ROCP::new(period), &input, talib_rs::momentum::rocp);
        }

        #[test]
        fn rocr_parity(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let period = 14;
            assert_momentum_parity(ROCR::new(period), &input, talib_rs::momentum::rocr);
        }

        #[test]
        fn rocr100_parity(input in prop::collection::vec(1.0..100.0, 10..100)) {
            let period = 14;
            assert_momentum_parity(ROCR100::new(period), &input, talib_rs::momentum::rocr100);
        }
    }
}
