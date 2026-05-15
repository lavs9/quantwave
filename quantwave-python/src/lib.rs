use quantwave_core::traits::Next;
use quantwave_core::indicators::smoothing::{SMA as CoreSMA, EMA as CoreEMA, WMA as CoreWMA};
use quantwave_core::indicators::supertrend::SuperTrend as CoreSuperTrend;
use quantwave_core::indicators::momentum::*;
use quantwave_core::indicators::overlap::{DEMA, KAMA, MAMA, SAR, T3 as CoreT3, BBANDS};
use quantwave_core::indicators::volatility::*;
use quantwave_core::indicators::tema::*;
use quantwave_core::indicators::ichimoku::IchimokuCloud as CoreIchimoku;
use quantwave_core::indicators::alligator::Alligator as CoreAlligator;
use quantwave_core::indicators::alma::ALMA as CoreALMA;
use quantwave_core::indicators::atr_ts::ATRTrailingStop as CoreAtrTs;
use quantwave_core::indicators::bandpass::BandPass as CoreBandpass;
use quantwave_core::indicators::butterworth::{Butterworth2 as CoreButterworth2, Butterworth3 as CoreButterworth3};
use quantwave_core::indicators::cg::CenterOfGravity as CoreCG;
use quantwave_core::indicators::channel_cycle::ChannelCycle as CoreChannelCycle;
use quantwave_core::indicators::choppiness_index::ChoppinessIndex as CoreChoppinessIndex;
use quantwave_core::indicators::classic_laguerre::ClassicLaguerre as CoreClassicLaguerre;
use quantwave_core::indicators::continuation_index::ContinuationIndex as CoreContinuationIndex;
use quantwave_core::indicators::correlation_cycle::CorrelationCycle as CoreCorrelationCycle;
use quantwave_core::indicators::correlation_trend::CorrelationTrend as CoreCorrelationTrend;
use quantwave_core::indicators::cyber_cycle::CyberCycle as CoreCyberCycle;
use quantwave_core::indicators::cybernetic_oscillator::CyberneticOscillator as CoreCyberneticOscillator;
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
use quantwave_core::indicators::fisher_high_pass::FisherHighPass as CoreFisherHighPass;
use quantwave_core::indicators::fisher::FisherTransform as CoreFisher;
use quantwave_core::indicators::fourier_series::FourierSeriesModel as CoreFourierSeries;
use quantwave_core::indicators::fourier_transform::FourierDominantCycle as CoreFourierDominantCycle;
use quantwave_core::indicators::fractals::BillWilliamsFractals as CoreFractals;
use quantwave_core::indicators::frama::FRAMA as CoreFRAMA;
use quantwave_core::indicators::gaussian::GaussianFilter as CoreGaussian;
use quantwave_core::indicators::generalized_laguerre::GeneralizedLaguerre as CoreGeneralizedLaguerre;
use quantwave_core::indicators::griffiths_dominant_cycle::GriffithsDominantCycle as CoreGriffithsDominantCycle;
use quantwave_core::indicators::griffiths_predictor::GriffithsPredictor as CoreGriffithsPredictor;
use quantwave_core::indicators::griffiths_spectrum::GriffithsSpectrum as CoreGriffithsSpectrum;
use quantwave_core::indicators::hamming::HammingFilter as CoreHamming;
use quantwave_core::indicators::hann::HannFilter as CoreHann;
use quantwave_core::indicators::heikin_ashi::HeikinAshi as CoreHeikinAshi;
use quantwave_core::indicators::high_pass::HighPass as CoreHighPass;
use quantwave_core::indicators::hma::HMA as CoreHMA;
use quantwave_core::indicators::homodyne_discriminator::HomodyneDiscriminator as CoreHomodyneDiscriminator;
use quantwave_core::indicators::hurst::HurstExponent as CoreHurstExponent;
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
use quantwave_core::indicators::noise_elimination::NoiseElimination as CoreNoiseElimination;
use quantwave_core::indicators::oc_price_rsi::OCPriceRSI as CoreOcPriceRSI;
use quantwave_core::indicators::one_euro_filter::OneEuroFilter as CoreOneEuroFilter;
use quantwave_core::indicators::pairs_rotation::PairsRotation as CorePairsRotation;
use quantwave_core::indicators::phasor::Phasor as CorePhasor;
use quantwave_core::indicators::pivot_points::PivotPoints as CorePivotPoints;
use quantwave_core::indicators::pma::ProjectedMovingAverage as CoreProjectedMovingAverage;
use quantwave_core::indicators::precision_trend::PrecisionTrendAnalysis as CorePrecisionTrend;
use quantwave_core::indicators::recursive_median::{RecursiveMedian as CoreRecursiveMedian, RecursiveMedianOscillator as CoreRMO};
use quantwave_core::indicators::reflex::Reflex as CoreReflex;
use quantwave_core::indicators::reversion_index::ReversionIndex as CoreReversionIndex;
use quantwave_core::indicators::robustness::RobustnessEvaluator as CoreRobustnessEvaluator;
use quantwave_core::indicators::rocket_rsi::RocketRSI as CoreRocketRSI;
use quantwave_core::indicators::roofing_filter::RoofingFilter as CoreRoofingFilter;
use quantwave_core::indicators::rsih::RSIH as CoreRSIH;
use quantwave_core::indicators::simple_predictor::SimplePredictor as CoreSimplePredictor;
use quantwave_core::indicators::sine_wave::SineWave as CoreSineWave;
use quantwave_core::indicators::stc::SchaffTrendCycle as CoreSTC;
use quantwave_core::indicators::super_smoother::SuperSmoother as CoreSuperSmoother;
use quantwave_core::indicators::swiss_army_knife::{SwissArmyKnife as CoreSwissArmyKnife, SwissArmyKnifeMode};
use quantwave_core::indicators::synthetic_oscillator::SyntheticOscillator as CoreSyntheticOscillator;
use quantwave_core::indicators::system_evaluator::SystemEvaluator as CoreSystemEvaluator;
use quantwave_core::indicators::trendflex::Trendflex as CoreTrendflex;
use quantwave_core::indicators::triangle::TriangleFilter as CoreTriangleFilter;
use quantwave_core::indicators::truncated_bandpass::TruncatedBandpass as CoreTruncatedBandpass;
use quantwave_core::indicators::ttm_squeeze::TTMSqueeze as CoreTtmSqueeze;
use quantwave_core::indicators::ultimate_bands::UltimateBands as CoreUltimateBands;
use quantwave_core::indicators::ultimate_channel::UltimateChannel as CoreUltimateChannel;
use quantwave_core::indicators::ultimate_smoother::UltimateSmoother as CoreUltimateSmoother;
use quantwave_core::indicators::universal_oscillator::UniversalOscillator as CoreUniversalOscillator;
use quantwave_core::indicators::usi::USI as CoreUSI;
use quantwave_core::indicators::volume::{AD as CoreAD, ADOSC as CoreADOSC, OBV as CoreOBV};
use quantwave_core::indicators::vortex::VortexIndicator as CoreVortex;
use quantwave_core::indicators::voss_predictor::VossPredictor as CoreVossPredictor;
use quantwave_core::indicators::vwap::AnchoredVWAP as CoreAnchoredVWAP;
use quantwave_core::indicators::wavetrend::WaveTrend as CoreWaveTrend;
use quantwave_core::indicators::zero_lag::ZeroLag as CoreZeroLag;
use quantwave_core::indicators::amfm::{AMDetector as CoreAMDetector, FMDemodulator as CoreFMDemodulator};
use quantwave_core::indicators::cycle::{HT_DCPERIOD, HT_PHASOR, HT_DCPHASE, HT_SINE, HT_TRENDMODE};
use quantwave_core::indicators::volume_profile::VolumeProfile as CoreVolumeProfile;
use quantwave_core::indicators::hilbert_transform::EhlersWma4 as CoreEhlersWma4;
use quantwave_core::indicators::just_ignore_them::UndersampledDoubleMA as CoreUDMA;

