use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use quantwave_core::indicators::alligator::Alligator as CoreAlligator;
use quantwave_core::indicators::alma::ALMA as CoreALMA;
use quantwave_core::indicators::amfm::{
    AMDetector as CoreAMDetector, FMDemodulator as CoreFMDemodulator,
};
use quantwave_core::indicators::atr_ts::ATRTrailingStop as CoreAtrTs;
use quantwave_core::indicators::bandpass::BandPass as CoreBandpass;
use quantwave_core::indicators::butterworth::{
    Butterworth2 as CoreButterworth2, Butterworth3 as CoreButterworth3,
};
use quantwave_core::indicators::cg::CenterOfGravity as CoreCG;
use quantwave_core::indicators::channel_cycle::ChannelCycle as CoreChannelCycle;
use quantwave_core::indicators::choppiness_index::ChoppinessIndex as CoreChoppinessIndex;
use quantwave_core::indicators::classic_laguerre::ClassicLaguerre as CoreClassicLaguerre;
use quantwave_core::indicators::continuation_index::ContinuationIndex as CoreContinuationIndex;
use quantwave_core::indicators::correlation_cycle::CorrelationCycle as CoreCorrelationCycle;
use quantwave_core::indicators::correlation_trend::CorrelationTrend as CoreCorrelationTrend;
use quantwave_core::indicators::cyber_cycle::CyberCycle as CoreCyberCycle;
use quantwave_core::indicators::cybernetic_oscillator::CyberneticOscillator as CoreCyberneticOscillator;
use quantwave_core::indicators::cycle::{
    HT_DCPERIOD, HT_DCPHASE, HT_PHASOR, HT_SINE, HT_TRENDMODE,
};
use quantwave_core::indicators::cycle_trend_analytics::CycleTrendAnalytics as CoreCycleTrendAnalytics;
use quantwave_core::indicators::dmh::DMH as CoreDMH;
use quantwave_core::indicators::donchian::DonchianChannels as CoreDonchian;
use quantwave_core::indicators::dsma::DSMA as CoreDSMA;
use quantwave_core::indicators::ehlers_autocorrelation::EhlersAutocorrelation as CoreEhlersAutocorrelation;
use quantwave_core::indicators::ehlers_filter::EhlersFilter as CoreEhlersFilter;
use quantwave_core::indicators::ehlers_loops::EhlersLoops as CoreEhlersLoops;
use quantwave_core::indicators::ehlers_stochastic::EhlersStochastic as CoreEhlersStochastic;
use quantwave_core::indicators::ehlers_ultimate_oscillator::EhlersUltimateOscillator as CoreEhlersUltimateOscillator;
use quantwave_core::indicators::emd::EMD as CoreEMD;
use quantwave_core::indicators::fisher::FisherTransform as CoreFisher;
use quantwave_core::indicators::fisher_high_pass::FisherHighPass as CoreFisherHighPass;
use quantwave_core::indicators::fourier_series::FourierSeriesModel as CoreFourierSeries;
use quantwave_core::indicators::fourier_transform::FourierDominantCycle as CoreFourierDominantCycle;
use quantwave_core::indicators::frac_diff::FracDiff as CoreFracDiff;
use quantwave_core::indicators::fractals::BillWilliamsFractals as CoreFractals;
use quantwave_core::indicators::frama::FRAMA as CoreFRAMA;
use quantwave_core::indicators::gaussian::GaussianFilter as CoreGaussian;
use quantwave_core::indicators::generalized_laguerre::GeneralizedLaguerre as CoreGeneralizedLaguerre;
use quantwave_core::indicators::griffiths_dominant_cycle::GriffithsDominantCycle as CoreGriffithsDominantCycle;
use quantwave_core::indicators::griffiths_predictor::GriffithsPredictor as CoreGriffithsPredictor;
use quantwave_core::indicators::griffiths_spectrum::GriffithsSpectrum as CoreGriffithsSpectrum;
use quantwave_core::indicators::hamming::HammingFilter as CoreHamming;
use quantwave_core::indicators::hann::HannFilter as CoreHann;
use quantwave_core::indicators::harrington_adx::HarringtonADXOscillator as CoreHarringtonAdx;
use quantwave_core::indicators::heikin_ashi::HeikinAshi as CoreHeikinAshi;
use quantwave_core::indicators::high_pass::HighPass as CoreHighPass;
use quantwave_core::indicators::hma::HMA as CoreHMA;
use quantwave_core::indicators::homodyne_discriminator::HomodyneDiscriminator as CoreHomodyneDiscriminator;
use quantwave_core::indicators::hurst::HurstExponent as CoreHurstExponent;
use quantwave_core::indicators::ichimoku::IchimokuCloud as CoreIchimoku;
use quantwave_core::indicators::incremental::ta_atr::TaATR as CoreTaATR;
use quantwave_core::indicators::instantaneous_trendline::InstantaneousTrendline as CoreInstantaneousTrendline;
use quantwave_core::indicators::inverse_fisher::InverseFisherTransform as CoreInverseFisher;
use quantwave_core::indicators::kalman::KalmanFilter as CoreKalmanFilter;
use quantwave_core::indicators::keltner::KeltnerChannels as CoreKeltner;
use quantwave_core::indicators::laguerre_filter::LaguerreFilter as CoreLaguerreFilter;
use quantwave_core::indicators::laguerre_oscillator::LaguerreOscillator as CoreLaguerreOscillator;
use quantwave_core::indicators::laguerre_rsi::LaguerreRSI as CoreLaguerreRSI;
use quantwave_core::indicators::mad::MAD as CoreMAD;
use quantwave_core::indicators::madh::MADH as CoreMADH;
use quantwave_core::indicators::market_state::MarketState as CoreMarketState;
use quantwave_core::indicators::mesa_stochastic::MESAStochastic as CoreMesaStochastic;
use quantwave_core::indicators::momentum::*;
use quantwave_core::indicators::noise_elimination::NoiseElimination as CoreNoiseElimination;
use quantwave_core::indicators::oc_price_rsi::OCPriceRSI as CoreOcPriceRSI;
use quantwave_core::indicators::one_euro_filter::OneEuroFilter as CoreOneEuroFilter;
use quantwave_core::indicators::overlap::{DEMA, KAMA, MAMA, SAR, T3 as CoreT3};
use quantwave_core::indicators::pairs_rotation::PairsRotation as CorePairsRotation;
use quantwave_core::indicators::phasor::Phasor as CorePhasor;
use quantwave_core::indicators::pivot_points::PivotPoints as CorePivotPoints;
use quantwave_core::indicators::pma::ProjectedMovingAverage as CoreProjectedMovingAverage;
use quantwave_core::indicators::precision_trend::PrecisionTrendAnalysis as CorePrecisionTrend;
use quantwave_core::indicators::recursive_median::{
    RecursiveMedian as CoreRecursiveMedian, RecursiveMedianOscillator as CoreRMO,
};
use quantwave_core::indicators::reflex::Reflex as CoreReflex;
use quantwave_core::indicators::reversion_index::ReversionIndex as CoreReversionIndex;
use quantwave_core::indicators::robustness::RobustnessEvaluator as CoreRobustnessEvaluator;
use quantwave_core::indicators::rocket_rsi::RocketRSI as CoreRocketRSI;
use quantwave_core::indicators::roofing_filter::RoofingFilter as CoreRoofingFilter;
use quantwave_core::indicators::rsih::RSIH as CoreRSIH;
use quantwave_core::indicators::simple_predictor::SimplePredictor as CoreSimplePredictor;
use quantwave_core::indicators::sine_wave::SineWave as CoreSineWave;
use quantwave_core::indicators::smoothing::{
    EMA as CoreEMA, SMA as CoreSMA, WMA as CoreWMA, WarmupPolicy,
};
use quantwave_core::indicators::stc::SchaffTrendCycle as CoreSTC;
use quantwave_core::indicators::super_smoother::SuperSmoother as CoreSuperSmoother;
use quantwave_core::indicators::supertrend::{
    SuperTrend as CoreSuperTrend, SuperTrendAtrSmoothing, SuperTrendInitialDirection,
};
use quantwave_core::indicators::swiss_army_knife::{
    SwissArmyKnife as CoreSwissArmyKnife, SwissArmyKnifeMode,
};
use quantwave_core::indicators::synthetic_oscillator::SyntheticOscillator as CoreSyntheticOscillator;
use quantwave_core::indicators::system_evaluator::SystemEvaluator as CoreSystemEvaluator;
use quantwave_core::indicators::tema::*;
use quantwave_core::indicators::trendflex::Trendflex as CoreTrendflex;
use quantwave_core::indicators::triangle::TriangleFilter as CoreTriangleFilter;
use quantwave_core::indicators::truncated_bandpass::TruncatedBandpass as CoreTruncatedBandpass;
use quantwave_core::indicators::ttm_squeeze::TTMSqueeze as CoreTtmSqueeze;
use quantwave_core::indicators::ultimate_bands::UltimateBands as CoreUltimateBands;
use quantwave_core::indicators::ultimate_channel::UltimateChannel as CoreUltimateChannel;
use quantwave_core::indicators::ultimate_smoother::UltimateSmoother as CoreUltimateSmoother;
use quantwave_core::indicators::universal_oscillator::UniversalOscillator as CoreUniversalOscillator;
use quantwave_core::indicators::usi::USI as CoreUSI;
use quantwave_core::indicators::volatility::ATR as CoreATR;
use quantwave_core::indicators::volume::{AD as CoreAD, ADOSC as CoreADOSC, OBV as CoreOBV};
use quantwave_core::indicators::vortex::VortexIndicator as CoreVortex;
use quantwave_core::indicators::voss_predictor::VossPredictor as CoreVossPredictor;
use quantwave_core::indicators::vwap::AnchoredVWAP as CoreAnchoredVWAP;
use quantwave_core::indicators::wavetrend::WaveTrend as CoreWaveTrend;
use quantwave_core::indicators::zero_lag::ZeroLag as CoreZeroLag;
use quantwave_core::traits::Next;

// -- quantwave-lt3t: streaming-class PyO3 binding gap fixes (batch/streaming parity) --
use quantwave_core::indicators::adaptive_ema::AdaptiveEMA as CoreAdaptiveEma;
use quantwave_core::indicators::autotune::AutoTuneFilter as CoreAutotuneFilter;
use quantwave_core::indicators::gap_momentum::GapMomentum as CoreGapMomentum;
use quantwave_core::indicators::incremental::apo::{APO as CoreApo, PPO as CorePpo};
use quantwave_core::indicators::incremental::cmo::CMO as CoreCmo;
use quantwave_core::indicators::incremental::overlap_ta::TRIMA as CoreTrima;
use quantwave_core::indicators::incremental::price_transform::{
    AVGPRICE as CoreAvgprice, MEDPRICE as CoreMedprice, TYPPRICE as CoreTypprice,
    WCLPRICE as CoreWclprice,
};
use quantwave_core::indicators::incremental::rolling::{MAX as CoreMax, MIN as CoreMin};
use quantwave_core::indicators::incremental::simple::MFI as CoreMfi;
use quantwave_core::indicators::incremental::statistics_ta::{
    TaBETA as CoreBeta, TaCORREL as CoreCorrel, TaLINEARREG as CoreLinreg, TaSTDDEV as CoreStddev,
};
use quantwave_core::indicators::incremental::trange::TaNATR as CoreNatr;
use quantwave_core::indicators::incremental::trix::TRIX as CoreTrix;
use quantwave_core::indicators::incremental::ultosc::ULTOSC as CoreUltosc;
use quantwave_core::indicators::kinematic_kalman::KinematicKalmanFilter as CoreKinematicKalman;
use quantwave_core::indicators::price_transform::OC2 as CoreOc2;
use quantwave_core::indicators::reverse_ema::ReverseEMA as CoreReverseEma;
use quantwave_core::indicators::sdo::SDO as CoreSdo;
use quantwave_core::indicators::tema::ZLEMA as CoreZlema;
use quantwave_core::indicators::tradj_ema::TRAdjEMA as CoreTradjEma;
use quantwave_core::indicators::volatility::TrueRange as CoreTrueRange;
use quantwave_core::indicators::vpn::VPNIndicator as CoreVpn;
use quantwave_core::{
    CDL2CROWS, CDL3BLACKCROWS, CDL3INSIDE, CDL3LINESTRIKE, CDL3OUTSIDE, CDL3STARSINSOUTH,
    CDL3WHITESOLDIERS, CDLABANDONEDBABY, CDLADVANCEBLOCK, CDLBELTHOLD, CDLBREAKAWAY,
    CDLCLOSINGMARUBOZU, CDLCONCEALBABYSWALL, CDLCOUNTERATTACK, CDLDARKCLOUDCOVER, CDLDOJI,
    CDLDOJISTAR, CDLDRAGONFLYDOJI, CDLENGULFING, CDLEVENINGDOJISTAR, CDLEVENINGSTAR,
    CDLGAPSIDESIDEWHITE, CDLGRAVESTONEDOJI, CDLHAMMER, CDLHANGINGMAN, CDLHARAMI, CDLHARAMICROSS,
    CDLHIGHWAVE, CDLHIKKAKE, CDLHIKKAKEMOD, CDLHOMINGPIGEON, CDLIDENTICAL3CROWS, CDLINNECK,
    CDLINVERTEDHAMMER, CDLKICKING, CDLKICKINGBYLENGTH, CDLLADDERBOTTOM, CDLLONGLEGGEDDOJI,
    CDLLONGLINE, CDLMARUBOZU, CDLMATCHINGLOW, CDLMATHOLD, CDLMORNINGDOJISTAR, CDLMORNINGSTAR,
    CDLONNECK, CDLPIERCING, CDLRICKSHAWMAN, CDLRISEFALL3METHODS, CDLSEPARATINGLINES,
    CDLSHOOTINGSTAR, CDLSHORTLINE, CDLSPINNINGTOP, CDLSTALLEDPATTERN, CDLSTICKSANDWICH, CDLTAKURI,
    CDLTASUKIGAP, CDLTHRUSTING, CDLTRISTAR, CDLUNIQUE3RIVER, CDLUPSIDEGAP2CROWS,
    CDLXSIDEGAP3METHODS,
};

// ML Feature extractors (for quantwave-gw7s canonical notebook + validation)
use quantwave_core::features::cyber_cycle::CyberCycleFeatureExtractor as CoreCyberCycleFE;
use quantwave_core::features::griffiths_dominant_cycle::GriffithsDominantCycleFeatureExtractor as CoreGriffithsDCFE;
use quantwave_core::features::hurst::HurstFeatureExtractor as CoreHurstFE;
use quantwave_core::features::instantaneous_trendline::InstantaneousTrendlineFeatureExtractor as CoreITFE;
use quantwave_core::features::regime::regime_to_features as core_regime_to_features;
use quantwave_core::features::trendflex::TrendflexFeatureExtractor as CoreTrendflexFE;
use quantwave_core::indicators::hilbert_transform::EhlersWma4 as CoreEhlersWma4;
use quantwave_core::indicators::just_ignore_them::UndersampledDoubleMA as CoreUDMA;
use quantwave_core::indicators::volume_profile::VolumeProfile as CoreVolumeProfile;
use quantwave_core::options_india;
use quantwave_core::regimes::MarketRegime;
use quantwave_core::regimes::gaussian_hmm::{
    EmissionFamily as CoreEmissionFamily, GaussianHmmFilter as CoreGaussianHmmFilter,
    GaussianHmmFitConfig as CoreGaussianHmmFitConfig, GaussianHmmParams as CoreGaussianHmmParams,
    fit_em as core_fit_em,
};
use quantwave_core::regimes::hmm::HMM as CoreHMM;
use quantwave_core::regimes::hmm_forecast::{
    forecast_state as core_forecast_state, forecast_volatility as core_forecast_volatility,
};

use paste::paste;
use std::sync::Mutex;

pub trait ValidateParam {
    fn validate(&self, name: &str) -> PyResult<()>;
}

impl ValidateParam for i64 {
    fn validate(&self, name: &str) -> PyResult<()> {
        if *self < 1 {
            return Err(PyValueError::new_err(format!(
                "parameter '{name}' must be >= 1, got {self}"
            )));
        }
        Ok(())
    }
}

impl ValidateParam for u64 {
    fn validate(&self, name: &str) -> PyResult<()> {
        if *self < 1 {
            return Err(PyValueError::new_err(format!(
                "parameter '{name}' must be >= 1, got {self}"
            )));
        }
        Ok(())
    }
}

impl ValidateParam for usize {
    fn validate(&self, name: &str) -> PyResult<()> {
        if *self < 1 {
            return Err(PyValueError::new_err(format!(
                "parameter '{name}' must be >= 1, got {self}"
            )));
        }
        Ok(())
    }
}

impl ValidateParam for f64 {
    fn validate(&self, name: &str) -> PyResult<()> {
        if self.is_nan() {
            return Err(PyValueError::new_err(format!(
                "parameter '{name}' must not be NaN"
            )));
        }
        Ok(())
    }
}

#[inline]
fn lock_inner<T>(mutex: &Mutex<T>) -> PyResult<std::sync::MutexGuard<'_, T>> {
    mutex
        .lock()
        .map_err(|_| PyRuntimeError::new_err("mutex lock poisoned"))
}
// --- Records ---

