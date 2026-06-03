//! TA-Lib compatible streaming moving averages (SMA / EMA).

use crate::indicators::incremental::talib_ema::TalibEma;
use crate::indicators::incremental::talib_sma::TalibSma;
use crate::traits::Next;
use talib_rs::MaType;

/// Streaming MA matching TA-Lib `compute_ma` for SMA and EMA.
#[derive(Debug, Clone)]
pub enum MaStream {
    Sma(TalibSma),
    Ema(TalibEma),
}

impl MaStream {
    pub fn new(period: usize, ma_type: MaType) -> Self {
        match ma_type {
            MaType::Ema => Self::Ema(TalibEma::new(period)),
            _ => Self::Sma(TalibSma::new(period)),
        }
    }

    pub fn next(&mut self, v: f64) -> f64 {
        match self {
            Self::Sma(s) => s.next(v),
            Self::Ema(e) => e.next(v),
        }
    }
}