use std::sync::Mutex;
use paste::paste;

uniffi::setup_scaffolding!();

// --- Records ---

#[derive(uniffi::Record)] pub struct SuperTrendResult { pub value: f64, pub direction: i8 }
#[derive(uniffi::Record)] pub struct MacdResult { pub macd: f64, pub signal: f64, pub histogram: f64 }
#[derive(uniffi::Record)] pub struct BbandsResult { pub upper: f64, pub middle: f64, pub lower: f64 }
#[derive(uniffi::Record)] pub struct StochResult { pub k: f64, pub d: f64 }
#[derive(uniffi::Record)] pub struct MamaResult { pub mama: f64, pub fama: f64 }
#[derive(uniffi::Record)] pub struct AroonResult { pub up: f64, pub down: f64 }
#[derive(uniffi::Record)] pub struct IchimokuResult { pub tenkan: f64, pub kijun: f64, pub senkou_a: f64, pub senkou_b: f64 }
#[derive(uniffi::Record)] pub struct AlligatorResult { pub jaw: f64, pub teeth: f64, pub lips: f64 }
#[derive(uniffi::Record)] pub struct AtrTsResult { pub stop: f64, pub direction: i8 }
#[derive(uniffi::Record)] pub struct DonchianResult { pub upper: f64, pub middle: f64, pub lower: f64 }
#[derive(uniffi::Record)] pub struct EmdResult { pub trend: f64, pub upper: f64, pub lower: f64 }
#[derive(uniffi::Record)] pub struct EhlersLoopsResult { pub price_rms: f64, pub vol_rms: f64 }
#[derive(uniffi::Record)] pub struct FractalsResult { pub bearish: bool, pub bullish: bool }
#[derive(uniffi::Record)] pub struct HeikinAshiResult { pub open: f64, pub high: f64, pub low: f64, pub close: f64 }
#[derive(uniffi::Record)] pub struct KeltnerResult { pub upper: f64, pub middle: f64, pub lower: f64 }
#[derive(uniffi::Record)] pub struct PairsRotationResult { pub ratio: f64, pub angle: f64 }
#[derive(uniffi::Record)] pub struct PhasorResult { pub in_phase: f64, pub quadrature: f64 }
#[derive(uniffi::Record)] pub struct PivotPointsResult { pub p: f64, pub r1: f64, pub s1: f64, pub r2: f64, pub s2: f64 }
#[derive(uniffi::Record)] pub struct SystemEvaluatorResult { pub average_win_loss_ratio: f64, pub average_trade: f64, pub profit_factor: f64, pub percent_winners: f64, pub breakeven_profit_factor: f64, pub weighted_average_trade: f64, pub theoretical_consecutive_losers: f64 }
#[derive(uniffi::Record)] pub struct UltimateBandsResult { pub upper: f64, pub middle: f64, pub lower: f64 }
#[derive(uniffi::Record)] pub struct UltimateChannelResult { pub upper: f64, pub center: f64, pub lower: f64 }
#[derive(uniffi::Record)] pub struct VortexResult { pub plus: f64, pub minus: f64 }
#[derive(uniffi::Record)] pub struct WaveTrendResult { pub wt1: f64, pub wt2: f64 }
#[derive(uniffi::Record)] pub struct VossPredictorResult { pub filt: f64, pub voss: f64 }
#[derive(uniffi::Record)] pub struct CycleTrendAnalyticsResult { pub cycle: f64, pub trend: f64 }
#[derive(uniffi::Record)] pub struct ZeroLagResult { pub value: f64, pub trigger: f64 }
#[derive(uniffi::Record)] pub struct CyberCycleResult { pub value: f64, pub trigger: f64 }
#[derive(uniffi::Record)] pub struct HtSineResult { pub sine: f64, pub leadsine: f64 }
#[derive(uniffi::Record)] pub struct VolumeProfileResult { pub poc: f64, pub vah: f64, pub val: f64 }
#[derive(uniffi::Record)] pub struct PmaResult { pub pma: f64, pub predict: f64 }
#[derive(uniffi::Record)] pub struct TrendRocResult { pub trend: f64, pub roc: f64 }
#[derive(uniffi::Record)] pub struct UdmaResult { pub fast: f64, pub slow: f64 }