#[pyclass(get_all)]
#[derive(Clone)]
pub struct SuperTrendResult {
    pub value: f64,
    pub direction: i8,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct TtmSqueezeResult {
    pub value: f64,
    pub direction: i8,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct MacdResult {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct BbandsResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct StochResult {
    pub k: f64,
    pub d: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct MamaResult {
    pub mama: f64,
    pub fama: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct AroonResult {
    pub up: f64,
    pub down: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct IchimokuResult {
    pub tenkan: f64,
    pub kijun: f64,
    pub senkou_a: f64,
    pub senkou_b: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct AlligatorResult {
    pub jaw: f64,
    pub teeth: f64,
    pub lips: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct AtrTsResult {
    pub stop: f64,
    pub direction: i8,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct DonchianResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct EmdResult {
    pub trend: f64,
    pub upper: f64,
    pub lower: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct EhlersLoopsResult {
    pub price_rms: f64,
    pub vol_rms: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct FractalsResult {
    pub bearish: bool,
    pub bullish: bool,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct HeikinAshiResult {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

// --- PA / MarketStructure + Geometric foundation (quantwave-5thj) ---
#[pyclass(get_all)]
#[derive(Clone)]
pub struct SwingPointResult {
    pub bar: u64,
    pub price: f64,
    pub is_high: bool,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct FlipEventResult {
    pub is_bearish: bool,
    pub price: f64,
    pub bar: u64,
    pub structure_strength: u32,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct MarketStructureStateResult {
    pub bias: u32, // 0=Neutral, 1=Bullish, 2=Bearish
    pub last_swing_high: Option<SwingPointResult>,
    pub last_swing_low: Option<SwingPointResult>,
    pub current_flip: Option<FlipEventResult>,
    pub swing_depth_used: u32,
    pub bar_index: u64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct FlagPatternResult {
    pub id: u32,
    pub is_bull: bool,
    pub pole_start_bar: u64,
    pub pole_end_bar: u64,
    pub flag_start_bar: u64,
    pub flag_end_bar: u64,
    pub pole_length: f64,
    pub pole_length_atr: f64,
    pub breakout_confirmed: bool,
    pub breakout_price: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct HsPatternResult {
    pub id: u32,
    pub is_bearish: bool,
    pub height: f64,
    pub height_atr: f64,
    pub score: f64,
    pub breakout_confirmed: bool,
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct GeometricNextResult {
    pub market_structure: MarketStructureStateResult,
    pub flag: Option<FlagPatternResult>,
    pub hs: Option<HsPatternResult>,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct KeltnerResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct PairsRotationResult {
    pub ratio: f64,
    pub angle: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct PhasorResult {
    pub in_phase: f64,
    pub quadrature: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct PivotPointsResult {
    pub p: f64,
    pub r1: f64,
    pub s1: f64,
    pub r2: f64,
    pub s2: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct SystemEvaluatorResult {
    pub average_win_loss_ratio: f64,
    pub average_trade: f64,
    pub profit_factor: f64,
    pub percent_winners: f64,
    pub breakeven_profit_factor: f64,
    pub weighted_average_trade: f64,
    pub theoretical_consecutive_losers: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct UltimateBandsResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct UltimateChannelResult {
    pub upper: f64,
    pub center: f64,
    pub lower: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct VortexResult {
    pub plus: f64,
    pub minus: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct WaveTrendResult {
    pub wt1: f64,
    pub wt2: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct VossPredictorResult {
    pub filt: f64,
    pub voss: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct CycleTrendAnalyticsResult {
    pub cycle: f64,
    pub trend: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct ZeroLagResult {
    pub value: f64,
    pub trigger: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct CyberCycleResult {
    pub value: f64,
    pub trigger: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct HtSineResult {
    pub sine: f64,
    pub leadsine: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct VolumeProfileResult {
    pub poc: f64,
    pub vah: f64,
    pub val: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct PmaResult {
    pub pma: f64,
    pub predict: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct TrendRocResult {
    pub trend: f64,
    pub roc: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct UdmaResult {
    pub fast: f64,
    pub slow: f64,
}
// name = "OiZonesResult" preserves the uniffi-era Python symbol (uniffi re-cased the
// `OI` capital-run via heck).
#[pyclass(name = "OiZonesResult", get_all)]
#[derive(Clone)]
pub struct OIZonesResult {
    pub resistance_strikes: Vec<f64>,
    pub support_strikes: Vec<f64>,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct GexResult {
    pub ce_gex: f64,
    pub pe_gex: f64,
    pub net_gex: f64,
}
#[pyclass(get_all)]
#[derive(Clone)]
pub struct StraddleResult {
    pub atm_strike: f64,
    pub straddle_premium: f64,
    pub implied_move_pct: f64,
}

#[pyclass(eq, eq_int)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SwissMode {
    EMA,
    SMA,
    Gauss,
    Butterworth,
    Smooth,
    HighPass,
    TwoPoleHighPass,
    BandPass,
    BandStop,
}
impl From<SwissMode> for SwissArmyKnifeMode {
    fn from(m: SwissMode) -> Self {
        match m {
            SwissMode::EMA => Self::EMA,
            SwissMode::SMA => Self::SMA,
            SwissMode::Gauss => Self::Gauss,
            SwissMode::Butterworth => Self::Butterworth,
            SwissMode::Smooth => Self::Smooth,
            SwissMode::HighPass => Self::HighPass,
            SwissMode::TwoPoleHighPass => Self::TwoPoleHighPass,
            SwissMode::BandPass => Self::BandPass,
            SwissMode::BandStop => Self::BandStop,
        }
    }
}

// --- Macros ---

macro_rules! export_1_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* series: Vec<f64>) -> PyResult<Vec<f64>> {
                $( $param.validate(stringify!($param))?; )*
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(series.iter().map(|&x| indicator.next(x)).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, input: f64) -> PyResult<f64> {
                    let mut guard = lock_inner(&self.inner)?;
                    Ok(guard.next(input))
                }
            }
        }
    }
}

macro_rules! export_1_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* series: Vec<f64>) -> PyResult<Vec<$result_type>> {
                $( $param.validate(stringify!($param))?; )*
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(series.iter().map(|&x| { let $res_var = indicator.next(x); $body }).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, input: f64) -> PyResult<$result_type> { let mut guard = lock_inner(&self.inner)?; let $res_var = guard.next(input); Ok($body) }
            }
        }
    }
}

macro_rules! export_1_in_vec_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* series: Vec<f64>) -> PyResult<Vec<Vec<f64>>> {
                $( $param.validate(stringify!($param))?; )*
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(series.iter().map(|&x| indicator.next(x)).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, input: f64) -> PyResult<Vec<f64>> { let mut guard = lock_inner(&self.inner)?; Ok(guard.next(input)) }
            }
        }
    }
}

macro_rules! export_ohlc_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high, low, and close must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>, close: Vec<f64>) -> PyResult<Vec<f64>> {
                $( $param.validate(stringify!($param))?; )*
                if high.len() != low.len() || low.len() != close.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: high={}, low={}, close={}",
                        high.len(), low.len(), close.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| indicator.next((h, l, c))).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<f64> { let mut guard = lock_inner(&self.inner)?; Ok(guard.next((high, low, close))) }
            }
        }
    }
}

macro_rules! export_ohlc_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high, low, and close must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>, close: Vec<f64>) -> PyResult<Vec<$result_type>> {
                $( $param.validate(stringify!($param))?; )*
                if high.len() != low.len() || low.len() != close.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: high={}, low={}, close={}",
                        high.len(), low.len(), close.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| { let $res_var = indicator.next((h, l, c)); $body }).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<$result_type> { let mut guard = lock_inner(&self.inner)?; let $res_var = guard.next((high, low, close)); Ok($body) }
            }
        }
    }
}

macro_rules! export_hl_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high and low must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>) -> PyResult<Vec<f64>> {
                $( $param.validate(stringify!($param))?; )*
                if high.len() != low.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: high={}, low={}",
                        high.len(), low.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(high.iter().zip(low.iter()).map(|(&h, &l)| indicator.next((h, l))).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, high: f64, low: f64) -> PyResult<f64> { let mut guard = lock_inner(&self.inner)?; Ok(guard.next((high, low))) }
            }
        }
    }
}

macro_rules! export_hl_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high and low must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>) -> PyResult<Vec<$result_type>> {
                $( $param.validate(stringify!($param))?; )*
                if high.len() != low.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: high={}, low={}",
                        high.len(), low.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(high.iter().zip(low.iter()).map(|(&h, &l)| { let $res_var = indicator.next((h, l)); $body }).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, high: f64, low: f64) -> PyResult<$result_type> { let mut guard = lock_inner(&self.inner)?; let $res_var = guard.next((high, low)); Ok($body) }
            }
        }
    }
}

macro_rules! export_co_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: close and open must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* close: Vec<f64>, open: Vec<f64>) -> PyResult<Vec<f64>> {
                $( $param.validate(stringify!($param))?; )*
                if close.len() != open.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: close={}, open={}",
                        close.len(), open.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(close.iter().zip(open.iter()).map(|(&c, &o)| indicator.next((c, o))).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, close: f64, open: f64) -> PyResult<f64> { let mut guard = lock_inner(&self.inner)?; Ok(guard.next((close, open))) }
            }
        }
    }
}

macro_rules! export_co_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: close and open must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* close: Vec<f64>, open: Vec<f64>) -> PyResult<Vec<$result_type>> {
                $( $param.validate(stringify!($param))?; )*
                if close.len() != open.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: close={}, open={}",
                        close.len(), open.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(close.iter().zip(open.iter()).map(|(&c, &o)| { let $res_var = indicator.next((c, o)); $body }).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, close: f64, open: f64) -> PyResult<$result_type> { let mut guard = lock_inner(&self.inner)?; let $res_var = guard.next((close, open)); Ok($body) }
            }
        }
    }
}

macro_rules! export_pv_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: price and volume must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* price: Vec<f64>, volume: Vec<f64>) -> PyResult<Vec<f64>> {
                $( $param.validate(stringify!($param))?; )*
                if price.len() != volume.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: price={}, volume={}",
                        price.len(), volume.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(price.iter().zip(volume.iter()).map(|(&p, &v)| indicator.next((p, v))).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, price: f64, volume: f64) -> PyResult<f64> { let mut guard = lock_inner(&self.inner)?; Ok(guard.next((price, volume))) }
            }
        }
    }
}

macro_rules! export_pv_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: price and volume must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* price: Vec<f64>, volume: Vec<f64>) -> PyResult<Vec<$result_type>> {
                $( $param.validate(stringify!($param))?; )*
                if price.len() != volume.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: price={}, volume={}",
                        price.len(), volume.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(price.iter().zip(volume.iter()).map(|(&p, &v)| { let $res_var = indicator.next((p, v)); $body }).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, price: f64, volume: f64) -> PyResult<$result_type> { let mut guard = lock_inner(&self.inner)?; let $res_var = guard.next((price, volume)); Ok($body) }
            }
        }
    }
}

macro_rules! export_ohlc4_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: open, high, low, and close must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* open: Vec<f64>, high: Vec<f64>, low: Vec<f64>, close: Vec<f64>) -> PyResult<Vec<f64>> {
                $( $param.validate(stringify!($param))?; )*
                if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: open={}, high={}, low={}, close={}",
                        open.len(), high.len(), low.len(), close.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(open.iter().zip(high.iter()).zip(low.iter()).zip(close.iter()).map(|(((&o, &h), &l), &c)| indicator.next((o, h, l, c))).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, open: f64, high: f64, low: f64, close: f64) -> PyResult<f64> { let mut guard = lock_inner(&self.inner)?; Ok(guard.next((open, high, low, close))) }
            }
        }
    }
}

macro_rules! export_hlcv_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[doc = concat!("Calculates ", stringify!($name), ".\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high, low, close, and volume must have identical lengths, otherwise ValueError is raised.\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyfunction]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, volume: Vec<f64>) -> PyResult<Vec<f64>> {
                $( $param.validate(stringify!($param))?; )*
                if high.len() != low.len() || low.len() != close.len() || close.len() != volume.len() {
                    return Err(PyValueError::new_err(format!(
                        "input series lengths must match: high={}, low={}, close={}, volume={}",
                        high.len(), low.len(), close.len(), volume.len()
                    )));
                }
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                Ok(high.iter().zip(low.iter()).zip(close.iter()).zip(volume.iter()).map(|(((&h, &l), &c), &v)| indicator.next((h, l, c, v))).collect())
            }
            #[doc = concat!(stringify!($name), " streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\n- Negative Params: Negative period/length parameters will raise a ValueError.")]
            #[pyclass] pub struct $name { inner: Mutex<$core_type> }
            #[pymethods] impl $name {
                #[new] pub fn new($( $param: $param_type ),*) -> PyResult<Self> {
                    $( $param.validate(stringify!($param))?; )*
                    Ok(Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) })
                }
                pub fn next(&self, high: f64, low: f64, close: f64, volume: f64) -> PyResult<f64> { let mut guard = lock_inner(&self.inner)?; Ok(guard.next((high, low, close, volume))) }
            }
        }
    }
}

// --- Indicators ---

// --- Indicators ---

fn parse_warmup(warmup: Option<&str>) -> PyResult<WarmupPolicy> {
    match warmup {
        None | Some("partial") | Some("PARTIAL") => Ok(WarmupPolicy::Partial),
        Some("nan") | Some("NAN") => Ok(WarmupPolicy::Nan),
        Some(other) => Err(PyValueError::new_err(format!(
            "unknown warmup policy: '{other}' (expected 'partial' or 'nan')"
        ))),
    }
}

/// Simple Moving Average (SMA).
///
/// Parameters:
///   period: Window length in bars (must be >= 1).
///   series: Input price series.
///   warmup: Warm-up behavior for the initial `period - 1` bars:
///     - "partial" (default): Returns partial-window averages as values accumulate.
///     - "nan": Returns NaN until a full window of `period` bars has accumulated.
#[pyfunction]
#[pyo3(signature = (period, series, warmup=None))]
pub fn sma(period: i64, series: Vec<f64>, warmup: Option<&str>) -> PyResult<Vec<f64>> {
    period.validate("period")?;
    let policy = parse_warmup(warmup)?;
    let mut indicator = CoreSMA::with_warmup(period as usize, policy);
    Ok(series.iter().map(|&x| indicator.next(x)).collect())
}

#[doc = "Simple Moving Average (SMA) streaming indicator.\n\nParameters:\n  period: Window length in bars (must be >= 1).\n  warmup: 'partial' (default) or 'nan'."]
#[pyclass]
pub struct Sma {
    inner: Mutex<CoreSMA>,
}

#[pymethods]
impl Sma {
    #[new]
    #[pyo3(signature = (period, warmup=None))]
    pub fn new(period: i64, warmup: Option<&str>) -> PyResult<Self> {
        period.validate("period")?;
        let policy = parse_warmup(warmup)?;
        Ok(Self {
            inner: Mutex::new(CoreSMA::with_warmup(period as usize, policy)),
        })
    }

    pub fn next(&self, input: f64) -> PyResult<f64> {
        let mut guard = lock_inner(&self.inner)?;
        Ok(guard.next(input))
    }
}

/// Exponential Moving Average (EMA).
///
/// Calculation convention & NaN recovery:
/// - First value seeds the EMA directly (or SMA of first `period` bars when `warmup="nan"`).
/// - An isolated NaN input yields NaN for that bar without poisoning the internal EMA state.
///   Subsequent finite bars smoothly resume calculations.
///
/// Parameters:
///   period: Smoothing period (must be >= 1).
///   series: Input price series.
///   warmup: Warm-up behavior:
///     - "partial" (default): First value seeds EMA directly; outputs finite values from bar 0.
///     - "nan": Outputs NaN for first `period - 1` bars, then seeds with SMA of first `period` bars.
#[pyfunction]
#[pyo3(signature = (period, series, warmup=None))]
pub fn ema(period: i64, series: Vec<f64>, warmup: Option<&str>) -> PyResult<Vec<f64>> {
    period.validate("period")?;
    let policy = parse_warmup(warmup)?;
    let mut indicator = CoreEMA::with_warmup(period as usize, policy);
    Ok(series.iter().map(|&x| indicator.next(x)).collect())
}

#[doc = "Exponential Moving Average (EMA) streaming indicator.\n\nParameters:\n  period: Smoothing period (must be >= 1).\n  warmup: 'partial' (default) or 'nan'."]
#[pyclass]
pub struct Ema {
    inner: Mutex<CoreEMA>,
}

#[pymethods]
impl Ema {
    #[new]
    #[pyo3(signature = (period, warmup=None))]
    pub fn new(period: i64, warmup: Option<&str>) -> PyResult<Self> {
        period.validate("period")?;
        let policy = parse_warmup(warmup)?;
        Ok(Self {
            inner: Mutex::new(CoreEMA::with_warmup(period as usize, policy)),
        })
    }

    pub fn next(&self, input: f64) -> PyResult<f64> {
        let mut guard = lock_inner(&self.inner)?;
        Ok(guard.next(input))
    }
}

fn parse_supertrend_options(
    initial_direction: Option<&str>,
    smoothing: Option<&str>,
) -> PyResult<(SuperTrendInitialDirection, SuperTrendAtrSmoothing)> {
    let init_dir = match initial_direction {
        None | Some("up") | Some("UP") => SuperTrendInitialDirection::Up,
        Some("down") | Some("DOWN") => SuperTrendInitialDirection::Down,
        Some("from_close") | Some("FROM_CLOSE") | Some("fromclose") => {
            SuperTrendInitialDirection::FromClose
        }
        Some(other) => {
            return Err(PyValueError::new_err(format!(
                "unknown initial_direction: '{other}' (expected 'up', 'down', or 'from_close')"
            )));
        }
    };
    let sm = match smoothing {
        None | Some("ema") | Some("EMA") => SuperTrendAtrSmoothing::Ema,
        Some("wilder") | Some("WILDER") | Some("rma") | Some("RMA") => {
            SuperTrendAtrSmoothing::Wilder
        }
        Some(other) => {
            return Err(PyValueError::new_err(format!(
                "unknown smoothing: '{other}' (expected 'ema' or 'wilder')"
            )));
        }
    };
    Ok((init_dir, sm))
}

/// SuperTrend Trend-Following Indicator.
///
/// Note on smoothing convention:
///   SuperTrend's internal ATR defaults to EMA smoothing (`smoothing="ema"`).
///   In contrast, standalone `atr()` defaults to Wilder's RMA (`smoothing="wilder"`).
///   Pass `smoothing="wilder"` to use Wilder's RMA inside SuperTrend.
///
/// Parameters:
///   period: ATR period (must be >= 1).
///   multiplier: ATR multiplier (e.g. 3.0).
///   high: High price series.
///   low: Low price series.
///   close: Close price series.
///   initial_direction: Direction prior to first trend reversal:
///     - "up" (default): Assumes uptrend (+1) from start (legacy quantwave behavior).
///     - "down": Assumes downtrend (-1) from start; first breakout above band flips to +1.
///     - "from_close": Sets direction based on close vs midpoint of first bar.
///   smoothing: ATR smoothing algorithm:
///     - "ema" (default): EMA smoothing of true range.
///     - "wilder": Wilder's RMA smoothing with SMA seed (emits NaN during warm-up).
#[pyfunction]
#[pyo3(signature = (period, multiplier, high, low, close, initial_direction=None, smoothing=None))]
pub fn supertrend(
    period: i64,
    multiplier: f64,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    initial_direction: Option<&str>,
    smoothing: Option<&str>,
) -> PyResult<Vec<SuperTrendResult>> {
    period.validate("period")?;
    multiplier.validate("multiplier")?;
    if high.len() != low.len() || low.len() != close.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: high={}, low={}, close={}",
            high.len(),
            low.len(),
            close.len()
        )));
    }
    let (init_dir, sm) = parse_supertrend_options(initial_direction, smoothing)?;
    let mut indicator = CoreSuperTrend::with_options(period as usize, multiplier, init_dir, sm);
    Ok(high
        .iter()
        .zip(low.iter())
        .zip(close.iter())
        .map(|((&h, &l), &c)| {
            let res = indicator.next((h, l, c));
            SuperTrendResult {
                value: res.0,
                direction: res.1,
            }
        })
        .collect())
}

#[doc = "SuperTrend streaming indicator.\n\nNote: SuperTrend defaults to EMA-smoothed ATR (`smoothing=\"ema\"`), whereas standalone `Atr` defaults to Wilder's RMA (`smoothing=\"wilder\"`).\n\nParameters:\n  period: ATR period (must be >= 1).\n  multiplier: ATR multiplier (e.g. 3.0).\n  initial_direction: 'up' (default), 'down', or 'from_close'.\n  smoothing: 'ema' (default) or 'wilder'."]
#[pyclass]
pub struct SuperTrend {
    inner: Mutex<CoreSuperTrend>,
}

#[pymethods]
impl SuperTrend {
    #[new]
    #[pyo3(signature = (period, multiplier, initial_direction=None, smoothing=None))]
    pub fn new(
        period: i64,
        multiplier: f64,
        initial_direction: Option<&str>,
        smoothing: Option<&str>,
    ) -> PyResult<Self> {
        period.validate("period")?;
        multiplier.validate("multiplier")?;
        let (init_dir, sm) = parse_supertrend_options(initial_direction, smoothing)?;
        Ok(Self {
            inner: Mutex::new(CoreSuperTrend::with_options(
                period as usize,
                multiplier,
                init_dir,
                sm,
            )),
        })
    }

    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<SuperTrendResult> {
        let mut guard = lock_inner(&self.inner)?;
        let res = guard.next((high, low, close));
        Ok(SuperTrendResult {
            value: res.0,
            direction: res.1,
        })
    }
}

/// Average True Range (ATR) with EMA smoothing.
///
/// NOTE: This function computes ATR using an Exponential Moving Average (EMA) of true range.
/// For Wilder's RMA smoothing (the TA-Lib and Pine Script convention), use `atr_wilder` or `ta_atr`.
/// Average True Range (ATR) with Wilder RMA smoothing (default, TA-Lib standard) or EMA smoothing.
///
/// Note on smoothing convention:
///   Standalone `atr()` defaults to Wilder's RMA (`smoothing="wilder"`).
///   In contrast, `supertrend()`'s internal ATR defaults to EMA smoothing (`smoothing="ema"`).
///   Pass `smoothing="ema"` if EMA smoothing of true range is desired.
///
/// Parameters:
///   period: Window period (must be >= 1).
///   high: High price series.
///   low: Low price series.
///   close: Close price series.
///   smoothing: 'wilder' (default, TA-Lib standard) or 'ema'.
#[pyfunction]
#[pyo3(signature = (period, high, low, close, smoothing=None))]
pub fn atr(
    period: i64,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    smoothing: Option<&str>,
) -> PyResult<Vec<f64>> {
    period.validate("period")?;
    if high.len() != low.len() || low.len() != close.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: high={}, low={}, close={}",
            high.len(),
            low.len(),
            close.len()
        )));
    }
    let is_ema = match smoothing {
        None | Some("wilder") | Some("WILDER") | Some("rma") | Some("RMA") => false,
        Some("ema") | Some("EMA") => true,
        Some(other) => {
            return Err(PyValueError::new_err(format!(
                "unrecognized smoothing: expected 'wilder' or 'ema', got '{other}'"
            )));
        }
    };
    if is_ema {
        let mut indicator = CoreATR::new(period as usize);
        Ok(high
            .iter()
            .zip(low.iter())
            .zip(close.iter())
            .map(|((&h, &l), &c)| indicator.next((h, l, c)))
            .collect())
    } else {
        let mut indicator = CoreTaATR::new(period as usize);
        Ok(high
            .iter()
            .zip(low.iter())
            .zip(close.iter())
            .map(|((&h, &l), &c)| indicator.next((h, l, c)))
            .collect())
    }
}

enum AtrInner {
    Wilder(CoreTaATR),
    Ema(CoreATR),
}

