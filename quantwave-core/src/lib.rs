pub mod indicators;
pub mod options_india;
pub mod traits;

#[cfg(any(test, feature = "test_utils"))]
pub mod test_utils;

pub use indicators::alma::ALMA;
pub use indicators::atr_ts::ATRTrailingStop;
pub use indicators::cycle::*;
pub use indicators::donchian::DonchianChannels;
pub use indicators::fractals::BillWilliamsFractals;
pub use indicators::heikin_ashi::HeikinAshi;
pub use indicators::hma::HMA;
pub use indicators::ichimoku::IchimokuCloud;
pub use indicators::keltner::KeltnerChannels;
pub use indicators::math::*;
pub use indicators::momentum::*;
pub use indicators::overlap::*;
pub use indicators::pattern::*;
pub use indicators::pivot_points::PivotPoints;
pub use indicators::price_transform::*;
pub use indicators::smoothing::{EMA, SMA, WMA};
pub use indicators::statistics::{
    LinearRegression, StandardDeviation, TaBETA, TaCORREL, TaLINEARREG, TaLINEARREG_ANGLE,
    TaLINEARREG_INTERCEPT, TaLINEARREG_SLOPE, TaSTDDEV, TaTSF, TaVAR,
};
pub use indicators::supertrend::SuperTrend;
pub use indicators::tema::{TEMA, ZLEMA};
pub use indicators::ttm_squeeze::TTMSqueeze;
pub use indicators::volatility::{ATR, TaATR, TaNATR, TaTRANGE, TrueRange};
pub use indicators::volume::*;
pub use indicators::vortex::VortexIndicator;
pub use indicators::vpn::VPNIndicator;
pub use indicators::gap_momentum::GapMomentum;
pub use indicators::autotune::AutoTuneFilter;
pub use indicators::adaptive_ema::AdaptiveEMA;
pub use indicators::tradj_ema::TRAdjEMA;
pub use indicators::obvm::Obvm;
pub use indicators::vfi::Vfi;
pub use indicators::sve_volatility_bands::SVEVolatilityBands;
pub use indicators::exp_dev_bands::ExpDevBands;
pub use indicators::sdo::SDO;
pub use indicators::rsmk::RSMK;
pub use indicators::rodc::RODC;
pub use indicators::reverse_ema::ReverseEMA;
pub use indicators::harrington_adx::HarringtonADXOscillator;
pub use indicators::vwap::AnchoredVWAP;
pub use indicators::wavetrend::WaveTrend;
pub use traits::{IndicatorConfig, Next, SmoothingAlgorithm};

/// Re-export talib-rs for convenience
pub use talib_rs as talib;
