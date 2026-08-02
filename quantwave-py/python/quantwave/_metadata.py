"""
Internal metadata registry for indicators (Python side).

Primary source of truth: Rust `*_METADATA` constants in quantwave-core, exported via
`scripts/generate_indicator_metadata.py` into `_metadata_generated.py`.

Hand-curated entries in `_HAND_METADATA` override generated slugs for Python DX
(TA-Lib names, data_inputs, warmup).

Warmup / NaN semantics
----------------------
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
from typing import Optional, List, Dict, Any, Mapping


@dataclass(frozen=True)
class BoundaryInfo:
    """Boundary conditions and error behavior for an indicator."""

    warmup_behavior: str
    period_gt_len: str
    nan_inputs: str
    invalid_params: str
    empty_data: str


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


# Hand-curated overrides for high-traffic Python/Ta-Lib indicators (win on slug collision).
_HAND_METADATA: Dict[str, IndicatorMeta] = {
    # Momentum / Oscillators
    "rsi": IndicatorMeta("rsi", ["period"], {}, ["close"], ["rsi"], 14, "Momentum", description="Relative Strength Index"),
    "macd": IndicatorMeta("macd", [], {"fast": 12, "slow": 26, "signal": 9}, ["close"], ["macd", "signal", "histogram"], 26, "Momentum"),
    "stoch": IndicatorMeta("stoch", ["fastk", "slowk", "slowd"], {}, ["high", "low", "close"], ["slowk", "slowd"], 14, "Momentum"),
    "adx": IndicatorMeta("adx", ["period"], {}, ["high", "low", "close"], ["adx"], 14, "Momentum"),
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
    "geometric_patterns": IndicatorMeta(
        "geometric_patterns", ["swing_strength"], {"min_pole_atr": 1.0},
        ["high", "low"], ["flag", "hs"], 0, "Price Action",
        description="Flag + Head & Shoulders scanner (event output; warmup_bars=0)",
    ),
    "sr_monitor": IndicatorMeta(
        "sr_monitor",
        ["swing_strength", "touch_tolerance", "approach_zone"],
        {"touch_tol_atr_mult": 0.5, "approach_zone_atr_mult": 2.0},
        ["high", "low", "close"],
        ["bias", "active_levels", "interaction_count", "has_interaction", "interaction_type", "level_price", "strength", "distance", "atr"],
        0,
        "Price Action",
        description="S/R interaction monitor (Approach/Touch/Breakout/Reversal/Retest); event struct output",
    ),
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

}


def _build_metadata_registry() -> Dict[str, IndicatorMeta]:
    """Merge Rust-generated entries with hand-curated overrides."""
    merged: Dict[str, IndicatorMeta] = {}
    try:
        from quantwave._metadata_generated import GENERATED_ENTRIES

        for slug, fields in GENERATED_ENTRIES.items():
            merged[slug] = IndicatorMeta(
                slug,
                fields.get("required_params", []),
                fields.get("optional_params", {}),
                fields.get("data_inputs", ["close"]),
                fields.get("outputs", [slug]),
                fields.get("warmup_bars"),
                fields.get("category"),
                description=fields.get("description"),
            )
    except ImportError:
        pass
    merged.update(_HAND_METADATA)
    return merged


_METADATA: Dict[str, IndicatorMeta] = _build_metadata_registry()


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

    Warmup is emitted as **NaN, not null**, so ``drop_nulls()`` will not remove it.
    Use :func:`quantwave.trim_warmup` to slice a frame by the max warmup across
    several indicators while keeping the columns row-aligned.

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


# --- Boundary conditions ---

_BOUNDARY_BY_KIND: Dict[str, BoundaryInfo] = {
    "scalar": BoundaryInfo(
        warmup_behavior="Leading bars return NaN until warmup_bars is satisfied.",
        period_gt_len="When period exceeds series length, output is all NaN.",
        nan_inputs="NaN in input propagates to output (NaN out).",
        invalid_params="Non-positive period or missing required params raise ValueError.",
        empty_data="Empty input returns an empty result series.",
    ),
    "cumulative": BoundaryInfo(
        warmup_behavior="Output starts from bar 1; warmup_bars marks period-stability, not NaN.",
        period_gt_len="Cumulative sum continues; period only affects smoothed variants.",
        nan_inputs="NaN inputs may produce NaN or skip depending on indicator.",
        invalid_params="Invalid params raise ValueError.",
        empty_data="Empty input returns an empty result series.",
    ),
    "event": BoundaryInfo(
        warmup_behavior="Early bars return empty event lists or default structs (no scalar NaN).",
        period_gt_len="Insufficient history yields no events rather than NaN scalars.",
        nan_inputs="NaN OHLC typically suppresses event detection for that bar.",
        invalid_params="Invalid swing_strength or tolerance raises ValueError.",
        empty_data="Empty input returns empty event collections.",
    ),
    "pattern": BoundaryInfo(
        warmup_behavior="Pattern functions emit 0 (no pattern) until enough bars exist.",
        period_gt_len="Short series returns all zeros (no pattern detected).",
        nan_inputs="Bars with NaN OHLC are treated as no pattern (0).",
        invalid_params="N/A for most candlestick patterns.",
        empty_data="Empty input returns an empty integer series.",
    ),
}


def _boundary_kind(meta: IndicatorMeta) -> str:
    cat = (meta.category or "").lower()
    if "price action" in cat or meta.name in ("sr_monitor", "geometric_patterns", "market_structure"):
        return "event"
    if "pattern" in cat or meta.name.startswith("cdl"):
        return "pattern"
    if meta.name in ("obv", "ad", "nvi", "pvi", "vwap", "anchored_vwap"):
        return "cumulative"
    return "scalar"


def boundary_info(name: str) -> Optional[BoundaryInfo]:
    """Return boundary conditions and error behavior for an indicator.

    Use alongside ``metadata(name)`` for UIs, doc generation, and defensive
    strategy code. Values are curated defaults by indicator kind; see the
    per-indicator guides for formula-specific edge cases.

    Example:
        >>> info = boundary_info("rsi")
        >>> "NaN" in info.warmup_behavior
        True
    """
    meta = metadata(name)
    if not meta:
        return None
    return _BOUNDARY_BY_KIND[_boundary_kind(meta)]


# --- Categories API ---

_UNCATEGORIZED = "Uncategorized"


def categories() -> List[str]:
    """Return sorted unique indicator categories from the metadata registry."""
    cats = {_UNCATEGORIZED}
    for m in _METADATA.values():
        cats.add(m.category or _UNCATEGORIZED)
    return sorted(cats)


def indicators_by_category() -> Dict[str, List[str]]:
    """Map each category to a sorted list of indicator slugs."""
    by_cat: Dict[str, List[str]] = {}
    for slug, m in sorted(_METADATA.items()):
        cat = m.category or _UNCATEGORIZED
        by_cat.setdefault(cat, []).append(slug)
    return by_cat


def category(name: str) -> List[str]:
    """Return indicator slugs belonging to a category (case-insensitive)."""
    if not name:
        return []
    key = name.strip()
    by_cat = indicators_by_category()
    # Exact match first
    for cat, slugs in by_cat.items():
        if cat.lower() == key.lower():
            return list(slugs)
    return []