#[doc = "Average True Range (ATR) streaming indicator with Wilder RMA smoothing (default) or EMA smoothing.\n\nNote: Standalone `Atr` defaults to Wilder's RMA (`smoothing=\"wilder\"`), whereas `SuperTrend` defaults to EMA-smoothed ATR (`smoothing=\"ema\"`).\n\nParameters:\n  period: Window period (must be >= 1).\n  smoothing: 'wilder' (default, TA-Lib standard) or 'ema'."]
#[pyclass]
pub struct Atr {
    inner: Mutex<AtrInner>,
}

#[pymethods]
impl Atr {
    #[new]
    #[pyo3(signature = (period, smoothing=None))]
    pub fn new(period: i64, smoothing: Option<&str>) -> PyResult<Self> {
        period.validate("period")?;
        let is_ema = match smoothing {
            None | Some("wilder") | Some("WILDER") | Some("rma") | Some("RMA") => false,
            Some("ema") | Some("EMA") => true,
            Some(other) => {
                return Err(PyValueError::new_err(format!(
                    "unrecognized smoothing: expected 'wilder' or 'ema', got '{other}'"
                )));
            }
        };
        let inner = if is_ema {
            AtrInner::Ema(CoreATR::new(period as usize))
        } else {
            AtrInner::Wilder(CoreTaATR::new(period as usize))
        };
        Ok(Self {
            inner: Mutex::new(inner),
        })
    }

    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<f64> {
        let mut guard = lock_inner(&self.inner)?;
        match &mut *guard {
            AtrInner::Wilder(ind) => Ok(ind.next((high, low, close))),
            AtrInner::Ema(ind) => Ok(ind.next((high, low, close))),
        }
    }
}

/// Average True Range (Wilder RMA smoothing, TA-Lib standard).
///
/// Parameters:
///   timeperiod: Window period (must be >= 1).
///   high: High price series.
///   low: Low price series.
///   close: Close price series.
#[pyfunction]
pub fn atr_wilder(
    timeperiod: i64,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
) -> PyResult<Vec<f64>> {
    timeperiod.validate("timeperiod")?;
    if high.len() != low.len() || low.len() != close.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: high={}, low={}, close={}",
            high.len(),
            low.len(),
            close.len()
        )));
    }
    let mut indicator = CoreTaATR::new(timeperiod as usize);
    Ok(high
        .iter()
        .zip(low.iter())
        .zip(close.iter())
        .map(|((&h, &l), &c)| indicator.next((h, l, c)))
        .collect())
}

#[pyfunction]
pub fn ta_atr(
    timeperiod: i64,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
) -> PyResult<Vec<f64>> {
    atr_wilder(timeperiod, high, low, close)
}

#[doc = "Average True Range (Wilder RMA smoothing, TA-Lib standard)."]
#[pyclass]
pub struct AtrWilder {
    inner: Mutex<CoreTaATR>,
}

#[pymethods]
impl AtrWilder {
    #[new]
    pub fn new(timeperiod: i64) -> PyResult<Self> {
        timeperiod.validate("timeperiod")?;
        Ok(Self {
            inner: Mutex::new(CoreTaATR::new(timeperiod as usize)),
        })
    }

    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<f64> {
        let mut guard = lock_inner(&self.inner)?;
        Ok(guard.next((high, low, close)))
    }
}

#[doc = "Average True Range (Wilder RMA smoothing, TA-Lib standard). Alias for AtrWilder."]
#[pyclass]
pub struct TaAtr {
    inner: Mutex<CoreTaATR>,
}

#[pymethods]
impl TaAtr {
    #[new]
    pub fn new(timeperiod: i64) -> PyResult<Self> {
        timeperiod.validate("timeperiod")?;
        Ok(Self {
            inner: Mutex::new(CoreTaATR::new(timeperiod as usize)),
        })
    }

    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<f64> {
        let mut guard = lock_inner(&self.inner)?;
        Ok(guard.next((high, low, close)))
    }
}

export_1_in_1_out!(Wma, CoreWMA, (period: i64));
export_1_in_1_out!(Rsi, RSI, (period: i64));
export_1_in_record_out!(Macd, MACD, MacdResult, (fast: i64, slow: i64, signal: i64), res, MacdResult { macd: res.0, signal: res.1, histogram: res.2 });
export_ohlc_in_1_out!(Adx, ADX, (period: i64));
export_ohlc_in_1_out!(Cci, CCI, (period: i64));

#[doc = "Calculates Stochastic Oscillator.\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high, low, and close must have identical lengths, otherwise ValueError is raised.\n- Negative Params: Negative period/length parameters will raise a ValueError."]
#[pyfunction]
pub fn stoch(
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    fastk: i64,
    slowk: i64,
    slowd: i64,
) -> PyResult<Vec<StochResult>> {
    fastk.validate("fastk")?;
    slowk.validate("slowk")?;
    slowd.validate("slowd")?;
    if high.len() != low.len() || low.len() != close.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: high={}, low={}, close={}",
            high.len(),
            low.len(),
            close.len()
        )));
    }
    let mut it = STOCH::new(
        fastk as usize,
        slowk as usize,
        quantwave_core::MaType::Sma,
        slowd as usize,
        quantwave_core::MaType::Sma,
    );
    Ok(high
        .iter()
        .zip(low.iter())
        .zip(close.iter())
        .map(|((&h, &l), &c)| {
            let (k, d) = it.next((h, l, c));
            StochResult { k, d }
        })
        .collect())
}
#[pyclass]
pub struct Stoch {
    inner: Mutex<STOCH>,
}
#[pymethods]
impl Stoch {
    #[new]
    pub fn new(fastk: i64, slowk: i64, slowd: i64) -> PyResult<Self> {
        fastk.validate("fastk")?;
        slowk.validate("slowk")?;
        slowd.validate("slowd")?;
        Ok(Self {
            inner: Mutex::new(STOCH::new(
                fastk as usize,
                slowk as usize,
                quantwave_core::MaType::Sma,
                slowd as usize,
                quantwave_core::MaType::Sma,
            )),
        })
    }
    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<StochResult> {
        let (k, d) = lock_inner(&self.inner)?.next((high, low, close));
        Ok(StochResult { k, d })
    }
}

export_hl_in_record_out!(Aroon, AROON, AroonResult, (period: i64), res, AroonResult { up: res.0, down: res.1 });
export_1_in_record_out!(Mama, MAMA, MamaResult, (fastlimit: f64, slowlimit: f64), res, MamaResult { mama: res.0, fama: res.1 });
export_1_in_1_out!(Kama, KAMA, (period: i64));
export_1_in_1_out!(T3, CoreT3, (period: i64, v_factor: f64));
export_hl_in_1_out!(Sar, SAR, (acceleration: f64, maximum: f64));
export_1_in_1_out!(Mom, MOM, (period: i64));
export_1_in_1_out!(Roc, ROC, (period: i64));
export_1_in_1_out!(Rocp, ROCP, (period: i64));
export_ohlc_in_1_out!(Willr, WILLR, (period: i64));
export_1_in_1_out!(Dema, DEMA, (period: i64));
export_1_in_1_out!(Tema, TEMA, (period: i64));

#[doc = "Calculates Ichimoku Cloud.\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high and low must have identical lengths, otherwise ValueError is raised.\n- Negative Params: Negative period/length parameters will raise a ValueError."]
#[pyfunction]
pub fn ichimoku(
    high: Vec<f64>,
    low: Vec<f64>,
    tenkan: i64,
    kijun: i64,
    senkou_b: i64,
) -> PyResult<Vec<IchimokuResult>> {
    tenkan.validate("tenkan")?;
    kijun.validate("kijun")?;
    senkou_b.validate("senkou_b")?;
    if high.len() != low.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: high={}, low={}",
            high.len(),
            low.len()
        )));
    }
    let mut it = CoreIchimoku::new(tenkan as usize, kijun as usize, senkou_b as usize);
    Ok(high
        .iter()
        .zip(low.iter())
        .map(|(&h, &l)| {
            let (t, k, sa, sb) = it.next((h, l));
            IchimokuResult {
                tenkan: t,
                kijun: k,
                senkou_a: sa,
                senkou_b: sb,
            }
        })
        .collect())
}
#[pyclass]
pub struct Ichimoku {
    inner: Mutex<CoreIchimoku>,
}
#[pymethods]
impl Ichimoku {
    #[new]
    pub fn new(tenkan: i64, kijun: i64, senkou_b: i64) -> PyResult<Self> {
        tenkan.validate("tenkan")?;
        kijun.validate("kijun")?;
        senkou_b.validate("senkou_b")?;
        Ok(Self {
            inner: Mutex::new(CoreIchimoku::new(
                tenkan as usize,
                kijun as usize,
                senkou_b as usize,
            )),
        })
    }
    pub fn next(&self, high: f64, low: f64) -> PyResult<IchimokuResult> {
        let (t, k, sa, sb) = lock_inner(&self.inner)?.next((high, low));
        Ok(IchimokuResult {
            tenkan: t,
            kijun: k,
            senkou_a: sa,
            senkou_b: sb,
        })
    }
}

export_1_in_1_out!(Cg, CoreCG, (period: i64));
export_1_in_record_out!(CyberCycle, CoreCyberCycle, CyberCycleResult, (length: i64), res, CyberCycleResult { value: res.0, trigger: res.1 });
export_1_in_1_out!(Fisher, CoreFisher, ());
export_1_in_1_out!(InverseFisher, CoreInverseFisher, ());
export_1_in_1_out!(SuperSmoother, CoreSuperSmoother, (period: i64));
export_1_in_1_out!(Bandpass, CoreBandpass, (period: i64, bandwidth: f64));
export_1_in_1_out!(RoofingFilter, CoreRoofingFilter, (hp_period: i64, ss_period: i64));
export_1_in_record_out!(ZeroLag, CoreZeroLag, ZeroLagResult, (length: i64, gain_limit: f64), res, ZeroLagResult { value: res.0, trigger: res.1 });
export_ohlc_in_1_out!(ChoppinessIndex, CoreChoppinessIndex, (period: i64));
export_1_in_1_out!(ClassicLaguerre, CoreClassicLaguerre, (gamma: f64));

export_1_in_record_out!(
    Alligator,
    CoreAlligator,
    AlligatorResult,
    (),
    res,
    AlligatorResult {
        jaw: res.0,
        teeth: res.1,
        lips: res.2
    }
);
#[doc = "Arnaud Legoux Moving Average (ALMA).\n\nParameters:\n  period: Window period (must be >= 1).\n  offset: Gaussian peak offset in [0, 1] (typically 0.85).\n  sigma: Filter width (typically 6.0).\n  series: Input price series.\n  warmup: 'partial' (default) or 'nan'."]
#[pyfunction]
#[pyo3(signature = (period, offset, sigma, series, warmup=None))]
pub fn alma(
    period: i64,
    offset: f64,
    sigma: f64,
    series: Vec<f64>,
    warmup: Option<&str>,
) -> PyResult<Vec<f64>> {
    period.validate("period")?;
    let policy = parse_warmup(warmup)?;
    let mut indicator = CoreALMA::with_warmup(period as usize, offset, sigma, policy);
    Ok(series.iter().map(|&x| indicator.next(x)).collect())
}

#[doc = "Arnaud Legoux Moving Average (ALMA) streaming indicator.\n\nParameters:\n  period: Window period (must be >= 1).\n  offset: Gaussian peak offset in [0, 1] (typically 0.85).\n  sigma: Filter width (typically 6.0).\n  warmup: 'partial' (default) or 'nan'."]
#[pyclass]
pub struct Alma {
    inner: Mutex<CoreALMA>,
}

#[pymethods]
impl Alma {
    #[new]
    #[pyo3(signature = (period, offset, sigma, warmup=None))]
    pub fn new(period: i64, offset: f64, sigma: f64, warmup: Option<&str>) -> PyResult<Self> {
        period.validate("period")?;
        let policy = parse_warmup(warmup)?;
        Ok(Self {
            inner: Mutex::new(CoreALMA::with_warmup(
                period as usize,
                offset,
                sigma,
                policy,
            )),
        })
    }

    pub fn next(&self, input: f64) -> PyResult<f64> {
        let mut guard = lock_inner(&self.inner)?;
        Ok(guard.next(input))
    }
}
export_ohlc_in_record_out!(AtrTs, CoreAtrTs, AtrTsResult, (period: i64, multiplier: f64), res, AtrTsResult { stop: res.0, direction: res.1 });
export_1_in_1_out!(Butterworth2, CoreButterworth2, (period: i64));
export_1_in_1_out!(Butterworth3, CoreButterworth3, (period: i64));
export_1_in_record_out!(ChannelCycle, CoreChannelCycle, CyberCycleResult, (period: i64), res, CyberCycleResult { value: res.0, trigger: res.1 });
export_1_in_1_out!(ContinuationIndex, CoreContinuationIndex, (gamma: f64, order: i64, length: i64));
export_1_in_record_out!(CorrelationCycle, CoreCorrelationCycle, PhasorResult, (period: i64), res, PhasorResult { in_phase: res.0, quadrature: res.1 });
export_1_in_1_out!(CorrelationTrend, CoreCorrelationTrend, (length: i64));
export_1_in_1_out!(CyberneticOscillator, CoreCyberneticOscillator, (hp_length: i64, lp_length: i64, rms_len: i64));
export_hl_in_1_out!(Dmh, CoreDMH, (length: i64));

#[doc = "Calculates Donchian Channels.\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high and low must have identical lengths, otherwise ValueError is raised.\n- Negative Params: Negative period/length parameters will raise a ValueError."]
#[pyfunction]
pub fn donchian(high: Vec<f64>, low: Vec<f64>, period: i64) -> PyResult<Vec<DonchianResult>> {
    period.validate("period")?;
    if high.len() != low.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: high={}, low={}",
            high.len(),
            low.len()
        )));
    }
    let mut it = CoreDonchian::new(period as usize);
    Ok(high
        .iter()
        .zip(low.iter())
        .map(|(&h, &l)| {
            let (u, m, lo) = it.next((h, l));
            DonchianResult {
                upper: u,
                middle: m,
                lower: lo,
            }
        })
        .collect())
}
#[pyclass]
pub struct Donchian {
    inner: Mutex<CoreDonchian>,
}
#[pymethods]
impl Donchian {
    #[new]
    pub fn new(period: i64) -> PyResult<Self> {
        period.validate("period")?;
        Ok(Self {
            inner: Mutex::new(CoreDonchian::new(period as usize)),
        })
    }
    pub fn next(&self, high: f64, low: f64) -> PyResult<DonchianResult> {
        let (u, m, lo) = lock_inner(&self.inner)?.next((high, low));
        Ok(DonchianResult {
            upper: u,
            middle: m,
            lower: lo,
        })
    }
}

export_1_in_1_out!(Dsma, CoreDSMA, (period: i64));
export_1_in_record_out!(Emd, CoreEMD, EmdResult, (period: i64, delta: f64, fraction: f64), res, EmdResult { trend: res.0, upper: res.1, lower: res.2 });
export_co_in_1_out!(AmDetector, CoreAMDetector, (highest_len: i64, avg_len: i64));
export_co_in_1_out!(FmDemodulator, CoreFMDemodulator, (period: i64));
export_1_in_vec_out!(EhlersAutocorrelation, CoreEhlersAutocorrelation, (length: i64, num_lags: i64));
export_1_in_1_out!(EhlersFilter, CoreEhlersFilter, (length: i64));
export_pv_in_record_out!(EhlersLoops, CoreEhlersLoops, EhlersLoopsResult, (lp_period: i64, hp_period: i64), res, EhlersLoopsResult { price_rms: res.0, vol_rms: res.1 });
export_1_in_1_out!(EhlersStochastic, CoreEhlersStochastic, (hp_period: i64, ss_period: i64, stoch_period: i64));
export_1_in_1_out!(EhlersUltimateOscillator, CoreEhlersUltimateOscillator, (band_edge: i64, bandwidth: f64));
export_1_in_1_out!(FisherHighPass, CoreFisherHighPass, (hp_len: i64, norm_len: i64));
export_1_in_1_out!(FourierSeries, CoreFourierSeries, (fundamental: i64));
export_1_in_1_out!(FourierDominantCycle, CoreFourierDominantCycle, (window_len: i64));
export_hl_in_record_out!(
    Fractals,
    CoreFractals,
    FractalsResult,
    (),
    res,
    FractalsResult {
        bearish: res.0,
        bullish: res.1
    }
);
export_ohlc_in_1_out!(Frama, CoreFRAMA, (length: i64));
export_1_in_1_out!(Gaussian, CoreGaussian, (period: i64, poles: i64));
export_1_in_1_out!(GeneralizedLaguerre, CoreGeneralizedLaguerre, (length: i64, gamma: f64, order: i64));
export_1_in_1_out!(GriffithsDominantCycle, CoreGriffithsDominantCycle, (lower_bound: i64, upper_bound: i64, length: i64));
export_1_in_1_out!(GriffithsPredictor, CoreGriffithsPredictor, (lower_bound: i64, upper_bound: i64, length: i64, bars_fwd: i64));
export_1_in_vec_out!(GriffithsSpectrum, CoreGriffithsSpectrum, (lower_bound: i64, upper_bound: i64, length: i64));
export_1_in_1_out!(Hamming, CoreHamming, (period: i64, pedestal: f64));
export_1_in_1_out!(Hann, CoreHann, (period: i64));

#[doc = "Calculates Heikin-Ashi candles.\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: open, high, low, and close must have identical lengths, otherwise ValueError is raised."]
#[pyfunction]
pub fn heikin_ashi(
    open: Vec<f64>,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
) -> PyResult<Vec<HeikinAshiResult>> {
    if open.len() != high.len() || high.len() != low.len() || low.len() != close.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: open={}, high={}, low={}, close={}",
            open.len(),
            high.len(),
            low.len(),
            close.len()
        )));
    }
    let mut it = CoreHeikinAshi::new();
    Ok(open
        .iter()
        .zip(high.iter())
        .zip(low.iter())
        .zip(close.iter())
        .map(|(((&o, &h), &l), &c)| {
            let (ho, hh, hl, hc) = it.next((o, h, l, c));
            HeikinAshiResult {
                open: ho,
                high: hh,
                low: hl,
                close: hc,
            }
        })
        .collect())
}
#[pyclass]
pub struct HeikinAshi {
    inner: Mutex<CoreHeikinAshi>,
}
#[pymethods]
impl HeikinAshi {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(CoreHeikinAshi::new()),
        }
    }
    pub fn next(&self, open: f64, high: f64, low: f64, close: f64) -> PyResult<HeikinAshiResult> {
        let (ho, hh, hl, hc) = lock_inner(&self.inner)?.next((open, high, low, close));
        Ok(HeikinAshiResult {
            open: ho,
            high: hh,
            low: hl,
            close: hc,
        })
    }
}

export_1_in_1_out!(HighPass, CoreHighPass, (period: i64));
export_1_in_1_out!(Hma, CoreHMA, (period: i64));
export_1_in_1_out!(EhlersWma4, CoreEhlersWma4, ());
export_1_in_1_out!(InstantaneousTrendline, CoreInstantaneousTrendline, ());
export_1_in_record_out!(UndersampledDoubleMa, CoreUDMA, UdmaResult, (fast_len: i64, slow_len: i64, samp_per: i64), res, UdmaResult { fast: res.0, slow: res.1 });

#[doc = "Calculates Keltner Channels.\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: high, low, and close must have identical lengths, otherwise ValueError is raised.\n- Negative Params: Negative period/length parameters will raise a ValueError."]
#[pyfunction]
pub fn keltner(
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    ema_period: i64,
    atr_period: i64,
    multiplier: f64,
) -> PyResult<Vec<KeltnerResult>> {
    ema_period.validate("ema_period")?;
    atr_period.validate("atr_period")?;
    multiplier.validate("multiplier")?;
    if high.len() != low.len() || low.len() != close.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: high={}, low={}, close={}",
            high.len(),
            low.len(),
            close.len()
        )));
    }
    let mut it = CoreKeltner::new(ema_period as usize, atr_period as usize, multiplier);
    Ok(high
        .iter()
        .zip(low.iter())
        .zip(close.iter())
        .map(|((&h, &l), &c)| {
            let (u, m, lo) = it.next((h, l, c));
            KeltnerResult {
                upper: u,
                middle: m,
                lower: lo,
            }
        })
        .collect())
}
#[pyclass]
pub struct Keltner {
    inner: Mutex<CoreKeltner>,
}
#[pymethods]
impl Keltner {
    #[new]
    pub fn new(ema_period: i64, atr_period: i64, multiplier: f64) -> PyResult<Self> {
        ema_period.validate("ema_period")?;
        atr_period.validate("atr_period")?;
        multiplier.validate("multiplier")?;
        Ok(Self {
            inner: Mutex::new(CoreKeltner::new(
                ema_period as usize,
                atr_period as usize,
                multiplier,
            )),
        })
    }
    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<KeltnerResult> {
        let (u, m, lo) = lock_inner(&self.inner)?.next((high, low, close));
        Ok(KeltnerResult {
            upper: u,
            middle: m,
            lower: lo,
        })
    }
}

