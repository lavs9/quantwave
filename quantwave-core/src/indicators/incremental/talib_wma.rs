//! TA-Lib compatible streaming WMA — O(1) per bar, bounded memory.
//!
//! Mirrors the `talib_rs::overlap::wma` recurrence exactly:
//!   `ws[i] = ws[i-1] + period * x[i] - ps[i-1]`
//!   `ps[i] = ps[i-1] + x[i] - x[i-period]`
//! so the streaming and batch paths agree bit-for-bit on the same inputs.

use crate::traits::Next;
use crate::utils::RingBuffer;

/// Weighted moving average matching `talib_rs::overlap::wma` (lookback `period - 1`).
#[derive(Debug, Clone)]
pub struct TalibWma {
    period: usize,
    divider: f64,
    /// Weighted sum over the current window.
    ws: f64,
    /// Plain sum over the current window.
    ps: f64,
    /// Last `period` inputs — needed only for the value leaving the window.
    window: RingBuffer<f64>,
    bars_seen: usize,
}

impl TalibWma {
    pub fn new(period: usize) -> Self {
        let p = period as f64;
        Self {
            period,
            divider: p * (p + 1.0) / 2.0,
            ws: 0.0,
            ps: 0.0,
            window: RingBuffer::with_capacity(period.max(1)),
            bars_seen: 0,
        }
    }
}

impl Next<f64> for TalibWma {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let p = self.period;
        if p == 0 {
            return f64::NAN;
        }
        let i = self.bars_seen;
        self.bars_seen += 1;

        if i < p {
            // Seeding window: replicate the batch initialisation loop.
            self.window.push_back(input);
            self.ws += input * (i + 1) as f64;
            self.ps += input;
            if i + 1 < p {
                return f64::NAN;
            }
            return self.ws / self.divider;
        }

        let old = self.window.pop_front().unwrap_or(0.0);
        self.window.push_back(input);
        self.ws = self.ws + p as f64 * input - self.ps;
        self.ps = self.ps + input - old;
        self.ws / self.divider
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_talib_wma_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let mut wma = TalibWma::new(period);
            let streaming: Vec<f64> = input.iter().map(|&x| wma.next(x)).collect();
            let batch = talib_rs::overlap::wma(&input, period)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-9); }
            }
        }
    }
}
