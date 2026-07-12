"""
quantwave - High-performance technical analysis library (Python bindings).

Public surface for 0.6.x Python DX.

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
Use ``boundary_info(name)`` for curated per-kind semantics. Invalid parameters
typically raise ``InvalidParameterError`` or ``ValueError``.

Exception contract (``quantwave.QuantwaveError`` hierarchy):
``IndicatorNotFoundError``, ``InvalidParameterError``, ``ParityError``,
``StreamingError``, and ``InternalError`` (native FFI). See ``quantwave._errors``.
"""

from typing import List, Dict, Any
import warnings

# Version
try:
    from importlib.metadata import version, PackageNotFoundError
    __version__ = version("quantwave")
except (PackageNotFoundError, Exception):
    __version__ = "0.6.0.dev"

# Core compiled extension
from . import _quantwave  # noqa

# Polars helpers submodule (optional; core streaming/metadata/DX work without it).
try:
    from . import polars
except Exception as _e:  # pragma: no cover
    warnings.warn(
        f"quantwave.polars submodule unavailable (polars not installed; "
        f"use `pip install \"quantwave[polars]\"` for Polars layer): {_e}"
    )
    class _DummyNS: pass
    polars = _DummyNS()

# Native backtest extension (bundled in unified PyPI wheel as quantwave._backtest).
_backtest_available = False
try:
    from . import _backtest  # noqa: F401
    _backtest_available = True
except Exception as _e:  # pragma: no cover
    warnings.warn(
        f"quantwave._backtest unavailable (reinstall quantwave wheel; unified build bundles backtest): {_e}"
    )

# LazyFrame `.bt` namespace — requires polars + _backtest.
if _backtest_available:
    try:
        import polars as _pl  # noqa: F401
        from . import bt_polars  # noqa: F401
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.bt_polars unavailable (requires polars; "
            f"use `pip install \"quantwave[polars]\"`): {_e}"
        )

# Polars expression plugins — bundled in unified wheel; registers pl.col().ta.
try:
    import quantwave_plugins  # noqa: F401
except ImportError:
    pass
except Exception as _e:  # pragma: no cover
    warnings.warn(f"quantwave_plugins unavailable: {_e}")

# Backtest Python API (PyO3 + pyo3-polars; requires polars extra).
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

# Public exception types (must survive partial native load).
from ._errors import (
    QuantwaveError,
    IndicatorNotFoundError,
    InvalidParameterError,
    ParityError,
    StreamingError,
)

_native_internal = getattr(_quantwave, "InternalError", None)
if _native_internal is None:
    class InternalError(QuantwaveError):
        """Internal uniffi/FFI error from the native binding."""
        pass
else:
    class InternalError(QuantwaveError, _native_internal):
        """Internal uniffi/FFI error from the native binding."""
        pass

globals()["InternalError"] = InternalError

# Pull in the clean metadata system (now the robust source for discovery too)
from ._metadata import (
    IndicatorMeta,
    BoundaryInfo,
    metadata,
    list_metadata,
    warmup_bars,
    get_indicator_signature,
    boundary_info,
    categories,
    indicators_by_category,
    category,
    Param,
    Series,
)

# DX helpers (defined later in this file for now to avoid circularity during cleanup)
# We will gradually move more of these out.

from .backtest_types import PerformanceMetrics, BacktestStats

# Options India helpers — namespaced under quantwave.options.
# Must NOT pollute top-level indicator discovery or qw.ta.
_OPTIONS_SYMBOLS = frozenset({
    "bs_call_price", "bs_put_price", "bs_delta", "bs_gamma", "bs_theta",
    "bs_vega", "bs_rho", "implied_vol", "max_pain", "strike_pcr",
    "chain_pcr", "oi_zones", "gex_per_strike", "gex_flip_strike",
    "atm_straddle", "synthetic_futures", "moneyness", "nse_lot_size",
    "nse_risk_free_rate",
})

# --- Explicit ta namespace (codegen registry) ---
from ._ta_registry_generated import SPECIAL_SYMBOLS, TA_REGISTRY


class ta:
    """Indicator namespace: one attribute per metadata slug (+ explicit ML/PA helpers)."""

    pass


def _is_internal_symbol(name: str) -> bool:
    return (
        name.endswith("Result")
        or name.endswith("Protocol")
        or name.endswith("Error")
        or "Protocol" in name
    )