#[derive(uniffi::Enum)]
pub enum SwissMode { EMA, SMA, Gauss, Butterworth, Smooth, HighPass, TwoPoleHighPass, BandPass, BandStop }
impl From<SwissMode> for SwissArmyKnifeMode {
    fn from(m: SwissMode) -> Self {
        match m {
            SwissMode::EMA => Self::EMA, SwissMode::SMA => Self::SMA, SwissMode::Gauss => Self::Gauss,
            SwissMode::Butterworth => Self::Butterworth, SwissMode::Smooth => Self::Smooth, SwissMode::HighPass => Self::HighPass,
            SwissMode::TwoPoleHighPass => Self::TwoPoleHighPass, SwissMode::BandPass => Self::BandPass, SwissMode::BandStop => Self::BandStop,
        }
    }
}

// --- Macros ---

macro_rules! export_1_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* series: Vec<f64>) -> Vec<f64> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                series.iter().map(|&x| indicator.next(x)).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, input: f64) -> f64 { self.inner.lock().unwrap().next(input) }
            }
        }
    }
}

macro_rules! export_1_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* series: Vec<f64>) -> Vec<$result_type> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                series.iter().map(|&x| { let $res_var = indicator.next(x); $body }).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, input: f64) -> $result_type { let $res_var = self.inner.lock().unwrap().next(input); $body }
            }
        }
    }
}

macro_rules! export_1_in_vec_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* series: Vec<f64>) -> Vec<Vec<f64>> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                series.iter().map(|&x| indicator.next(x)).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, input: f64) -> Vec<f64> { self.inner.lock().unwrap().next(input) }
            }
        }
    }
}

macro_rules! export_ohlc_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>, close: Vec<f64>) -> Vec<f64> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| indicator.next((h, l, c))).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, high: f64, low: f64, close: f64) -> f64 { self.inner.lock().unwrap().next((high, low, close)) }
            }
        }
    }
}

macro_rules! export_ohlc_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>, close: Vec<f64>) -> Vec<$result_type> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| { let $res_var = indicator.next((h, l, c)); $body }).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, high: f64, low: f64, close: f64) -> $result_type { let $res_var = self.inner.lock().unwrap().next((high, low, close)); $body }
            }
        }
    }
}

macro_rules! export_hl_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>) -> Vec<f64> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                high.iter().zip(low.iter()).map(|(&h, &l)| indicator.next((h, l))).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, high: f64, low: f64) -> f64 { self.inner.lock().unwrap().next((high, low)) }
            }
        }
    }
}

macro_rules! export_hl_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* high: Vec<f64>, low: Vec<f64>) -> Vec<$result_type> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                high.iter().zip(low.iter()).map(|(&h, &l)| { let $res_var = indicator.next((h, l)); $body }).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, high: f64, low: f64) -> $result_type { let $res_var = self.inner.lock().unwrap().next((high, low)); $body }
            }
        }
    }
}

macro_rules! export_co_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* close: Vec<f64>, open: Vec<f64>) -> Vec<f64> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                close.iter().zip(open.iter()).map(|(&c, &o)| indicator.next((c, o))).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, close: f64, open: f64) -> f64 { self.inner.lock().unwrap().next((close, open)) }
            }
        }
    }
}

macro_rules! export_co_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* close: Vec<f64>, open: Vec<f64>) -> Vec<$result_type> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                close.iter().zip(open.iter()).map(|(&c, &o)| { let $res_var = indicator.next((c, o)); $body }).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, close: f64, open: f64) -> $result_type { let $res_var = self.inner.lock().unwrap().next((close, open)); $body }
            }
        }
    }
}

macro_rules! export_pv_in_1_out {
    ($name:ident, $core_type:ty, ($($param:ident: $param_type:ty),*)) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* price: Vec<f64>, volume: Vec<f64>) -> Vec<f64> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                price.iter().zip(volume.iter()).map(|(&p, &v)| indicator.next((p, v))).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, price: f64, volume: f64) -> f64 { self.inner.lock().unwrap().next((price, volume)) }
            }
        }
    }
}

