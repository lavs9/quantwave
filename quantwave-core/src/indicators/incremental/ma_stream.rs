//! Streaming moving average dispatched on [`MaType`] — TA-Lib `compute_ma` parity.
//!
//! Every one of the nine TA-Lib MA families is wired to its native QuantWave
//! streaming implementation. Previously this enum carried only `Sma` and `Ema`
//! variants and a catch-all arm collapsed Wma/Dema/Tema/Trima/Kama/Mama/T3 to
//! SMA, so `APO::new(12, 26, MaType::Wma)` silently returned an SMA-based APO.
//! There is no silent substitution left: the dispatch is total.

use crate::indicators::incremental::dema::DEMA;
use crate::indicators::incremental::hilbert_ta::TalibMama;
use crate::indicators::incremental::overlap_ta::{KAMA, T3, TRIMA};
use crate::indicators::incremental::talib_ema::TalibEma;
use crate::indicators::incremental::talib_sma::TalibSma;
use crate::indicators::incremental::talib_tema::TalibTema;
use crate::indicators::incremental::talib_wma::TalibWma;
use crate::indicators::ma_type::MaType;
use crate::traits::Next;

/// `vfactor` C TA-Lib's `ta_MA.c` uses when T3 is reached through the MA dispatcher.
pub const MA_DISPATCH_T3_VFACTOR: f64 = 0.7;
/// `fastlimit` C TA-Lib's `ta_MA.c` uses when MAMA is reached through the MA dispatcher.
pub const MA_DISPATCH_MAMA_FASTLIMIT: f64 = 0.5;
/// `slowlimit` C TA-Lib's `ta_MA.c` uses when MAMA is reached through the MA dispatcher.
pub const MA_DISPATCH_MAMA_SLOWLIMIT: f64 = 0.05;

/// Streaming MA matching `talib_rs`'s `compute_ma` for all nine `MaType`s.
#[derive(Debug, Clone)]
pub enum MaStream {
    Sma(TalibSma),
    Ema(TalibEma),
    Wma(TalibWma),
    Dema(DEMA),
    Tema(TalibTema),
    Trima(TRIMA),
    Kama(KAMA),
    /// Boxed: the Hilbert engine dwarfs the other variants.
    Mama(Box<TalibMama>),
    T3(T3),
}

impl MaStream {
    /// Build the streaming MA for `ma_type`.
    ///
    /// `MaType::Mama` ignores `period` and uses `fastlimit = 0.5`,
    /// `slowlimit = 0.05`; `MaType::T3` uses `vfactor = 0.7`. Both match what
    /// C TA-Lib's `ta_MA.c` does when those families are selected through a
    /// `matype` argument.
    pub fn new(period: usize, ma_type: MaType) -> Self {
        match ma_type {
            MaType::Sma => Self::Sma(TalibSma::new(period)),
            MaType::Ema => Self::Ema(TalibEma::new(period)),
            MaType::Wma => Self::Wma(TalibWma::new(period)),
            MaType::Dema => Self::Dema(DEMA::new(period)),
            MaType::Tema => Self::Tema(TalibTema::new(period)),
            MaType::Trima => Self::Trima(TRIMA::new(period)),
            MaType::Kama => Self::Kama(KAMA::new(period)),
            MaType::Mama => Self::Mama(Box::new(TalibMama::new(
                MA_DISPATCH_MAMA_FASTLIMIT,
                MA_DISPATCH_MAMA_SLOWLIMIT,
            ))),
            MaType::T3 => Self::T3(T3::new(period, MA_DISPATCH_T3_VFACTOR)),
        }
    }

    /// The `MaType` this stream was built from.
    pub fn ma_type(&self) -> MaType {
        match self {
            Self::Sma(_) => MaType::Sma,
            Self::Ema(_) => MaType::Ema,
            Self::Wma(_) => MaType::Wma,
            Self::Dema(_) => MaType::Dema,
            Self::Tema(_) => MaType::Tema,
            Self::Trima(_) => MaType::Trima,
            Self::Kama(_) => MaType::Kama,
            Self::Mama(_) => MaType::Mama,
            Self::T3(_) => MaType::T3,
        }
    }

