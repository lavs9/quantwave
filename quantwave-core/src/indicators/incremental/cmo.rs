//! Native O(1) CMO — TA-Lib Wilder smoothing parity (`talib_rs::momentum::cmo`).

use crate::traits::Next;

#[inline]
fn cmo_from_sums(sum_up: f64, sum_down: f64) -> f64 {
    let total = sum_up + sum_down;
    if total > 0.0 {
        100.0 * (sum_up - sum_down) / total
    } else {
        0.0
    }
}

/// Chande Momentum Oscillator — matches `talib_rs::momentum::cmo`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CMO {
    pub timeperiod: usize,
    period_f: f64,
    prev_close: Option<f64>,
    sum_up: f64,
    sum_down: f64,
    warmup_changes: usize,
}

impl CMO {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            period_f: timeperiod as f64,
            prev_close: None,
            sum_up: 0.0,
            sum_down: 0.0,
            warmup_changes: 0,
        }
    }
}

impl Next<f64> for CMO {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let period = self.timeperiod;
        if period < 2 {
            return f64::NAN;
        }

        let Some(prev) = self.prev_close else {
            self.prev_close = Some(input);
            return f64::NAN;
        };
        self.prev_close = Some(input);

        let change = input - prev;
        let (cur_up, cur_down) = if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, -change)
        };

        if self.warmup_changes < period {
            self.warmup_changes += 1;
            self.sum_up += cur_up;
            self.sum_down += cur_down;
            if self.warmup_changes < period {
                return f64::NAN;
            }
            return cmo_from_sums(self.sum_up, self.sum_down);
        }

        self.sum_up = self.sum_up - (self.sum_up / self.period_f) + cur_up;
        self.sum_down = self.sum_down - (self.sum_down / self.period_f) + cur_down;
        cmo_from_sums(self.sum_up, self.sum_down)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_cmo_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 14;
            let mut cmo = CMO::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| cmo.next(x)).collect();
            let batch = talib_rs::momentum::cmo(&input, period)
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