macro_rules! export_pv_in_record_out {
    ($name:ident, $core_type:ty, $result_type:ty, ($($param:ident: $param_type:ty),*), $res_var:ident, $body:expr) => {
        paste! {
            #[uniffi::export]
            pub fn [<$name:lower>]($( $param: $param_type , )* price: Vec<f64>, volume: Vec<f64>) -> Vec<$result_type> {
                let mut indicator = <$core_type>::new($( $param as _ ),*);
                price.iter().zip(volume.iter()).map(|(&p, &v)| { let $res_var = indicator.next((p, v)); $body }).collect()
            }
            #[derive(uniffi::Object)] pub struct $name { inner: Mutex<$core_type> }
            #[uniffi::export] impl $name {
                #[uniffi::constructor] pub fn new($( $param: $param_type ),*) -> Self { Self { inner: Mutex::new(<$core_type>::new($( $param as _ ),*)) } }
                pub fn next(&self, price: f64, volume: f64) -> $result_type { let $res_var = self.inner.lock().unwrap().next((price, volume)); $body }
            }
        }
    }
}

// --- Indicators ---

export_1_in_1_out!(Sma, CoreSMA, (period: u64));
export_1_in_1_out!(Ema, CoreEMA, (period: u64));
export_1_in_1_out!(Wma, CoreWMA, (period: u64));
export_1_in_1_out!(Rsi, RSI, (period: u64));
export_ohlc_in_record_out!(SuperTrend, CoreSuperTrend, SuperTrendResult, (period: u64, multiplier: f64), res, SuperTrendResult { value: res.0, direction: res.1 });
export_1_in_record_out!(Macd, MACD, MacdResult, (fast: u64, slow: u64, signal: u64), res, MacdResult { macd: res.0, signal: res.1, histogram: res.2 });
export_ohlc_in_1_out!(Atr, ATR, (period: u64));
export_ohlc_in_1_out!(Adx, ADX, (period: u64));
export_ohlc_in_1_out!(Cci, CCI, (period: u64));

#[uniffi::export]
pub fn stoch(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, fastk: u64, slowk: u64, slowd: u64) -> Vec<StochResult> {
    let mut it = STOCH::new(fastk as usize, slowk as usize, quantwave_core::talib::MaType::Sma, slowd as usize, quantwave_core::talib::MaType::Sma);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| { let (k, d) = it.next((h, l, c)); StochResult { k, d } }).collect()
}
#[derive(uniffi::Object)] pub struct Stoch { inner: Mutex<STOCH> }
#[uniffi::export] impl Stoch {
    #[uniffi::constructor] pub fn new(fastk: u64, slowk: u64, slowd: u64) -> Self { Self { inner: Mutex::new(STOCH::new(fastk as usize, slowk as usize, quantwave_core::talib::MaType::Sma, slowd as usize, quantwave_core::talib::MaType::Sma)) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> StochResult { let (k, d) = self.inner.lock().unwrap().next((high, low, close)); StochResult { k, d } }
}

export_hl_in_record_out!(Aroon, AROON, AroonResult, (period: u64), res, AroonResult { up: res.0, down: res.1 });
export_1_in_record_out!(Mama, MAMA, MamaResult, (fastlimit: f64, slowlimit: f64), res, MamaResult { mama: res.0, fama: res.1 });
export_1_in_1_out!(Kama, KAMA, (period: u64));
export_1_in_1_out!(T3, CoreT3, (period: u64, v_factor: f64));
export_hl_in_1_out!(Sar, SAR, (acceleration: f64, maximum: f64));
export_1_in_1_out!(Mom, MOM, (period: u64));
export_1_in_1_out!(Roc, ROC, (period: u64));
export_ohlc_in_1_out!(Willr, WILLR, (period: u64));
export_1_in_1_out!(Dema, DEMA, (period: u64));
export_1_in_1_out!(Tema, TEMA, (period: u64));

#[uniffi::export]
pub fn ichimoku(high: Vec<f64>, low: Vec<f64>, tenkan: u64, kijun: u64, senkou_b: u64) -> Vec<IchimokuResult> {
    let mut it = CoreIchimoku::new(tenkan as usize, kijun as usize, senkou_b as usize);
    high.iter().zip(low.iter()).map(|(&h, &l)| { let (t, k, sa, sb) = it.next((h, l)); IchimokuResult { tenkan: t, kijun: k, senkou_a: sa, senkou_b: sb } }).collect()
}
#[derive(uniffi::Object)] pub struct Ichimoku { inner: Mutex<CoreIchimoku> }
#[uniffi::export] impl Ichimoku {
    #[uniffi::constructor] pub fn new(tenkan: u64, kijun: u64, senkou_b: u64) -> Self { Self { inner: Mutex::new(CoreIchimoku::new(tenkan as usize, kijun as usize, senkou_b as usize)) } }
    pub fn next(&self, high: f64, low: f64) -> IchimokuResult { let (t, k, sa, sb) = self.inner.lock().unwrap().next((high, low)); IchimokuResult { tenkan: t, kijun: k, senkou_a: sa, senkou_b: sb } }
}

export_1_in_1_out!(Cg, CoreCG, (period: u64));
export_1_in_record_out!(CyberCycle, CoreCyberCycle, CyberCycleResult, (length: u64), res, CyberCycleResult { value: res.0, trigger: res.1 });
export_1_in_1_out!(Fisher, CoreFisher, ());
export_1_in_1_out!(InverseFisher, CoreInverseFisher, ());
export_1_in_1_out!(SuperSmoother, CoreSuperSmoother, (period: u64));
export_1_in_1_out!(Bandpass, CoreBandpass, (period: u64, bandwidth: f64));
export_1_in_1_out!(RoofingFilter, CoreRoofingFilter, (hp_period: u64, ss_period: u64));
export_1_in_record_out!(ZeroLag, CoreZeroLag, ZeroLagResult, (length: u64, gain_limit: f64), res, ZeroLagResult { value: res.0, trigger: res.1 });
export_ohlc_in_1_out!(ChoppinessIndex, CoreChoppinessIndex, (period: u64));
export_1_in_1_out!(ClassicLaguerre, CoreClassicLaguerre, (gamma: f64));