export_1_in_1_out!(LaguerreFilter, CoreLaguerreFilter, (length: i64, gamma: f64));
export_1_in_1_out!(LaguerreOscillator, CoreLaguerreOscillator, (length: i64, gamma: f64, rms_period: i64));
export_1_in_1_out!(LaguerreRsi, CoreLaguerreRSI, (gamma: f64));
export_1_in_1_out!(NoiseElimination, CoreNoiseElimination, (period: i64));
export_co_in_record_out!(PairsRotation, CorePairsRotation, PairsRotationResult, (hp_len: i64, lp_len: i64), res, PairsRotationResult { ratio: res.0, angle: res.1 });
export_1_in_record_out!(
    Phasor,
    CorePhasor,
    PhasorResult,
    (),
    res,
    PhasorResult {
        in_phase: res.0,
        quadrature: res.1
    }
);
export_co_in_1_out!(OcPriceRsi, CoreOcPriceRSI, (period: i64));

#[pyfunction]
pub fn pivot_points(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>) -> Vec<PivotPointsResult> {
    let mut it = CorePivotPoints::new();
    high.iter()
        .zip(low.iter())
        .zip(close.iter())
        .map(|((&h, &l), &c)| {
            let r = it.next((h, l, c));
            PivotPointsResult {
                p: r.0,
                r1: r.1,
                s1: r.2,
                r2: r.3,
                s2: r.4,
            }
        })
        .collect()
}
#[pyclass]
pub struct PivotPoints {
    inner: Mutex<CorePivotPoints>,
}
#[pymethods]
impl PivotPoints {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(CorePivotPoints::new()),
        }
    }
    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<PivotPointsResult> {
        let r = lock_inner(&self.inner)?.next((high, low, close));
        Ok(PivotPointsResult {
            p: r.0,
            r1: r.1,
            s1: r.2,
            r2: r.3,
            s2: r.4,
        })
    }
}

export_1_in_1_out!(OneEuroFilter, CoreOneEuroFilter, (period_min: i64, beta: f64));
export_1_in_record_out!(ProjectedMovingAverage, CoreProjectedMovingAverage, PmaResult, (period: i64), res, PmaResult { pma: res.0, predict: res.1 });
export_1_in_record_out!(PrecisionTrend, CorePrecisionTrend, TrendRocResult, (length1: i64, length2: i64), res, TrendRocResult { trend: res.0, roc: res.1 });
export_1_in_record_out!(ReversionIndex, CoreReversionIndex, TrendRocResult, (period: i64), res, TrendRocResult { trend: res.0, roc: res.1 });
export_1_in_record_out!(
    SineWave,
    CoreSineWave,
    PhasorResult,
    (),
    res,
    PhasorResult {
        in_phase: res.0,
        quadrature: res.1
    }
);

#[pyfunction]
pub fn swiss_army_knife(series: Vec<f64>, mode: SwissMode, period: u64, delta: f64) -> Vec<f64> {
    let mut it = CoreSwissArmyKnife::new(mode.into(), period as usize, delta);
    series.iter().map(|&x| it.next(x)).collect()
}
#[pyclass]
pub struct SwissArmyKnife {
    inner: Mutex<CoreSwissArmyKnife>,
}
#[pymethods]
impl SwissArmyKnife {
    #[new]
    pub fn new(mode: SwissMode, period: u64, delta: f64) -> Self {
        Self {
            inner: Mutex::new(CoreSwissArmyKnife::new(mode.into(), period as usize, delta)),
        }
    }
    pub fn next(&self, input: f64) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next(input))
    }
}

export_1_in_record_out!(
    SystemEvaluator,
    CoreSystemEvaluator,
    SystemEvaluatorResult,
    (),
    res,
    res.into()
);
#[pyclass]
pub struct RobustnessEvaluator {
    inner: Mutex<CoreRobustnessEvaluator>,
}
#[pymethods]
impl RobustnessEvaluator {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(CoreRobustnessEvaluator::new()),
        }
    }
    pub fn add_test_result(&self, net_profit: f64) -> PyResult<()> {
        lock_inner(&self.inner)?.add_test_result(net_profit);
        Ok(())
    }
    pub fn calculate_score(&self) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.calculate_score())
    }
}

export_ohlc_in_record_out!(TtmSqueeze, CoreTtmSqueeze, TtmSqueezeResult, (period: i64, mult_bb: f64, mult_kc: f64), res, TtmSqueezeResult { value: res.0, direction: if res.1 { 1 } else { 0 } });

#[pyfunction]
pub fn ultimate_bands(series: Vec<f64>, length: u64, num_sds: f64) -> Vec<UltimateBandsResult> {
    let mut it = CoreUltimateBands::new(length as usize, num_sds);
    series
        .iter()
        .map(|&x| {
            let (u, m, lo) = it.next(x);
            UltimateBandsResult {
                upper: u,
                middle: m,
                lower: lo,
            }
        })
        .collect()
}
#[pyclass]
pub struct UltimateBands {
    inner: Mutex<CoreUltimateBands>,
}
#[pymethods]
impl UltimateBands {
    #[new]
    pub fn new(length: u64, num_sds: f64) -> Self {
        Self {
            inner: Mutex::new(CoreUltimateBands::new(length as usize, num_sds)),
        }
    }
    pub fn next(&self, input: f64) -> PyResult<UltimateBandsResult> {
        let (u, m, lo) = lock_inner(&self.inner)?.next(input);
        Ok(UltimateBandsResult {
            upper: u,
            middle: m,
            lower: lo,
        })
    }
}

#[pyfunction]
pub fn ultimate_channel(
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    length: u64,
    str_length: u64,
    num_strs: f64,
) -> Vec<UltimateChannelResult> {
    let mut it = CoreUltimateChannel::new(length as usize, str_length as usize, num_strs);
    high.iter()
        .zip(low.iter())
        .zip(close.iter())
        .map(|((&h, &l), &c)| {
            let (u, ce, lo) = it.next((h, l, c));
            UltimateChannelResult {
                upper: u,
                center: ce,
                lower: lo,
            }
        })
        .collect()
}
#[pyclass]
pub struct UltimateChannel {
    inner: Mutex<CoreUltimateChannel>,
}
#[pymethods]
impl UltimateChannel {
    #[new]
    pub fn new(length: u64, str_length: u64, num_strs: f64) -> Self {
        Self {
            inner: Mutex::new(CoreUltimateChannel::new(
                length as usize,
                str_length as usize,
                num_strs,
            )),
        }
    }
    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<UltimateChannelResult> {
        let (u, ce, lo) = lock_inner(&self.inner)?.next((high, low, close));
        Ok(UltimateChannelResult {
            upper: u,
            center: ce,
            lower: lo,
        })
    }
}

export_1_in_1_out!(UltimateSmoother, CoreUltimateSmoother, (period: i64));
export_1_in_1_out!(Usi, CoreUSI, (length: i64));

#[pyfunction]
pub fn ad(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, volume: Vec<f64>) -> Vec<f64> {
    let mut it = CoreAD::new();
    high.iter()
        .zip(low.iter())
        .zip(close.iter())
        .zip(volume.iter())
        .map(|(((&h, &l), &c), &v)| it.next((h, l, c, v)))
        .collect()
}
#[pyclass]
pub struct Ad {
    inner: Mutex<CoreAD>,
}
#[pymethods]
impl Ad {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(CoreAD::new()),
        }
    }
    pub fn next(&self, high: f64, low: f64, close: f64, volume: f64) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next((high, low, close, volume)))
    }
}

#[pyfunction]
pub fn adosc(
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
    volume: Vec<f64>,
    fast: u64,
    slow: u64,
) -> Vec<f64> {
    let mut it = CoreADOSC::new(fast as usize, slow as usize);
    high.iter()
        .zip(low.iter())
        .zip(close.iter())
        .zip(volume.iter())
        .map(|(((&h, &l), &c), &v)| it.next((h, l, c, v)))
        .collect()
}
#[pyclass]
pub struct Adosc {
    inner: Mutex<CoreADOSC>,
}
#[pymethods]
impl Adosc {
    #[new]
    pub fn new(fast: u64, slow: u64) -> Self {
        Self {
            inner: Mutex::new(CoreADOSC::new(fast as usize, slow as usize)),
        }
    }
    pub fn next(&self, high: f64, low: f64, close: f64, volume: f64) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next((high, low, close, volume)))
    }
}

export_pv_in_1_out!(Obv, CoreOBV, ());
export_ohlc_in_record_out!(Vortex, CoreVortex, VortexResult, (period: i64), res, VortexResult { plus: res.0, minus: res.1 });

#[pyfunction]
pub fn anchored_vwap(price: Vec<f64>, volume: Vec<f64>, anchor: Vec<bool>) -> Vec<f64> {
    let mut it = CoreAnchoredVWAP::new();
    price
        .iter()
        .zip(volume.iter())
        .zip(anchor.iter())
        .map(|((&p, &v), &a)| it.next((p, v, a)))
        .collect()
}
#[pyclass]
pub struct AnchoredVwap {
    inner: Mutex<CoreAnchoredVWAP>,
}
#[pymethods]
impl AnchoredVwap {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(CoreAnchoredVWAP::new()),
        }
    }
    pub fn next(&self, price: f64, volume: f64, anchor: bool) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next((price, volume, anchor)))
    }
}

export_ohlc_in_record_out!(WaveTrend, CoreWaveTrend, WaveTrendResult, (n1: i64, n2: i64, n3: i64), res, WaveTrendResult { wt1: res.0, wt2: res.1 });
export_1_in_1_out!(SimplePredictor, CoreSimplePredictor, (hp_len: i64, lp_len: i64, q: f64));
export_1_in_1_out!(Mad, CoreMAD, (short_period: i64, long_period: i64));
export_1_in_1_out!(MesaStochastic, CoreMesaStochastic, (len: i64, hp: i64, ss: i64));
export_1_in_1_out!(Rsih, CoreRSIH, (length: i64));
export_1_in_record_out!(VossPredictor, CoreVossPredictor, VossPredictorResult, (period: i64, predict: i64), res, VossPredictorResult { filt: res.0, voss: res.1 });
export_1_in_1_out!(SyntheticOscillator, CoreSyntheticOscillator, (lower_bound: i64, upper_bound: i64));

#[pyfunction]
pub fn cycletrendanalytics(
    series: Vec<f64>,
    min_length: u64,
    max_length: u64,
) -> Vec<CycleTrendAnalyticsResult> {
    let mut it = CoreCycleTrendAnalytics::new(min_length as usize, max_length as usize);
    series
        .iter()
        .map(|&x| {
            let r = it.next(x);
            CycleTrendAnalyticsResult {
                cycle: r[0],
                trend: r[1],
            }
        })
        .collect()
}
#[pyclass]
pub struct CycleTrendAnalytics {
    inner: Mutex<CoreCycleTrendAnalytics>,
}
#[pymethods]
impl CycleTrendAnalytics {
    #[new]
    pub fn new(min_length: u64, max_length: u64) -> Self {
        Self {
            inner: Mutex::new(CoreCycleTrendAnalytics::new(
                min_length as usize,
                max_length as usize,
            )),
        }
    }
    pub fn next(&self, input: f64) -> PyResult<CycleTrendAnalyticsResult> {
        let r = lock_inner(&self.inner)?.next(input);
        Ok(CycleTrendAnalyticsResult {
            cycle: r[0],
            trend: r[1],
        })
    }
}

export_1_in_1_out!(Madh, CoreMADH, (short_length: i64, dominant_cycle: i64));
export_1_in_1_out!(Stc, CoreSTC, (cycle_period: i64, fast_period: i64, slow_period: i64));
export_1_in_1_out!(HomodyneDiscriminator, CoreHomodyneDiscriminator, ());
export_1_in_1_out!(UniversalOscillator, CoreUniversalOscillator, (band_edge: i64));
export_1_in_1_out!(TriangleFilter, CoreTriangleFilter, (length: i64));
export_1_in_1_out!(HtDcPeriod, HT_DCPERIOD, ());
export_1_in_record_out!(
    HtPhasor,
    HT_PHASOR,
    PhasorResult,
    (),
    res,
    PhasorResult {
        in_phase: res.0,
        quadrature: res.1
    }
);
export_1_in_1_out!(HtDcPhase, HT_DCPHASE, ());
export_1_in_record_out!(
    HtSine,
    HT_SINE,
    HtSineResult,
    (),
    res,
    HtSineResult {
        sine: res.0,
        leadsine: res.1
    }
);
export_1_in_1_out!(HtTrendMode, HT_TRENDMODE, ());
export_1_in_1_out!(HurstExponent, CoreHurstExponent, (length: i64));
export_1_in_1_out!(FracDiff, CoreFracDiff, (d: f64, threshold: f64));
export_1_in_1_out!(KalmanFilter, CoreKalmanFilter, (gain: f64, noise: f64));
export_1_in_1_out!(MarketState, CoreMarketState, (period: i64, threshold: f64));
export_1_in_1_out!(RecursiveMedian, CoreRecursiveMedian, (length: i64));
export_1_in_1_out!(RecursiveMedianOscillator, CoreRMO, (lp_period: i64, hp_period: i64));
export_1_in_1_out!(Reflex, CoreReflex, (length: i64));
export_1_in_1_out!(RocketRsi, CoreRocketRSI, (rsi_length: i64, smooth_length: i64));
export_1_in_1_out!(Trendflex, CoreTrendflex, (length: i64));
export_1_in_1_out!(TruncatedBandpass, CoreTruncatedBandpass, (period: i64, bandwidth: f64, length: i64));

#[pyfunction]
pub fn volumeprofile(
    price: Vec<f64>,
    volume: Vec<f64>,
    period: u64,
    bins: u64,
) -> Vec<VolumeProfileResult> {
    let mut it = CoreVolumeProfile::new(period as usize, bins as usize);
    price
        .iter()
        .zip(volume.iter())
        .map(|(&p, &v)| {
            let poc = it.next((p, v));
            VolumeProfileResult {
                poc,
                vah: 0.0,
                val: 0.0,
            }
        })
        .collect()
}
#[pyclass]
pub struct VolumeProfile {
    inner: Mutex<CoreVolumeProfile>,
}
#[pymethods]
impl VolumeProfile {
    #[new]
    pub fn new(period: u64, bins: u64) -> Self {
        Self {
            inner: Mutex::new(CoreVolumeProfile::new(period as usize, bins as usize)),
        }
    }
    pub fn next(&self, price: f64, volume: f64) -> PyResult<VolumeProfileResult> {
        let poc = lock_inner(&self.inner)?.next((price, volume));
        Ok(VolumeProfileResult {
            poc,
            vah: 0.0,
            val: 0.0,
        })
    }
}

// Into implementations for records
impl From<quantwave_core::indicators::system_evaluator::SystemEvaluationResults>
    for SystemEvaluatorResult
{
    fn from(r: quantwave_core::indicators::system_evaluator::SystemEvaluationResults) -> Self {
        Self {
            average_win_loss_ratio: r.average_win_loss_ratio,
            average_trade: r.average_trade,
            profit_factor: r.profit_factor,
            percent_winners: r.percent_winners,
            breakeven_profit_factor: r.breakeven_profit_factor,
            weighted_average_trade: r.weighted_average_trade,
            theoretical_consecutive_losers: r.theoretical_consecutive_losers,
        }
    }
}

// --- Options India ---

#[pyfunction]
pub fn bs_call_price(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    options_india::bs_call_price(s, k, r, t, sigma)
}
#[pyfunction]
pub fn bs_put_price(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    options_india::bs_put_price(s, k, r, t, sigma)
}
#[pyfunction]
pub fn bs_delta(s: f64, k: f64, r: f64, t: f64, sigma: f64, is_call: bool) -> f64 {
    options_india::bs_delta(s, k, r, t, sigma, options_india::OptionType::from(is_call))
}
#[pyfunction]
pub fn bs_gamma(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    options_india::bs_gamma(s, k, r, t, sigma)
}
#[pyfunction]
pub fn bs_theta(s: f64, k: f64, r: f64, t: f64, sigma: f64, is_call: bool) -> f64 {
    options_india::bs_theta(s, k, r, t, sigma, options_india::OptionType::from(is_call))
}
#[pyfunction]
pub fn bs_vega(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    options_india::bs_vega(s, k, r, t, sigma)
}
#[pyfunction]
pub fn bs_rho(s: f64, k: f64, r: f64, t: f64, sigma: f64, is_call: bool) -> f64 {
    options_india::bs_rho(s, k, r, t, sigma, options_india::OptionType::from(is_call))
}
#[pyfunction]
pub fn implied_vol(
    market_price: f64,
    s: f64,
    k: f64,
    r: f64,
    t: f64,
    is_call: bool,
) -> Option<f64> {
    if t <= 0.0 {
        return None;
    }
    let theta = if is_call { 1.0 } else { -1.0 };
    let forward = s * (r * t).exp();
    let undiscounted_price = market_price * (r * t).exp();
    let iv = options_india::implied_black_volatility(undiscounted_price, forward, k, t, theta);
    if iv >= f64::MAX || iv <= -f64::MAX {
        None
    } else {
        Some(iv)
    }
}
#[pyfunction]
pub fn max_pain(strikes: Vec<f64>, ce_oi: Vec<u64>, pe_oi: Vec<u64>, lot_size: u32) -> f64 {
    options_india::max_pain(&strikes, &ce_oi, &pe_oi, lot_size)
}
#[pyfunction]
pub fn strike_pcr(ce_oi: Vec<u64>, pe_oi: Vec<u64>) -> Vec<f64> {
    options_india::strike_pcr(&ce_oi, &pe_oi)
}
#[pyfunction]
pub fn chain_pcr(ce_oi: Vec<u64>, pe_oi: Vec<u64>) -> f64 {
    options_india::chain_pcr(&ce_oi, &pe_oi)
}
#[pyfunction]
pub fn oi_zones(strikes: Vec<f64>, ce_oi: Vec<u64>, pe_oi: Vec<u64>, n: u64) -> OIZonesResult {
    let zones = options_india::oi_zones(&strikes, &ce_oi, &pe_oi, n as usize);
    OIZonesResult {
        resistance_strikes: zones.resistance_strikes,
        support_strikes: zones.support_strikes,
    }
}
#[pyfunction]
pub fn gex_per_strike(
    spot: f64,
    strikes: Vec<f64>,
    ce_gamma: Vec<f64>,
    pe_gamma: Vec<f64>,
    ce_oi: Vec<u64>,
    pe_oi: Vec<u64>,
    lot_size: u32,
) -> Vec<GexResult> {
    options_india::gex_per_strike(
        spot, &strikes, &ce_gamma, &pe_gamma, &ce_oi, &pe_oi, lot_size,
    )
    .into_iter()
    .map(|(ce, pe, net)| GexResult {
        ce_gex: ce,
        pe_gex: pe,
        net_gex: net,
    })
    .collect()
}
#[pyfunction]
pub fn gex_flip_strike(strikes: Vec<f64>, net_gex: Vec<f64>) -> Option<f64> {
    options_india::gex_flip_strike(&strikes, &net_gex)
}
#[pyfunction]
pub fn gextotal(
    spot: f64,
    strikes: Vec<f64>,
    ce_gamma: Vec<f64>,
    pe_gamma: Vec<f64>,
    ce_oi: Vec<u64>,
    pe_oi: Vec<u64>,
    lot_size: u32,
) -> f64 {
    options_india::gextotal(
        spot, &strikes, &ce_gamma, &pe_gamma, &ce_oi, &pe_oi, lot_size,
    )
}
#[pyfunction]
pub fn atm_straddle(
    spot: f64,
    strikes: Vec<f64>,
    ce_ltp: Vec<f64>,
    pe_ltp: Vec<f64>,
) -> StraddleResult {
    let (atm, premium, pct) = options_india::atm_straddle(spot, &strikes, &ce_ltp, &pe_ltp);
    StraddleResult {
        atm_strike: atm,
        straddle_premium: premium,
        implied_move_pct: pct,
    }
}
#[pyfunction]
pub fn synthetic_futures(strikes: Vec<f64>, ce_ltp: Vec<f64>, pe_ltp: Vec<f64>) -> Vec<f64> {
    options_india::synthetic_futures(&strikes, &ce_ltp, &pe_ltp)
}
#[pyfunction]
pub fn moneyness(spot: f64, strike: f64) -> String {
    options_india::moneyness(spot, strike).to_string()
}
#[pyfunction]
pub fn nse_lot_size(symbol: String) -> Option<u32> {
    options_india::nse_lot_size(&symbol)
}
#[pyfunction]
pub fn nse_risk_free_rate() -> f64 {
    options_india::NSE_RISK_FREE_RATE
}

