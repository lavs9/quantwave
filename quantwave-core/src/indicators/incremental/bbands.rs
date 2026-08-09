//! Native Bollinger Bands — TA-Lib parity, incremental for every `matype`.
//!
//! Both paths are streaming and bounded-memory:
//! * `MaType::Sma` — O(1) amortised per bar via [`RollingVariance`], a
//!   shifted-data accumulator with a deterministic periodic exact refresh. It
//!   replaced a `sum` / `sum_sq` pair whose `E[X²] - E[X]²` evaluation lost
//!   ~1e-6 relative precision on slowly-varying inputs (quantwave-qkft).
//! * every other `MaType` — the middle band comes from [`MaStream`], and the
//!   deviation is a two-pass sum of squared deviations over a rolling window of
//!   `timeperiod` inputs (O(timeperiod) per bar), exactly as the batch
//!   `talib_rs::overlap::bbands` non-SMA path computes it.
//!
//! The non-SMA path used to push every bar onto an ever-growing `history` and
//! re-run the *batch* `bbands` over the whole thing, i.e. O(n) per bar / O(n²)
//! per series plus an unbounded memory leak in long-running streams.

use crate::indicators::incremental::ma_stream::MaStream;
use crate::indicators::incremental::rolling_variance::RollingVariance;
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
    /// Rolling window of the last `timeperiod` inputs (both paths). Carries the
    /// stable mean/variance accumulator used by the SMA fast path; the non-SMA
    /// path only reads back the window contents.
    window: RollingVariance,
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
            window: RollingVariance::new(timeperiod),
            ma: (matype != MaType::Sma).then(|| MaStream::new(timeperiod, matype)),
            bars_seen: 0,
        }
    }

    #[inline]
    fn bands_from_window(&self) -> (f64, f64, f64) {
        let ma_val = self.window.mean();
        let stddev = self.window.stddev();
        let upper = ma_val + self.nbdevup * stddev;
        let lower = ma_val - self.nbdevdn * stddev;
        (upper, ma_val, lower)
    }

    fn next_sma(&mut self, input: f64) -> (f64, f64, f64) {
        let tp = self.timeperiod;
        if tp == 0 {
            return NAN_TRIPLE;
        }

        if !self.window.push(input) {
            return NAN_TRIPLE;
        }

        self.bands_from_window()
    }

    fn next_non_sma(&mut self, input: f64) -> (f64, f64, f64) {
        let tp = self.timeperiod;
        if tp == 0 {
            return NAN_TRIPLE;
        }
        let i = self.bars_seen;
        self.bars_seen += 1;

        self.window.push(input);
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

    // ---- quantwave-qkft ----------------------------------------------------

    /// BBANDS is the headline consumer of the stddev accumulator. On log-price
    /// the SMA path's band half-width must track a compensated two-pass
    /// reference to well inside the strict 1e-7 gate.
    #[test]
    fn sma_bands_match_two_pass_reference_on_log_price() {
        use crate::test_utils::{log_price_series, reference_mean, reference_stddev};
        let period = 20;
        let nbdev = 2.0;
        let data = log_price_series(600);
        let mut bbands = BBANDS::new(period, nbdev, nbdev, MaType::Sma);
        let mut worst_mid: f64 = 0.0;
        let mut worst_half: f64 = 0.0;
        for (i, &x) in data.iter().enumerate() {
            let (upper, middle, lower) = bbands.next(x);
            if middle.is_nan() {
                continue;
            }
            let window = &data[i + 1 - period..=i];
            let ref_mid = reference_mean(window);
            let ref_half = nbdev * reference_stddev(window);
            assert!(ref_half > 0.0);
            worst_mid = worst_mid.max(((middle - ref_mid) / ref_mid).abs());
            // Compare the half-width, not the band level: `upper` is
            // dominated by the ~8.0 middle band and would hide the error.
            worst_half = worst_half.max((((upper - lower) / 2.0 - ref_half) / ref_half).abs());
        }
        assert!(
            worst_half < 1e-7,
            "BBANDS half-width vs reference on log-price: {worst_half:e} (gate 1e-7)"
        );
        // Noise floor here is the *test's* extraction, not the accumulator:
        // recovering a ~1e-4 half-width by differencing two bands sitting at
        // level ~8 costs ~eps * 8 / 1e-4 ≈ 1e-11 relative. The accumulator
        // itself is at ~1e-15 (see rolling_variance's own tests).
        assert!(worst_half < 1e-10, "expected ~1e-11, got {worst_half:e}");
        assert!(worst_mid < 1e-12, "middle band drift {worst_mid:e}");
    }

    /// Bit-identical batch vs chunked streaming, for every `matype`.
    #[test]
    fn batch_streaming_bitwise_parity_all_matypes() {
        use crate::test_utils::log_price_series;
        let data = log_price_series(400);
        for &matype in MaType::ALL.iter() {
            let mut batch = BBANDS::new(10, 2.0, 2.5, matype);
            let batch_out: Vec<_> = data.iter().map(|&x| batch.next(x)).collect();
            for chunk in [1usize, 7, 64] {
                let mut streaming = BBANDS::new(10, 2.0, 2.5, matype);
                let mut out = Vec::with_capacity(data.len());
                for part in data.chunks(chunk) {
                    for &x in part {
                        out.push(streaming.next(x));
                    }
                }
                for (i, (b, s)) in batch_out.iter().zip(out.iter()).enumerate() {
                    assert_eq!(
                        (b.0.to_bits(), b.1.to_bits(), b.2.to_bits()),
                        (s.0.to_bits(), s.1.to_bits(), s.2.to_bits()),
                        "{matype:?} bar {i} chunk {chunk}: {b:?} vs {s:?}"
                    );
                }
            }
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
