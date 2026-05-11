pub mod traits;
pub mod indicators;

#[cfg(any(test, feature = "test_utils"))]
pub mod test_utils;

pub use traits::{Next, SmoothingAlgorithm, IndicatorConfig};
pub use indicators::smoothing::{SMA, EMA};
pub use indicators::volatility::{TrueRange, ATR};
pub use indicators::supertrend::SuperTrend;

/// Re-export talib-rs for convenience
pub use talib_rs as talib;
