//! TA-Lib compatible EMA state machine (SMA seed, NaN until lookback).

use crate::traits::Next;

/// Exponential moving average matching `talib_rs::overlap::ema`.
#[derive(Debug, Clone)]
pub struct TalibEma {
    period: usize,
    k: f64,
    lookback: usize,
    bars_seen: usize,
    seed_sum: f64,
    value: f64,
}

impl TalibEma {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            k: 2.0 / (period as f64 + 1.0),
            lookback: period.saturating_sub(1),
            bars_seen: 0,
            seed_sum: 0.0,
            value: f64::NAN,
        }
    }
}

impl Next<f64> for TalibEma {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        if self.period == 0 || input.is_nan() {
            return f64::NAN;
        }
        let i = self.bars_seen;
        self.bars_seen += 1;

        if i < self.lookback {
            self.seed_sum += input;
            return f64::NAN;
        }

        if i == self.lookback {
            self.seed_sum += input;
            self.value = self.seed_sum / self.period as f64;
            return self.value;
        }

        self.value = self.k.mul_add(input - self.value, self.value);
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_talib_ema_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut ema = TalibEma::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| ema.next(x)).collect();
            let batch = talib_rs::overlap::ema(&input, period)
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