export_1_in_record_out!(Alligator, CoreAlligator, AlligatorResult, (), res, AlligatorResult { jaw: res.0, teeth: res.1, lips: res.2 });
export_1_in_1_out!(Alma, CoreALMA, (period: u64, offset: f64, sigma: f64));
export_ohlc_in_record_out!(AtrTs, CoreAtrTs, AtrTsResult, (period: u64, multiplier: f64), res, AtrTsResult { stop: res.0, direction: res.1 });
export_1_in_1_out!(Butterworth2, CoreButterworth2, (period: u64));
export_1_in_1_out!(Butterworth3, CoreButterworth3, (period: u64));
export_1_in_record_out!(ChannelCycle, CoreChannelCycle, CyberCycleResult, (period: u64), res, CyberCycleResult { value: res.0, trigger: res.1 });
export_1_in_1_out!(ContinuationIndex, CoreContinuationIndex, (gamma: f64, order: u64, length: u64));
export_1_in_record_out!(CorrelationCycle, CoreCorrelationCycle, PhasorResult, (period: u64), res, PhasorResult { in_phase: res.0, quadrature: res.1 });
export_1_in_1_out!(CorrelationTrend, CoreCorrelationTrend, (length: u64));
export_1_in_1_out!(CyberneticOscillator, CoreCyberneticOscillator, (hp_length: u64, lp_length: u64, rms_len: u64));
export_hl_in_1_out!(Dmh, CoreDMH, (length: u64));

#[uniffi::export]
pub fn donchian(high: Vec<f64>, low: Vec<f64>, period: u64) -> Vec<DonchianResult> {
    let mut it = CoreDonchian::new(period as usize);
    high.iter().zip(low.iter()).map(|(&h, &l)| { let (u, m, lo) = it.next((h, l)); DonchianResult { upper: u, middle: m, lower: lo } }).collect()
}
#[derive(uniffi::Object)] pub struct Donchian { inner: Mutex<CoreDonchian> }
#[uniffi::export] impl Donchian {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(CoreDonchian::new(period as usize)) } }
    pub fn next(&self, high: f64, low: f64) -> DonchianResult { let (u, m, lo) = self.inner.lock().unwrap().next((high, low)); DonchianResult { upper: u, middle: m, lower: lo } }
}

export_1_in_1_out!(Dsma, CoreDSMA, (period: u64));
export_co_in_1_out!(AmDetector, CoreAMDetector, (highest_len: u64, avg_len: u64));
export_co_in_1_out!(FmDemodulator, CoreFMDemodulator, (period: u64));
export_1_in_vec_out!(EhlersAutocorrelation, CoreEhlersAutocorrelation, (length: u64, num_lags: u64));
export_1_in_1_out!(EhlersFilter, CoreEhlersFilter, (length: u64));
export_pv_in_record_out!(EhlersLoops, CoreEhlersLoops, EhlersLoopsResult, (lp_period: u64, hp_period: u64), res, EhlersLoopsResult { price_rms: res.0, vol_rms: res.1 });
export_1_in_1_out!(EhlersStochastic, CoreEhlersStochastic, (hp_period: u64, ss_period: u64, stoch_period: u64));
export_1_in_1_out!(EhlersUltimateOscillator, CoreEhlersUltimateOscillator, (band_edge: u64, bandwidth: f64));
export_1_in_1_out!(FisherHighPass, CoreFisherHighPass, (hp_len: u64, norm_len: u64));
export_1_in_1_out!(FourierSeries, CoreFourierSeries, (fundamental: u64));
export_1_in_1_out!(FourierDominantCycle, CoreFourierDominantCycle, (window_len: u64));
export_hl_in_record_out!(Fractals, CoreFractals, FractalsResult, (), res, FractalsResult { bearish: res.0, bullish: res.1 });
export_ohlc_in_1_out!(Frama, CoreFRAMA, (length: u64));
export_1_in_1_out!(Gaussian, CoreGaussian, (period: u64, poles: u64));
export_1_in_1_out!(GeneralizedLaguerre, CoreGeneralizedLaguerre, (length: u64, gamma: f64, order: u64));
export_1_in_1_out!(GriffithsDominantCycle, CoreGriffithsDominantCycle, (lower_bound: u64, upper_bound: u64, length: u64));
export_1_in_1_out!(GriffithsPredictor, CoreGriffithsPredictor, (lower_bound: u64, upper_bound: u64, length: u64, bars_fwd: u64));
export_1_in_vec_out!(GriffithsSpectrum, CoreGriffithsSpectrum, (lower_bound: u64, upper_bound: u64, length: u64));
export_1_in_1_out!(Hamming, CoreHamming, (period: u64, pedestal: f64));
export_1_in_1_out!(Hann, CoreHann, (period: u64));

#[uniffi::export]
pub fn heikin_ashi(open: Vec<f64>, high: Vec<f64>, low: Vec<f64>, close: Vec<f64>) -> Vec<HeikinAshiResult> {
    let mut it = CoreHeikinAshi::new();
    open.iter().zip(high.iter()).zip(low.iter()).zip(close.iter()).map(|(((&o, &h), &l), &c)| {
        let (ho, hh, hl, hc) = it.next((o, h, l, c));
        HeikinAshiResult { open: ho, high: hh, low: hl, close: hc }
    }).collect()
}
#[derive(uniffi::Object)] pub struct HeikinAshi { inner: Mutex<CoreHeikinAshi> }
#[uniffi::export] impl HeikinAshi {
    #[uniffi::constructor] pub fn new() -> Self { Self { inner: Mutex::new(CoreHeikinAshi::new()) } }
    pub fn next(&self, open: f64, high: f64, low: f64, close: f64) -> HeikinAshiResult {
        let (ho, hh, hl, hc) = self.inner.lock().unwrap().next((open, high, low, close));
        HeikinAshiResult { open: ho, high: hh, low: hl, close: hc }
    }
}

