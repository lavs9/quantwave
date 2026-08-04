//! Native Bollinger Bands — TA-Lib parity, incremental for every `matype`.
//!
//! Both paths are streaming and bounded-memory:
//! * `MaType::Sma` — O(1) per bar via a sliding `sum` / `sum_sq`.
//! * every other `MaType` — the middle band comes from [`MaStream`], and the
//!   deviation is a two-pass sum of squared deviations over a rolling window of
//!   `timeperiod` inputs (O(timeperiod) per bar), exactly as the batch
//!   `talib_rs::overlap::bbands` non-SMA path computes it.
//!
//! The non-SMA path used to push every bar onto an ever-growing `history` and
//! re-run the *batch* `bbands` over the whole thing, i.e. O(n) per bar / O(n²)
//! per series plus an unbounded memory leak in long-running streams.

use crate::indicators::incremental::ma_stream::MaStream;
use crate::indicators::incremental::utils::RingBuffer;
use crate::indicators::ma_type::MaType;
use crate::traits::Next;

const NAN_TRIPLE: (f64, f64, f64) = (f64::NAN, f64::NAN, f64::NAN);

/// Bollinger Bands — matches `talib_rs::overlap::bbands` for every `matype`.
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct BBANDS {
    pub timeperiod: usize,
    pub nbdevup: f64,
    pub nbdevdn: f64,
    pub matype: MaType,
    /// Rolling window of the last `timeperiod` inputs (both paths).
    window: RingBuffer<f64>,
    // --- SMA fast path ---
    sum: f64,
    sum_sq: f64,
    // --- non-SMA path ---
    ma: Option<MaStream>,
    bars_seen: usize,
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
            ma: (matype != MaType::Sma).then(|| MaStream::new(timeperiod, matype)),
            bars_seen: 0,
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

    /// Push onto the rolling window, evicting the oldest value once full.
    #[inline]
    fn push_window(&mut self, input: f64) -> Option<f64> {
        let evicted = if self.window.len() == self.timeperiod {
            self.window.pop_front()
        } else {
            None
        };
        self.window.push_back(input);
        evicted
    }

    fn next_sma(&mut self, input: f64) -> (f64, f64, f64) {
        let tp = self.timeperiod;
        if tp == 0 {
            return NAN_TRIPLE;
        }

        if let Some(old) = self.push_window(input) {
            self.sum -= old;
            self.sum_sq -= old * old;
        }
        self.sum += input;
        self.sum_sq += input * input;

        if self.window.len() < tp {
            return NAN_TRIPLE;
        }

        self.bands_from_sums()
    }

    fn next_non_sma(&mut self, input: f64) -> (f64, f64, f64) {
        let tp = self.timeperiod;
        if tp == 0 {
            return NAN_TRIPLE;
        }
        let i = self.bars_seen;
        self.bars_seen += 1;

        self.push_window(input);
        let ma_val = match self.ma {
            Some(ref mut ma) => ma.next(input),
            None => return NAN_TRIPLE,
        };

        if i + 1 < tp {
            // Before `lookback = timeperiod - 1` the batch fills NaN everywhere.
            return NAN_TRIPLE;
        }
        if ma_val.is_nan() {
            // Past the BBANDS lookback but still inside the MA's own (longer)
            // warmup — e.g. DEMA/TEMA/T3/MAMA. The batch leaves the zero-
            // initialised band values in place here and writes NaN only into
            // the middle band, so reproduce exactly that rather than inventing
            // a different warmup convention. See the `continue` in the non-SMA
            // loop of `talib_rs::overlap::bbands`.
            return (0.0, f64::NAN, 0.0);
        }

        // Two-pass deviation about the MA value over the rolling window. Matches
        // the batch `sum_sq_diff(window, ma_val)` and is the numerically stable
        // form quantwave-qkft wants everywhere (no E[X²] - E[X]² cancellation).
        let mut sum_sq_diff = 0.0;
        for &v in self.window.iter() {
            let d = v - ma_val;
            sum_sq_diff += d * d;
        }
        let stddev = (sum_sq_diff / tp as f64).max(0.0).sqrt();
        (
            ma_val + self.nbdevup * stddev,
            ma_val,
            ma_val - self.nbdevdn * stddev,
        )
    }
}

impl Next<f64> for BBANDS {
    type Output = (f64, f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        if self.matype == MaType::Sma {
            self.next_sma(input)
        } else {
            self.next_non_sma(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn batch_bbands(
        input: &[f64],
        period: usize,
        nbdevup: f64,
        nbdevdn: f64,
        matype: MaType,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        talib_rs::overlap::bbands(input, period, nbdevup, nbdevdn, matype.into()).unwrap_or_else(
            |_| {
                (
                    vec![f64::NAN; input.len()],
                    vec![f64::NAN; input.len()],
                    vec![f64::NAN; input.len()],
                )
            },
        )
    }

    fn assert_parity(streaming: &[(f64, f64, f64)], batch: &(Vec<f64>, Vec<f64>, Vec<f64>)) {
        let (b_upper, b_middle, b_lower) = batch;
        for (i, &(s_upper, s_middle, s_lower)) in streaming.iter().enumerate() {
            for (s, b, name) in [
                (s_upper, b_upper[i], "upper"),
                (s_middle, b_middle[i], "middle"),
                (s_lower, b_lower[i], "lower"),
            ] {
                if s.is_nan() {
                    assert!(b.is_nan(), "bar {i} {name}: streaming NaN, batch {b}");
                } else {
                    assert!(!b.is_nan(), "bar {i} {name}: streaming {s}, batch NaN");
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }

    proptest! {
        #[test]
        fn test_bbands_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 10;
            let matype = MaType::Sma;
            let mut bbands = BBANDS::new(period, 2.0, 2.0, matype);
            let streaming: Vec<(f64, f64, f64)> =
                input.iter().map(|&x| bbands.next(x)).collect();
            assert_parity(&streaming, &batch_bbands(&input, period, 2.0, 2.0, matype));
        }

        /// Regression coverage for the non-SMA path: the old suite only ever
        /// exercised `MaType::Sma`, which is why an O(n)-per-bar batch call hid
        /// inside `next()` for every other matype. Lengths start past the
        /// longest MA warmup (MAMA's 32, TEMA's 3*(p-1)) so the batch oracle
        /// returns real values rather than an `InsufficientData` error.
        #[test]
        fn test_bbands_parity_all_matypes(
            input in prop::collection::vec(0.1..100.0, 120..200),
            idx in 0usize..9,
        ) {
            let matype = MaType::ALL[idx];
            let period = 10;
            let mut bbands = BBANDS::new(period, 2.0, 2.5, matype);
            let streaming: Vec<(f64, f64, f64)> =
                input.iter().map(|&x| bbands.next(x)).collect();
            assert_parity(&streaming, &batch_bbands(&input, period, 2.0, 2.5, matype));
        }
    }

    /// The non-SMA path must not retain more than `timeperiod` samples.
    #[test]
    fn non_sma_window_is_bounded() {
        let mut bbands = BBANDS::new(10, 2.0, 2.0, MaType::Wma);
        for i in 0..10_000 {
            bbands.next(50.0 + (i as f64 * 0.01).sin());
        }
        assert_eq!(bbands.window.len(), 10);
    }
}