// ============================================================================
// ML Feature Extractors (quantwave-gw7s: validation harness + canonical notebook)
// Exposes the rich feature structs from quantwave-core/src/features/* as
// streaming Objects + batch fns. This enables the notebook to build feature
// matrices from the *new toolkit* using the exact same Next impls tested in Rust.
// Sources: features/mod.rs + the four * .rs files (wrapping Ehlers + Hurst).
// ============================================================================

#[pyclass(get_all)]
#[derive(Clone)]
pub struct CyberCycleFeaturesResult {
    pub cycle: f64,
    pub trigger: f64,
    pub cycle_momentum: f64,
    pub trigger_signal: f64,
}

#[pyclass]
pub struct CyberCycleFeatureExtractor {
    inner: Mutex<CoreCyberCycleFE>,
}
#[pymethods]
impl CyberCycleFeatureExtractor {
    #[new]
    pub fn new(length: u64) -> Self {
        Self {
            inner: Mutex::new(CoreCyberCycleFE::new(length as usize)),
        }
    }
    pub fn next(&self, input: f64) -> PyResult<CyberCycleFeaturesResult> {
        let f = lock_inner(&self.inner)?.next(input);
        Ok(CyberCycleFeaturesResult {
            cycle: f.cycle,
            trigger: f.trigger,
            cycle_momentum: f.cycle_momentum,
            trigger_signal: f.trigger_signal,
        })
    }
}
#[pyfunction]
pub fn cyber_cycle_features(length: u64, series: Vec<f64>) -> Vec<CyberCycleFeaturesResult> {
    let mut ext = CoreCyberCycleFE::new(length as usize);
    series
        .into_iter()
        .map(|x| {
            let f = ext.next(x);
            CyberCycleFeaturesResult {
                cycle: f.cycle,
                trigger: f.trigger,
                cycle_momentum: f.cycle_momentum,
                trigger_signal: f.trigger_signal,
            }
        })
        .collect()
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct HurstFeaturesResult {
    pub persistence: f64,
    /// -1 mean-reverting, 0 random, +1 trending (or None -> -99 sentinel for FFI simplicity in some consumers)
    pub regime_label: i32,
}

#[pyclass]
pub struct HurstFeatureExtractor {
    inner: Mutex<CoreHurstFE>,
}
#[pymethods]
impl HurstFeatureExtractor {
    #[new]
    pub fn new(period: u64) -> Self {
        Self {
            inner: Mutex::new(CoreHurstFE::new(period as usize)),
        }
    }
    pub fn next(&self, input: f64) -> PyResult<HurstFeaturesResult> {
        let f = lock_inner(&self.inner)?.next(input);
        Ok(HurstFeaturesResult {
            persistence: f.persistence,
            regime_label: f.regime_label.unwrap_or(-99) as i32,
        })
    }
}
#[pyfunction]
pub fn hurst_features(period: u64, series: Vec<f64>) -> Vec<HurstFeaturesResult> {
    let mut ext = CoreHurstFE::new(period as usize);
    series
        .into_iter()
        .map(|x| {
            let f = ext.next(x);
            HurstFeaturesResult {
                persistence: f.persistence,
                regime_label: f.regime_label.unwrap_or(-99) as i32,
            }
        })
        .collect()
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct InstantaneousTrendlineFeaturesResult {
    pub trend: f64,
    pub strength: f64,
}

#[pyclass]
pub struct InstantaneousTrendlineFeatureExtractor {
    inner: Mutex<CoreITFE>,
}
#[pymethods]
impl InstantaneousTrendlineFeatureExtractor {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(CoreITFE::new()),
        }
    }
    pub fn next(&self, input: f64) -> PyResult<InstantaneousTrendlineFeaturesResult> {
        let f = lock_inner(&self.inner)?.next(input);
        Ok(InstantaneousTrendlineFeaturesResult {
            trend: f.trend,
            strength: f.strength,
        })
    }
}
#[pyfunction]
pub fn instantaneous_trendline_features(
    series: Vec<f64>,
) -> Vec<InstantaneousTrendlineFeaturesResult> {
    let mut ext = CoreITFE::new();
    series
        .into_iter()
        .map(|x| {
            let f = ext.next(x);
            InstantaneousTrendlineFeaturesResult {
                trend: f.trend,
                strength: f.strength,
            }
        })
        .collect()
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct TrendflexFeaturesResult {
    pub trendflex: f64,
}

#[pyclass]
pub struct TrendflexFeatureExtractor {
    inner: Mutex<CoreTrendflexFE>,
}
#[pymethods]
impl TrendflexFeatureExtractor {
    #[new]
    pub fn new(length: u64) -> Self {
        Self {
            inner: Mutex::new(CoreTrendflexFE::new(length as usize)),
        }
    }
    pub fn next(&self, input: f64) -> PyResult<TrendflexFeaturesResult> {
        let f = lock_inner(&self.inner)?.next(input);
        Ok(TrendflexFeaturesResult {
            trendflex: f.trendflex,
        })
    }
}
#[pyfunction]
pub fn trendflex_features(length: u64, series: Vec<f64>) -> Vec<TrendflexFeaturesResult> {
    let mut ext = CoreTrendflexFE::new(length as usize);
    series
        .into_iter()
        .map(|x| {
            let f = ext.next(x);
            TrendflexFeaturesResult {
                trendflex: f.trendflex,
            }
        })
        .collect()
}

// === Additional ML feature extractors for 4ps/gwx cross-epic E2E notebook (trivial wiring of core Next<T> wrappers) ===
// Sources: quantwave-core/src/features/griffiths_dominant_cycle.rs + regime.rs + regimes/hmm.rs (HMM::bull_bear)
// These complete the 4 locked .ta.features.* surface for Python consumers (Hurst + Cyber already present; griffiths + regime now added).

#[pyclass(get_all)]
#[derive(Clone)]
pub struct GriffithsDominantCycleFeaturesResult {
    pub dominant_cycle: f64,
}

#[pyclass]
pub struct GriffithsDominantCycleFeatureExtractor {
    inner: Mutex<CoreGriffithsDCFE>,
}
#[pymethods]
impl GriffithsDominantCycleFeatureExtractor {
    #[new]
    pub fn new(lower: u64, upper: u64, length: u64) -> Self {
        Self {
            inner: Mutex::new(CoreGriffithsDCFE::new(
                lower as usize,
                upper as usize,
                length as usize,
            )),
        }
    }
    pub fn next(&self, input: f64) -> PyResult<GriffithsDominantCycleFeaturesResult> {
        let f = lock_inner(&self.inner)?.next(input);
        Ok(GriffithsDominantCycleFeaturesResult {
            dominant_cycle: f.dominant_cycle,
        })
    }
}
#[pyfunction]
pub fn griffiths_dominant_cycle_features(
    lower: u64,
    upper: u64,
    length: u64,
    series: Vec<f64>,
) -> Vec<GriffithsDominantCycleFeaturesResult> {
    let mut ext = CoreGriffithsDCFE::new(lower as usize, upper as usize, length as usize);
    series
        .into_iter()
        .map(|x| {
            let f = ext.next(x);
            GriffithsDominantCycleFeaturesResult {
                dominant_cycle: f.dominant_cycle,
            }
        })
        .collect()
}

// name = "BullBearHmm" preserves the uniffi-era Python symbol (uniffi re-cased the
// `HMM` capital-run via heck); keeps the generated _ta registry / metadata unchanged.
//
// Input contract: `next()` expects daily returns (e.g. pct_change/rocp), not raw
// price — `CoreHMM::bull_bear()`'s Gaussian emissions are hardcoded to returns
// scale. On price-scale input both emissions underflow and every call ties at
// state 0 (Bull); see `quantwave_core::regimes::hmm::HMM::{bull_bear,
// is_degenerate}` and the `hmm_bull_bear` polars plugin (custom_4.rs), which
// rejects that case outright. This streaming `next()` returns a plain `i32`
// (not a `PyResult`) to match this crate's existing streaming-class convention,
// so on degenerate input it silently keeps returning `1` here — callers driving
// this class directly with raw data should validate their input scale
// themselves rather than relying on an exception.
#[pyclass(name = "BullBearHmm")]
pub struct BullBearHMM {
    inner: Mutex<CoreHMM>,
}
#[pymethods]
impl BullBearHMM {
    // Named alternate constructor: uniffi exposed `#[uniffi::constructor]`-with-a-name
    // as a classmethod (`BullBearHmm.bull_bear()`), so it maps to #[staticmethod], not #[new].
    #[staticmethod]
    pub fn bull_bear() -> Self {
        Self {
            inner: Mutex::new(CoreHMM::bull_bear()),
        }
    }
    /// Advance the HMM by one bar of **daily returns** (not price) and return the
    /// decoded state: `0` = no regime decided yet (or non-finite input), `1` =
    /// Bull, `2` = Bear. See the struct-level doc comment above for the
    /// price-vs-returns input contract.
    pub fn next(&self, price: f64) -> PyResult<i32> {
        if !price.is_finite() {
            return Ok(0); // Steady sentinel
        }
        let regime = lock_inner(&self.inner)?.next(price);
        Ok(match regime {
            MarketRegime::Bull => 1,
            MarketRegime::Bear => 2,
            MarketRegime::Crisis => 3,
            MarketRegime::Steady => 0,
            MarketRegime::Cluster(c) => 4 + (c as i32),
        })
    }
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct GaussianHmmParamsPy {
    pub n_states: u32,
    pub delta: Vec<f64>,
    pub gamma_flat: Vec<f64>,
    pub means: Vec<f64>,
    pub stds: Vec<f64>,
    /// Per-state λ (empty → 1.0 per state, Gaussian mode).
    pub lambdas: Vec<f64>,
}
#[pymethods]
impl GaussianHmmParamsPy {
    // Constructable from Python (input params, not a read-only result record):
    // `qw.GaussianHmmParamsPy(n_states, delta, gamma_flat, means, stds, lambdas=[])`.
    #[new]
    #[pyo3(signature = (n_states, delta, gamma_flat, means, stds, lambdas = Vec::new()))]
    pub fn new(
        n_states: u32,
        delta: Vec<f64>,
        gamma_flat: Vec<f64>,
        means: Vec<f64>,
        stds: Vec<f64>,
        lambdas: Vec<f64>,
    ) -> Self {
        Self {
            n_states,
            delta,
            gamma_flat,
            means,
            stds,
            lambdas,
        }
    }
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct GaussianHmmFitResultPy {
    pub params: GaussianHmmParamsPy,
    pub log_likelihood: f64,
    pub aic: f64,
    pub bic: f64,
    pub iterations: u32,
    pub viterbi_path: Vec<u32>,
    pub smooth_probs_flat: Vec<f64>,
    pub n_observations: u32,
}

fn gaussian_hmm_params_to_py(p: &CoreGaussianHmmParams) -> GaussianHmmParamsPy {
    let m = p.n_states;
    let mut gamma_flat = Vec::with_capacity(m * m);
    for row in &p.gamma {
        gamma_flat.extend_from_slice(row);
    }
    GaussianHmmParamsPy {
        n_states: m as u32,
        delta: p.delta.clone(),
        gamma_flat,
        means: p.means.clone(),
        stds: p.stds.clone(),
        lambdas: if p.lambdas.is_empty() {
            vec![1.0; m]
        } else {
            p.lambdas.clone()
        },
    }
}

fn core_params_from_py(params: &GaussianHmmParamsPy) -> PyResult<CoreGaussianHmmParams> {
    let m = params.n_states as usize;
    if params.gamma_flat.len() != m * m {
        return Err(PyValueError::new_err(format!(
            "gamma_flat length {} does not match n_states^2 ({})",
            params.gamma_flat.len(),
            m * m
        )));
    }
    let mut gamma = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            gamma[i][j] = params.gamma_flat[i * m + j];
        }
    }
    let lambdas = if params.lambdas.is_empty() {
        vec![1.0; m]
    } else {
        params.lambdas.clone()
    };
    CoreGaussianHmmParams::new_with_lambdas(
        params.delta.clone(),
        gamma,
        params.means.clone(),
        params.stds.clone(),
        lambdas,
    )
    .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
pub fn fit_gaussian_hmm(
    observations: Vec<f64>,
    n_states: u32,
    max_iter: u32,
    fit_lambdas: bool,
) -> PyResult<GaussianHmmFitResultPy> {
    let obs: Vec<f64> = observations.into_iter().filter(|v| v.is_finite()).collect();
    if obs.is_empty() {
        return Err(PyValueError::new_err(
            "cannot fit HMM on empty or all-non-finite observations",
        ));
    }
    let m = n_states.max(2) as usize;
    let config = CoreGaussianHmmFitConfig {
        n_states: m,
        max_iter: max_iter.max(1) as usize,
        emission_family: if fit_lambdas {
            CoreEmissionFamily::Lambda
        } else {
            CoreEmissionFamily::Gaussian
        },
        fit_lambdas,
        ..Default::default()
    };
    let fit = core_fit_em(&obs, &config).map_err(|e| PyValueError::new_err(e.to_string()))?;
    let decode = fit
        .params
        .decode(&obs)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let t_len = obs.len();
    let mut smooth_flat = Vec::with_capacity(m * t_len);
    for t in 0..t_len {
        for st in 0..m {
            smooth_flat.push(decode.smooth_probs[st][t]);
        }
    }
    Ok(GaussianHmmFitResultPy {
        params: gaussian_hmm_params_to_py(&fit.params),
        log_likelihood: fit.log_likelihood,
        aic: fit.aic,
        bic: fit.bic,
        iterations: fit.iterations as u32,
        viterbi_path: decode.viterbi_path.iter().map(|&s| s as u32).collect(),
        smooth_probs_flat: smooth_flat,
        n_observations: t_len as u32,
    })
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct GaussianHmmDiagnosticsPy {
    pub pseudo_residuals: Vec<f64>,
    pub decode_weighted_means: Vec<f64>,
    pub decode_weighted_vols: Vec<f64>,
    pub decode_weighted_lambdas: Vec<f64>,
    pub forecast_state_h1: Vec<f64>,
    pub forecast_vol_h1: f64,
    pub forecast_mean_h1: f64,
}

#[pyfunction]
pub fn gaussian_hmm_diagnostics(
    params: GaussianHmmParamsPy,
    observations: Vec<f64>,
) -> PyResult<GaussianHmmDiagnosticsPy> {
    let obs: Vec<f64> = observations.into_iter().filter(|v| v.is_finite()).collect();
    let core_params = core_params_from_py(&params)?;
    let decode = core_params
        .decode(&obs)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let diag = core_params
        .diagnostics(&decode, &obs)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(GaussianHmmDiagnosticsPy {
        pseudo_residuals: diag.pseudo_residuals,
        decode_weighted_means: diag.decode_stats.iter().map(|r| r.weighted_mean).collect(),
        decode_weighted_vols: diag.decode_stats.iter().map(|r| r.weighted_vol).collect(),
        decode_weighted_lambdas: diag
            .decode_stats
            .iter()
            .map(|r| r.weighted_lambda)
            .collect(),
        forecast_state_h1: diag.forecast_state_h1,
        forecast_vol_h1: diag.forecast_vol_h1,
        forecast_mean_h1: diag.forecast_mean_h1,
    })
}

#[pyfunction]
pub fn gaussian_hmm_forecast_vol(
    params: GaussianHmmParamsPy,
    current_state_probs: Vec<f64>,
    horizon: u32,
) -> PyResult<f64> {
    let core_params = core_params_from_py(&params)?;
    core_forecast_volatility(&core_params, &current_state_probs, horizon.max(1) as usize)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
pub fn gaussian_hmm_forecast_state(
    params: GaussianHmmParamsPy,
    current_state_probs: Vec<f64>,
    horizon: u32,
) -> PyResult<Vec<f64>> {
    let core_params = core_params_from_py(&params)?;
    core_forecast_state(&core_params, &current_state_probs, horizon.max(1) as usize)
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyclass]
pub struct GaussianHmmFilterPy {
    inner: Mutex<CoreGaussianHmmFilter>,
}
#[pymethods]
impl GaussianHmmFilterPy {
    // Named alternate constructor (see BullBearHMM::bull_bear): classmethod, not #[new].
    #[staticmethod]
    pub fn from_params(params: GaussianHmmParamsPy) -> PyResult<Self> {
        let core_params = core_params_from_py(&params)?;
        Ok(Self {
            inner: Mutex::new(core_params.filter()),
        })
    }

    pub fn next(&self, observation: f64) -> PyResult<Vec<f64>> {
        if !observation.is_finite() {
            return self.state_probabilities();
        }
        Ok(lock_inner(&self.inner)?.next(observation))
    }

    pub fn state_probabilities(&self) -> PyResult<Vec<f64>> {
        Ok(lock_inner(&self.inner)?.state_probabilities())
    }
}

// Simple regime -> feature vector helper (for completeness; notebook primarily uses the extractors)
#[pyclass(get_all)]
#[derive(Clone)]
pub struct RegimeFeaturesResult {
    pub regime_vector: Vec<f64>, // 5-elem one-hot style
    pub regime_label: i32,       // 0=Bull,1=Bear,2=Crisis,3=Steady,4+=Cluster
}
#[pyfunction]
pub fn regime_to_features(regime_id: u32) -> RegimeFeaturesResult {
    let regime = match regime_id {
        0 => quantwave_core::regimes::MarketRegime::Bull,
        1 => quantwave_core::regimes::MarketRegime::Bear,
        2 => quantwave_core::regimes::MarketRegime::Crisis,
        3 => quantwave_core::regimes::MarketRegime::Steady,
        v => quantwave_core::regimes::MarketRegime::Cluster((v.saturating_sub(4)) as u8),
    };
    let core_f = core_regime_to_features(regime);
    let vec = core_f.regime_vector.to_vec();
    let label = match core_f.regime_label {
        quantwave_core::regimes::MarketRegime::Bull => 0,
        quantwave_core::regimes::MarketRegime::Bear => 1,
        quantwave_core::regimes::MarketRegime::Crisis => 2,
        quantwave_core::regimes::MarketRegime::Steady => 3,
        quantwave_core::regimes::MarketRegime::Cluster(c) => 4 + c as i32,
    };
    RegimeFeaturesResult {
        regime_vector: vec,
        regime_label: label,
    }
}

// === 5thj PA foundation Python surface (MarketStructure + Geometric) ===
// Enables pure-Python notebook usage (streaming loops to build columns) + parity with Polars Rust path.
// Rich outputs (esp. pole_length_atr, flips, bias) feed strategy filters + dynamic sizing in backtester demo.
// Sources: quantwave-core indicators/market_structure + geometric_patterns (MQL5 21/66/69).

use quantwave_core::indicators::geometric_patterns::GeometricPatternScanner as CoreGeo;
use quantwave_core::indicators::market_structure::{
    Bias as CoreBias, ExtremeReclaim as CoreReclaim, MarketStructure as CoreMS,
    PriceImbalance as CoreImbalance,
};
use quantwave_core::indicators::series_norm::{
    PercentRank as CorePercentRank, Zscore as CoreZscore,
};

#[pyclass]
pub struct MarketStructure {
    inner: Mutex<CoreMS>,
}
#[pymethods]
impl MarketStructure {
    #[new]
    pub fn new(swing_strength: u64) -> Self {
        Self {
            inner: Mutex::new(CoreMS::new(swing_strength as usize)),
        }
    }
    pub fn next(&self, high: f64, low: f64) -> PyResult<MarketStructureStateResult> {
        let mut guard = lock_inner(&self.inner)?;
        let state = guard.next((high, low));
        Ok(MarketStructureStateResult {
            bias: match state.bias {
                CoreBias::Neutral => 0,
                CoreBias::Bullish => 1,
                CoreBias::Bearish => 2,
            },
            last_swing_high: state.last_swing_high.map(|sp| SwingPointResult {
                bar: sp.bar as u64,
                price: sp.price,
                is_high: sp.is_high,
            }),
            last_swing_low: state.last_swing_low.map(|sp| SwingPointResult {
                bar: sp.bar as u64,
                price: sp.price,
                is_high: sp.is_high,
            }),
            current_flip: state.current_flip.map(|f| FlipEventResult {
                is_bearish: f.is_bearish,
                price: f.price,
                bar: f.bar as u64,
                structure_strength: f.structure_strength,
            }),
            swing_depth_used: state.swing_depth_used as u32,
            bar_index: state.bar_index as u64,
        })
    }
}

#[pyclass]
pub struct GeometricPatternScanner {
    inner: Mutex<CoreGeo>,
}
#[pymethods]
impl GeometricPatternScanner {
    #[new]
    pub fn new(swing_strength: u64) -> Self {
        Self {
            inner: Mutex::new(CoreGeo::new(swing_strength as usize)),
        }
    }
    pub fn next(&self, high: f64, low: f64) -> PyResult<GeometricNextResult> {
        let mut guard = lock_inner(&self.inner)?;
        let (state, flag, hs) = guard.next((high, low));
        let ms_res = MarketStructureStateResult {
            bias: match state.bias {
                CoreBias::Neutral => 0,
                CoreBias::Bullish => 1,
                CoreBias::Bearish => 2,
            },
            last_swing_high: state.last_swing_high.map(|sp| SwingPointResult {
                bar: sp.bar as u64,
                price: sp.price,
                is_high: sp.is_high,
            }),
            last_swing_low: state.last_swing_low.map(|sp| SwingPointResult {
                bar: sp.bar as u64,
                price: sp.price,
                is_high: sp.is_high,
            }),
            current_flip: state.current_flip.map(|f| FlipEventResult {
                is_bearish: f.is_bearish,
                price: f.price,
                bar: f.bar as u64,
                structure_strength: f.structure_strength,
            }),
            swing_depth_used: state.swing_depth_used as u32,
            bar_index: state.bar_index as u64,
        };
        let flag_res = flag.map(|f| FlagPatternResult {
            id: f.id,
            is_bull: f.is_bull,
            pole_start_bar: f.pole_start_bar as u64,
            pole_end_bar: f.pole_end_bar as u64,
            flag_start_bar: f.flag_start_bar as u64,
            flag_end_bar: f.flag_end_bar as u64,
            pole_length: f.pole_length,
            pole_length_atr: f.pole_length_atr,
            breakout_confirmed: f.breakout_confirmed,
            breakout_price: f.breakout_price,
        });
        let hs_res = hs.map(|h| HsPatternResult {
            id: h.id,
            is_bearish: h.is_bearish,
            height: h.height,
            height_atr: h.height_atr,
            score: h.score,
            breakout_confirmed: h.breakout_confirmed,
        });
        Ok(GeometricNextResult {
            market_structure: ms_res,
            flag: flag_res,
            hs: hs_res,
        })
    }
}

// Batch helpers for notebook convenience (equivalent to Polars collect over synthetic/real series)
#[pyfunction]
pub fn market_structure_batch(
    swing_strength: u64,
    highs: Vec<f64>,
    lows: Vec<f64>,
) -> Vec<MarketStructureStateResult> {
    let mut ms = CoreMS::new(swing_strength as usize);
    highs
        .into_iter()
        .zip(lows)
        .map(|(h, l)| {
            let st = ms.next((h, l));
            MarketStructureStateResult {
                bias: match st.bias {
                    CoreBias::Neutral => 0,
                    CoreBias::Bullish => 1,
                    CoreBias::Bearish => 2,
                },
                last_swing_high: st.last_swing_high.map(|sp| SwingPointResult {
                    bar: sp.bar as u64,
                    price: sp.price,
                    is_high: sp.is_high,
                }),
                last_swing_low: st.last_swing_low.map(|sp| SwingPointResult {
                    bar: sp.bar as u64,
                    price: sp.price,
                    is_high: sp.is_high,
                }),
                current_flip: st.current_flip.map(|f| FlipEventResult {
                    is_bearish: f.is_bearish,
                    price: f.price,
                    bar: f.bar as u64,
                    structure_strength: f.structure_strength,
                }),
                swing_depth_used: st.swing_depth_used as u32,
                bar_index: st.bar_index as u64,
            }
        })
        .collect()
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct PriceImbalanceResult {
    pub bull_top: f64,
    pub bull_bottom: f64,
    pub bull_gap: f64,
    pub bull_open: bool,
    pub bull_size_ok: bool,
    pub bear_top: f64,
    pub bear_bottom: f64,
    pub bear_gap: f64,
    pub bear_open: bool,
    pub bear_size_ok: bool,
    pub atr: f64,
    pub bar_index: u64,
}

fn imbalance_result(st: quantwave_core::PriceImbalanceState) -> PriceImbalanceResult {
    PriceImbalanceResult {
        bull_top: st.bull_top,
        bull_bottom: st.bull_bottom,
        bull_gap: st.bull_gap,
        bull_open: st.bull_open,
        bull_size_ok: st.bull_size_ok,
        bear_top: st.bear_top,
        bear_bottom: st.bear_bottom,
        bear_gap: st.bear_gap,
        bear_open: st.bear_open,
        bear_size_ok: st.bear_size_ok,
        atr: st.atr,
        bar_index: st.bar_index as u64,
    }
}

#[pyclass]
pub struct PriceImbalance {
    inner: Mutex<CoreImbalance>,
}
#[pymethods]
impl PriceImbalance {
    #[new]
    pub fn new(atr_period: u64, size_k: f64) -> Self {
        Self {
            inner: Mutex::new(CoreImbalance::new(atr_period as usize, size_k)),
        }
    }
    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<PriceImbalanceResult> {
        Ok(imbalance_result(
            lock_inner(&self.inner)?.next((high, low, close)),
        ))
    }
}

#[pyfunction]
pub fn price_imbalance(
    atr_period: u64,
    size_k: f64,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
) -> Vec<PriceImbalanceResult> {
    let mut ind = CoreImbalance::new(atr_period as usize, size_k);
    high.into_iter()
        .zip(low)
        .zip(close)
        .map(|((h, l), c)| imbalance_result(ind.next((h, l, c))))
        .collect()
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct ExtremeReclaimResult {
    pub bull_pierce: bool,
    pub bull_reclaim: bool,
    pub bull_size_ok: bool,
    pub bullish: bool,
    pub bull_level: f64,
    pub bull_depth: f64,
    pub bear_pierce: bool,
    pub bear_reclaim: bool,
    pub bear_size_ok: bool,
    pub bearish: bool,
    pub bear_level: f64,
    pub bear_depth: f64,
    pub atr: f64,
    pub bar_index: u64,
}

fn reclaim_result(st: quantwave_core::ExtremeReclaimState) -> ExtremeReclaimResult {
    ExtremeReclaimResult {
        bull_pierce: st.bull_pierce,
        bull_reclaim: st.bull_reclaim,
        bull_size_ok: st.bull_size_ok,
        bullish: st.bullish,
        bull_level: st.bull_level,
        bull_depth: st.bull_depth,
        bear_pierce: st.bear_pierce,
        bear_reclaim: st.bear_reclaim,
        bear_size_ok: st.bear_size_ok,
        bearish: st.bearish,
        bear_level: st.bear_level,
        bear_depth: st.bear_depth,
        atr: st.atr,
        bar_index: st.bar_index as u64,
    }
}

#[pyclass]
pub struct ExtremeReclaim {
    inner: Mutex<CoreReclaim>,
}
#[pymethods]
impl ExtremeReclaim {
    #[new]
    pub fn new(window: u64, atr_period: u64, size_k: f64) -> Self {
        Self {
            inner: Mutex::new(CoreReclaim::new(
                window as usize,
                atr_period as usize,
                size_k,
            )),
        }
    }
    pub fn next(&self, high: f64, low: f64, close: f64) -> PyResult<ExtremeReclaimResult> {
        Ok(reclaim_result(
            lock_inner(&self.inner)?.next((high, low, close)),
        ))
    }
}

#[pyfunction]
pub fn extreme_reclaim(
    window: u64,
    atr_period: u64,
    size_k: f64,
    high: Vec<f64>,
    low: Vec<f64>,
    close: Vec<f64>,
) -> Vec<ExtremeReclaimResult> {
    let mut ind = CoreReclaim::new(window as usize, atr_period as usize, size_k);
    high.into_iter()
        .zip(low)
        .zip(close)
        .map(|((h, l), c)| reclaim_result(ind.next((h, l, c))))
        .collect()
}

#[pyclass]
pub struct PercentRank {
    inner: Mutex<CorePercentRank>,
}
#[pymethods]
impl PercentRank {
    #[new]
    pub fn new(period: u64) -> Self {
        Self {
            inner: Mutex::new(CorePercentRank::new(period as usize)),
        }
    }
    pub fn next(&self, value: f64) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next(value))
    }
}

#[pyfunction]
pub fn percent_rank(period: u64, series: Vec<f64>) -> Vec<f64> {
    let mut ind = CorePercentRank::new(period as usize);
    series.into_iter().map(|x| ind.next(x)).collect()
}

#[pyclass]
pub struct Zscore {
    inner: Mutex<CoreZscore>,
}
#[pymethods]
impl Zscore {
    #[new]
    pub fn new(period: u64) -> Self {
        Self {
            inner: Mutex::new(CoreZscore::new(period as usize)),
        }
    }
    pub fn next(&self, value: f64) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next(value))
    }
}

#[pyfunction]
pub fn zscore(period: u64, series: Vec<f64>) -> Vec<f64> {
    let mut ind = CoreZscore::new(period as usize);
    series.into_iter().map(|x| ind.next(x)).collect()
}

// --- quantwave-lt3t: streaming_class() PyO3-binding gap fixes ---
//
// These indicators already had a real `Next<T>` streaming implementation in
// quantwave-core; only the PyO3 binding (and thus `quantwave.streaming_class()`)
// was missing. See docs/generated/streaming_gap_findings.md for the full audit.

export_1_in_1_out!(Cmo, CoreCmo, (timeperiod: i64));
export_1_in_1_out!(Trix, CoreTrix, (timeperiod: i64));
export_1_in_1_out!(Trima, CoreTrima, (timeperiod: i64));
export_1_in_1_out!(Stddev, CoreStddev, (timeperiod: i64, nbdev: f64));
export_1_in_1_out!(Linreg, CoreLinreg, (timeperiod: i64));
export_1_in_1_out!(Zlema, CoreZlema, (period: i64));
export_1_in_1_out!(ReverseEma, CoreReverseEma, (alpha: f64));
export_1_in_1_out!(AutotuneFilter, CoreAutotuneFilter, (window: i64, bandwidth: f64));
export_1_in_1_out!(Sdo, CoreSdo, (lookback_period: i64, period: i64, ema_pds: i64));
export_1_in_1_out!(KinematicKalman, CoreKinematicKalman, (q_pos: f64, q_vel: f64, r: f64));

export_ohlc_in_1_out!(Ultosc, CoreUltosc, (timeperiod1: i64, timeperiod2: i64, timeperiod3: i64));
export_ohlc_in_1_out!(Natr, CoreNatr, (timeperiod: i64));
export_ohlc_in_1_out!(TrueRange, CoreTrueRange, ());
export_ohlc_in_1_out!(Typprice, CoreTypprice, ());
export_ohlc_in_1_out!(Wclprice, CoreWclprice, ());
export_ohlc_in_1_out!(AdaptiveEma, CoreAdaptiveEma, (period: i64, pds: i64));
export_ohlc_in_1_out!(TradjEma, CoreTradjEma, (period: i64, pds: i64, mltp: f64));
export_ohlc_in_1_out!(HarringtonAdx, CoreHarringtonAdx, (adx_length: i64, adx_smooth_length: i64));

export_hl_in_1_out!(Medprice, CoreMedprice, ());

export_co_in_1_out!(Oc2, CoreOc2, ());

export_hlcv_in_1_out!(Mfi, CoreMfi, (timeperiod: i64));
export_hlcv_in_1_out!(Vpn, CoreVpn, (period: i64, smooth_period: i64));

export_ohlc4_in_1_out!(Avgprice, CoreAvgprice, ());

// APO/PPO: `matype` mirrors the classic TA-Lib default (SMA) — the .ta batch
// surface for apo/ppo does not expose an alternate MA type either.
#[doc = "Calculates Absolute Price Oscillator (APO).\n\nBoundary Conditions & Error Behavior:\n- Negative Params: Negative period/length parameters will raise a ValueError."]
#[pyfunction]
pub fn apo(fastperiod: i64, slowperiod: i64, series: Vec<f64>) -> PyResult<Vec<f64>> {
    fastperiod.validate("fastperiod")?;
    slowperiod.validate("slowperiod")?;
    let mut indicator = CoreApo::new(
        fastperiod as usize,
        slowperiod as usize,
        quantwave_core::MaType::Sma,
    );
    Ok(series.iter().map(|&x| indicator.next(x)).collect())
}
#[pyclass]
pub struct Apo {
    inner: Mutex<CoreApo>,
}
#[pymethods]
impl Apo {
    #[new]
    pub fn new(fastperiod: i64, slowperiod: i64) -> PyResult<Self> {
        fastperiod.validate("fastperiod")?;
        slowperiod.validate("slowperiod")?;
        Ok(Self {
            inner: Mutex::new(CoreApo::new(
                fastperiod as usize,
                slowperiod as usize,
                quantwave_core::MaType::Sma,
            )),
        })
    }
    pub fn next(&self, input: f64) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next(input))
    }
}

