//! TA-Lib compatible SMA (NaN until lookback, then sliding window).

use crate::traits::Next;
use crate::utils::RingBuffer;

/// Simple moving average matching `talib_rs::overlap::sma`.
#[derive(Debug, Clone)]
pub struct TalibSma {
    period: usize,
    window: RingBuffer<f64>,
    sum: f64,
}

impl TalibSma {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            window: RingBuffer::with_capacity(period),
            sum: 0.0,
        }
    }
}

impl Next<f64> for TalibSma {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let p = self.period;
        if p == 0 {
            return f64::NAN;
        }

        if self.window.len() >= p
            && let Some(old) = self.window.pop_front()
        {
            self.sum -= old;
        }
        self.window.push_back(input);
        self.sum += input;

        if self.window.len() < p {
            return f64::NAN;
        }
        self.sum / p as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_talib_sma_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut sma = TalibSma::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| sma.next(x)).collect();
            let batch = talib_rs::overlap::sma(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
