//! Native O(1) RSI — TA-Lib Wilder smoothing parity.

use crate::traits::Next;

#[inline]
fn rsi_from_avgs(avg_gain: f64, avg_loss: f64) -> f64 {
    if avg_loss == 0.0 {
        100.0
    } else {
        let rs = avg_gain / avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    }
}

/// Relative Strength Index (RSI) — Wilder smoothing, matches `talib_rs::momentum::rsi`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct RSI {
    pub timeperiod: usize,
    period_f: f64,
    prev_close: Option<f64>,
    avg_gain: f64,
    avg_loss: f64,
    warmup_changes: usize,
    sum_gain: f64,
    sum_loss: f64,
}

impl RSI {
    pub fn new(timeperiod: usize) -> Self {
        Self {
            timeperiod,
            period_f: timeperiod as f64,
            prev_close: None,
            avg_gain: 0.0,
            avg_loss: 0.0,
            warmup_changes: 0,
            sum_gain: 0.0,
            sum_loss: 0.0,
        }
    }
}

impl Next<f64> for RSI {
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

        let change = input - prev;
        self.prev_close = Some(input);

        let (gain, loss) = if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, -change)
        };

        if self.warmup_changes < period {
            self.warmup_changes += 1;
            self.sum_gain += gain;
            self.sum_loss += loss;
            if self.warmup_changes < period {
                return f64::NAN;
            }
            self.avg_gain = self.sum_gain / self.period_f;
            self.avg_loss = self.sum_loss / self.period_f;
            return rsi_from_avgs(self.avg_gain, self.avg_loss);
        }

        self.avg_gain = (self.avg_gain * (self.period_f - 1.0) + gain) / self.period_f;
        self.avg_loss = (self.avg_loss * (self.period_f - 1.0) + loss) / self.period_f;
        rsi_from_avgs(self.avg_gain, self.avg_loss)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_rsi_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 14;
            let mut rsi = RSI::new(period);
            let streaming_results: Vec<f64> = input.iter().map(|&x| rsi.next(x)).collect();
            let batch_results = talib_rs::momentum::rsi(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }
}