#[doc = "Calculates Percentage Price Oscillator (PPO).\n\nBoundary Conditions & Error Behavior:\n- Negative Params: Negative period/length parameters will raise a ValueError."]
#[pyfunction]
pub fn ppo(fastperiod: i64, slowperiod: i64, series: Vec<f64>) -> PyResult<Vec<f64>> {
    fastperiod.validate("fastperiod")?;
    slowperiod.validate("slowperiod")?;
    let mut indicator = CorePpo::new(
        fastperiod as usize,
        slowperiod as usize,
        quantwave_core::MaType::Sma,
    );
    Ok(series.iter().map(|&x| indicator.next(x)).collect())
}
#[pyclass]
pub struct Ppo {
    inner: Mutex<CorePpo>,
}
#[pymethods]
impl Ppo {
    #[new]
    pub fn new(fastperiod: i64, slowperiod: i64) -> PyResult<Self> {
        fastperiod.validate("fastperiod")?;
        slowperiod.validate("slowperiod")?;
        Ok(Self {
            inner: Mutex::new(CorePpo::new(
                fastperiod as usize,
                slowperiod as usize,
                quantwave_core::MaType::Sma,
            )),
        })
    }
    pub fn next(&self, input: f64) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next(input))
    }
}

// CORREL/BETA: TA-Lib's real0/real1 — two arbitrary equal-length price series,
// not high/low. Named to match the existing talib.CORREL/talib.BETA convention.
#[doc = "Calculates Pearson correlation coefficient between two series.\n\nBoundary Conditions & Error Behavior:\n- Input Lengths: real0 and real1 must have identical lengths, otherwise ValueError is raised.\n- Negative Params: Negative period/length parameters will raise a ValueError."]
#[pyfunction]
pub fn correl(timeperiod: i64, real0: Vec<f64>, real1: Vec<f64>) -> PyResult<Vec<f64>> {
    timeperiod.validate("timeperiod")?;
    if real0.len() != real1.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: real0={}, real1={}",
            real0.len(),
            real1.len()
        )));
    }
    let mut indicator = CoreCorrel::new(timeperiod as usize);
    Ok(real0
        .iter()
        .zip(real1.iter())
        .map(|(&x, &y)| indicator.next((x, y)))
        .collect())
}

#[doc = "Pearson correlation coefficient streaming indicator.\n\nBoundary Conditions & Error Behavior:\n- Negative Params: Negative period/length parameters will raise a ValueError."]
#[pyclass]
pub struct Correl {
    inner: Mutex<CoreCorrel>,
}

#[pymethods]
impl Correl {
    #[new]
    pub fn new(timeperiod: i64) -> PyResult<Self> {
        timeperiod.validate("timeperiod")?;
        Ok(Self {
            inner: Mutex::new(CoreCorrel::new(timeperiod as usize)),
        })
    }

    pub fn next(&self, real0: f64, real1: f64) -> PyResult<f64> {
        Ok(lock_inner(&self.inner)?.next((real0, real1)))
    }
}

/// Beta of a stock / asset against a market benchmark: Cov(stock, market) / Var(market).
///
/// Parameters:
///   timeperiod: Rolling window length in bars (must be >= 1).
///   market: Benchmark index/market series (X, denominator).
///   stock: Stock/asset series (Y, numerator).
///   real0: Alias for market (TA-Lib convention).
///   real1: Alias for stock (TA-Lib convention).
#[pyfunction]
#[pyo3(signature = (timeperiod, market=None, stock=None, real0=None, real1=None))]
pub fn beta(
    timeperiod: i64,
    market: Option<Vec<f64>>,
    stock: Option<Vec<f64>>,
    real0: Option<Vec<f64>>,
    real1: Option<Vec<f64>>,
) -> PyResult<Vec<f64>> {
    timeperiod.validate("timeperiod")?;
    let m = market
        .or(real0)
        .ok_or_else(|| PyValueError::new_err("missing required series: 'market' (or 'real0')"))?;
    let s = stock
        .or(real1)
        .ok_or_else(|| PyValueError::new_err("missing required series: 'stock' (or 'real1')"))?;
    if m.len() != s.len() {
        return Err(PyValueError::new_err(format!(
            "input series lengths must match: market={}, stock={}",
            m.len(),
            s.len()
        )));
    }
    let mut indicator = CoreBeta::new(timeperiod as usize);
    Ok(m.iter()
        .zip(s.iter())
        .map(|(&x, &y)| indicator.next((x, y)))
        .collect())
}

#[doc = "Beta streaming indicator: Cov(stock, market) / Var(market).\n\nParameters:\n  timeperiod: Rolling window length (must be >= 1)."]
#[pyclass]
pub struct Beta {
    inner: Mutex<CoreBeta>,
}

#[pymethods]
impl Beta {
    #[new]
    pub fn new(timeperiod: i64) -> PyResult<Self> {
        timeperiod.validate("timeperiod")?;
        Ok(Self {
            inner: Mutex::new(CoreBeta::new(timeperiod as usize)),
        })
    }

    #[pyo3(signature = (market=None, stock=None, real0=None, real1=None))]
    pub fn next(
        &self,
        market: Option<f64>,
        stock: Option<f64>,
        real0: Option<f64>,
        real1: Option<f64>,
    ) -> PyResult<f64> {
        let m = market
            .or(real0)
            .ok_or_else(|| PyValueError::new_err("missing required argument: market (or real0)"))?;
        let s = stock
            .or(real1)
            .ok_or_else(|| PyValueError::new_err("missing required argument: stock (or real1)"))?;
        let mut guard = lock_inner(&self.inner)?;
        Ok(guard.next((m, s)))
    }
}

#[doc = "Rolling Maximum (MAX) over lookback period.\n\nParameters:\n  series: Input price/data series.\n  timeperiod: Lookback window length in bars (must be >= 1)."]
#[pyfunction]
#[pyo3(signature = (series, timeperiod=14))]
pub fn ta_max(series: Vec<f64>, timeperiod: i64) -> PyResult<Vec<f64>> {
    timeperiod.validate("timeperiod")?;
    let mut indicator = CoreMax::new(timeperiod as usize);
    Ok(series.iter().map(|&x| indicator.next(x)).collect())
}

#[doc = "Rolling Minimum (MIN) over lookback period.\n\nParameters:\n  series: Input price/data series.\n  timeperiod: Lookback window length in bars (must be >= 1)."]
#[pyfunction]
#[pyo3(signature = (series, timeperiod=14))]
pub fn ta_min(series: Vec<f64>, timeperiod: i64) -> PyResult<Vec<f64>> {
    timeperiod.validate("timeperiod")?;
    let mut indicator = CoreMin::new(timeperiod as usize);
    Ok(series.iter().map(|&x| indicator.next(x)).collect())
}

// GapMomentum: Next<(open, close)> -> (gap_ratio, signal). Result field names
// match the `gap_momentum` entry already recorded in scripts/metadata_overlay.json.
#[pyclass(get_all)]
#[derive(Clone)]
pub struct GapMomentumResult {
    pub gap_ratio: f64,
    pub gap_signal: f64,
}
#[pyfunction]
pub fn gap_momentum(
    period: u64,
    signal_period: u64,
    open: Vec<f64>,
    close: Vec<f64>,
) -> Vec<GapMomentumResult> {
    let mut indicator = CoreGapMomentum::new(period as usize, signal_period as usize);
    open.iter()
        .zip(close.iter())
        .map(|(&o, &c)| {
            let res = indicator.next((o, c));
            GapMomentumResult {
                gap_ratio: res.0,
                gap_signal: res.1,
            }
        })
        .collect()
}
#[pyclass]
pub struct GapMomentum {
    inner: Mutex<CoreGapMomentum>,
}
#[pymethods]
impl GapMomentum {
    #[new]
    pub fn new(period: u64, signal_period: u64) -> Self {
        Self {
            inner: Mutex::new(CoreGapMomentum::new(
                period as usize,
                signal_period as usize,
            )),
        }
    }
    pub fn next(&self, open: f64, close: f64) -> PyResult<GapMomentumResult> {
        let res = lock_inner(&self.inner)?.next((open, close));
        Ok(GapMomentumResult {
            gap_ratio: res.0,
            gap_signal: res.1,
        })
    }
}

