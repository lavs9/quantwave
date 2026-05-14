use quantwave_core::traits::Next;
use quantwave_core::indicators::smoothing::{SMA as CoreSMA, EMA as CoreEMA, WMA as CoreWMA};
use quantwave_core::indicators::supertrend::SuperTrend as CoreSuperTrend;
use quantwave_core::indicators::momentum::*;
use quantwave_core::indicators::overlap::{self, DEMA, KAMA, MAMA, MIDPOINT, MIDPRICE, SAR, T3 as CoreT3, TRIMA, BBANDS};
use quantwave_core::indicators::volatility::*;
use quantwave_core::indicators::tema::*;
use quantwave_core::indicators::ichimoku::IchimokuCloud;
use std::sync::Mutex;

// --- Records ---

#[derive(uniffi::Record)]
pub struct SuperTrendResult {
    pub value: f64,
    pub direction: i8,
}

#[derive(uniffi::Record)]
pub struct MacdResult {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
}

#[derive(uniffi::Record)]
pub struct BbandsResult {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
}

#[derive(uniffi::Record)]
pub struct StochResult {
    pub k: f64,
    pub d: f64,
}

#[derive(uniffi::Record)]
pub struct MamaResult {
    pub mama: f64,
    pub fama: f64,
}

#[derive(uniffi::Record)]
pub struct AroonResult {
    pub up: f64,
    pub down: f64,
}

#[derive(uniffi::Record)]
pub struct IchimokuResult {
    pub tenkan: f64,
    pub kijun: f64,
    pub senkou_a: f64,
    pub senkou_b: f64,
}

// --- Batch Functions ---

#[uniffi::export]
pub fn sma(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut sma = CoreSMA::new(period as usize);
    series.iter().map(|&x| sma.next(x)).collect()
}

#[uniffi::export]
pub fn ema(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut ema = CoreEMA::new(period as usize);
    series.iter().map(|&x| ema.next(x)).collect()
}

#[uniffi::export]
pub fn wma(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut wma = CoreWMA::new(period as usize);
    series.iter().map(|&x| wma.next(x)).collect()
}

#[uniffi::export]
pub fn rsi(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut rsi = RSI::new(period as usize);
    series.iter().map(|&x| rsi.next(x)).collect()
}

#[uniffi::export]
pub fn macd(series: Vec<f64>, fast_period: u64, slow_period: u64, signal_period: u64) -> Vec<MacdResult> {
    let mut macd = MACD::new(fast_period as usize, slow_period as usize, signal_period as usize);
    series.iter().map(|&x| {
        let (macd, signal, histogram) = macd.next(x);
        MacdResult { macd, signal, histogram }
    }).collect()
}

#[uniffi::export]
pub fn bbands(series: Vec<f64>, period: u64, nbdevup: f64, nbdevdn: f64) -> Vec<BbandsResult> {
    let mut bbands = BBANDS::new(period as usize, nbdevup, nbdevdn, quantwave_core::talib::MaType::Sma);
    series.iter().map(|&x| {
        let (upper, middle, lower) = bbands.next(x);
        BbandsResult { upper, middle, lower }
    }).collect()
}

#[uniffi::export]
pub fn atr(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, period: u64) -> Vec<f64> {
    let mut atr = ATR::new(period as usize);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| {
        atr.next((h, l, c))
    }).collect()
}

#[uniffi::export]
pub fn supertrend_batch(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, period: u64, multiplier: f64) -> Vec<SuperTrendResult> {
    let mut st = CoreSuperTrend::new(period as usize, multiplier);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| {
        let (value, direction) = st.next((h, l, c));
        SuperTrendResult { value, direction }
    }).collect()
}

#[uniffi::export]
pub fn adx(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, period: u64) -> Vec<f64> {
    let mut adx = ADX::new(period as usize);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| {
        adx.next((h, l, c))
    }).collect()
}

#[uniffi::export]
pub fn cci(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, period: u64) -> Vec<f64> {
    let mut cci = CCI::new(period as usize);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| {
        cci.next((h, l, c))
    }).collect()
}

#[uniffi::export]
pub fn stoch(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, fastk_period: u64, slowk_period: u64, slowd_period: u64) -> Vec<StochResult> {
    let mut stoch = STOCH::new(fastk_period as usize, slowk_period as usize, quantwave_core::talib::MaType::Sma, slowd_period as usize, quantwave_core::talib::MaType::Sma);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| {
        let (k, d) = stoch.next((h, l, c));
        StochResult { k, d }
    }).collect()
}

#[uniffi::export]
pub fn aroon(high: Vec<f64>, low: Vec<f64>, period: u64) -> Vec<AroonResult> {
    let mut aroon = AROON::new(period as usize);
    high.iter().zip(low.iter()).map(|(&h, &l)| {
        let (up, down) = aroon.next((h, l));
        AroonResult { up, down }
    }).collect()
}

