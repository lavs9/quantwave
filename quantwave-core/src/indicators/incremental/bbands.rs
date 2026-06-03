//! Native O(1) Bollinger Bands (SMA middle) — TA-Lib parity.

use crate::indicators::incremental::utils::RingBuffer;
use crate::traits::Next;
use talib_rs::MaType;

const NAN_TRIPLE: (f64, f64, f64) = (f64::NAN, f64::NAN, f64::NAN);

/// Bollinger Bands — matches `talib_rs::overlap::bbands` (SMA path O(1); other `matype` via batch on history).
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct BBANDS {
    pub timeperiod: usize,
    pub nbdevup: f64,
    pub nbdevdn: f64,
    pub matype: MaType,
    window: RingBuffer<f64>,
    sum: f64,
    sum_sq: f64,
    history: Vec<f64>,
}

impl BBANDS {
    pub fn new(timeperiod: usize, nbdevup: f64, nbdevdn: f64, matype: MaType) -> Self {
        Self {
            timeperiod,
            nbdevup,
            nbdevdn,
            matype,
            window: RingBuffer::with_capacity(timeperiod.max(1)),
            sum: 0.0,
            sum_sq: 0.0,
            history: Vec::new(),
        }
    }

    #[inline]
    fn bands_from_sums(&self) -> (f64, f64, f64) {
        let n = self.timeperiod as f64;
        let inv_n = 1.0 / n;
        let ma_val = self.sum * inv_n;
        let variance = self.sum_sq * inv_n - ma_val * ma_val;
        let stddev = variance.max(0.0).sqrt();
        let upper = ma_val + self.nbdevup * stddev;
        let lower = ma_val - self.nbdevdn * stddev;
        (upper, ma_val, lower)
    }

    fn next_sma(&mut self, input: f64) -> (f64, f64, f64) {
        let tp = self.timeperiod;
        if tp == 0 {
            return NAN_TRIPLE;
        }

        if self.window.len() == tp {
            if let Some(old) = self.window.pop_front() {
                self.sum -= old;
                self.sum_sq -= old * old;
            }
        }

        self.window.push_back(input);
        self.sum += input;
        self.sum_sq += input * input;

        if self.window.len() < tp {
            return NAN_TRIPLE;
        }

        self.bands_from_sums()
    }

    fn next_fallback(&mut self, input: f64) -> (f64, f64, f64) {
        self.history.push(input);
        let (u, m, l) = talib_rs::overlap::bbands(
            &self.history,
            self.timeperiod,
            self.nbdevup,
            self.nbdevdn,
            self.matype,
        )
        .unwrap_or_else(|_| {
            let n = self.history.len();
            (vec![f64::NAN; n], vec![f64::NAN; n], vec![f64::NAN; n])
        });
        (
            *u.last().unwrap_or(&f64::NAN),
            *m.last().unwrap_or(&f64::NAN),
            *l.last().unwrap_or(&f64::NAN),
        )
    }
}

impl Next<f64> for BBANDS {
    type Output = (f64, f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        if self.matype == MaType::Sma {
            self.next_sma(input)
        } else {
            self.next_fallback(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_bbands_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let nbdevup = 2.0;
            let nbdevdn = 2.0;
            let matype = MaType::Sma;
            let mut bbands = BBANDS::new(period, nbdevup, nbdevdn, matype);
            let streaming_results: Vec<(f64, f64, f64)> =
                input.iter().map(|&x| bbands.next(x)).collect();
            let (b_upper, b_middle, b_lower) = talib_rs::overlap::bbands(
                &input,
                period,
                nbdevup,
                nbdevdn,
                matype,
            )
            .unwrap_or_else(|_| {
                (
                    vec![f64::NAN; input.len()],
                    vec![f64::NAN; input.len()],
                    vec![f64::NAN; input.len()],
                )
            });

            for (i, (s_upper, s_middle, s_lower)) in streaming_results.into_iter().enumerate() {
                if s_upper.is_nan() {
                    assert!(b_upper[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_upper, b_upper[i], epsilon = 1e-6);
                }
                if s_middle.is_nan() {
                    assert!(b_middle[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_middle, b_middle[i], epsilon = 1e-6);
                }
                if s_lower.is_nan() {
                    assert!(b_lower[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_lower, b_lower[i], epsilon = 1e-6);
                }
            }
        }
    }
}