// --- Candlestick patterns (quantwave-lt3t): all 61 already have native
// O(1) `Next<(f64,f64,f64,f64)>` streaming implementations in quantwave-core
// (see quantwave-core/src/indicators/patterns/); only the PyO3 binding was
// missing. Zero-arg constructors — no configurable `penetration`/settings
// surface exists in the Rust layer today.
export_ohlc4_in_1_out!(Cdl2Crows, CDL2CROWS, ());
export_ohlc4_in_1_out!(Cdl3BlackCrows, CDL3BLACKCROWS, ());
export_ohlc4_in_1_out!(Cdl3Inside, CDL3INSIDE, ());
export_ohlc4_in_1_out!(Cdl3LineStrike, CDL3LINESTRIKE, ());
export_ohlc4_in_1_out!(Cdl3Outside, CDL3OUTSIDE, ());
export_ohlc4_in_1_out!(Cdl3StarsInSouth, CDL3STARSINSOUTH, ());
export_ohlc4_in_1_out!(Cdl3WhiteSoldiers, CDL3WHITESOLDIERS, ());
export_ohlc4_in_1_out!(CdlAbandonedBaby, CDLABANDONEDBABY, ());
export_ohlc4_in_1_out!(CdlAdvanceBlock, CDLADVANCEBLOCK, ());
export_ohlc4_in_1_out!(CdlBeltHold, CDLBELTHOLD, ());
export_ohlc4_in_1_out!(CdlBreakaway, CDLBREAKAWAY, ());
export_ohlc4_in_1_out!(CdlClosingMarubozu, CDLCLOSINGMARUBOZU, ());
export_ohlc4_in_1_out!(CdlConcealBabySwall, CDLCONCEALBABYSWALL, ());
export_ohlc4_in_1_out!(CdlCounterattack, CDLCOUNTERATTACK, ());
export_ohlc4_in_1_out!(CdlDarkCloudCover, CDLDARKCLOUDCOVER, ());
export_ohlc4_in_1_out!(CdlDoji, CDLDOJI, ());
export_ohlc4_in_1_out!(CdlDojiStar, CDLDOJISTAR, ());
export_ohlc4_in_1_out!(CdlDragonflyDoji, CDLDRAGONFLYDOJI, ());
export_ohlc4_in_1_out!(CdlEngulfing, CDLENGULFING, ());
export_ohlc4_in_1_out!(CdlEveningDojiStar, CDLEVENINGDOJISTAR, ());
export_ohlc4_in_1_out!(CdlEveningStar, CDLEVENINGSTAR, ());
export_ohlc4_in_1_out!(CdlGapSideSideWhite, CDLGAPSIDESIDEWHITE, ());
export_ohlc4_in_1_out!(CdlGravestoneDoji, CDLGRAVESTONEDOJI, ());
export_ohlc4_in_1_out!(CdlHammer, CDLHAMMER, ());
export_ohlc4_in_1_out!(CdlHangingMan, CDLHANGINGMAN, ());
export_ohlc4_in_1_out!(CdlHarami, CDLHARAMI, ());
export_ohlc4_in_1_out!(CdlHaramiCross, CDLHARAMICROSS, ());
export_ohlc4_in_1_out!(CdlHighWave, CDLHIGHWAVE, ());
export_ohlc4_in_1_out!(CdlHikkake, CDLHIKKAKE, ());
export_ohlc4_in_1_out!(CdlHikkakeMod, CDLHIKKAKEMOD, ());
export_ohlc4_in_1_out!(CdlHomingPigeon, CDLHOMINGPIGEON, ());
export_ohlc4_in_1_out!(CdlIdentical3Crows, CDLIDENTICAL3CROWS, ());
export_ohlc4_in_1_out!(CdlInNeck, CDLINNECK, ());
export_ohlc4_in_1_out!(CdlInvertedHammer, CDLINVERTEDHAMMER, ());
export_ohlc4_in_1_out!(CdlKicking, CDLKICKING, ());
export_ohlc4_in_1_out!(CdlKickingByLength, CDLKICKINGBYLENGTH, ());
export_ohlc4_in_1_out!(CdlLadderBottom, CDLLADDERBOTTOM, ());
export_ohlc4_in_1_out!(CdlLongLeggedDoji, CDLLONGLEGGEDDOJI, ());
export_ohlc4_in_1_out!(CdlLongLine, CDLLONGLINE, ());
export_ohlc4_in_1_out!(CdlMarubozu, CDLMARUBOZU, ());
export_ohlc4_in_1_out!(CdlMatchingLow, CDLMATCHINGLOW, ());
export_ohlc4_in_1_out!(CdlMatHold, CDLMATHOLD, ());
export_ohlc4_in_1_out!(CdlMorningDojiStar, CDLMORNINGDOJISTAR, ());
export_ohlc4_in_1_out!(CdlMorningStar, CDLMORNINGSTAR, ());
export_ohlc4_in_1_out!(CdlOnNeck, CDLONNECK, ());
export_ohlc4_in_1_out!(CdlPiercing, CDLPIERCING, ());
export_ohlc4_in_1_out!(CdlRickshawMan, CDLRICKSHAWMAN, ());
export_ohlc4_in_1_out!(CdlRiseFall3Methods, CDLRISEFALL3METHODS, ());
export_ohlc4_in_1_out!(CdlSeparatingLines, CDLSEPARATINGLINES, ());
export_ohlc4_in_1_out!(CdlShootingStar, CDLSHOOTINGSTAR, ());
export_ohlc4_in_1_out!(CdlShortLine, CDLSHORTLINE, ());
export_ohlc4_in_1_out!(CdlSpinningTop, CDLSPINNINGTOP, ());
export_ohlc4_in_1_out!(CdlStalledPattern, CDLSTALLEDPATTERN, ());
export_ohlc4_in_1_out!(CdlStickSandwich, CDLSTICKSANDWICH, ());
export_ohlc4_in_1_out!(CdlTakuri, CDLTAKURI, ());
export_ohlc4_in_1_out!(CdlTasukiGap, CDLTASUKIGAP, ());
export_ohlc4_in_1_out!(CdlThrusting, CDLTHRUSTING, ());
export_ohlc4_in_1_out!(CdlTristar, CDLTRISTAR, ());
export_ohlc4_in_1_out!(CdlUnique3River, CDLUNIQUE3RIVER, ());
export_ohlc4_in_1_out!(CdlUpsideGap2Crows, CDLUPSIDEGAP2CROWS, ());
export_ohlc4_in_1_out!(CdlXSideGap3Methods, CDLXSIDEGAP3METHODS, ());

