"""
Internal metadata registry for indicators (Python side).

NOTE: The primary source of truth is in Rust (`quantwave-core/src/indicators/metadata.rs`
and the per-indicator `*_METADATA` constants).

This Python file is currently manually synced. See task quantwave-i9dn for the
mandatory process rule.

Long-term plan: Auto-generated from Rust (see quantwave-iqq7 and scripts/generate_indicator_metadata.py).
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

    # Add more as needed...
}

def metadata(name: str) -> Optional[IndicatorMeta]:
    key = name.lower()
    return _METADATA.get(key)

def list_metadata() -> List[IndicatorMeta]:
    return list(_METADATA.values())

def warmup_bars(name: str, params: dict = None) -> int:
    meta = metadata(name)
    if not meta:
        return 0
    if meta.warmup_bars is not None:
        return meta.warmup_bars

    p = params or {}
    max_period = 0
    for key in ["period", "fast", "slow", "signal", "length", "fastk", "slowk", "slowd"]:
        if key in p:
            try:
                max_period = max(max_period, int(p[key]))
            except (ValueError, TypeError):
                pass
    return max_period + 5 if max_period > 0 else 20

def get_indicator_signature(name: str):
    meta = metadata(name)
    if not meta:
        return None
    return {
        "params": meta.required_params + list(meta.optional_params.keys()),
        "data": meta.data_inputs,
        "outputs": meta.outputs,
    }

    # More common indicators (expanded for 0.5.2)
    "bollinger_bands": IndicatorMeta("bollinger_bands", ["period"], {"std_dev": 2.0}, ["close"], ["upper", "middle", "lower"], 20, "Volatility"),
    "atr_trailing_stop": IndicatorMeta("atr_trailing_stop", ["period", "multiplier"], {}, ["high", "low", "close"], ["stop"], None, "Volatility"),
    "pivot_points": IndicatorMeta("pivot_points", [], {}, ["high", "low", "close"], ["pivot", "r1", "s1"], 1, "Support/Resistance"),
    "fractals": IndicatorMeta("fractals", [], {}, ["high", "low"], ["fractal_high", "fractal_low"], 5, "Price Action"),
    "heikin_ashi": IndicatorMeta("heikin_ashi", [], {}, ["open", "high", "low", "close"], ["ha_open", "ha_high", "ha_low", "ha_close"], 1, "Candlestick"),
    "mfi": IndicatorMeta("mfi", ["period"], {}, ["high", "low", "close", "volume"], ["mfi"], 14, "Volume / Momentum"),
    "cmf": IndicatorMeta("cmf", ["period"], {}, ["high", "low", "close", "volume"], ["cmf"], 20, "Volume"),
    "force_index": IndicatorMeta("force_index", ["period"], {}, ["close", "volume"], ["force"], 13, "Volume"),
    "eom": IndicatorMeta("eom", ["period"], {}, ["high", "low", "close", "volume"], ["eom"], 14, "Volume"),
    "nvi": IndicatorMeta("nvi", [], {}, ["close", "volume"], ["nvi"], 1, "Volume"),
    "pvi": IndicatorMeta("pvi", [], {}, ["close", "volume"], ["pvi"], 1, "Volume"),
