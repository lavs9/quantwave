pub mod traits;
pub mod indicators;

#[cfg(any(test, feature = "test_utils"))]
pub mod test_utils;

pub use traits::{Next, SmoothingAlgorithm, IndicatorConfig};
pub use indicators::smoothing::{SMA, EMA, WMA};
pub use indicators::volatility::{TrueRange, ATR, TaATR, TaNATR, TaTRANGE};
pub use indicators::supertrend::SuperTrend;
pub use indicators::vwap::AnchoredVWAP;
pub use indicators::hma::HMA;
pub use indicators::keltner::KeltnerChannels;
pub use indicators::alma::ALMA;
pub use indicators::donchian::DonchianChannels;
pub use indicators::statistics::{StandardDeviation, LinearRegression, TaSTDDEV, TaVAR, TaBETA, TaCORREL, TaLINEARREG, TaLINEARREG_SLOPE, TaLINEARREG_INTERCEPT, TaLINEARREG_ANGLE, TaTSF};
pub use indicators::ttm_squeeze::TTMSqueeze;
pub use indicators::vortex::VortexIndicator;
pub use indicators::heikin_ashi::HeikinAshi;
pub use indicators::wavetrend::WaveTrend;
pub use indicators::tema::{TEMA, ZLEMA};
pub use indicators::atr_ts::ATRTrailingStop;
pub use indicators::pivot_points::PivotPoints;
pub use indicators::fractals::BillWilliamsFractals;
pub use indicators::ichimoku::IchimokuCloud;
pub use indicators::math::*;
pub use indicators::overlap::*;
pub use indicators::momentum::*;
pub use indicators::volume::*;
pub use indicators::pattern::*;
pub use indicators::price_transform::*;
pub use indicators::cycle::*;

/// Re-export talib-rs for convenience
pub use talib_rs as talib;
