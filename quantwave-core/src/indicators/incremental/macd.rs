//! Native O(1) MACD — TA-Lib aligned EMA seeding parity.

use crate::traits::Next;

const NAN_TRIPLE: (f64, f64, f64) = (f64::NAN, f64::NAN, f64::NAN);

/// MACD (12, 26, 9 default) — matches `talib_rs::momentum::macd` aligned internal EMAs.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MACD {
    pub fastperiod: usize,
    pub slowperiod: usize,
    pub signalperiod: usize,
    fp: usize,
    sp: usize,
    k_fast: f64,
    k_slow: f64,
    k_signal: f64,
    out_start: usize,
    bars_seen: usize,
    seed_closes: Vec<f64>,
    slow_ema: f64,
    fast_ema: f64,
    macd_values: Vec<f64>,
    signal_ema: f64,
}

impl MACD {
    pub fn new(fastperiod: usize, slowperiod: usize, signalperiod: usize) -> Self {
        let (fp, sp) = if fastperiod < slowperiod {
            (fastperiod, slowperiod)
        } else {
            (slowperiod, fastperiod)
        };
        Self {
            fastperiod,
            slowperiod,
            signalperiod,
            fp,
            sp,
            k_fast: 2.0 / (fp as f64 + 1.0),
            k_slow: 2.0 / (sp as f64 + 1.0),
            k_signal: 2.0 / (signalperiod as f64 + 1.0),
            out_start: sp - 1 + signalperiod - 1,
            bars_seen: 0,
            seed_closes: Vec::with_capacity(sp),
            slow_ema: 0.0,
            fast_ema: 0.0,
            macd_values: Vec::new(),
            signal_ema: 0.0,
        }
    }

    #[inline]
    fn update_emas(&mut self, input: f64) {
        self.slow_ema = self.k_slow.mul_add(input - self.slow_ema, self.slow_ema);
        self.fast_ema = self.k_fast.mul_add(input - self.fast_ema, self.fast_ema);
        self.macd_values.push(self.fast_ema - self.slow_ema);
    }
}

impl Next<f64> for MACD {
    type Output = (f64, f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        let i = self.bars_seen;
        self.bars_seen += 1;

        if i < self.sp - 1 {
            self.seed_closes.push(input);
            return NAN_TRIPLE;
        }

        if i == self.sp - 1 {
            self.seed_closes.push(input);
            let slow_seed: f64 =
                self.seed_closes.iter().sum::<f64>() / self.sp as f64;
            let fast_seed: f64 = self.seed_closes[self.sp - self.fp..self.sp]
                .iter()
                .sum::<f64>()
                / self.fp as f64;
            self.slow_ema = slow_seed;
            self.fast_ema = fast_seed;
            let macd0 = fast_seed - slow_seed;
            self.macd_values.push(macd0);
            if self.out_start == self.sp - 1 {
                let signal_seed = macd0;
                self.signal_ema = signal_seed;
                return (macd0, signal_seed, 0.0);
            }
            return NAN_TRIPLE;
        }

        if i < self.out_start {
            self.update_emas(input);
            return NAN_TRIPLE;
        }

        if i == self.out_start {
            if i >= self.sp {
                self.update_emas(input);
            }
            let signal_seed: f64 = self.macd_values[..self.signalperiod]
                .iter()
                .sum::<f64>()
                / self.signalperiod as f64;
            self.signal_ema = signal_seed;
            let macd = self.macd_values[self.signalperiod - 1];
            return (macd, signal_seed, macd - signal_seed);
        }

        self.update_emas(input);
        let macd = *self.macd_values.last().unwrap_or(&f64::NAN);
        self.signal_ema = self
            .k_signal
            .mul_add(macd - self.signal_ema, self.signal_ema);
        (macd, self.signal_ema, macd - self.signal_ema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_macd_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let fast = 12;
            let slow = 26;
            let signal = 9;
            let mut macd = MACD::new(fast, slow, signal);
            let streaming_results: Vec<(f64, f64, f64)> =
                input.iter().map(|&x| macd.next(x)).collect();
            let (b_macd, b_signal, b_hist) = talib_rs::momentum::macd(&input, fast, slow, signal)
                .unwrap_or_else(|_| {
                    (
                        vec![f64::NAN; input.len()],
                        vec![f64::NAN; input.len()],
                        vec![f64::NAN; input.len()],
                    )
                });

            for (i, (s_macd, s_signal, s_hist)) in streaming_results.into_iter().enumerate() {
                if s_macd.is_nan() {
                    assert!(b_macd[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_macd, b_macd[i], epsilon = 1e-6);
                }
                if s_signal.is_nan() {
                    assert!(b_signal[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_signal, b_signal[i], epsilon = 1e-6);
                }
                if s_hist.is_nan() {
                    assert!(b_hist[i].is_nan());
                } else {
                    approx::assert_relative_eq!(s_hist, b_hist[i], epsilon = 1e-6);
                }
            }
        }
    }
}