#[uniffi::export]
pub fn mama(series: Vec<f64>, fastlimit: f64, slowlimit: f64) -> Vec<MamaResult> {
    let mut mama = MAMA::new(fastlimit, slowlimit);
    series.iter().map(|&x| {
        let (mama, fama) = mama.next(x);
        MamaResult { mama, fama }
    }).collect()
}

#[uniffi::export]
pub fn kama(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut kama = KAMA::new(period as usize);
    series.iter().map(|&x| kama.next(x)).collect()
}

#[uniffi::export]
pub fn t3(series: Vec<f64>, period: u64, v_factor: f64) -> Vec<f64> {
    let mut t3 = CoreT3::new(period as usize, v_factor);
    series.iter().map(|&x| t3.next(x)).collect()
}

#[uniffi::export]
pub fn sar(high: Vec<f64>, low: Vec<f64>, acceleration: f64, maximum: f64) -> Vec<f64> {
    let mut sar = SAR::new(acceleration, maximum);
    high.iter().zip(low.iter()).map(|(&h, &l)| {
        sar.next((h, l))
    }).collect()
}

#[uniffi::export]
pub fn mom(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut mom = MOM::new(period as usize);
    series.iter().map(|&x| mom.next(x)).collect()
}

#[uniffi::export]
pub fn roc(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut roc = ROC::new(period as usize);
    series.iter().map(|&x| roc.next(x)).collect()
}

#[uniffi::export]
pub fn willr(high: Vec<f64>, low: Vec<f64>, close: Vec<f64>, period: u64) -> Vec<f64> {
    let mut willr = WILLR::new(period as usize);
    high.iter().zip(low.iter()).zip(close.iter()).map(|((&h, &l), &c)| {
        willr.next((h, l, c))
    }).collect()
}

#[uniffi::export]
pub fn dema(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut dema = DEMA::new(period as usize);
    series.iter().map(|&x| dema.next(x)).collect()
}

#[uniffi::export]
pub fn tema(series: Vec<f64>, period: u64) -> Vec<f64> {
    let mut tema = TEMA::new(period as usize);
    series.iter().map(|&x| tema.next(x)).collect()
}

#[uniffi::export]
pub fn ichimoku_batch(high: Vec<f64>, low: Vec<f64>, tenkan: u64, kijun: u64, senkou_b: u64) -> Vec<IchimokuResult> {
    let mut ic = IchimokuCloud::new(tenkan as usize, kijun as usize, senkou_b as usize);
    high.iter().zip(low.iter()).map(|(&h, &l)| {
        let (tenkan, kijun, senkou_a, senkou_b) = ic.next((h, l));
        IchimokuResult { tenkan, kijun, senkou_a, senkou_b }
    }).collect()
}

// --- Streaming Objects ---

#[derive(uniffi::Object)]
pub struct Sma { inner: Mutex<CoreSMA> }
#[uniffi::export]
impl Sma {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(CoreSMA::new(period as usize)) } }
    pub fn next(&self, input: f64) -> f64 { self.inner.lock().unwrap().next(input) }
}

#[derive(uniffi::Object)]
pub struct Ema { inner: Mutex<CoreEMA> }
#[uniffi::export]
impl Ema {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(CoreEMA::new(period as usize)) } }
    pub fn next(&self, input: f64) -> f64 { self.inner.lock().unwrap().next(input) }
}

#[derive(uniffi::Object)]
pub struct Wma { inner: Mutex<CoreWMA> }
#[uniffi::export]
impl Wma {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(CoreWMA::new(period as usize)) } }
    pub fn next(&self, input: f64) -> f64 { self.inner.lock().unwrap().next(input) }
}

#[derive(uniffi::Object)]
pub struct Rsi { inner: Mutex<RSI> }
#[uniffi::export]
impl Rsi {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(RSI::new(period as usize)) } }
    pub fn next(&self, input: f64) -> f64 { self.inner.lock().unwrap().next(input) }
}

#[derive(uniffi::Object)]
pub struct Macd { inner: Mutex<MACD> }
#[uniffi::export]
impl Macd {
    #[uniffi::constructor] pub fn new(fast: u64, slow: u64, signal: u64) -> Self { Self { inner: Mutex::new(MACD::new(fast as usize, slow as usize, signal as usize)) } }
    pub fn next(&self, input: f64) -> MacdResult {
        let (macd, signal, histogram) = self.inner.lock().unwrap().next(input);
        MacdResult { macd, signal, histogram }
    }
}

