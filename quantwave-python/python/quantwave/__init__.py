"""
quantwave - High-performance technical analysis library (Python bindings).

Public surface for 0.5.2 (Python DX improvements from quantwave-p3z9).

Recommended modern usage:
    import quantwave as qw

    # Discovery & introspection
    qw.indicators()
    meta = qw.metadata("supertrend")

    # Parity verification
    qw.assert_parity("rsi", {"period": 14}, closes)

    # Streaming with readiness tracking
    cls = qw.streaming_class("rsi")
    inst = qw.wrap_streaming(cls(14))

    # New recommended namespaces (reduces top-level pollution)
    from quantwave import results, options, talib

Legacy top-level access to results (MacdResult, etc.) and options helpers
still works but is deprecated and will be removed in a future major version.

Boundary conditions & error behavior:
Most indicators return NaN or truncated output during their warmup period.
Passing invalid parameters (negative period, etc.) will typically raise ValueError.
See individual docstrings and `metadata(name)` for details.
"""

from typing import List, Dict, Any
import warnings

# Version
try:
    from importlib.metadata import version, PackageNotFoundError
    __version__ = version("quantwave")
except (PackageNotFoundError, Exception):
    __version__ = "0.5.2.dev"

# Core compiled extension
from . import _quantwave  # noqa

# polars layer is optional (core streaming/metadata/DX work without it; the
# quantwave-plugins package is the Polars Expressions path and declares the dep).
try:
    from . import polars
    from . import bt_polars  # noqa: F401 — registers LazyFrame.bt namespace
except Exception as _e:  # pragma: no cover
    warnings.warn(
        f"quantwave.polars submodule unavailable (polars not installed; "
        f"use `pip install \"quantwave[polars]\"` for Polars layer): {_e}"
    )
    class _DummyNS: pass
    polars = _DummyNS()

# Backtest engine (PyO3 + pyo3-polars; requires polars extra)
try:
    from . import backtest
except Exception as _e:  # pragma: no cover
    warnings.warn(
        f"quantwave.backtest submodule unavailable (requires polars; "
        f"use `pip install \"quantwave[polars]\"`): {_e}"
    )
    class _DummyNS: pass
    backtest = _DummyNS()

# Popular namespaces (guarded: some may require full maturin build or follow-up on gqem/05q7)
try:
    from . import results
    from . import options
    from . import talib
except Exception as _e:  # pragma: no cover
    warnings.warn(f"quantwave namespace submodules (results/options/talib) partial load: {_e}")
    # Provide minimal stand-ins so core DX (metadata, discovery, parity) still works
    class _DummyNS: pass
    results = _DummyNS()
    options = _DummyNS()
    talib = _DummyNS()

# Public exception types (must be available even if native load is partial).
# These are listed in __all__ and documented as part of the public surface.
class QuantwaveError(Exception):
    """Base exception for quantwave errors."""
    pass

InternalError = getattr(_quantwave, "InternalError", None)
if InternalError is None:
    class InternalError(QuantwaveError):
        """Internal uniffi/FFI error from the native binding."""
        pass
else:
    # Bind under our name so `from quantwave import InternalError` and
    # `except quantwave.InternalError` work as advertised.
    globals()["InternalError"] = InternalError

globals()["QuantwaveError"] = QuantwaveError
globals()["InternalError"] = InternalError

# Pull in the clean metadata system (now the robust source for discovery too)
from ._metadata import (
    IndicatorMeta,
    metadata,
    list_metadata,
    warmup_bars,
    get_indicator_signature,
    Param,
    Series,
)

# DX helpers (defined later in this file for now to avoid circularity during cleanup)
# We will gradually move more of these out.

# Options India helpers — namespaced under quantwave.options (quantwave-05q7).
# Must NOT pollute top-level indicator discovery or qw.ta.
_OPTIONS_SYMBOLS = frozenset({
    "bs_call_price", "bs_put_price", "bs_delta", "bs_gamma", "bs_theta",
    "bs_vega", "bs_rho", "implied_vol", "max_pain", "strike_pcr",
    "chain_pcr", "oi_zones", "gex_per_strike", "gex_flip_strike",
    "atm_straddle", "synthetic_futures", "moneyness", "nse_lot_size",
    "nse_risk_free_rate",
})

# --- Dynamic ta namespace population (replaces fragile manual class + "from . import ta") ---
# This makes indicators(), is_indicator(), streaming_class, and qw.ta.* robust
# even when some advanced ML/PA feature objects are added in later tasks (tha, gw7s, ej8b).
# Pulls from the native _quantwave (source of truth for compiled indicators + new uniffi/PyO3 objects)
# + falls back to metadata keys. Also exposes at top-level for backward compat.

class ta:
    """Dynamic namespace aggregating available indicators (batch + streaming classes)
    and feature/PA helpers. Populated at import time from the compiled extension.
    """
    pass

