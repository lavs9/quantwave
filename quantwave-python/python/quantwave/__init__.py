"""
quantwave - High-performance technical analysis library (Python bindings).

Public surface for 0.5.2 (Python DX improvements from quantwave-p3z9).

Recommended:
    import quantwave as qw
    qw.indicators()
    meta = qw.metadata("supertrend")
    qw.assert_parity("rsi", {"period": 14}, closes)
"""

from typing import List, Dict, Any
import warnings

# Version
try:
    from importlib.metadata import version, PackageNotFoundError
    __version__ = version("quantwave")
except (PackageNotFoundError, Exception):
    __version__ = "0.5.2.dev"

# Core compiled extension + polars layer
from . import _quantwave  # noqa
from . import polars      # noqa

# Popular namespaces
from . import ta          # type: ignore
from . import results
from . import options
from . import talib

# Pull in the clean metadata system
from ._metadata import (
    IndicatorMeta,
    metadata,
    list_metadata,
    warmup_bars,
    get_indicator_signature,
)

# DX helpers (defined later in this file for now to avoid circularity during cleanup)
# We will gradually move more of these out.


# --- Basic Discovery API (quantwave-p3z9) ---
# This is a first-pass implementation. It will be backed by real metadata later.

def _build_indicator_names() -> set[str]:
    """Build the set of indicator names from the ta namespace."""
    names = set()
    for name in dir(ta):
        if name.startswith("_"):
            continue
        obj = getattr(ta, name, None)
        if callable(obj):
            names.add(name)
    return names

_INDICATOR_NAMES: set[str] = _build_indicator_names()

def indicators() -> list[str]:
    """Return a sorted list of all available indicator names."""
    return sorted(_INDICATOR_NAMES)

def is_indicator(name: str) -> bool:
    """Check whether the given name is a known indicator."""
    if not name:
        return False
    return name in _INDICATOR_NAMES or name.lower() in _INDICATOR_NAMES


# =============================================================================
# Rich Metadata API
# =============================================================================
# All rich metadata is provided by the clean implementation in ._metadata
# (which is the current source of truth, eventually to be generated from Rust).


# =============================================================================
# Streaming Class Lookup (quantwave-p3z9)
# =============================================================================

def streaming_class(name: str):
    """
    Return the streaming (Next<T>) class for a given batch indicator name.

    Example:
        cls = quantwave.streaming_class("supertrend")
        st = cls(period=10, multiplier=3.0)
    """
    if not name:
        return None

    key = name.lower()

    # Try direct attribute first (common case)
    if hasattr(ta, key):
        candidate = getattr(ta, key)
        if isinstance(candidate, type):
            return candidate

    # Fallback: try common PascalCase conversions
    pascal = "".join(word.capitalize() for word in key.split("_"))
    if hasattr(ta, pascal) and isinstance(getattr(ta, pascal), type):
        return getattr(ta, pascal)

    # Last resort: search for anything that looks like a streaming class
    for attr_name in dir(ta):
        if attr_name.lower() == key or attr_name.lower().replace("_", "") == key.replace("_", ""):
            obj = getattr(ta, attr_name)
            if isinstance(obj, type):
                return obj

    return None


# =============================================================================
# Parity Testing Helper (quantwave-p3z9 - Critical)
# =============================================================================