#[derive(uniffi::Object)]
pub struct SuperTrend { inner: Mutex<CoreSuperTrend> }
#[uniffi::export]
impl SuperTrend {
    #[uniffi::constructor] pub fn new(period: u64, multiplier: f64) -> Self { Self { inner: Mutex::new(CoreSuperTrend::new(period as usize, multiplier)) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> SuperTrendResult {
        let (value, direction) = self.inner.lock().unwrap().next((high, low, close));
        SuperTrendResult { value, direction }
    }
}

#[derive(uniffi::Object)]
pub struct Atr { inner: Mutex<ATR> }
#[uniffi::export]
impl Atr {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(ATR::new(period as usize)) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> f64 { self.inner.lock().unwrap().next((high, low, close)) }
}

#[derive(uniffi::Object)]
pub struct Bbands { inner: Mutex<BBANDS> }
#[uniffi::export]
impl Bbands {
    #[uniffi::constructor] pub fn new(period: u64, nbdevup: f64, nbdevdn: f64) -> Self { Self { inner: Mutex::new(BBANDS::new(period as usize, nbdevup, nbdevdn, quantwave_core::talib::MaType::Sma)) } }
    pub fn next(&self, input: f64) -> BbandsResult {
        let (upper, middle, lower) = self.inner.lock().unwrap().next(input);
        BbandsResult { upper, middle, lower }
    }
}

#[derive(uniffi::Object)]
pub struct Stoch { inner: Mutex<STOCH> }
#[uniffi::export]
impl Stoch {
    #[uniffi::constructor] pub fn new(fastk: u64, slowk: u64, slowd: u64) -> Self { Self { inner: Mutex::new(STOCH::new(fastk as usize, slowk as usize, quantwave_core::talib::MaType::Sma, slowd as usize, quantwave_core::talib::MaType::Sma)) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> StochResult {
        let (k, d) = self.inner.lock().unwrap().next((high, low, close));
        StochResult { k, d }
    }
}

#[derive(uniffi::Object)]
pub struct Adx { inner: Mutex<ADX> }
#[uniffi::export]
impl Adx {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(ADX::new(period as usize)) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> f64 { self.inner.lock().unwrap().next((high, low, close)) }
}

#[derive(uniffi::Object)]
pub struct Cci { inner: Mutex<CCI> }
#[uniffi::export]
impl Cci {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(CCI::new(period as usize)) } }
    pub fn next(&self, high: f64, low: f64, close: f64) -> f64 { self.inner.lock().unwrap().next((high, low, close)) }
}

#[derive(uniffi::Object)]
pub struct Kama { inner: Mutex<KAMA> }
#[uniffi::export]
impl Kama {
    #[uniffi::constructor] pub fn new(period: u64) -> Self { Self { inner: Mutex::new(KAMA::new(period as usize)) } }
    pub fn next(&self, input: f64) -> f64 { self.inner.lock().unwrap().next(input) }
}

#[derive(uniffi::Object)]
pub struct T3 { inner: Mutex<CoreT3> }
#[uniffi::export]
impl T3 {
    #[uniffi::constructor] pub fn new(period: u64, v_factor: f64) -> Self { Self { inner: Mutex::new(CoreT3::new(period as usize, v_factor)) } }
    pub fn next(&self, input: f64) -> f64 { self.inner.lock().unwrap().next(input) }
}

#[derive(uniffi::Object)]
pub struct Mama { inner: Mutex<MAMA> }
#[uniffi::export]
impl Mama {
    #[uniffi::constructor] pub fn new(fastlimit: f64, slowlimit: f64) -> Self { Self { inner: Mutex::new(MAMA::new(fastlimit, slowlimit)) } }
    pub fn next(&self, input: f64) -> MamaResult {
        let (mama, fama) = self.inner.lock().unwrap().next(input);
        MamaResult { mama, fama }
    }
}

#[derive(uniffi::Object)]
pub struct Sar { inner: Mutex<SAR> }
#[uniffi::export]
impl Sar {
    #[uniffi::constructor] pub fn new(acceleration: f64, maximum: f64) -> Self { Self { inner: Mutex::new(SAR::new(acceleration, maximum)) } }
    pub fn next(&self, high: f64, low: f64) -> f64 { self.inner.lock().unwrap().next((high, low)) }
}

#[derive(uniffi::Object)]
pub struct Ichimoku { inner: Mutex<IchimokuCloud> }
#[uniffi::export]
impl Ichimoku {
    #[uniffi::constructor] pub fn new(tenkan: u64, kijun: u64, senkou_b: u64) -> Self { Self { inner: Mutex::new(IchimokuCloud::new(tenkan as usize, kijun as usize, senkou_b as usize)) } }
    pub fn next(&self, high: f64, low: f64) -> IchimokuResult {
        let (tenkan, kijun, senkou_a, senkou_b) = self.inner.lock().unwrap().next((high, low));
        IchimokuResult { tenkan, kijun, senkou_a, senkou_b }
    }
}

uniffi::setup_scaffolding!();