    pub fn next(&mut self, v: f64) -> f64 {
        match self {
            Self::Sma(s) => s.next(v),
            Self::Ema(e) => e.next(v),
            Self::Wma(w) => w.next(v),
            Self::Dema(d) => d.next(v),
            Self::Tema(t) => t.next(v),
            Self::Trima(t) => t.next(v),
            Self::Kama(k) => k.next(v),
            Self::Mama(m) => m.next(v).0,
            Self::T3(t) => t.next(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Batch oracle: `talib_rs`'s `compute_ma` dispatch, spelled out.
    pub(crate) fn ma_oracle(input: &[f64], period: usize, ma_type: MaType) -> Vec<f64> {
        let batch = match ma_type {
            MaType::Sma => talib_rs::overlap::sma(input, period),
            MaType::Ema => talib_rs::overlap::ema(input, period),
            MaType::Wma => talib_rs::overlap::wma(input, period),
            MaType::Dema => talib_rs::overlap::dema(input, period),
            MaType::Tema => talib_rs::overlap::tema(input, period),
            MaType::Trima => talib_rs::overlap::trima(input, period),
            MaType::Kama => talib_rs::overlap::kama(input, period),
            MaType::Mama => talib_rs::overlap::mama(
                input,
                MA_DISPATCH_MAMA_FASTLIMIT,
                MA_DISPATCH_MAMA_SLOWLIMIT,
            )
            .map(|(mama, _fama)| mama),
            MaType::T3 => talib_rs::overlap::t3(input, period, MA_DISPATCH_T3_VFACTOR),
        };
        batch.unwrap_or_else(|_| vec![f64::NAN; input.len()])
    }

    proptest! {
        /// Regression test for the silent-SMA-substitution bug: every `MaType`
        /// must track its own batch oracle. Parameterised over all nine
        /// variants — the old suite only ever constructed `MaType::Sma`, which
        /// is exactly why the catch-all arm survived unnoticed.
        #[test]
        fn test_ma_stream_parity_all_matypes(
            input in prop::collection::vec(0.1..100.0, 120..200),
            idx in 0usize..9,
        ) {
            let ma_type = MaType::ALL[idx];
            let period = 10;
            let mut stream = MaStream::new(period, ma_type);
            let streaming: Vec<f64> = input.iter().map(|&x| stream.next(x)).collect();
            let batch = ma_oracle(&input, period, ma_type);
            prop_assert_eq!(stream.ma_type(), ma_type);
            for (i, (s, b)) in streaming.iter().zip(batch.iter()).enumerate() {
                if s.is_nan() {
                    prop_assert!(b.is_nan(), "{ma_type} bar {i}: streaming NaN, batch {b}");
                } else {
                    prop_assert!(!b.is_nan(), "{ma_type} bar {i}: streaming {s}, batch NaN");
                    approx::assert_relative_eq!(s, b, epsilon = 1e-6);
                }
            }
        }
    }

    /// Non-SMA `MaType`s used to silently collapse to SMA. They must not any more.
    #[test]
    fn non_sma_matypes_differ_from_sma() {
        let input: Vec<f64> = (0..200)
            .map(|i| 50.0 + 10.0 * (i as f64 * 0.3).sin() + i as f64 * 0.05)
            .collect();
        let sma: Vec<f64> = {
            let mut s = MaStream::new(10, MaType::Sma);
            input.iter().map(|&x| s.next(x)).collect()
        };
        for ma_type in MaType::ALL {
            if ma_type == MaType::Sma {
                continue;
            }
            let mut s = MaStream::new(10, ma_type);
            let got: Vec<f64> = input.iter().map(|&x| s.next(x)).collect();
            let identical = got
                .iter()
                .zip(sma.iter())
                .all(|(a, b)| (a.is_nan() && b.is_nan()) || (a - b).abs() < 1e-12);
            assert!(!identical, "{ma_type} still produces SMA values");
        }
    }
}
