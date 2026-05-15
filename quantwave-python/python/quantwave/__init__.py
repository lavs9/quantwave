from ._quantwave import *  # noqa

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
    aroon = aroon
    mama = mama
    kama = kama
    t3 = t3
    sar = sar
    mom = mom
    roc = roc
    willr = willr
    dema = dema
    tema = tema
    ichimoku = ichimoku_batch
    cg = cg
    cybercycle = cybercycle
    fisher = fisher
    inverse_fisher = inverse_fisher
    supersmoother = supersmoother
    bandpass = bandpass
    roofing_filter = roofing_filter
    zerolag = zerolag
    choppiness_index = choppiness_index
    classic_laguerre = classic_laguerre
    alligator = Alligator
    alma = alma
    atr_ts = AtrTs
    butterworth2 = butterworth2
    butterworth3 = butterworth3
    channel_cycle = channel_cycle
    continuation_index = continuation_index
    correlation_cycle = correlation_cycle
    correlation_trend = correlation_trend
    cybernetic_oscillator = cybernetic_oscillator
    dmh = dmh
    donchian = donchian
    dsma = dsma
    emd = emd
    frama = frama
    am_detector = am_detector
    fm_demodulator = fm_demodulator
    ehlers_autocorrelation = EhlersAutocorrelation
    ehlers_filter = EhlersFilter
    ehlers_loops = EhlersLoops
    ehlers_stochastic = EhlersStochastic
    ehlers_ultimate_oscillator = EhlersUltimateOscillator
    fisher_high_pass = FisherHighPass
    fourier_series = FourierSeries
    fourier_dominant_cycle = FourierDominantCycle
    fractals = Fractals
    gaussian = Gaussian
    generalized_laguerre = GeneralizedLaguerre
    griffiths_dominant_cycle = GriffithsDominantCycle
    griffiths_predictor = GriffithsPredictor
    griffiths_spectrum = GriffithsSpectrum
    hamming = Hamming
    hann = Hann
    heikin_ashi = HeikinAshi
    high_pass = HighPass
    hma = Hma
    ehlers_wma4 = EhlersWma4
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
    pivot_points = pivotpoints
    one_euro_filter = oneeurofilter
    projected_moving_average = projectedmovingaverage
    precision_trend = precisiontrend
    reversion_index = reversionindex
    sine_wave = sinewave
    swiss_army_knife = swiss_army_knife
    system_evaluator = systemevaluator
    robustness_evaluator = RobustnessEvaluator
    ttm_squeeze = ttmsqueeze
    ultimate_bands = ultimatebands
    ultimate_channel = ultimatechannel
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

__all__ = [
    "ta", "Sma", "Ema", "Rsi", "SuperTrend", "Macd", "Atr", "Adx", "Cci", "Stoch", 
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
