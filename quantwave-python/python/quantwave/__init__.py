"""
quantwave - High-performance technical analysis library (Python bindings).
"""

from dataclasses import dataclass
from typing import Optional, List, Dict, Any

# Version
try:
    from importlib.metadata import version, PackageNotFoundError
    __version__ = version("quantwave")
except (PackageNotFoundError, Exception):
    __version__ = "0.5.2.dev"

# Import everything from the Rust extension.
# Note: This still pollutes the namespace (a known issue being addressed in 0.5.2+).
# We are gradually cleaning this up.
from ._quantwave import *  # noqa
from . import polars      # noqa

# The ta namespace (Polars-style usage)
from . import ta  # type: ignore

# New recommended namespaces (0.5.2+)
from . import results
from . import options

# Re-export key items from submodules for convenience (without polluting top level too badly)
# Users are encouraged to use quantwave.results.XXX and quantwave.options.XXX going forward.

import warnings

# =============================================================================
# Public Exception Hierarchy (quantwave-p3z9)
# =============================================================================

class QuantwaveError(Exception):
    """Base exception for all quantwave errors."""
    pass

class InternalError(QuantwaveError):
    """Internal error (likely a bug). Please report it."""
    pass

# --- Deprecation handling for old top-level access ---
_DEPRECATED_RESULTS = {
    "MacdResult", "SuperTrendResult", "BbandsResult", "StochResult",
    "IchimokuResult", "DonchianResult", "KeltnerResult", "PivotPointsResult",
    # ... more will be added
}

_DEPRECATED_OPTIONS = {
    "bs_call_price", "bs_delta", "bs_gamma", "max_pain", "chain_pcr",
    "implied_vol", "nse_lot_size", "atm_straddle",
    # etc.
}

def __getattr__(name: str):
    if name in _DEPRECATED_RESULTS:
        warnings.warn(
            f"{name} is deprecated and will be removed in a future version. "
            f"Use quantwave.results.{name} instead.",
            DeprecationWarning,
            stacklevel=2
        )
        return getattr(results, name)

    if name in _DEPRECATED_OPTIONS:
        warnings.warn(
            f"{name} is deprecated and will be removed in a future version. "
            f"Use quantwave.options.{name} instead.",
            DeprecationWarning,
            stacklevel=2
        )
        return getattr(options, name)

    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


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
# Rich Metadata API (quantwave-p3z9 - Critical)
# =============================================================================

@dataclass(frozen=True)
class IndicatorMeta:
    """Structured metadata for an indicator."""
    name: str
    required_params: List[str]
    optional_params: Dict[str, Any]          # name -> default value
    data_inputs: List[str]                   # e.g. ["high", "low", "close"]
    outputs: List[str]
    warmup_bars: Optional[int] = None        # Rough estimate or None if dynamic
    category: Optional[str] = None
    has_streaming: bool = True
    has_polars: bool = True
    description: Optional[str] = None


# Initial metadata registry (will grow significantly)
# This is the foundation for many other DX improvements.
_METADATA: Dict[str, IndicatorMeta] = {
    "rsi": IndicatorMeta(
        name="rsi",
        required_params=["period"],
        optional_params={},
        data_inputs=["close"],
        outputs=["rsi"],
        warmup_bars=14,
        category="Momentum",
        description="Relative Strength Index",
    ),
    "macd": IndicatorMeta(
        name="macd",
        required_params=[],
        optional_params={"fast": 12, "slow": 26, "signal": 9},
        data_inputs=["close"],
        outputs=["macd", "signal", "histogram"],
        warmup_bars=26,
        category="Momentum",
        description="Moving Average Convergence Divergence",
    ),
    "supertrend": IndicatorMeta(
        name="supertrend",
        required_params=["period", "multiplier"],
        optional_params={},
        data_inputs=["high", "low", "close"],
        outputs=["supertrend", "direction"],
        warmup_bars=None,  # Depends on ATR period
        category="Volatility / Trend",
        description="SuperTrend",
    ),
    "atr": IndicatorMeta(
        name="atr",
        required_params=["period"],
        optional_params={},
        data_inputs=["high", "low", "close"],
        outputs=["atr"],
        warmup_bars=14,
        category="Volatility",
        description="Average True Range",
    ),
    "bbands": IndicatorMeta(
        name="bbands",
        required_params=["period"],
        optional_params={"std_dev": 2.0},
        data_inputs=["close"],
        outputs=["upper", "middle", "lower"],
        warmup_bars=20,
        category="Volatility",
        description="Bollinger Bands",
    ),
    "ema": IndicatorMeta(
        name="ema",
        required_params=["period"],
        optional_params={},
        data_inputs=["close"],
        outputs=["ema"],
        warmup_bars=20,
        category="Overlap",
    ),
    "sma": IndicatorMeta(
        name="sma",
        required_params=["period"],
        optional_params={},
        data_inputs=["close"],
        outputs=["sma"],
        warmup_bars=20,
        category="Overlap",
    ),
    "stoch": IndicatorMeta(
        name="stoch",
        required_params=["fastk", "slowk", "slowd"],
        optional_params={},
        data_inputs=["high", "low", "close"],
        outputs=["slowk", "slowd"],
        warmup_bars=14,
        category="Momentum",
    ),
    "willr": IndicatorMeta(
        name="willr",
        required_params=["period"],
        optional_params={},
        data_inputs=["high", "low", "close"],
        outputs=["willr"],
        warmup_bars=14,
        category="Momentum",
    ),
    "obv": IndicatorMeta(
        name="obv",
        required_params=[],
        optional_params={},
        data_inputs=["close", "volume"],
        outputs=["obv"],
        warmup_bars=1,
        category="Volume",
    ),
}


def metadata(name: str) -> Optional[IndicatorMeta]:
    """
    Return rich metadata for an indicator.

    Returns None if the indicator is unknown.
    This is the foundation for better tooling, docs, and UIs.
    """
    key = name.lower()
    return _METADATA.get(key)


def list_metadata() -> List[IndicatorMeta]:
    """Return metadata for all known indicators."""
    return list(_METADATA.values())


def warmup_bars(name: str, params: dict = None) -> int:
    """
    Return the approximate number of bars required before this indicator
    produces reliable (non-warmup) output.

    This is extremely useful for backtesting and rolling window calculations.
    """
    meta = metadata(name)
    if not meta:
        return 0

    if meta.warmup_bars is not None:
        return meta.warmup_bars

    # Dynamic calculation based on params
    p = params or {}
    max_period = 0
    for key in ["period", "fast", "slow", "signal", "length", "fastk", "slowk", "slowd"]:
        if key in p:
            try:
                max_period = max(max_period, int(p[key]))
            except (ValueError, TypeError):
                pass

    if max_period > 0:
        return max_period + 5  # small safety margin

    return 20  # reasonable default for most indicators


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
