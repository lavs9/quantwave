"""
Internal metadata registry for indicators (Python side).

NOTE: The primary source of truth is in Rust (`quantwave-core/src/indicators/metadata.rs`
and the per-indicator `*_METADATA` constants).

This Python file is currently manually synced. See task quantwave-i9dn for the
mandatory process rule.

Long-term plan: Auto-generated from Rust (see quantwave-iqq7 and scripts/generate_indicator_metadata.py).

Warmup / NaN semantics (quantwave-976r)
---------------------------------------
During the warmup period an indicator has not yet accumulated enough history to
produce a meaningful value. QuantWave uses three conventions:

1. **NaN (most common)** — batch Polars columns and streaming `next()` return
   `float('nan')` (or a struct whose float fields are NaN) until `warmup_bars`
   bars have been consumed. TA-Lib-compatible indicators (EMA, RSI, etc.) follow
   this pattern.

2. **Truncated / partial** — some cumulative indicators (OBV, NVI, PVI) emit a
   value from bar 1 but the series is not yet "stable" for the configured period.
   Treat `warmup_bars` as the bar count before the output is period-complete.

3. **Event / struct output** — price-action indicators (MarketStructure, S/R monitor,
   geometric patterns) return empty event lists or default structs during early bars;
   there is no scalar NaN. Use `wrap_streaming(..., name=...)` and `is_ready`.

Use `warmup_bars(name, params)` before backtesting or parity checks to skip or
mask the initial bars. `metadata(name).warmup_bars` holds the curated value when
known; otherwise a conservative heuristic derives it from `period` / `fast` / `slow`.
"""

from dataclasses import dataclass
from typing import Optional, List, Dict, Any


@dataclass(frozen=True)
class IndicatorMeta:
    """Structured metadata for an indicator."""
    name: str
    required_params: List[str]
    optional_params: Dict[str, Any]          # name -> default value
    data_inputs: List[str]                   # e.g. ["high", "low", "close"]
    outputs: List[str]
    warmup_bars: Optional[int] = None
    category: Optional[str] = None
    has_streaming: bool = True
    has_polars: bool = True
    description: Optional[str] = None