def _require_native(symbol: str):
    if not hasattr(_quantwave, symbol):
        raise ImportError(
            f"quantwave native binding missing required symbol {symbol!r}. "
            "Rebuild/install the wheel (maturin develop --manifest-path quantwave-python/Cargo.toml)."
        )
    return getattr(_quantwave, symbol)


class _PolarsOnlySurface:
    """Metadata slug exposed on ``qw.ta``; batch/streaming via Polars ``.ta`` plugins."""

    __slots__ = ("slug", "polars_method")

    def __init__(self, slug: str, polars_method: str | None) -> None:
        self.slug = slug
        self.polars_method = polars_method or slug

    def __call__(self, *args, **kwargs):
        raise NotImplementedError(
            f"Indicator {self.slug!r} is Polars-batch only. "
            f"Use: import quantwave; pl.col('close').ta.{self.polars_method}(...)"
        )

    def __repr__(self) -> str:
        return f"<quantwave.ta.{self.slug} polars-only -> .ta.{self.polars_method}>"


def _bind_ta(slug: str, obj) -> None:
    setattr(ta, slug, obj)
    if not _is_internal_symbol(slug) and not isinstance(obj, _PolarsOnlySurface):
        globals()[slug] = obj


def _resolve_ta_binding(slug: str, entry: dict) -> object:
    for key in ("native_batch", "native_streaming"):
        native_name = entry.get(key)
        if native_name and hasattr(_quantwave, native_name):
            return getattr(_quantwave, native_name)
    polars_method = entry.get("polars_method")
    if polars_method:
        return _PolarsOnlySurface(slug, polars_method)
    raise ImportError(
        f"TA registry entry {slug!r} is missing native symbols "
        f"{entry.get('native_batch')!r}/{entry.get('native_streaming')!r} "
        "and has no polars_method fallback"
    )


for _slug, _entry in TA_REGISTRY.items():
    _bind_ta(_slug, _resolve_ta_binding(_slug, _entry))

for _pub, _native in SPECIAL_SYMBOLS.items():
    if hasattr(_quantwave, _native):
        _bind_ta(_pub, getattr(_quantwave, _native))
    else:
        warnings.warn(
            f"Optional quantwave.ta.{_pub} unavailable (native {_native!r} not in build)",
            stacklevel=1,
        )

# Backward-compat: expose native symbols at the top level by their exact native
# name — streaming classes (``qw.FracDiff``, ``qw.SuperTrend``) and batch functions
# (``qw.fracdiff``, ``qw.rsi``). The slug-based ``.ta`` bindings above take
# precedence (we never overwrite an existing name); result records / errors are
# skipped via ``_is_internal_symbol``. Iterating ``dir(_quantwave)`` keeps this in
# sync as new indicators are added, restoring the aliases the old hardcoded
# ``class ta:`` used to provide.
for _native_name in dir(_quantwave):
    if _native_name.startswith("_") or _native_name in globals():
        continue
    if _is_internal_symbol(_native_name) or _native_name in _OPTIONS_SYMBOLS:
        # Options helpers stay deprecated-only at the top level (routed through
        # __getattr__ so ``qw.bs_call_price`` still warns); use ``qw.options.*``.
        continue
    _native_obj = getattr(_quantwave, _native_name)
    if callable(_native_obj):
        globals()[_native_name] = _native_obj

# --- Basic Discovery API ---

def _build_indicator_names() -> set[str]:
    """Canonical indicator slugs from Rust metadata export (221); never dir(ta) pollution."""
    try:
        from quantwave._metadata_generated import GENERATED_ENTRIES

        return set(GENERATED_ENTRIES.keys())
    except ImportError:
        return {m.name for m in list_metadata() if not _is_internal_symbol(m.name)}

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
# Streaming Class Lookup
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
    entry = TA_REGISTRY.get(key)
    if entry and entry.get("native_streaming"):
        candidate = _require_native(entry["native_streaming"])
        if isinstance(candidate, type):
            return candidate

    candidate = getattr(ta, key, None)
    if isinstance(candidate, type):
        return candidate

    return None