export_1_in_1_out!(HighPass, CoreHighPass, (period: u64));
export_1_in_1_out!(Hma, CoreHMA, (period: u64));
export_1_in_1_out!(EhlersWma4, CoreEhlersWma4, ());
export_1_in_1_out!(InstantaneousTrendline, CoreInstantaneousTrendline, ());
export_1_in_record_out!(UndersampledDoubleMa, CoreUDMA, UdmaResult, (fast_len: u64, slow_len: u64, samp_per: u64), res, UdmaResult { fast: res.0, slow: res.1 });

#[uniffi::export]
pub fn keltner(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, ema_period: u64, atr_period: u64, multiplier: f64) -> Vec<KeltnerResult> {
    let mut it = CoreKeltner::new(ema_period as usize, atr_period as usize, multiplier);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| { let (u, m, lo) = it.next((h, l, c)); KeltnerResult { upper: u, middle: m, lower: lo } }).collect()
}
#[derive(uniffi::Object)] pub struct Keltner { inner: Mutex<CoreKeltner> }
#[uniffi::export] impl Keltner {
    #[uniffi::constructor] pub fn new(ema_period: u64, atr_period: u64, multiplier: f64) -> Self { Self { inner: Mutex::new(CoreKeltner::new(ema_period as usize, atr_period as usize, multiplier)) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> KeltnerResult { let (u, m, lo) = self.inner.lock().unwrap().next((high, low, close)); KeltnerResult { upper: u, middle: m, lower: lo } }
}

export_1_in_1_out!(LaguerreFilter, CoreLaguerreFilter, (length: u64, gamma: f64));
export_1_in_1_out!(LaguerreOscillator, CoreLaguerreOscillator, (length: u64, gamma: f64, rms_period: u64));
export_1_in_1_out!(LaguerreRsi, CoreLaguerreRSI, (gamma: f64));
export_1_in_1_out!(NoiseElimination, CoreNoiseElimination, (period: u64));
export_co_in_record_out!(PairsRotation, CorePairsRotation, PairsRotationResult, (hp_len: u64, lp_len: u64), res, PairsRotationResult { ratio: res.0, angle: res.1 });
export_1_in_record_out!(Phasor, CorePhasor, PhasorResult, (), res, PhasorResult { in_phase: res.0, quadrature: res.1 });
export_co_in_1_out!(OcPriceRsi, CoreOcPriceRSI, (period: u64));

#[uniffi::export]
pub fn pivot_points(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>) -> Vec<PivotPointsResult> {
    let mut it = CorePivotPoints::new();
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| { let r = it.next((h, l, c)); PivotPointsResult { p: r.0, r1: r.1, s1: r.2, r2: r.3, s2: r.4 } }).collect()
}
#[derive(uniffi::Object)] pub struct PivotPoints { inner: Mutex<CorePivotPoints> }
#[uniffi::export] impl PivotPoints {
    #[uniffi::constructor] pub fn new() -> Self { Self { inner: Mutex::new(CorePivotPoints::new()) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> PivotPointsResult { let r = self.inner.lock().unwrap().next((high, low, close)); PivotPointsResult { p: r.0, r1: r.1, s1: r.2, r2: r.3, s2: r.4 } }
}

export_1_in_1_out!(OneEuroFilter, CoreOneEuroFilter, (period_min: u64, beta: f64));
export_1_in_record_out!(ProjectedMovingAverage, CoreProjectedMovingAverage, PmaResult, (period: u64), res, PmaResult { pma: res.0, predict: res.1 });
export_1_in_record_out!(PrecisionTrend, CorePrecisionTrend, TrendRocResult, (length1: u64, length2: u64), res, TrendRocResult { trend: res.0, roc: res.1 });
export_1_in_record_out!(ReversionIndex, CoreReversionIndex, TrendRocResult, (period: u64), res, TrendRocResult { trend: res.0, roc: res.1 });
export_1_in_record_out!(SineWave, CoreSineWave, PhasorResult, (), res, PhasorResult { in_phase: res.0, quadrature: res.1 });

#[uniffi::export]
pub fn swiss_army_knife(series: Vec<f64>, mode: SwissMode, period: u64, delta: f64) -> Vec<f64> {
    let mut it = CoreSwissArmyKnife::new(mode.into(), period as usize, delta);
    series.iter().map(|&x| it.next(x)).collect()
}
#[derive(uniffi::Object)] pub struct SwissArmyKnife { inner: Mutex<CoreSwissArmyKnife> }
#[uniffi::export] impl SwissArmyKnife {
    #[uniffi::constructor] pub fn new(mode: SwissMode, period: u64, delta: f64) -> Self { Self { inner: Mutex::new(CoreSwissArmyKnife::new(mode.into(), period as usize, delta)) } }
    pub fn next(&self, input: f64) -> f64 { self.inner.lock().unwrap().next(input) }
}

