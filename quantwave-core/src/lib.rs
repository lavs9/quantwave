pub mod traits;
pub mod indicators;

#[cfg(any(test, feature = "test_utils"))]
pub mod test_utils;

pub use traits::{Next, SmoothingAlgorithm, IndicatorConfig};
pub use indicators::smoothing::{SMA, EMA, WMA};
pub use indicators::volatility::{TrueRange, ATR};
pub use indicators::supertrend::SuperTrend;
pub use indicators::vwap::AnchoredVWAP;
pub use indicators::hma::HMA;
pub use indicators::keltner::KeltnerChannels;
pub use indicators::alma::ALMA;
pub use indicators::donchian::DonchianChannels;
pub use indicators::statistics::{StandardDeviation, LinearRegression};
pub use indicators::ttm_squeeze::TTMSqueeze;
pub use indicators::vortex::VortexIndicator;
pub use indicators::heikin_ashi::HeikinAshi;
pub use indicators::wavetrend::WaveTrend;
pub use indicators::tema::{TEMA, ZLEMA};
pub use indicators::atr_ts::ATRTrailingStop;
pub use indicators::pivot_points::PivotPoints;
pub use indicators::fractals::BillWilliamsFractals;
pub use indicators::ichimoku::IchimokuCloud;

/// Re-export talib-rs for convenience
pub use talib_rs as talib;