# Populate from native extension.
# IMPORTANT for gqem (namespace cleanup): Do NOT dump *Result / *Protocol / internal
# types to top-level globals (that was the ~150 item pollution). Only clean indicator
# names go to top-level + ta. Result types live in quantwave.results (and ta if useful).
# This directly advances the "move MacdResult, *Protocol etc out of top-level" goal.
try:
    if hasattr(_quantwave, "__all__"):
        _native_syms = getattr(_quantwave, "__all__")
    else:
        _native_syms = [x for x in dir(_quantwave) if not x.startswith("_") and not x.startswith("Py")]
    for _sym in _native_syms:
        if _sym in _OPTIONS_SYMBOLS:
            continue  # options live in quantwave.options only (05q7)
        try:
            _val = getattr(_quantwave, _sym)
            setattr(ta, _sym, _val)  # always available under qw.ta for discovery / advanced use
            is_internal = _sym.endswith("Result") or _sym.endswith("Protocol") or _sym.endswith("Error") or "Protocol" in _sym
            if not is_internal:
                globals()[_sym] = _val  # clean top-level exposure only for indicators / public fns
        except Exception:
            pass
except Exception as _e:
    warnings.warn(f"Partial native symbol import into quantwave.ta / top-level: {_e}")

# Also ensure any metadata-registered indicators are attached (for pure-Py or future)
for _meta in list_metadata():
    _n = _meta.name
    if not hasattr(ta, _n):
        for _cand in (_n, _n.capitalize(), _n.replace("_", "")):
            if hasattr(_quantwave, _cand):
                _v = getattr(_quantwave, _cand)
                setattr(ta, _n, _v)
                globals()[_n] = _v
                break

# Special recently-wired objects (ML features, PA structs, regime) that may have specific casing
for _special in [
    "CyberCycleFeatureExtractor", "HurstFeatureExtractor",
    "InstantaneousTrendlineFeatureExtractor", "TrendflexFeatureExtractor",
    "GriffithsDominantCycleFeatureExtractor", "BullBearHMM",
    "regime_to_features", "griffiths_dominant_cycle_features",
    "MarketStructure", "GeometricPatternScanner", "market_structure_batch",
]:
    if not hasattr(ta, _special):
        for _cand in (_special, _special.lower().replace("featureextractor", "feature_extractor")):
            if hasattr(_quantwave, _cand):
                _v = getattr(_quantwave, _cand)
                setattr(ta, _special, _v)
                globals()[_special] = _v
                break

# --- Basic Discovery API (quantwave-p3z9 / p0s) ---
# Now backed primarily by metadata (reliable) + dir(ta) for anything extra the native exposes.
# This closes the "first-pass" + "will be refined as metadata built" gap.

def _build_indicator_names() -> set[str]:
    """Build the set of indicator names from metadata (primary) + ta namespace."""
    names = set()
    # Primary: our curated metadata (includes Ehlers, PA, ML core, classics)
    for m in list_metadata():
        names.add(m.name)
        if m.name != m.name.lower():
            names.add(m.name.lower())
    # Secondary: anything callable exposed via the native/ ta namespace
    for name in dir(ta):
        if name.startswith("_"):
            continue
        if name in _OPTIONS_SYMBOLS:
            continue
        if name.endswith("Protocol") or "Protocol" in name:
            continue  # uniffi internal, not user indicators
        obj = getattr(ta, name, None)
        if callable(obj) or isinstance(obj, type):
            names.add(name)
            names.add(name.lower())
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

    # Run batch - native fns usually take explicit params then series= (or data as last positional).
    # Use flexible try to support the generated signatures without hardcoding per-indicator.
    meta = metadata(indicator_name)
    call_params = {k: v for k, v in (params or {}).items()}
    try:
        batch_result = batch_fn(series=data, **call_params, **(kwargs or {}))
    except TypeError:
        try:
            batch_result = batch_fn(data, **call_params, **(kwargs or {}))
        except TypeError:
            # Give up with clear error
            raise RuntimeError(f"Batch call failed for {indicator_name} with params={call_params}. Check native signature.") from None

    # Run streaming
    try:
        streamer = streaming_cls(**params, **kwargs)
        stream_result = [streamer.next(v) for v in data]
    except Exception as e:
        raise RuntimeError(f"Streaming mode failed: {e}") from e

    # Use metadata for warmup awareness (976r) - skip or note initial bars in comparison
    meta = metadata(indicator_name)
    warmup = warmup_bars(indicator_name, params) if meta else 0

    # Normalize for comparison (handle single vs multi output + result dataclasses)
    def _normalize(x):
        if isinstance(x, (list, tuple)):
            return x
        return [x]

    def _close_enough(bv, sv):
        import math
        if isinstance(bv, (int, float)) and isinstance(sv, (int, float)):
            if math.isnan(bv) and math.isnan(sv):
                return True
            if math.isinf(bv) and math.isinf(sv) and (bv > 0) == (sv > 0):
                return True
            return abs(bv - sv) <= tolerance
        if repr(bv) == repr(sv):
            return True
        # Dataclass / result object: compare fields with tolerance on floats
        try:
            bdict = vars(bv) if hasattr(bv, "__dict__") else bv._asdict() if hasattr(bv, "_asdict") else None
            sdict = vars(sv) if hasattr(sv, "__dict__") else sv._asdict() if hasattr(sv, "_asdict") else None
            if bdict and sdict and set(bdict.keys()) == set(sdict.keys()):
                for k in bdict:
                    if not _close_enough(bdict[k], sdict[k]):
                        return False
                return True
        except Exception:
            pass
        return False

    b = _normalize(batch_result)
    s = _normalize(stream_result)

    if len(b) != len(s):
        raise AssertionError(f"Length mismatch: batch={len(b)}, stream={len(s)}")

    for i, (bv, sv) in enumerate(zip(b, s)):
        if i < warmup:
            # During warmup, both paths should agree on NaN / sentinel values.
            if not _close_enough(bv, sv):
                raise AssertionError(
                    f"Warmup mismatch at index {i} (warmup={warmup}): batch={bv}, stream={sv}"
                )
            continue
        if not _close_enough(bv, sv):
            raise AssertionError(
                f"Mismatch at index {i} (post-warmup, warmup={warmup}): batch={bv}, stream={sv}"
            )

    return True