export_1_in_record_out!(SystemEvaluator, CoreSystemEvaluator, SystemEvaluatorResult, (), res, res.into());
#[derive(uniffi::Object)] pub struct RobustnessEvaluator { inner: Mutex<CoreRobustnessEvaluator> }
#[uniffi::export] impl RobustnessEvaluator {
    #[uniffi::constructor] pub fn new() -> Self { Self { inner: Mutex::new(CoreRobustnessEvaluator::new()) } }
    pub fn add_test_result(&self, net_profit: f64) { self.inner.lock().unwrap().add_test_result(net_profit); }
    pub fn calculate_score(&self) -> f64 { self.inner.lock().unwrap().calculate_score() }
}

export_ohlc_in_record_out!(TtmSqueeze, CoreTtmSqueeze, SuperTrendResult, (period: u64, mult_bb: f64, mult_kc: f64), res, SuperTrendResult { value: res.0, direction: if res.1 { 1 } else { 0 } });

#[uniffi::export]
pub fn ultimate_bands(series: Vec<f64>, length: u64, num_sds: f64) -> Vec<UltimateBandsResult> {
    let mut it = CoreUltimateBands::new(length as usize, num_sds);
    series.iter().map(|&x| { let (u, m, lo) = it.next(x); UltimateBandsResult { upper: u, middle: m, lower: lo } }).collect()
}
#[derive(uniffi::Object)] pub struct UltimateBands { inner: Mutex<CoreUltimateBands> }
#[uniffi::export] impl UltimateBands {
    #[uniffi::constructor] pub fn new(length: u64, num_sds: f64) -> Self { Self { inner: Mutex::new(CoreUltimateBands::new(length as usize, num_sds)) } }
    pub fn next(&self, input: f64) -> UltimateBandsResult { let (u, m, lo) = self.inner.lock().unwrap().next(input); UltimateBandsResult { upper: u, middle: m, lower: lo } }
}

#[uniffi::export]
pub fn ultimate_channel(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, length: u64, str_length: u64, num_strs: f64) -> Vec<UltimateChannelResult> {
    let mut it = CoreUltimateChannel::new(length as usize, str_length as usize, num_strs);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| { let (u, ce, lo) = it.next((h, l, c)); UltimateChannelResult { upper: u, center: ce, lower: lo } }).collect()
}
#[derive(uniffi::Object)] pub struct UltimateChannel { inner: Mutex<CoreUltimateChannel> }
#[uniffi::export] impl UltimateChannel {
    #[uniffi::constructor] pub fn new(length: u64, str_length: u64, num_strs: f64) -> Self { Self { inner: Mutex::new(CoreUltimateChannel::new(length as usize, str_length as usize, num_strs)) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> UltimateChannelResult { let (u, ce, lo) = self.inner.lock().unwrap().next((high, low, close)); UltimateChannelResult { upper: u, center: ce, lower: lo } }
}

export_1_in_1_out!(UltimateSmoother, CoreUltimateSmoother, (period: u64));
export_1_in_1_out!(Usi, CoreUSI, (length: u64));

#[uniffi::export]
pub fn ad(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, volume: Vec<f64>) -> Vec<f64> {
    let mut it = CoreAD::new();
    high.iter().zip(low.iter()).zip(close.iter()).zip(volume.iter()).map(|(((&h, &l), &c), &v)| it.next((h, l, c, v))).collect()
}
#[derive(uniffi::Object)] pub struct Ad { inner: Mutex<CoreAD> }
#[uniffi::export] impl Ad {
    #[uniffi::constructor] pub fn new() -> Self { Self { inner: Mutex::new(CoreAD::new()) } }
    pub fn next(&self, high: f64, low: f64, close: f64, volume: f64) -> f64 { self.inner.lock().unwrap().next((high, low, close, volume)) }
}

#[uniffi::export]
pub fn adosc(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, volume: Vec<f64>, fast: u64, slow: u64) -> Vec<f64> {
    let mut it = CoreADOSC::new(fast as usize, slow as usize);
    high.iter().zip(low.iter()).zip(close.iter()).zip(volume.iter()).map(|(((&h, &l), &c), &v)| it.next((h, l, c, v))).collect()
}
#[derive(uniffi::Object)] pub struct Adosc { inner: Mutex<CoreADOSC> }
#[uniffi::export] impl Adosc {
    #[uniffi::constructor] pub fn new(fast: u64, slow: u64) -> Self { Self { inner: Mutex::new(CoreADOSC::new(fast as usize, slow as usize)) } }
    pub fn next(&self, high: f64, low: f64, close: f64, volume: f64) -> f64 { self.inner.lock().unwrap().next((high, low, close, volume)) }
}

export_pv_in_1_out!(Obv, CoreOBV, ());
export_ohlc_in_record_out!(Vortex, CoreVortex, VortexResult, (period: u64), res, VortexResult { plus: res.0, minus: res.1 });

#[uniffi::export]
pub fn anchored_vwap(price: Vec<f64>, volume: Vec<f64>, anchor: Vec<bool>) -> Vec<f64> {
    let mut it = CoreAnchoredVWAP::new();
    price.iter().zip(volume.iter()).zip(anchor.iter()).map(|((&p, &v), &a)| it.next((p, v, a))).collect()
}
#[derive(uniffi::Object)] pub struct AnchoredVwap { inner: Mutex<CoreAnchoredVWAP> }
#[uniffi::export] impl AnchoredVwap {
    #[uniffi::constructor] pub fn new() -> Self { Self { inner: Mutex::new(CoreAnchoredVWAP::new()) } }
    pub fn next(&self, price: f64, volume: f64, anchor: bool) -> f64 { self.inner.lock().unwrap().next((price, volume, anchor)) }
}