# This dictionary is currently hand-maintained.
# Goal for future releases: generate it automatically from Rust.
_METADATA: Dict[str, IndicatorMeta] = {
    # Momentum / Oscillators
    "rsi": IndicatorMeta("rsi", ["period"], {}, ["close"], ["rsi"], 14, "Momentum", description="Relative Strength Index"),
    "macd": IndicatorMeta("macd", [], {"fast": 12, "slow": 26, "signal": 9}, ["close"], ["macd", "signal", "histogram"], 26, "Momentum"),
    "stoch": IndicatorMeta("stoch", ["fastk", "slowk", "slowd"], {}, ["high", "low", "close"], ["slowk", "slowd"], 14, "Momentum"),
    "adx": IndicatorMeta("adx", ["period"], {}, ["high", "low", "close"], ["adx", "plus_di", "minus_di"], 14, "Momentum"),
    "cci": IndicatorMeta("cci", ["period"], {}, ["high", "low", "close"], ["cci"], 20, "Momentum"),
    "willr": IndicatorMeta("willr", ["period"], {}, ["high", "low", "close"], ["willr"], 14, "Momentum"),
    "roc": IndicatorMeta("roc", ["period"], {}, ["close"], ["roc"], 10, "Momentum"),
    "mom": IndicatorMeta("mom", ["period"], {}, ["close"], ["mom"], 10, "Momentum"),
    "aroon": IndicatorMeta("aroon", ["period"], {}, ["high", "low"], ["aroon_up", "aroon_down"], 14, "Momentum"),

    # Overlap / Moving Averages
    "sma": IndicatorMeta("sma", ["period"], {}, ["close"], ["sma"], 20, "Overlap"),
    "ema": IndicatorMeta("ema", ["period"], {}, ["close"], ["ema"], 20, "Overlap"),
    "wma": IndicatorMeta("wma", ["period"], {}, ["close"], ["wma"], 20, "Overlap"),
    "dema": IndicatorMeta("dema", ["period"], {}, ["close"], ["dema"], 20, "Overlap"),
    "tema": IndicatorMeta("tema", ["period"], {}, ["close"], ["tema"], 20, "Overlap"),
    "t3": IndicatorMeta("t3", ["period"], {}, ["close"], ["t3"], 20, "Overlap"),
    "kama": IndicatorMeta("kama", ["period"], {}, ["close"], ["kama"], 20, "Overlap"),
    "hma": IndicatorMeta("hma", ["period"], {}, ["close"], ["hma"], 20, "Overlap"),
    "alma": IndicatorMeta("alma", ["period"], {}, ["close"], ["alma"], 20, "Overlap"),
    "frama": IndicatorMeta("frama", ["period"], {}, ["close"], ["frama"], 20, "Overlap"),

    # Volatility / Trend
    "supertrend": IndicatorMeta("supertrend", ["period", "multiplier"], {}, ["high", "low", "close"], ["supertrend", "direction"], None, "Volatility / Trend"),
    "atr": IndicatorMeta("atr", ["period"], {}, ["high", "low", "close"], ["atr"], 14, "Volatility"),
    "bbands": IndicatorMeta("bbands", ["period"], {"std_dev": 2.0}, ["close"], ["upper", "middle", "lower"], 20, "Volatility"),
    "donchian": IndicatorMeta("donchian", ["period"], {}, ["high", "low"], ["upper", "middle", "lower"], 20, "Volatility"),
    "keltner": IndicatorMeta("keltner", ["ema_period", "atr_period", "multiplier"], {}, ["high", "low", "close"], ["upper", "middle", "lower"], 20, "Volatility"),

    # Volume
    "obv": IndicatorMeta("obv", [], {}, ["close", "volume"], ["obv"], 1, "Volume"),
    "ad": IndicatorMeta("ad", [], {}, ["high", "low", "close", "volume"], ["ad"], 1, "Volume"),
    "adosc": IndicatorMeta("adosc", ["fast", "slow"], {}, ["high", "low", "close", "volume"], ["adosc"], 20, "Volume"),
    "vwap": IndicatorMeta("vwap", [], {}, ["high", "low", "close", "volume"], ["vwap"], 1, "Volume"),
    "anchored_vwap": IndicatorMeta("anchored_vwap", [], {}, ["price", "volume", "anchor"], ["vwap"], 1, "Volume"),

    # Ichimoku & others
    "ichimoku": IndicatorMeta("ichimoku", ["tenkan", "kijun", "senkou_b"], {}, ["high", "low", "close"], ["tenkan", "kijun", "senkou_a", "senkou_b", "chikou"], 52, "Trend"),
    "pivot_points": IndicatorMeta("pivot_points", [], {}, ["high", "low", "close"], ["pivot", "r1", "s1", "r2", "s2"], 1, "Support/Resistance"),

    # Price Action / Structure (cu03 / MQL5 inspired)
    "fractals": IndicatorMeta("fractals", [], {}, ["high", "low"], ["fractal_high", "fractal_low"], 5, "Price Action"),
    "heikin_ashi": IndicatorMeta("heikin_ashi", [], {}, ["open", "high", "low", "close"], ["ha_open", "ha_high", "ha_low", "ha_close"], 1, "Candlestick"),

    # Volume / Money Flow
    "mfi": IndicatorMeta("mfi", ["period"], {}, ["high", "low", "close", "volume"], ["mfi"], 14, "Volume / Momentum"),
    "cmf": IndicatorMeta("cmf", ["period"], {}, ["high", "low", "close", "volume"], ["cmf"], 20, "Volume"),
    "force_index": IndicatorMeta("force_index", ["period"], {}, ["close", "volume"], ["force"], 13, "Volume"),
    "eom": IndicatorMeta("eom", ["period"], {}, ["high", "low", "close", "volume"], ["eom"], 14, "Volume"),
    "nvi": IndicatorMeta("nvi", [], {}, ["close", "volume"], ["nvi"], 1, "Volume"),
    "pvi": IndicatorMeta("pvi", [], {}, ["close", "volume"], ["pvi"], 1, "Volume"),

    # Ehlers DSP / Cycle (core for ML features tha/gw7s)
    "cybercycle": IndicatorMeta("cybercycle", ["period"], {}, ["close"], ["cycle", "trigger"], 30, "Cycle / Ehlers", description="Cyber Cycle (Ehlers)"),
    "trendflex": IndicatorMeta("trendflex", ["period"], {}, ["close"], ["trendflex"], 30, "Cycle / Ehlers"),
    "instantaneous_trendline": IndicatorMeta("instantaneous_trendline", ["period"], {}, ["close"], ["trendline", "price"], 30, "Cycle / Ehlers"),
    "hurst_exponent": IndicatorMeta("hurst_exponent", ["period"], {}, ["close"], ["hurst"], 100, "Regime / Statistics", description="Hurst Exponent (regime feature)"),
    "griffiths_dominant_cycle": IndicatorMeta("griffiths_dominant_cycle", ["period"], {}, ["close"], ["dominant_cycle"], 50, "Cycle / Ehlers"),

    # Regime / State
    "market_state": IndicatorMeta("market_state", [], {}, ["close"], ["state", "confidence"], 50, "Regime"),

    # Add more via metadata expansion or future auto-gen (iqq7)
}

