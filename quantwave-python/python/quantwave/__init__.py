from ._quantwave import *  # noqa
from . import polars  # noqa

# Nice namespace
class ta:
    sma = sma
    ema = ema
    rsi = rsi
    supertrend = SuperTrend
    macd = Macd
    atr = atr
    adx = adx
    cci = cci
    stoch = stoch
    aroon = Aroon
    mama = Mama
    kama = kama
    t3 = T3
    sar = sar
    mom = mom
    roc = roc
    willr = willr
    dema = dema
    tema = tema
    ichimoku = ichimoku
    cg = cg
    cybercycle = cybercycle
    fisher = fisher
    inverse_fisher = inversefisher
    supersmoother = supersmoother
    bandpass = bandpass
    roofing_filter = roofingfilter
    zerolag = zerolag
    choppiness_index = choppinessindex
    classic_laguerre = classiclaguerre
    alligator = Alligator
    alma = alma
    atr_ts = AtrTs
    butterworth2 = butterworth2
    butterworth3 = butterworth3
    channel_cycle = channelcycle
    continuation_index = continuationindex
    correlation_cycle = correlationcycle
    correlation_trend = correlationtrend
    cybernetic_oscillator = cyberneticoscillator
    dmh = dmh
    donchian = donchian
    dsma = dsma
    emd = emd
    frama = frama
    am_detector = amdetector
    fm_demodulator = fmdemodulator
    ehlers_autocorrelation = ehlersautocorrelation
    ehlers_filter = ehlersfilter
    ehlers_loops = ehlersloops
    ehlers_stochastic = ehlersstochastic
    ehlers_ultimate_oscillator = ehlersultimateoscillator
    fisher_high_pass = fisherhighpass
    fourier_series = fourierseries
    fourier_dominant_cycle = fourierdominantcycle
    fractals = fractals
    gaussian = gaussian
    generalized_laguerre = generalizedlaguerre
    griffiths_dominant_cycle = griffithsdominantcycle
    griffiths_predictor = griffithspredictor
    griffiths_spectrum = griffithsspectrum
    hamming = hamming
    hann = hann
    heikin_ashi = heikin_ashi
    high_pass = highpass
    hma = hma
    ehlers_wma4 = ehlerswma4
    instantaneous_trendline = instantaneoustrendline
    undersampled_double_ma = undersampleddoublema
    keltner = keltner
    laguerre_filter = laguerrefilter
    laguerre_oscillator = laguerreoscillator
    laguerre_rsi = laguerrersi
    noise_elimination = noiseelimination
    pairs_rotation = pairsrotation
    phasor = phasor
    oc_price_rsi = ocpricersi
    pivot_points = pivot_points
    one_euro_filter = oneeurofilter
    projected_moving_average = projectedmovingaverage
    precision_trend = precisiontrend
    reversion_index = reversionindex
    sine_wave = sinewave
    swiss_army_knife = swiss_army_knife
    system_evaluator = systemevaluator
    ttm_squeeze = ttmsqueeze
    ultimate_bands = ultimate_bands
    ultimate_channel = ultimate_channel
    ultimate_smoother = ultimatesmoother
    usi = usi
    ad = ad
    adosc = adosc
    obv = obv
    vortex = vortex
    anchored_vwap = anchored_vwap
    wavetrend = wavetrend
    simple_predictor = simplepredictor
    mad = mad
    mesa_stochastic = mesastochastic
    rsih = rsih
    voss_predictor = vosspredictor
    synthetic_oscillator = syntheticoscillator
    cycle_trend_analytics = cycletrendanalytics
    madh = madh
    stc = stc
    homodyne_discriminator = homodynediscriminator
    universal_oscillator = universaloscillator
    triangle_filter = trianglefilter
    ht_dc_period = htdcperiod
    ht_phasor = htphasor
    ht_dc_phase = htdcphase
    ht_sine = htsine
    ht_trend_mode = httrendmode
    hurst_exponent = hurstexponent
    kalman_filter = kalmanfilter
    market_state = marketstate
    recursive_median = recursivemedian
    recursive_median_oscillator = recursivemedianoscillator
    reflex = reflex
    rocket_rsi = rocketrsi
    trendflex = trendflex
    truncated_bandpass = truncatedbandpass
    volume_profile = volumeprofile

class options_india:
    bs_call_price = bs_call_price
    bs_put_price = bs_put_price
    bs_delta = bs_delta
    bs_gamma = bs_gamma
    bs_theta = bs_theta
    bs_vega = bs_vega
    bs_rho = bs_rho
    implied_vol = implied_vol
    max_pain = max_pain
    strike_pcr = strike_pcr
    chain_pcr = chain_pcr
    oi_zones = oi_zones
    gex_per_strike = gex_per_strike
    gex_flip_strike = gex_flip_strike
    atm_straddle = atm_straddle
    synthetic_futures = synthetic_futures
    moneyness = moneyness
    nse_lot_size = nse_lot_size
    nse_risk_free_rate = nse_risk_free_rate

__all__ = [
    "ta", "options_india", "Sma", "Ema", "Rsi", "SuperTrend", "Macd", "Atr", "Adx", "Cci", "Stoch", 
    "Aroon", "Mama", "Kama", "T3", "Sar", "Mom", "Roc", "Willr", "Dema", "Tema", 
    "Ichimoku", "Cg", "CyberCycle", "Fisher", "InverseFisher", "SuperSmoother", 
    "Bandpass", "RoofingFilter", "ZeroLag", "ChoppinessIndex", "ClassicLaguerre",
    "Alligator", "AtrTs", "Aroon", "Donchian", "Emd", "ChannelCycle", "CorrelationCycle",
    "EhlersAutocorrelation", "EhlersFilter", "EhlersLoops", "EhlersStochastic", 
    "EhlersUltimateOscillator", "FisherHighPass", "FourierSeries", "FourierDominantCycle",
    "Fractals", "Gaussian", "GeneralizedLaguerre", "GriffithsDominantCycle", 
    "GriffithsPredictor", "GriffithsSpectrum", "Hamming", "Hann", "HeikinAshi", 
    "HighPass", "Hma", "EhlersWma4", "InstantaneousTrendline", "UndersampledDoubleMa",
    "Keltner", "LaguerreFilter", "LaguerreOscillator", "LaguerreRsi", "NoiseElimination",
    "PairsRotation", "Phasor", "OcPriceRsi", "PivotPoints", "OneEuroFilter",
    "ProjectedMovingAverage", "PrecisionTrend", "ReversionIndex", "SineWave",
    "SwissArmyKnife", "SystemEvaluator", "RobustnessEvaluator", "TtmSqueeze",
    "UltimateBands", "UltimateChannel", "UltimateSmoother", "Usi", "Ad", "Adosc", "Obv",
    "Vortex", "AnchoredVwap", "WaveTrend", "SimplePredictor", "Mad", "MesaStochastic",
    "Rsih", "VossPredictor", "SyntheticOscillator", "CycleTrendAnalytics", "Madh", "Stc",
    "HomodyneDiscriminator", "UniversalOscillator", "TriangleFilter",
    "HtDcPeriod", "HtPhasor", "HtDcPhase", "HtSine", "HtTrendMode",
    "HurstExponent", "KalmanFilter", "MarketState", "RecursiveMedian",
    "RecursiveMedianOscillator", "Reflex", "RocketRsi", "Trendflex",
    "TruncatedBandpass", "VolumeProfile"
]
