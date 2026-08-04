//! Native O(1) MACDEXT and MACDFIX — TA-Lib parity.

use crate::indicators::incremental::ma_stream::MaStream;
use crate::indicators::incremental::macd::MACD;
use crate::indicators::ma_type::MaType;
use crate::traits::Next;

const NAN_TRIPLE: (f64, f64, f64) = (f64::NAN, f64::NAN, f64::NAN);

/// MACD with controllable MA types — matches `talib_rs::momentum::macd_ext`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MACDEXT {
    pub fastperiod: usize,
    pub fastmatype: MaType,
    pub slowperiod: usize,
    pub slowmatype: MaType,
    pub signalperiod: usize,
    pub signalmatype: MaType,
    ema_macd: Option<MACD>,
    fast_ma: MaStream,
    slow_ma: MaStream,
    signal_ma: MaStream,
    macd_valid_count: usize,
}

impl MACDEXT {
    pub fn new(
        fastperiod: usize,
        fastmatype: MaType,
        slowperiod: usize,
        slowmatype: MaType,
        signalperiod: usize,
        signalmatype: MaType,
    ) -> Self {
        let ema_macd = if fastmatype == MaType::Ema
            && slowmatype == MaType::Ema
            && signalmatype == MaType::Ema
        {
            Some(MACD::new(fastperiod, slowperiod, signalperiod))
        } else {
            None
        };
        Self {
            fastperiod,
            fastmatype,
            slowperiod,
            slowmatype,
            signalperiod,
            signalmatype,
            ema_macd,
            fast_ma: MaStream::new(fastperiod, fastmatype),
            slow_ma: MaStream::new(slowperiod, slowmatype),
            signal_ma: MaStream::new(signalperiod, signalmatype),
            macd_valid_count: 0,
        }
    }
}

impl Next<f64> for MACDEXT {
    type Output = (f64, f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        if let Some(ref mut macd) = self.ema_macd {
            return macd.next(input);
        }

        let fast = self.fast_ma.next(input);
        let slow = self.slow_ma.next(input);
        if fast.is_nan() || slow.is_nan() {
            return NAN_TRIPLE;
        }

        let macd_line = fast - slow;
        self.macd_valid_count += 1;
        let signal = self.signal_ma.next(macd_line);
        if signal.is_nan() {
            return NAN_TRIPLE;
        }

        (macd_line, signal, macd_line - signal)
    }
}

/// MACD Fix (12/26 fixed) — matches `talib_rs::momentum::macd_fix`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct MACDFIX {
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

impl MACDFIX {
    pub fn new(signalperiod: usize) -> Self {
        let fp = 12usize;
        let sp = 26usize;
        Self {
            signalperiod,
            fp,
            sp,
            k_fast: 0.15,
            k_slow: 0.075,
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

impl Next<f64> for MACDFIX {
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
            let slow_seed: f64 = self.seed_closes.iter().sum::<f64>() / self.sp as f64;
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
            let signal_seed: f64 = self.macd_values[..self.signalperiod].iter().sum::<f64>()
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

    type Triples = (Vec<f64>, Vec<f64>, Vec<f64>);

    fn assert_parity(streaming: &[(f64, f64, f64)], batch: &Triples, label: &str) {
        let (b_macd, b_signal, b_hist) = batch;
        for (i, &(s_m, s_s, s_h)) in streaming.iter().enumerate() {
            for (s, b, name) in [
                (s_m, b_macd[i], "macd"),
                (s_s, b_signal[i], "signal"),
                (s_h, b_hist[i], "hist"),
            ] {
                if s.is_nan() {
                    assert!(
                        b.is_nan(),
                        "{label} bar {i} {name}: streaming NaN, batch {b}"
                    );
                } else {
                    assert!(
                        !b.is_nan(),
                        "{label} bar {i} {name}: streaming {s}, batch NaN"
                    );
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_macdext_ema_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let fast = 12usize;
            let slow = 26usize;
            let signal = 9usize;
            let matype = MaType::Ema;
            let mut ext = MACDEXT::new(fast, matype, slow, matype, signal, matype);
            let streaming: Vec<_> = input.iter().map(|&x| ext.next(x)).collect();
            let batch = talib_rs::momentum::macd_ext(
                &input, fast, matype.into(), slow, matype.into(), signal, matype.into(),
            ).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });
            assert_parity(&streaming, &batch, "macdext/ema");
        }

        /// MACDEXT smooths all three legs through `MaStream`, so every matype
        /// except EMA silently became SMA. Cover all nine.
        #[test]
        fn test_macdext_parity_all_matypes(
            input in prop::collection::vec(0.1..100.0, 150..220),
            idx in 0usize..9,
        ) {
            let matype = MaType::ALL[idx];
            let (fast, slow, signal) = (12usize, 26usize, 9usize);
            let mut ext = MACDEXT::new(fast, matype, slow, matype, signal, matype);
            let streaming: Vec<_> = input.iter().map(|&x| ext.next(x)).collect();
            let batch = talib_rs::momentum::macd_ext(
                &input, fast, matype.into(), slow, matype.into(), signal, matype.into(),
            ).unwrap_or_else(|_| {
                (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
            });
            assert_parity(&streaming, &batch, &format!("macdext/{matype}"));
        }

        #[test]
        fn test_macdfix_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let signal = 9usize;
            let mut fix = MACDFIX::new(signal);
            let streaming: Vec<_> = input.iter().map(|&x| fix.next(x)).collect();
            let batch = talib_rs::momentum::macd_fix(&input, signal)
                .unwrap_or_else(|_| {
                    (vec![f64::NAN; input.len()], vec![f64::NAN; input.len()], vec![f64::NAN; input.len()])
                });
            assert_parity(&streaming, &batch, "macdfix");
        }
    }
}
