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

/// Re-export talib-rs for convenience
pub use talib_rs as talib;