def metadata(name: str) -> Optional[IndicatorMeta]:
    key = name.lower()
    if key in _METADATA:
        return _METADATA[key]
    # simple aliases
    aliases = {"bollinger_bands": "bbands", "atr_trailing_stop": "atr_ts"}
    if key in aliases:
        return _METADATA.get(aliases[key])
    return None

def list_metadata() -> List[IndicatorMeta]:
    return list(_METADATA.values())

def warmup_bars(name: str, params: dict = None) -> int:
    """Return the number of initial bars to treat as warmup for an indicator.

    Warmup bars are the count of leading observations where output may be NaN,
    truncated, or otherwise not yet period-complete. After this many bars (0-indexed:
    indices ``0 .. warmup-1`` are warmup), the indicator is considered ready for
    signal generation and parity comparison.

    Args:
        name: Indicator name (case-insensitive), e.g. ``"rsi"``, ``"supertrend"``.
        params: Optional parameter dict used to refine the estimate when metadata
            does not pin an explicit warmup (e.g. ``{"period": 21}``).

    Returns:
        Non-negative bar count. Returns ``0`` for unknown indicators (caller should
        apply a conservative default or inspect output manually).

    Example:
        >>> warmup_bars("rsi", {"period": 14})
        14
        >>> warmup_bars("macd")  # uses metadata default slow=26
        26
    """
    meta = metadata(name)
    if not meta:
        return 0

    p = dict(meta.optional_params)
    if params:
        p.update(params)

    period_keys = (
        "period", "fast", "slow", "signal", "length",
        "fastk", "slowk", "slowd", "tenkan", "kijun", "senkou_b",
        "ema_period", "atr_period", "swing_strength", "rsi_length", "smooth_length",
    )
    explicit_period = params and any(k in params for k in period_keys)

    max_period = 0
    for key in period_keys:
        if key in p:
            try:
                max_period = max(max_period, int(p[key]))
            except (ValueError, TypeError):
                pass

    # Caller-supplied periods override the curated metadata default.
    if explicit_period and max_period > 0:
        return max_period + (5 if name.lower() in ("supertrend", "ichimoku") else 0)

    if meta.warmup_bars is not None:
        return meta.warmup_bars

    # Ichimoku and other multi-window indicators need the largest window + small buffer.
    if max_period > 0:
        return max_period + (5 if name.lower() in ("supertrend", "ichimoku") else 0)

    # Event-driven / unknown: no scalar warmup; streaming wrapper uses bars_consumed > 0.
    if meta.category and "price action" in meta.category.lower():
        return 0

    return 20

from typing import TypeVar, Generic

T = TypeVar("T")

class Param(Generic[T]):
    """Marker type to indicate a parameter (not a data series)."""
    pass

class Series(Generic[T]):
    """Marker type to indicate a data series input."""
    pass


def get_indicator_signature(name: str):
    """
    Returns a clear separation of parameters vs data inputs for an indicator.
    This helps solve the historical ambiguity in function signatures (s8s3).
    Use in combination with Param/Series markers for future typed APIs.
    """
    meta = metadata(name)
    if not meta:
        return None
    return {
        "params": meta.required_params + list(meta.optional_params.keys()),
        "data": meta.data_inputs,
        "outputs": meta.outputs,
    }