# =============================================================================
# Parity Testing Helper
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
        raise IndicatorNotFoundError(f"Unknown indicator: {indicator_name}")

    streaming_cls = streaming_class(indicator_name)
    if streaming_cls is None:
        raise IndicatorNotFoundError(f"No streaming class found for: {indicator_name}")

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
            raise InvalidParameterError(
                f"Batch call failed for {indicator_name} with params={call_params}. "
                "Check native signature and required parameters."
            ) from None

    # Run streaming
    try:
        streamer = streaming_cls(**params, **kwargs)
        stream_result = [streamer.next(v) for v in data]
    except (TypeError, ValueError) as e:
        raise InvalidParameterError(f"Invalid streaming params for {indicator_name}: {e}") from e
    except Exception as e:
        raise StreamingError(f"Streaming mode failed for {indicator_name}: {e}") from e

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
        raise ParityError(f"Length mismatch: batch={len(b)}, stream={len(s)}")

    for i, (bv, sv) in enumerate(zip(b, s)):
        if i < warmup:
            # During warmup, both paths should agree on NaN / sentinel values.
            if not _close_enough(bv, sv):
                raise ParityError(
                    f"Warmup mismatch at index {i} (warmup={warmup}): batch={bv}, stream={sv}"
                )
            continue
        if not _close_enough(bv, sv):
            raise ParityError(
                f"Mismatch at index {i} (post-warmup, warmup={warmup}): batch={bv}, stream={sv}"
            )

    return True


# =============================================================================
# Streaming Readiness Helpers
# =============================================================================

class StreamingWrapper:
    """
    Lightweight wrapper that adds is_ready and bars_consumed tracking
    around any streaming (Next<T>) indicator instance.

    Mirrors Rust ``quantwave_core::TrackedNext`` / ``StreamingReadiness`` (h6xe).
    Uses metadata.warmup_bars for accurate readiness when ``name`` is provided.

    Usage:
        st = quantwave.streaming_class("supertrend")(period=10, multiplier=3)
        wrapped = quantwave.wrap_streaming(st, name="supertrend")
        for price in data:
            val = wrapped.next(price)
            if wrapped.is_ready:
                ...
    """
    def __init__(self, streaming_instance, name: str = None, warmup_bars_count: int = None):
        self._inner = streaming_instance
        self._bars_consumed = 0
        self._is_ready = False
        self._name = name
        self._warmup = 0
        if warmup_bars_count is not None:
            self._warmup = int(warmup_bars_count)
        elif name:
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


def wrap_streaming(streaming_instance, name: str = None, warmup_bars_count: int = None):
    """Wrap a streaming indicator instance to get is_ready / bars_consumed tracking.

    Pass ``name=`` for warmup-based readiness from metadata, or ``warmup_bars_count=``
    for an explicit bar count (Rust ``TrackedNext`` parity).
    """
    return StreamingWrapper(streaming_instance, name=name, warmup_bars_count=warmup_bars_count)


def track_streaming(streaming_instance, warmup_bars_count: int = None, name: str = None):
    """Alias for ``wrap_streaming`` — explicit readiness tracking API (h6xe)."""
    return wrap_streaming(streaming_instance, name=name, warmup_bars_count=warmup_bars_count)

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
# Final clean public surface
# =============================================================================

# ML feature matrix (rdpk) — imported after ``ta`` is populated to avoid circular import.
from .features import (
    FeatureSpec,
    RECOMMENDED_PRESET,
    build_feature_matrix,
    feature_column_names,
)


def __getattr__(name: str):
    """Deprecated top-level access for options helpers."""
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
    "build_feature_matrix",
    "feature_column_names",
    "FeatureSpec",
    "RECOMMENDED_PRESET",
    "boundary_info",
    "categories",
    "indicators_by_category",
    "category",
    "IndicatorMeta",
    "BoundaryInfo",
    "Param",
    "Series",
    "assert_parity",
    "streaming_class",
    "wrap_streaming",
    "track_streaming",
    "StreamingWrapper",
    "QuantwaveError",
    "IndicatorNotFoundError",
    "InvalidParameterError",
    "ParityError",
    "StreamingError",
    "InternalError",
    "ta",
    "polars",
    "backtest",
    "results",
    "options",
    "talib",
    "options_india",  # legacy compat (will warn in future)
    "PerformanceMetrics",
    "BacktestStats",
]