# =============================================================================
# Streaming Readiness Helpers (quantwave-p3z9 / 1l79)
# =============================================================================

class StreamingWrapper:
    """
    Lightweight wrapper that adds is_ready and bars_consumed tracking
    around any streaming (Next<T>) indicator instance.

    Uses metadata.warmup_bars for accurate readiness (instead of conservative heuristic).

    Usage:
        st = quantwave.streaming_class("supertrend")(period=10, multiplier=3)
        wrapped = quantwave.wrap_streaming(st, name="supertrend")
        for price in data:
            val = wrapped.next(price)
            if wrapped.is_ready:
                ...
    """
    def __init__(self, streaming_instance, name: str = None):
        self._inner = streaming_instance
        self._bars_consumed = 0
        self._is_ready = False
        self._name = name
        self._warmup = 0
        if name:
            try:
                self._warmup = warmup_bars(name)
            except Exception:
                self._warmup = 0

    def next(self, value):
        result = self._inner.next(value)
        self._bars_consumed += 1
        if self._warmup > 0:
            self._is_ready = self._bars_consumed >= self._warmup
        else:
            # Fallback heuristic for unknown
            self._is_ready = self._bars_consumed > 0
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


def wrap_streaming(streaming_instance, name: str = None):
    """Wrap a streaming indicator instance to get is_ready / bars_consumed tracking.
    Pass name= for accurate warmup-based readiness from metadata.
    """
    return StreamingWrapper(streaming_instance, name=name)

# (The old manual "class ta:" with 100+ hardcoded aliases has been replaced by the
# dynamic population block earlier in this file. It pulls from _quantwave + _metadata
# so it stays in sync automatically as new indicators / ML / PA objects are added in
# tha, gw7s, cu03 children, etc. The options_india legacy compat class remains below.
# It is built safely via getattr to avoid NameError on import if the preceding
# population was partial for any reason.)

class options_india:
    """Legacy top-level access for Options India helpers.

    Prefer: from quantwave import options  (or from quantwave.options import ...)
    This will be deprecated in a future release.
    """
    pass

# Build safely (no bare-name RHS that can NameError if a symbol wasn't populated).
_option_legacy_names = [
    "bs_call_price", "bs_put_price", "bs_delta", "bs_gamma", "bs_theta",
    "bs_vega", "bs_rho", "implied_vol", "max_pain", "strike_pcr",
    "chain_pcr", "oi_zones", "gex_per_strike", "gex_flip_strike",
    "atm_straddle", "synthetic_futures", "moneyness", "nse_lot_size",
    "nse_risk_free_rate",
]
for _name in _option_legacy_names:
    _val = getattr(options, _name, None)
    if _val is not None:
        setattr(options_india, _name, _val)

# (Removed the giant legacy __all__ that referenced many PascalCase / old names no longer
# explicitly assigned here; the dynamic population + final clean __all__ below provide the
# supported public surface. Top-level indicator names like sma/rsi/SuperTrend are still
# available via the globals() population for backward compat.)

# =============================================================================
# Final clean public surface for 0.5.2 / P0 DX (quantwave-p3z9 children)
# =============================================================================

def __getattr__(name: str):
    """Deprecated top-level access for options helpers (quantwave-05q7)."""
    if name in _OPTIONS_SYMBOLS:
        warnings.warn(
            f"quantwave.{name} is deprecated; use quantwave.options.{name} instead",
            DeprecationWarning,
            stacklevel=2,
        )
        return getattr(options, name)
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


__all__ = [
    "__version__",
    "indicators",
    "is_indicator",
    "metadata",
    "list_metadata",
    "warmup_bars",
    "get_indicator_signature",
    "IndicatorMeta",
    "Param",
    "Series",
    "assert_parity",
    "streaming_class",
    "wrap_streaming",
    "StreamingWrapper",
    "QuantwaveError",
    "InternalError",
    "ta",
    "polars",
    "backtest",
    "results",
    "options",
    "talib",
    "options_india",  # legacy compat (will warn in future)
]
