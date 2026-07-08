//! Native O(1) APO and PPO — TA-Lib parity.

use crate::indicators::incremental::ma_stream::MaStream;
use crate::traits::Next;
use talib_rs::MaType;

/// Absolute Price Oscillator.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct APO {
    pub fastperiod: usize,
    pub slowperiod: usize,
    pub matype: MaType,
    fast_ma: MaStream,
    slow_ma: MaStream,
}

impl APO {
    pub fn new(fastperiod: usize, slowperiod: usize, matype: MaType) -> Self {
        Self {
            fastperiod,
            slowperiod,
            matype,
            fast_ma: MaStream::new(fastperiod, matype),
            slow_ma: MaStream::new(slowperiod, matype),
        }
    }
}

impl Next<f64> for APO {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let fast = self.fast_ma.next(input);
        let slow = self.slow_ma.next(input);
        if fast.is_nan() || slow.is_nan() {
            f64::NAN
        } else {
            fast - slow
        }
    }
}

/// Percentage Price Oscillator.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct PPO {
    pub fastperiod: usize,
    pub slowperiod: usize,
    pub matype: MaType,
    fast_ma: MaStream,
    slow_ma: MaStream,
}

impl PPO {
    pub fn new(fastperiod: usize, slowperiod: usize, matype: MaType) -> Self {
        Self {
            fastperiod,
            slowperiod,
            matype,
            fast_ma: MaStream::new(fastperiod, matype),
            slow_ma: MaStream::new(slowperiod, matype),
        }
    }
}

impl Next<f64> for PPO {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let fast = self.fast_ma.next(input);
        let slow = self.slow_ma.next(input);
        if fast.is_nan() || slow.is_nan() || slow == 0.0 {
            f64::NAN
        } else {
            (fast - slow) / slow * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_apo_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let fast = 12;
            let slow = 26;
            let matype = MaType::Ema;
            let mut apo = APO::new(fast, slow, matype);
            let streaming: Vec<f64> = input.iter().map(|&x| apo.next(x)).collect();
            let batch = talib_rs::momentum::apo(&input, fast, slow, matype)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }

        #[test]
        fn test_ppo_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let fast = 12;
            let slow = 26;
            let matype = MaType::Ema;
            let mut ppo = PPO::new(fast, slow, matype);
            let streaming: Vec<f64> = input.iter().map(|&x| ppo.next(x)).collect();
            let batch = talib_rs::momentum::ppo(&input, fast, slow, matype)
                .unwrap_or_else(|_| vec![f64::NAN; input.len()]);
            for (s, b) in streaming.iter().zip(batch.iter()) {
                if s.is_nan() { assert!(b.is_nan()); }
                else { approx::assert_relative_eq!(s, b, epsilon = 1e-6); }
            }
        }
    }
}