export_ohlc_in_record_out!(WaveTrend, CoreWaveTrend, WaveTrendResult, (n1: u64, n2: u64, n3: u64), res, WaveTrendResult { wt1: res.0, wt2: res.1 });
export_1_in_1_out!(SimplePredictor, CoreSimplePredictor, (hp_len: u64, lp_len: u64, q: f64));
export_1_in_1_out!(Mad, CoreMAD, (short_period: u64, long_period: u64));
export_1_in_1_out!(MesaStochastic, CoreMesaStochastic, (len: u64, hp: u64, ss: u64));
export_1_in_1_out!(Rsih, CoreRSIH, (length: u64));
export_1_in_record_out!(VossPredictor, CoreVossPredictor, VossPredictorResult, (period: u64, predict: u64), res, VossPredictorResult { filt: res.0, voss: res.1 });
export_1_in_1_out!(SyntheticOscillator, CoreSyntheticOscillator, (lower_bound: u64, upper_bound: u64));

#[uniffi::export]
pub fn cycletrendanalytics(series: Vec<f64>, min_length: u64, max_length: u64) -> Vec<CycleTrendAnalyticsResult> {
    let mut it = CoreCycleTrendAnalytics::new(min_length as usize, max_length as usize);
    series.iter().map(|&x| { let r = it.next(x); CycleTrendAnalyticsResult { cycle: r[0], trend: r[1] } }).collect()
}
#[derive(uniffi::Object)] pub struct CycleTrendAnalytics { inner: Mutex<CoreCycleTrendAnalytics> }
#[uniffi::export] impl CycleTrendAnalytics {
    #[uniffi::constructor] pub fn new(min_length: u64, max_length: u64) -> Self { Self { inner: Mutex::new(CoreCycleTrendAnalytics::new(min_length as usize, max_length as usize)) } }
    pub fn next(&self, input: f64) -> CycleTrendAnalyticsResult { let r = self.inner.lock().unwrap().next(input); CycleTrendAnalyticsResult { cycle: r[0], trend: r[1] } }
}

export_1_in_1_out!(Madh, CoreMADH, (short_length: u64, dominant_cycle: u64));
export_1_in_1_out!(Stc, CoreSTC, (cycle_period: u64, fast_period: u64, slow_period: u64));
export_1_in_1_out!(HomodyneDiscriminator, CoreHomodyneDiscriminator, ());
export_1_in_1_out!(UniversalOscillator, CoreUniversalOscillator, (band_edge: u64));
export_1_in_1_out!(TriangleFilter, CoreTriangleFilter, (length: u64));
export_1_in_1_out!(HtDcPeriod, HT_DCPERIOD, ());
export_1_in_record_out!(HtPhasor, HT_PHASOR, PhasorResult, (), res, PhasorResult { in_phase: res.0, quadrature: res.1 });
export_1_in_1_out!(HtDcPhase, HT_DCPHASE, ());
export_1_in_record_out!(HtSine, HT_SINE, HtSineResult, (), res, HtSineResult { sine: res.0, leadsine: res.1 });
export_1_in_1_out!(HtTrendMode, HT_TRENDMODE, ());
export_1_in_1_out!(HurstExponent, CoreHurstExponent, (length: u64));
export_1_in_1_out!(KalmanFilter, CoreKalmanFilter, (gain: f64, noise: f64));
export_1_in_1_out!(MarketState, CoreMarketState, (period: u64, threshold: f64));
export_1_in_1_out!(RecursiveMedian, CoreRecursiveMedian, (length: u64));
export_1_in_1_out!(RecursiveMedianOscillator, CoreRMO, (lp_period: u64, hp_period: u64));
export_1_in_1_out!(Reflex, CoreReflex, (length: u64));
export_1_in_1_out!(RocketRsi, CoreRocketRSI, (rsi_length: u64, smooth_length: u64));
export_1_in_1_out!(Trendflex, CoreTrendflex, (length: u64));
export_1_in_1_out!(TruncatedBandpass, CoreTruncatedBandpass, (period: u64, bandwidth: f64, length: u64));

#[uniffi::export]
pub fn volumeprofile(price: Vec<f64>, volume: Vec<f64>, period: u64, bins: u64) -> Vec<VolumeProfileResult> {
    let mut it = CoreVolumeProfile::new(period as usize, bins as usize);
    price.iter().zip(volume.iter()).map(|(&p, &v)| {
        let poc = it.next((p, v));
        VolumeProfileResult { poc, vah: 0.0, val: 0.0 }
    }).collect()
}
#[derive(uniffi::Object)] pub struct VolumeProfile { inner: Mutex<CoreVolumeProfile> }
#[uniffi::export] impl VolumeProfile {
    #[uniffi::constructor] pub fn new(period: u64, bins: u64) -> Self { Self { inner: Mutex::new(CoreVolumeProfile::new(period as usize, bins as usize)) } }
    pub fn next(&self, price: f64, volume: f64) -> VolumeProfileResult {
        let poc = self.inner.lock().unwrap().next((price, volume));
        VolumeProfileResult { poc, vah: 0.0, val: 0.0 }
    }
}

// Into implementations for records
impl From<quantwave_core::indicators::system_evaluator::SystemEvaluationResults> for SystemEvaluatorResult {
    fn from(r: quantwave_core::indicators::system_evaluator::SystemEvaluationResults) -> Self {
        Self { average_win_loss_ratio: r.average_win_loss_ratio, average_trade: r.average_trade, profit_factor: r.profit_factor, percent_winners: r.percent_winners, breakeven_profit_factor: r.breakeven_profit_factor, weighted_average_trade: r.weighted_average_trade, theoretical_consecutive_losers: r.theoretical_consecutive_losers }
    }
}
