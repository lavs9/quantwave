//! # quantwave-core
//!
//! Core technical-analysis engine for [QuantWave](https://lavs9.github.io/quantwave/):
//! **221** native indicators, Ehlers DSP, price-action detectors, regime features,
//! and the [`Next<Input>`](traits::Next) streaming trait that powers batch/streaming parity.
//!
//! ## Quick start (streaming)
//!
//! ```rust
//! use quantwave_core::indicators::RSI;
//! use quantwave_core::traits::Next;
//!
//! let mut rsi = RSI::new(14);
//! let value = rsi.next(44.5);
//! ```
//!
//! ## Architecture
//!
//! - **Indicators** — `indicators` module; each implements `Next` for scalar streaming
//! - **Batch** — consumed by `quantwave-polars` / `quantwave-plugins` expression plugins
//! - **Features / regimes** — ML-oriented transforms in `features` and `regimes`
//!
//! User guides: <https://lavs9.github.io/quantwave/guides/rust/>
//! Full API: <https://docs.rs/quantwave-core>

pub mod features;
pub mod indicators;
pub mod options_india;
pub mod portfolio;
pub mod regimes;
pub mod streaming;
pub mod traits;
pub mod utils;

pub use regimes::analytics::*;

#[cfg(any(test, feature = "test_utils"))]
pub mod test_utils;

pub use indicators::adaptive_ema::AdaptiveEMA;
pub use indicators::alma::ALMA;
pub use indicators::atr_ts::ATRTrailingStop;
pub use indicators::autotune::AutoTuneFilter;
pub use indicators::cycle::*;
pub use indicators::donchian::DonchianChannels;
pub use indicators::exp_dev_bands::ExpDevBands;
pub use indicators::frac_diff::FracDiff;
pub use indicators::fractals::BillWilliamsFractals;
pub use indicators::gap_momentum::GapMomentum;
pub use indicators::geometric_patterns::{FlagPattern, GeometricPatternScanner, HsPattern};
pub use indicators::harmonic::{
    HarmonicConfig, HarmonicKind, HarmonicPattern, HarmonicPatternScanner, harmonic_patterns_batch,
};
pub use indicators::harrington_adx::HarringtonADXOscillator;
pub use indicators::heikin_ashi::HeikinAshi;
pub use indicators::hma::HMA;
pub use indicators::ichimoku::IchimokuCloud;
pub use indicators::kagi::{KagiBuilder, KagiLine, kagi_atr_batch, kagi_batch};
pub use indicators::keltner::KeltnerChannels;
pub use indicators::market_structure::{
    Bias, FlipEvent, MarketStructure, MarketStructureState, PAEvent, PAEventKind, SwingPoint,
    extract_all_pa_events, extract_pa_events,
};
pub use indicators::math::*;
pub use indicators::momentum::*;
pub use indicators::obvm::Obvm;
pub use indicators::overlap::*;
pub use indicators::pa_confluence::{
    ConfluenceContext, enrich_pa_event, filter_confluent_events, passes_confluence_filter,
    regime_to_label, score_pa_event,
};
pub use indicators::pattern::*;
pub use indicators::pivot_points::PivotPoints;
pub use indicators::point_figure::{
    PointFigureBuilder, PointFigureColumn, point_figure_atr_batch, point_figure_batch,
};
pub use indicators::price_transform::*;
pub use indicators::range_bars::{
    RangeBar, RangeBarBuilder, range_bars_atr_batch, range_bars_batch,
};
pub use indicators::renko::{RenkoBrick, RenkoBuilder, renko_atr_batch, renko_batch};
pub use indicators::reverse_ema::ReverseEMA;
pub use indicators::rodc::RODC;
pub use indicators::rsmk::RSMK;
pub use indicators::sdo::SDO;
pub use indicators::smoothing::{EMA, SMA, WMA};
pub use indicators::sr_monitor::{
    LevelSource, SR_INTERACTION_MONITOR_METADATA, SRInteraction, SRInteractionMonitor,
    SRInteractionType, SRMonitorOutput,
};
pub use indicators::statistics::{
    LinearRegression, StandardDeviation, TaBETA, TaCORREL, TaLINEARREG, TaLINEARREG_ANGLE,
    TaLINEARREG_INTERCEPT, TaLINEARREG_SLOPE, TaSTDDEV, TaTSF, TaVAR,
};
pub use indicators::supertrend::SuperTrend;
pub use indicators::sve_volatility_bands::SVEVolatilityBands;
pub use indicators::tema::{TEMA, ZLEMA};
pub use indicators::tradj_ema::TRAdjEMA;
pub use indicators::ttm_squeeze::TTMSqueeze;
pub use indicators::vfi::Vfi;
pub use indicators::volatility::{ATR, TaATR, TaNATR, TaTRANGE, TrueRange};
pub use indicators::volume::*;
pub use indicators::vortex::VortexIndicator;
pub use indicators::vpn::VPNIndicator;
pub use indicators::vwap::AnchoredVWAP;
pub use indicators::wavetrend::WaveTrend;
pub use streaming::{StreamingReadiness, TrackedNext, track, warmup_from_params};
pub use traits::{IndicatorConfig, Next, SmoothingAlgorithm};

/// Re-export talib-rs for convenience
pub use talib_rs as talib;