// --- PyO3 module registration (generated by transform_lib.py; replaces the uniffi scaffolding macro) ---
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SuperTrendResult>()?;
    m.add_class::<TtmSqueezeResult>()?;
    m.add_class::<MacdResult>()?;
    m.add_class::<BbandsResult>()?;
    m.add_class::<StochResult>()?;
    m.add_class::<MamaResult>()?;
    m.add_class::<AroonResult>()?;
    m.add_class::<IchimokuResult>()?;
    m.add_class::<AlligatorResult>()?;
    m.add_class::<AtrTsResult>()?;
    m.add_class::<DonchianResult>()?;
    m.add_class::<EmdResult>()?;
    m.add_class::<EhlersLoopsResult>()?;
    m.add_class::<FractalsResult>()?;
    m.add_class::<HeikinAshiResult>()?;
    m.add_class::<SwingPointResult>()?;
    m.add_class::<FlipEventResult>()?;
    m.add_class::<MarketStructureStateResult>()?;
    m.add_class::<FlagPatternResult>()?;
    m.add_class::<HsPatternResult>()?;
    m.add_class::<GeometricNextResult>()?;
    m.add_class::<KeltnerResult>()?;
    m.add_class::<PairsRotationResult>()?;
    m.add_class::<PhasorResult>()?;
    m.add_class::<PivotPointsResult>()?;
    m.add_class::<SystemEvaluatorResult>()?;
    m.add_class::<UltimateBandsResult>()?;
    m.add_class::<UltimateChannelResult>()?;
    m.add_class::<VortexResult>()?;
    m.add_class::<WaveTrendResult>()?;
    m.add_class::<VossPredictorResult>()?;
    m.add_class::<CycleTrendAnalyticsResult>()?;
    m.add_class::<ZeroLagResult>()?;
    m.add_class::<CyberCycleResult>()?;
    m.add_class::<HtSineResult>()?;
    m.add_class::<VolumeProfileResult>()?;
    m.add_class::<PmaResult>()?;
    m.add_class::<TrendRocResult>()?;
    m.add_class::<UdmaResult>()?;
    m.add_class::<OIZonesResult>()?;
    m.add_class::<GexResult>()?;
    m.add_class::<StraddleResult>()?;
    m.add_class::<CyberCycleFeaturesResult>()?;
    m.add_class::<HurstFeaturesResult>()?;
    m.add_class::<InstantaneousTrendlineFeaturesResult>()?;
    m.add_class::<TrendflexFeaturesResult>()?;
    m.add_class::<GriffithsDominantCycleFeaturesResult>()?;
    m.add_class::<GaussianHmmParamsPy>()?;
    m.add_class::<GaussianHmmFitResultPy>()?;
    m.add_class::<GaussianHmmDiagnosticsPy>()?;
    m.add_class::<RegimeFeaturesResult>()?;
    m.add_class::<SwissMode>()?;
    m.add_class::<Stoch>()?;
    m.add_class::<Ichimoku>()?;
    m.add_class::<Donchian>()?;
    m.add_class::<HeikinAshi>()?;
    m.add_class::<Keltner>()?;
    m.add_class::<PivotPoints>()?;
    m.add_class::<SwissArmyKnife>()?;
    m.add_class::<RobustnessEvaluator>()?;
    m.add_class::<UltimateBands>()?;
    m.add_class::<UltimateChannel>()?;
    m.add_class::<Ad>()?;
    m.add_class::<Adosc>()?;
    m.add_class::<AnchoredVwap>()?;
    m.add_class::<CycleTrendAnalytics>()?;
    m.add_class::<VolumeProfile>()?;
    m.add_class::<CyberCycleFeatureExtractor>()?;
    m.add_class::<HurstFeatureExtractor>()?;
    m.add_class::<InstantaneousTrendlineFeatureExtractor>()?;
    m.add_class::<TrendflexFeatureExtractor>()?;
    m.add_class::<GriffithsDominantCycleFeatureExtractor>()?;
    m.add_class::<BullBearHMM>()?;
    m.add_class::<GaussianHmmFilterPy>()?;
    m.add_class::<MarketStructure>()?;
    m.add_class::<GeometricPatternScanner>()?;
    m.add_class::<PriceImbalance>()?;
    m.add_class::<PriceImbalanceResult>()?;
    m.add_class::<ExtremeReclaim>()?;
    m.add_class::<ExtremeReclaimResult>()?;
    m.add_class::<PercentRank>()?;
    m.add_class::<Zscore>()?;
    m.add_function(wrap_pyfunction!(price_imbalance, m)?)?;
    m.add_function(wrap_pyfunction!(extreme_reclaim, m)?)?;
    m.add_function(wrap_pyfunction!(percent_rank, m)?)?;
    m.add_function(wrap_pyfunction!(zscore, m)?)?;
    m.add_function(wrap_pyfunction!(stoch, m)?)?;
    m.add_function(wrap_pyfunction!(ichimoku, m)?)?;
    m.add_function(wrap_pyfunction!(donchian, m)?)?;
    m.add_function(wrap_pyfunction!(heikin_ashi, m)?)?;
    m.add_function(wrap_pyfunction!(keltner, m)?)?;
    m.add_function(wrap_pyfunction!(pivot_points, m)?)?;
    m.add_function(wrap_pyfunction!(swiss_army_knife, m)?)?;
    m.add_function(wrap_pyfunction!(ultimate_bands, m)?)?;
    m.add_function(wrap_pyfunction!(ultimate_channel, m)?)?;
    m.add_function(wrap_pyfunction!(ad, m)?)?;
    m.add_function(wrap_pyfunction!(adosc, m)?)?;
    m.add_function(wrap_pyfunction!(anchored_vwap, m)?)?;
    m.add_function(wrap_pyfunction!(cycletrendanalytics, m)?)?;
    m.add_function(wrap_pyfunction!(volumeprofile, m)?)?;
    m.add_function(wrap_pyfunction!(bs_call_price, m)?)?;
    m.add_function(wrap_pyfunction!(bs_put_price, m)?)?;
    m.add_function(wrap_pyfunction!(bs_delta, m)?)?;
    m.add_function(wrap_pyfunction!(bs_gamma, m)?)?;
    m.add_function(wrap_pyfunction!(bs_theta, m)?)?;
    m.add_function(wrap_pyfunction!(bs_vega, m)?)?;
    m.add_function(wrap_pyfunction!(bs_rho, m)?)?;
    m.add_function(wrap_pyfunction!(implied_vol, m)?)?;
    m.add_function(wrap_pyfunction!(max_pain, m)?)?;
    m.add_function(wrap_pyfunction!(strike_pcr, m)?)?;
    m.add_function(wrap_pyfunction!(chain_pcr, m)?)?;
    m.add_function(wrap_pyfunction!(oi_zones, m)?)?;
    m.add_function(wrap_pyfunction!(gex_per_strike, m)?)?;
    m.add_function(wrap_pyfunction!(gex_flip_strike, m)?)?;
    m.add_function(wrap_pyfunction!(gextotal, m)?)?;
    m.add_function(wrap_pyfunction!(atm_straddle, m)?)?;
    m.add_function(wrap_pyfunction!(synthetic_futures, m)?)?;
    m.add_function(wrap_pyfunction!(moneyness, m)?)?;
    m.add_function(wrap_pyfunction!(nse_lot_size, m)?)?;
    m.add_function(wrap_pyfunction!(nse_risk_free_rate, m)?)?;
    m.add_function(wrap_pyfunction!(cyber_cycle_features, m)?)?;
    m.add_function(wrap_pyfunction!(hurst_features, m)?)?;
    m.add_function(wrap_pyfunction!(instantaneous_trendline_features, m)?)?;
    m.add_function(wrap_pyfunction!(trendflex_features, m)?)?;
    m.add_function(wrap_pyfunction!(griffiths_dominant_cycle_features, m)?)?;
    m.add_function(wrap_pyfunction!(fit_gaussian_hmm, m)?)?;
    m.add_function(wrap_pyfunction!(gaussian_hmm_diagnostics, m)?)?;
    m.add_function(wrap_pyfunction!(gaussian_hmm_forecast_vol, m)?)?;
    m.add_function(wrap_pyfunction!(gaussian_hmm_forecast_state, m)?)?;
    m.add_function(wrap_pyfunction!(regime_to_features, m)?)?;
    m.add_function(wrap_pyfunction!(market_structure_batch, m)?)?;
    m.add_class::<Sma>()?;
    m.add_function(wrap_pyfunction!(sma, m)?)?;
    m.add_class::<Ema>()?;
    m.add_function(wrap_pyfunction!(ema, m)?)?;
    m.add_class::<Wma>()?;
    m.add_function(wrap_pyfunction!(wma, m)?)?;
    m.add_class::<Rsi>()?;
    m.add_function(wrap_pyfunction!(rsi, m)?)?;
    m.add_class::<SuperTrend>()?;
    m.add_function(wrap_pyfunction!(supertrend, m)?)?;
    m.add_class::<Macd>()?;
    m.add_function(wrap_pyfunction!(macd, m)?)?;
    m.add_class::<Atr>()?;
    m.add_function(wrap_pyfunction!(atr, m)?)?;
    m.add_class::<AtrWilder>()?;
    m.add_function(wrap_pyfunction!(atr_wilder, m)?)?;
    m.add_class::<TaAtr>()?;
    m.add_function(wrap_pyfunction!(ta_atr, m)?)?;
    m.add_class::<Adx>()?;
    m.add_function(wrap_pyfunction!(adx, m)?)?;
    m.add_class::<Cci>()?;
    m.add_function(wrap_pyfunction!(cci, m)?)?;
    m.add_class::<Aroon>()?;
    m.add_function(wrap_pyfunction!(aroon, m)?)?;
    m.add_class::<Mama>()?;
    m.add_function(wrap_pyfunction!(mama, m)?)?;
    m.add_class::<Kama>()?;
    m.add_function(wrap_pyfunction!(kama, m)?)?;
    m.add_class::<T3>()?;
    m.add_function(wrap_pyfunction!(t3, m)?)?;
    m.add_class::<Sar>()?;
    m.add_function(wrap_pyfunction!(sar, m)?)?;
    m.add_class::<Mom>()?;
    m.add_function(wrap_pyfunction!(mom, m)?)?;
    m.add_class::<Roc>()?;
    m.add_function(wrap_pyfunction!(roc, m)?)?;
    m.add_class::<Rocp>()?;
    m.add_function(wrap_pyfunction!(rocp, m)?)?;
    m.add_class::<Willr>()?;
    m.add_function(wrap_pyfunction!(willr, m)?)?;
    m.add_class::<Dema>()?;
    m.add_function(wrap_pyfunction!(dema, m)?)?;
    m.add_class::<Tema>()?;
    m.add_function(wrap_pyfunction!(tema, m)?)?;
    m.add_class::<Cg>()?;
    m.add_function(wrap_pyfunction!(cg, m)?)?;
    m.add_class::<CyberCycle>()?;
    m.add_function(wrap_pyfunction!(cybercycle, m)?)?;
    m.add_class::<Fisher>()?;
    m.add_function(wrap_pyfunction!(fisher, m)?)?;
    m.add_class::<InverseFisher>()?;
    m.add_function(wrap_pyfunction!(inversefisher, m)?)?;
    m.add_class::<SuperSmoother>()?;
    m.add_function(wrap_pyfunction!(supersmoother, m)?)?;
    m.add_class::<Bandpass>()?;
    m.add_function(wrap_pyfunction!(bandpass, m)?)?;
    m.add_class::<RoofingFilter>()?;
    m.add_function(wrap_pyfunction!(roofingfilter, m)?)?;
    m.add_class::<ZeroLag>()?;
    m.add_function(wrap_pyfunction!(zerolag, m)?)?;
    m.add_class::<ChoppinessIndex>()?;
    m.add_function(wrap_pyfunction!(choppinessindex, m)?)?;
    m.add_class::<ClassicLaguerre>()?;
    m.add_function(wrap_pyfunction!(classiclaguerre, m)?)?;
    m.add_class::<Alligator>()?;
    m.add_function(wrap_pyfunction!(alligator, m)?)?;
    m.add_class::<Alma>()?;
    m.add_function(wrap_pyfunction!(alma, m)?)?;
    m.add_class::<AtrTs>()?;
    m.add_function(wrap_pyfunction!(atrts, m)?)?;
    m.add_class::<Butterworth2>()?;
    m.add_function(wrap_pyfunction!(butterworth2, m)?)?;
    m.add_class::<Butterworth3>()?;
    m.add_function(wrap_pyfunction!(butterworth3, m)?)?;
    m.add_class::<ChannelCycle>()?;
    m.add_function(wrap_pyfunction!(channelcycle, m)?)?;
    m.add_class::<ContinuationIndex>()?;
    m.add_function(wrap_pyfunction!(continuationindex, m)?)?;
    m.add_class::<CorrelationCycle>()?;
    m.add_function(wrap_pyfunction!(correlationcycle, m)?)?;
    m.add_class::<CorrelationTrend>()?;
    m.add_function(wrap_pyfunction!(correlationtrend, m)?)?;
    m.add_class::<CyberneticOscillator>()?;
    m.add_function(wrap_pyfunction!(cyberneticoscillator, m)?)?;
    m.add_class::<Dmh>()?;
    m.add_function(wrap_pyfunction!(dmh, m)?)?;
    m.add_class::<Dsma>()?;
    m.add_function(wrap_pyfunction!(dsma, m)?)?;
    m.add_class::<Emd>()?;
    m.add_function(wrap_pyfunction!(emd, m)?)?;
    m.add_class::<AmDetector>()?;
    m.add_function(wrap_pyfunction!(amdetector, m)?)?;
    m.add_class::<FmDemodulator>()?;
    m.add_function(wrap_pyfunction!(fmdemodulator, m)?)?;
    m.add_class::<EhlersAutocorrelation>()?;
    m.add_function(wrap_pyfunction!(ehlersautocorrelation, m)?)?;
    m.add_class::<EhlersFilter>()?;
    m.add_function(wrap_pyfunction!(ehlersfilter, m)?)?;
    m.add_class::<EhlersLoops>()?;
    m.add_function(wrap_pyfunction!(ehlersloops, m)?)?;
    m.add_class::<EhlersStochastic>()?;
    m.add_function(wrap_pyfunction!(ehlersstochastic, m)?)?;
    m.add_class::<EhlersUltimateOscillator>()?;
    m.add_function(wrap_pyfunction!(ehlersultimateoscillator, m)?)?;
    m.add_class::<FisherHighPass>()?;
    m.add_function(wrap_pyfunction!(fisherhighpass, m)?)?;
    m.add_class::<FourierSeries>()?;
    m.add_function(wrap_pyfunction!(fourierseries, m)?)?;
    m.add_class::<FourierDominantCycle>()?;
    m.add_function(wrap_pyfunction!(fourierdominantcycle, m)?)?;
    m.add_class::<Fractals>()?;
    m.add_function(wrap_pyfunction!(fractals, m)?)?;
    m.add_class::<Frama>()?;
    m.add_function(wrap_pyfunction!(frama, m)?)?;
    m.add_class::<Gaussian>()?;
    m.add_function(wrap_pyfunction!(gaussian, m)?)?;
    m.add_class::<GeneralizedLaguerre>()?;
    m.add_function(wrap_pyfunction!(generalizedlaguerre, m)?)?;
    m.add_class::<GriffithsDominantCycle>()?;
    m.add_function(wrap_pyfunction!(griffithsdominantcycle, m)?)?;
    m.add_class::<GriffithsPredictor>()?;
    m.add_function(wrap_pyfunction!(griffithspredictor, m)?)?;
    m.add_class::<GriffithsSpectrum>()?;
    m.add_function(wrap_pyfunction!(griffithsspectrum, m)?)?;
    m.add_class::<Hamming>()?;
    m.add_function(wrap_pyfunction!(hamming, m)?)?;
    m.add_class::<Hann>()?;
    m.add_function(wrap_pyfunction!(hann, m)?)?;
    m.add_class::<HighPass>()?;
    m.add_function(wrap_pyfunction!(highpass, m)?)?;
    m.add_class::<Hma>()?;
    m.add_function(wrap_pyfunction!(hma, m)?)?;
    m.add_class::<EhlersWma4>()?;
    m.add_function(wrap_pyfunction!(ehlerswma4, m)?)?;
    m.add_class::<InstantaneousTrendline>()?;
    m.add_function(wrap_pyfunction!(instantaneoustrendline, m)?)?;
    m.add_class::<UndersampledDoubleMa>()?;
    m.add_function(wrap_pyfunction!(undersampleddoublema, m)?)?;
    m.add_class::<LaguerreFilter>()?;
    m.add_function(wrap_pyfunction!(laguerrefilter, m)?)?;
    m.add_class::<LaguerreOscillator>()?;
    m.add_function(wrap_pyfunction!(laguerreoscillator, m)?)?;
    m.add_class::<LaguerreRsi>()?;
    m.add_function(wrap_pyfunction!(laguerrersi, m)?)?;
    m.add_class::<NoiseElimination>()?;
    m.add_function(wrap_pyfunction!(noiseelimination, m)?)?;
    m.add_class::<PairsRotation>()?;
    m.add_function(wrap_pyfunction!(pairsrotation, m)?)?;
    m.add_class::<Phasor>()?;
    m.add_function(wrap_pyfunction!(phasor, m)?)?;
    m.add_class::<OcPriceRsi>()?;
    m.add_function(wrap_pyfunction!(ocpricersi, m)?)?;
    m.add_class::<OneEuroFilter>()?;
    m.add_function(wrap_pyfunction!(oneeurofilter, m)?)?;
    m.add_class::<ProjectedMovingAverage>()?;
    m.add_function(wrap_pyfunction!(projectedmovingaverage, m)?)?;
    m.add_class::<PrecisionTrend>()?;
    m.add_function(wrap_pyfunction!(precisiontrend, m)?)?;
    m.add_class::<ReversionIndex>()?;
    m.add_function(wrap_pyfunction!(reversionindex, m)?)?;
    m.add_class::<SineWave>()?;
    m.add_function(wrap_pyfunction!(sinewave, m)?)?;
    m.add_class::<SystemEvaluator>()?;
    m.add_function(wrap_pyfunction!(systemevaluator, m)?)?;
    m.add_class::<TtmSqueeze>()?;
    m.add_function(wrap_pyfunction!(ttmsqueeze, m)?)?;
    m.add_class::<UltimateSmoother>()?;
    m.add_function(wrap_pyfunction!(ultimatesmoother, m)?)?;
    m.add_class::<Usi>()?;
    m.add_function(wrap_pyfunction!(usi, m)?)?;
    m.add_class::<Obv>()?;
    m.add_function(wrap_pyfunction!(obv, m)?)?;
    m.add_class::<Vortex>()?;
    m.add_function(wrap_pyfunction!(vortex, m)?)?;
    m.add_class::<WaveTrend>()?;
    m.add_function(wrap_pyfunction!(wavetrend, m)?)?;
    m.add_class::<SimplePredictor>()?;
    m.add_function(wrap_pyfunction!(simplepredictor, m)?)?;
    m.add_class::<Mad>()?;
    m.add_function(wrap_pyfunction!(mad, m)?)?;
    m.add_class::<MesaStochastic>()?;
    m.add_function(wrap_pyfunction!(mesastochastic, m)?)?;
    m.add_class::<Rsih>()?;
    m.add_function(wrap_pyfunction!(rsih, m)?)?;
    m.add_class::<VossPredictor>()?;
    m.add_function(wrap_pyfunction!(vosspredictor, m)?)?;
    m.add_class::<SyntheticOscillator>()?;
    m.add_function(wrap_pyfunction!(syntheticoscillator, m)?)?;
    m.add_class::<Madh>()?;
    m.add_function(wrap_pyfunction!(madh, m)?)?;
    m.add_class::<Stc>()?;
    m.add_function(wrap_pyfunction!(stc, m)?)?;
    m.add_class::<HomodyneDiscriminator>()?;
    m.add_function(wrap_pyfunction!(homodynediscriminator, m)?)?;
    m.add_class::<UniversalOscillator>()?;
    m.add_function(wrap_pyfunction!(universaloscillator, m)?)?;
    m.add_class::<TriangleFilter>()?;
    m.add_function(wrap_pyfunction!(trianglefilter, m)?)?;
    m.add_class::<HtDcPeriod>()?;
    m.add_function(wrap_pyfunction!(htdcperiod, m)?)?;
    m.add_class::<HtPhasor>()?;
    m.add_function(wrap_pyfunction!(htphasor, m)?)?;
    m.add_class::<HtDcPhase>()?;
    m.add_function(wrap_pyfunction!(htdcphase, m)?)?;
    m.add_class::<HtSine>()?;
    m.add_function(wrap_pyfunction!(htsine, m)?)?;
    m.add_class::<HtTrendMode>()?;
    m.add_function(wrap_pyfunction!(httrendmode, m)?)?;
    m.add_class::<HurstExponent>()?;
    m.add_function(wrap_pyfunction!(hurstexponent, m)?)?;
    m.add_class::<FracDiff>()?;
    m.add_function(wrap_pyfunction!(fracdiff, m)?)?;
    m.add_class::<KalmanFilter>()?;
    m.add_function(wrap_pyfunction!(kalmanfilter, m)?)?;
    m.add_class::<MarketState>()?;
    m.add_function(wrap_pyfunction!(marketstate, m)?)?;
    m.add_class::<RecursiveMedian>()?;
    m.add_function(wrap_pyfunction!(recursivemedian, m)?)?;
    m.add_class::<RecursiveMedianOscillator>()?;
    m.add_function(wrap_pyfunction!(recursivemedianoscillator, m)?)?;
    m.add_class::<Reflex>()?;
    m.add_function(wrap_pyfunction!(reflex, m)?)?;
    m.add_class::<RocketRsi>()?;
    m.add_function(wrap_pyfunction!(rocketrsi, m)?)?;
    m.add_class::<Trendflex>()?;
    m.add_function(wrap_pyfunction!(trendflex, m)?)?;
    m.add_class::<TruncatedBandpass>()?;
    m.add_function(wrap_pyfunction!(truncatedbandpass, m)?)?;

    // -- quantwave-lt3t: streaming_class() PyO3-binding gap fixes --
    m.add_class::<Cmo>()?;
    m.add_function(wrap_pyfunction!(cmo, m)?)?;
    m.add_class::<Trix>()?;
    m.add_function(wrap_pyfunction!(trix, m)?)?;
    m.add_class::<Trima>()?;
    m.add_function(wrap_pyfunction!(trima, m)?)?;
    m.add_class::<Stddev>()?;
    m.add_function(wrap_pyfunction!(stddev, m)?)?;
    m.add_class::<Linreg>()?;
    m.add_function(wrap_pyfunction!(linreg, m)?)?;
    m.add_class::<Zlema>()?;
    m.add_function(wrap_pyfunction!(zlema, m)?)?;
    m.add_class::<ReverseEma>()?;
    m.add_function(wrap_pyfunction!(reverseema, m)?)?;
    m.add_class::<AutotuneFilter>()?;
    m.add_function(wrap_pyfunction!(autotunefilter, m)?)?;
    m.add_class::<Sdo>()?;
    m.add_function(wrap_pyfunction!(sdo, m)?)?;
    m.add_class::<KinematicKalman>()?;
    m.add_function(wrap_pyfunction!(kinematickalman, m)?)?;
    m.add_class::<Ultosc>()?;
    m.add_function(wrap_pyfunction!(ultosc, m)?)?;
    m.add_class::<Natr>()?;
    m.add_function(wrap_pyfunction!(natr, m)?)?;
    m.add_class::<TrueRange>()?;
    m.add_function(wrap_pyfunction!(truerange, m)?)?;
    m.add_class::<Typprice>()?;
    m.add_function(wrap_pyfunction!(typprice, m)?)?;
    m.add_class::<Wclprice>()?;
    m.add_function(wrap_pyfunction!(wclprice, m)?)?;
    m.add_class::<AdaptiveEma>()?;
    m.add_function(wrap_pyfunction!(adaptiveema, m)?)?;
    m.add_class::<TradjEma>()?;
    m.add_function(wrap_pyfunction!(tradjema, m)?)?;
    m.add_class::<HarringtonAdx>()?;
    m.add_function(wrap_pyfunction!(harringtonadx, m)?)?;
    m.add_class::<Medprice>()?;
    m.add_function(wrap_pyfunction!(medprice, m)?)?;
    m.add_class::<Oc2>()?;
    m.add_function(wrap_pyfunction!(oc2, m)?)?;
    m.add_class::<Mfi>()?;
    m.add_function(wrap_pyfunction!(mfi, m)?)?;
    m.add_class::<Vpn>()?;
    m.add_function(wrap_pyfunction!(vpn, m)?)?;
    m.add_class::<Avgprice>()?;
    m.add_function(wrap_pyfunction!(avgprice, m)?)?;
    m.add_class::<Apo>()?;
    m.add_function(wrap_pyfunction!(apo, m)?)?;
    m.add_class::<Ppo>()?;
    m.add_function(wrap_pyfunction!(ppo, m)?)?;
    m.add_class::<Correl>()?;
    m.add_function(wrap_pyfunction!(correl, m)?)?;
    m.add_class::<Beta>()?;
    m.add_function(wrap_pyfunction!(beta, m)?)?;
    m.add_class::<GapMomentumResult>()?;
    m.add_class::<GapMomentum>()?;
    m.add_function(wrap_pyfunction!(gap_momentum, m)?)?;

    // -- quantwave-lt3t: candlestick pattern streaming bindings (all 61) --
    m.add_class::<Cdl2Crows>()?;
    m.add_function(wrap_pyfunction!(cdl2crows, m)?)?;
    m.add_class::<Cdl3BlackCrows>()?;
    m.add_function(wrap_pyfunction!(cdl3blackcrows, m)?)?;
    m.add_class::<Cdl3Inside>()?;
    m.add_function(wrap_pyfunction!(cdl3inside, m)?)?;
    m.add_class::<Cdl3LineStrike>()?;
    m.add_function(wrap_pyfunction!(cdl3linestrike, m)?)?;
    m.add_class::<Cdl3Outside>()?;
    m.add_function(wrap_pyfunction!(cdl3outside, m)?)?;
    m.add_class::<Cdl3StarsInSouth>()?;
    m.add_function(wrap_pyfunction!(cdl3starsinsouth, m)?)?;
    m.add_class::<Cdl3WhiteSoldiers>()?;
    m.add_function(wrap_pyfunction!(cdl3whitesoldiers, m)?)?;
    m.add_class::<CdlAbandonedBaby>()?;
    m.add_function(wrap_pyfunction!(cdlabandonedbaby, m)?)?;
    m.add_class::<CdlAdvanceBlock>()?;
    m.add_function(wrap_pyfunction!(cdladvanceblock, m)?)?;
    m.add_class::<CdlBeltHold>()?;
    m.add_function(wrap_pyfunction!(cdlbelthold, m)?)?;
    m.add_class::<CdlBreakaway>()?;
    m.add_function(wrap_pyfunction!(cdlbreakaway, m)?)?;
    m.add_class::<CdlClosingMarubozu>()?;
    m.add_function(wrap_pyfunction!(cdlclosingmarubozu, m)?)?;
    m.add_class::<CdlConcealBabySwall>()?;
    m.add_function(wrap_pyfunction!(cdlconcealbabyswall, m)?)?;
    m.add_class::<CdlCounterattack>()?;
    m.add_function(wrap_pyfunction!(cdlcounterattack, m)?)?;
    m.add_class::<CdlDarkCloudCover>()?;
    m.add_function(wrap_pyfunction!(cdldarkcloudcover, m)?)?;
    m.add_class::<CdlDoji>()?;
    m.add_function(wrap_pyfunction!(cdldoji, m)?)?;
    m.add_class::<CdlDojiStar>()?;
    m.add_function(wrap_pyfunction!(cdldojistar, m)?)?;
    m.add_class::<CdlDragonflyDoji>()?;
    m.add_function(wrap_pyfunction!(cdldragonflydoji, m)?)?;
    m.add_class::<CdlEngulfing>()?;
    m.add_function(wrap_pyfunction!(cdlengulfing, m)?)?;
    m.add_class::<CdlEveningDojiStar>()?;
    m.add_function(wrap_pyfunction!(cdleveningdojistar, m)?)?;
    m.add_class::<CdlEveningStar>()?;
    m.add_function(wrap_pyfunction!(cdleveningstar, m)?)?;
    m.add_class::<CdlGapSideSideWhite>()?;
    m.add_function(wrap_pyfunction!(cdlgapsidesidewhite, m)?)?;
    m.add_class::<CdlGravestoneDoji>()?;
    m.add_function(wrap_pyfunction!(cdlgravestonedoji, m)?)?;
    m.add_class::<CdlHammer>()?;
    m.add_function(wrap_pyfunction!(cdlhammer, m)?)?;
    m.add_class::<CdlHangingMan>()?;
    m.add_function(wrap_pyfunction!(cdlhangingman, m)?)?;
    m.add_class::<CdlHarami>()?;
    m.add_function(wrap_pyfunction!(cdlharami, m)?)?;
    m.add_class::<CdlHaramiCross>()?;
    m.add_function(wrap_pyfunction!(cdlharamicross, m)?)?;
    m.add_class::<CdlHighWave>()?;
    m.add_function(wrap_pyfunction!(cdlhighwave, m)?)?;
    m.add_class::<CdlHikkake>()?;
    m.add_function(wrap_pyfunction!(cdlhikkake, m)?)?;
    m.add_class::<CdlHikkakeMod>()?;
    m.add_function(wrap_pyfunction!(cdlhikkakemod, m)?)?;
    m.add_class::<CdlHomingPigeon>()?;
    m.add_function(wrap_pyfunction!(cdlhomingpigeon, m)?)?;
    m.add_class::<CdlIdentical3Crows>()?;
    m.add_function(wrap_pyfunction!(cdlidentical3crows, m)?)?;
    m.add_class::<CdlInNeck>()?;
    m.add_function(wrap_pyfunction!(cdlinneck, m)?)?;
    m.add_class::<CdlInvertedHammer>()?;
    m.add_function(wrap_pyfunction!(cdlinvertedhammer, m)?)?;
    m.add_class::<CdlKicking>()?;
    m.add_function(wrap_pyfunction!(cdlkicking, m)?)?;
    m.add_class::<CdlKickingByLength>()?;
    m.add_function(wrap_pyfunction!(cdlkickingbylength, m)?)?;
    m.add_class::<CdlLadderBottom>()?;
    m.add_function(wrap_pyfunction!(cdlladderbottom, m)?)?;
    m.add_class::<CdlLongLeggedDoji>()?;
    m.add_function(wrap_pyfunction!(cdllongleggeddoji, m)?)?;
    m.add_class::<CdlLongLine>()?;
    m.add_function(wrap_pyfunction!(cdllongline, m)?)?;
    m.add_class::<CdlMarubozu>()?;
    m.add_function(wrap_pyfunction!(cdlmarubozu, m)?)?;
    m.add_class::<CdlMatchingLow>()?;
    m.add_function(wrap_pyfunction!(cdlmatchinglow, m)?)?;
    m.add_class::<CdlMatHold>()?;
    m.add_function(wrap_pyfunction!(cdlmathold, m)?)?;
    m.add_class::<CdlMorningDojiStar>()?;
    m.add_function(wrap_pyfunction!(cdlmorningdojistar, m)?)?;
    m.add_class::<CdlMorningStar>()?;
    m.add_function(wrap_pyfunction!(cdlmorningstar, m)?)?;
    m.add_class::<CdlOnNeck>()?;
    m.add_function(wrap_pyfunction!(cdlonneck, m)?)?;
    m.add_class::<CdlPiercing>()?;
    m.add_function(wrap_pyfunction!(cdlpiercing, m)?)?;
    m.add_class::<CdlRickshawMan>()?;
    m.add_function(wrap_pyfunction!(cdlrickshawman, m)?)?;
    m.add_class::<CdlRiseFall3Methods>()?;
    m.add_function(wrap_pyfunction!(cdlrisefall3methods, m)?)?;
    m.add_class::<CdlSeparatingLines>()?;
    m.add_function(wrap_pyfunction!(cdlseparatinglines, m)?)?;
    m.add_class::<CdlShootingStar>()?;
    m.add_function(wrap_pyfunction!(cdlshootingstar, m)?)?;
    m.add_class::<CdlShortLine>()?;
    m.add_function(wrap_pyfunction!(cdlshortline, m)?)?;
    m.add_class::<CdlSpinningTop>()?;
    m.add_function(wrap_pyfunction!(cdlspinningtop, m)?)?;
    m.add_class::<CdlStalledPattern>()?;
    m.add_function(wrap_pyfunction!(cdlstalledpattern, m)?)?;
    m.add_class::<CdlStickSandwich>()?;
    m.add_function(wrap_pyfunction!(cdlsticksandwich, m)?)?;
    m.add_class::<CdlTakuri>()?;
    m.add_function(wrap_pyfunction!(cdltakuri, m)?)?;
    m.add_class::<CdlTasukiGap>()?;
    m.add_function(wrap_pyfunction!(cdltasukigap, m)?)?;
    m.add_class::<CdlThrusting>()?;
    m.add_function(wrap_pyfunction!(cdlthrusting, m)?)?;
    m.add_class::<CdlTristar>()?;
    m.add_function(wrap_pyfunction!(cdltristar, m)?)?;
    m.add_class::<CdlUnique3River>()?;
    m.add_function(wrap_pyfunction!(cdlunique3river, m)?)?;
    m.add_class::<CdlUpsideGap2Crows>()?;
    m.add_function(wrap_pyfunction!(cdlupsidegap2crows, m)?)?;
    m.add_class::<CdlXSideGap3Methods>()?;
    m.add_function(wrap_pyfunction!(cdlxsidegap3methods, m)?)?;
    m.add_function(wrap_pyfunction!(ta_max, m)?)?;
    m.add_function(wrap_pyfunction!(ta_min, m)?)?;

    Ok(())
}