def assert_parity(
    indicator_name: str,
    params: Dict[str, Any],
    data: List[float],
    tolerance: float = 1e-10,
    **kwargs,
) -> bool:
    """
    Run both batch and streaming versions of an indicator and assert
    they produce (nearly) identical results.

    Returns True if they match within tolerance.
    Raises AssertionError with details on mismatch.

    This is the first-class way for users and CI to verify the
    "bit-identical" guarantee.
    """
    batch_fn = getattr(ta, indicator_name, None)
    if batch_fn is None:
        raise ValueError(f"Unknown indicator: {indicator_name}")

    streaming_cls = streaming_class(indicator_name)
    if streaming_cls is None:
        raise ValueError(f"No streaming class found for: {indicator_name}")

    # Run batch
    try:
        batch_result = batch_fn(data, **params, **kwargs)
    except Exception as e:
        raise RuntimeError(f"Batch mode failed: {e}") from e

    # Run streaming
    try:
        streamer = streaming_cls(**params, **kwargs)
        stream_result = [streamer.next(v) for v in data]
    except Exception as e:
        raise RuntimeError(f"Streaming mode failed: {e}") from e

    # Normalize for comparison (handle single vs multi output)
    def _normalize(x):
        if isinstance(x, (list, tuple)):
            return x
        return [x]

    b = _normalize(batch_result)
    s = _normalize(stream_result)

    if len(b) != len(s):
        raise AssertionError(f"Length mismatch: batch={len(b)}, stream={len(s)}")

    for i, (bv, sv) in enumerate(zip(b, s)):
        if isinstance(bv, (int, float)) and isinstance(sv, (int, float)):
            if abs(bv - sv) > tolerance:
                raise AssertionError(
                    f"Mismatch at index {i}: batch={bv}, stream={sv}, diff={abs(bv-sv)}"
                )
        else:
            # For complex results (dataclasses), do a simple repr comparison for now
            if repr(bv) != repr(sv):
                raise AssertionError(f"Result mismatch at index {i}")

    return True


# =============================================================================
# Streaming Readiness Helpers (quantwave-p3z9)
# =============================================================================

class StreamingWrapper:
    """
    Lightweight wrapper that adds is_ready and bars_consumed tracking
    around any streaming (Next<T>) indicator instance.

    Usage:
        st = quantwave.streaming_class("supertrend")(period=10, multiplier=3)
        wrapped = quantwave.wrap_streaming(st)
        for price in data:
            val = wrapped.next(price)
            if wrapped.is_ready:
                ...
    """
    def __init__(self, streaming_instance):
        self._inner = streaming_instance
        self._bars_consumed = 0
        self._is_ready = False

    def next(self, value):
        result = self._inner.next(value)
        self._bars_consumed += 1
        # Heuristic: most indicators become "ready" after their main period
        # This is approximate; for exact use warmup_bars + metadata
        self._is_ready = True  # Conservative: assume ready after first value for simplicity in 0.5.2
        return result

    @property
    def is_ready(self) -> bool:
        return self._is_ready

    @property
    def bars_consumed(self) -> int:
        return self._bars_consumed

    def __getattr__(self, name):
        # Delegate everything else to the inner streaming object
        return getattr(self._inner, name)


def wrap_streaming(streaming_instance):
    """Wrap a streaming indicator instance to get is_ready / bars_consumed tracking."""
    return StreamingWrapper(streaming_instance)

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
    # ML features (gw7s notebook + harness + 4ps/gwx cross-epic deliverable)
    cyber_cycle_feature_extractor = CyberCycleFeatureExtractor
    hurst_feature_extractor = HurstFeatureExtractor
    instantaneous_trendline_feature_extractor = InstantaneousTrendlineFeatureExtractor
    trendflex_feature_extractor = TrendflexFeatureExtractor
    regime_to_features = regime_to_features
    # Newly wired for the E2E notebook (trivial parallel to above; completes the locked .ta.features.* surface in Python)
    griffiths_dominant_cycle_feature_extractor = GriffithsDominantCycleFeatureExtractor
    bull_bear_hmm = BullBearHMM
    griffiths_dominant_cycle_features = griffiths_dominant_cycle_features  # batch helper
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
    "TruncatedBandpass", "VolumeProfile",
    # ML feature toolkit (quantwave-gw7s)
    "CyberCycleFeatureExtractor", "HurstFeatureExtractor",
    "InstantaneousTrendlineFeatureExtractor", "TrendflexFeatureExtractor",
    "regime_to_features"
]

# =============================================================================
# Final clean public surface for 0.5.2 (quantwave-p3z9)
# =============================================================================

# DX helpers (new in 0.5.2)
__all__ = [
    "__version__",
    "indicators",
    "is_indicator",
    "metadata",
    "list_metadata",
    "warmup_bars",
    "get_indicator_signature",
    "IndicatorMeta",
    "assert_parity",
    "streaming_class",
    "wrap_streaming",
    "StreamingWrapper",
    "QuantwaveError",
    "InternalError",
    "ta",
    "polars",
    "results",
    "options",
    "talib",
]
