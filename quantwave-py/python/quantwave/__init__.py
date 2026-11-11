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

Calculation conventions:
A handful of indicator names do not determine their formula — "ATR" may mean
Wilder's RMA or a plain EMA of true range, "stddev" may be ddof=0 or ddof=1,
"roc" may be scaled x100 or not. ``conventions(name)`` returns the divergences
QuantWave actually implements (empty for most indicators); the same notes appear
on ``metadata(name).conventions`` and ``boundary_info(name).conventions``. Use
``convention_slugs()`` to list every affected indicator. **``atr`` is the one to
know about**: the ``Atr`` streaming class and the ``atr()`` batch function use an
EMA of true range, while ``ta_atr`` / ``pl.col(...).ta.atr(...)`` /
``quantwave.talib.ATR`` use Wilder's RMA. See ``conventions("atr")``.

Exception contract (``quantwave.QuantwaveError`` hierarchy):
``IndicatorNotFoundError``, ``InvalidParameterError``, ``ParityError``,
``StreamingError``, and ``InternalError`` (native FFI). See ``quantwave._errors``.
"""

from __future__ import annotations  # PEP 604 (str | None) annotations on Python 3.9

from typing import List, Dict, Any
import warnings

# Version
try:
    from importlib.metadata import version, PackageNotFoundError
    __version__ = version("quantwave")
except (PackageNotFoundError, Exception):
    __version__ = "0.6.0.dev"

# Unified compiled extension: importing it registers the
# `_quantwave` (indicators) and `_backtest` submodules into sys.modules, so the
# historical `from . import _quantwave` / `_backtest` imports keep working.
from . import _lib  # noqa: F401

# Core compiled extension
from . import _quantwave  # noqa

# Check optional dependencies before importing optional submodules
try:
    import polars as _pl_probe  # noqa: F401
    _has_polars = True
except (ImportError, ModuleNotFoundError):
    _has_polars = False

try:
    import numpy as _np_probe  # noqa: F401
    _has_numpy = True
except (ImportError, ModuleNotFoundError):
    _has_numpy = False

# Polars helpers submodule (optional; core streaming/metadata/DX work without it).
if _has_polars:
    try:
        from . import polars
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.polars submodule unavailable: {_e}"
        )
        class _DummyNS: pass
        polars = _DummyNS()
else:
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
if _backtest_available and _has_polars:
    try:
        import polars as _pl  # noqa: F401
        from . import bt_polars  # noqa: F401
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.bt_polars unavailable: {_e}"
        )

# Polars expression plugins — the pl.col().ta namespace is registered by importing
# the in-package TaNamespace module; the plugin symbols live in the unified _lib
# extension (formerly the separate quantwave_plugins package).
if _has_polars:
    try:
        from . import _ta_namespace  # noqa: F401
    except ImportError:
        pass
    except Exception as _e:  # pragma: no cover
        warnings.warn(f"quantwave .ta namespace unavailable: {_e}")

# Backtest Python API (PyO3 + pyo3-polars; requires polars extra).
if _has_polars:
    try:
        from . import backtest
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.backtest submodule unavailable: {_e}"
        )
        class _DummyNS: pass
        backtest = _DummyNS()
else:
    class _DummyNS: pass
    backtest = _DummyNS()

# Multi-timeframe helpers: resample/apply/broadcast for base-vs-higher-timeframe
# pipelines (lookahead-safe join_asof broadcast). Built on Polars, so guarded
# like the other Polars-layer submodules (core install has no polars).
if _has_polars:
    try:
        from . import mtf  # noqa: F401
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.mtf unavailable: {_e}"
        )

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

# Data layer: synthetic()/load_sample() build Polars frames, so this needs the
# Polars layer and is guarded like the other polars-dependent submodules.
if _has_polars:
    try:
        from . import datasets  # noqa: F401
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.datasets unavailable: {_e}"
        )

# Alternative bar construction (Renko, ...) — frame-in/frame-out helpers built
# on the native _bars submodule; guarded like the other polars-dependent ones.
if _has_polars:
    try:
        from . import bars  # noqa: F401
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.bars unavailable: {_e}"
        )

# Price-action pattern detection (harmonic) — frame-in/frame-out helpers built
# on the native _patterns submodule; guarded like the other polars-dependent ones.
if _has_polars:
    try:
        from . import patterns  # noqa: F401
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.patterns unavailable: {_e}"
        )

# Portfolio optimization (numpy) — guarded so `import quantwave` survives the
# core install without numpy.
if _has_numpy:
    try:
        from . import portfolio  # noqa: F401
    except Exception as _e:  # pragma: no cover
        warnings.warn(f"quantwave.portfolio unavailable (requires numpy): {_e}")

# QuantStats interop (Polars returns + lazy pandas/quantstats) — guarded like
# the other polars-dependent submodules.
if _has_polars:
    try:
        from . import quantstats_interop  # noqa: F401
        from .quantstats_interop import backtest_returns, to_quantstats  # noqa: F401
    except Exception as _e:  # pragma: no cover
        warnings.warn(
            f"quantwave.quantstats_interop unavailable: {_e}"
        )

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
    ConventionNote,
    metadata,
    list_metadata,
    warmup_bars,
    get_indicator_signature,
    boundary_info,
    conventions,
    convention_slugs,
    categories,
    indicators_by_category,
    category,
    Param,
    Series,
)

# Warmup semantics (4rsq): indicator warmup is NaN, not null — drop_nulls() is a
# no-op on it. trim_warmup() is the alignment-preserving way to remove warmup.
from .warmup import (
    WarmupWarning,
    trim_warmup,
    warmup_rows,
)

# DX helpers (defined later in this file for now to avoid circularity during cleanup)
# We will gradually move more of these out.

from .backtest_types import PerformanceMetrics, BacktestStats

# Options India helpers — namespaced under quantwave.options.
# Must NOT pollute top-level indicator discovery or qw.ta.
_OPTIONS_SYMBOLS = frozenset({
    "bs_call_price", "bs_put_price", "bs_delta", "bs_gamma", "bs_theta",
    "bs_vega", "bs_rho", "implied_vol", "max_pain", "strike_pcr",
    "chain_pcr", "oi_zones", "gex_per_strike", "gex_flip_strike", "gextotal",
    "atm_straddle", "synthetic_futures", "moneyness", "nse_lot_size",
    "nse_risk_free_rate",
})

# --- Explicit ta namespace (codegen registry) ---
from ._ta_registry_generated import SPECIAL_SYMBOLS, TA_REGISTRY, UNBOUND_SLUGS


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


class _UnboundSurface:
    """Metadata slug exposed on ``qw.ta``; implemented in Rust core without Python bindings yet."""

    __slots__ = ("slug",)

    def __init__(self, slug: str) -> None:
        self.slug = slug

    def __call__(self, *args, **kwargs):
        raise NotImplementedError(
            f"Indicator {self.slug!r} is implemented in Rust core (quantwave-core) "
            f"and does not yet have Python bindings."
        )

    def __repr__(self) -> str:
        return f"<quantwave.ta.{self.slug} core-only>"


def _bind_ta(slug: str, obj) -> None:
    setattr(ta, slug, obj)
    if not _is_internal_symbol(slug) and not isinstance(obj, (_PolarsOnlySurface, _UnboundSurface)):
        globals()[slug] = obj


def _resolve_ta_binding(slug: str, entry: dict) -> object:
    if slug in UNBOUND_SLUGS:
        return _UnboundSurface(slug)
    native_batch = entry.get("native_batch")
    if native_batch:
        if not hasattr(_quantwave, native_batch):
            raise ImportError(
                f"TA registry entry {slug!r} declares native_batch={native_batch!r}, "
                f"but the compiled module exports no such symbol. The registry is stale "
                f"— regenerate it with scripts/generate_api_stubs.py. (Falling back to "
                f"the streaming class here would silently change {slug!r}'s calling "
                f"convention from a batch function to a class.)"
            )
        return getattr(_quantwave, native_batch)
    native_streaming = entry.get("native_streaming")
    if native_streaming and hasattr(_quantwave, native_streaming):
        return getattr(_quantwave, native_streaming)
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
    """Canonical indicator slugs from Rust metadata export (225); never dir(ta) pollution."""
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

def streaming_class(name: str, track_readiness: bool = False):
    """
    Return the streaming (Next<T>) class for a given batch indicator name.

    Example:
        cls = quantwave.streaming_class("supertrend")
        st = cls(period=10, multiplier=3.0)

    Args:
        track_readiness: When ``True``, instead of the raw PyO3 class this
            returns a factory whose instances are transparently wrapped in
            :class:`StreamingWrapper`, so ``.is_ready`` / ``.bars_consumed``
            are available immediately — no separate call to
            :func:`wrap_streaming` needed::

                cls = quantwave.streaming_class("rsi", track_readiness=True)
                inst = cls(14)          # already a StreamingWrapper
                inst.next(100.0)
                inst.is_ready           # readiness tracking, out of the box

            Defaults to ``False`` so the zero-arg call keeps returning the
            exact same type it always has (no change to existing behavior or
            to any ``isinstance`` check against the raw class) — this is an
            explicit opt-in, not a change to the default return type. See
            :func:`wrap_streaming` for wrapping an instance you already
            constructed.
    """
    if not name:
        return None

    key = name.lower()
    candidate = None
    entry = TA_REGISTRY.get(key)
    if entry and entry.get("native_streaming"):
        native = _require_native(entry["native_streaming"])
        if isinstance(native, type):
            candidate = native

    if candidate is None:
        maybe = getattr(ta, key, None)
        if isinstance(maybe, type):
            candidate = maybe

    if candidate is None:
        return None

    if not track_readiness:
        return candidate

    try:
        warmup = warmup_bars(key)
    except Exception:
        warmup = None

    def _tracked_factory(*args, **kwargs):
        instance = candidate(*args, **kwargs)
        return StreamingWrapper(instance, name=key, warmup_bars_count=warmup)

    _tracked_factory.__name__ = f"Tracked{candidate.__name__}"
    _tracked_factory.__qualname__ = _tracked_factory.__name__
    _tracked_factory.__doc__ = (
        f"Factory for {candidate.__name__!r} streaming instances, "
        "pre-wrapped in StreamingWrapper for is_ready/bars_consumed tracking."
    )
    _tracked_factory.__wrapped_class__ = candidate
    return _tracked_factory


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
    "bs_vega", "bs_rho",
    "black76_call_price", "black76_put_price", "black76_delta", "black76_gamma",
    "black76_theta", "black76_vega",
    "implied_vol", "max_pain", "strike_pcr",
    "chain_pcr", "oi_zones", "gex_per_strike", "gex_flip_strike", "gextotal",
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

# TA-Lib abstract-API-style introspection (imported after metadata + talib are
# loaded). `quantwave.abstract.Function(name)` + top-level get_functions /
# get_function_groups let tools drive every indicator generically.
try:
    from . import abstract
    from .abstract import get_functions, get_function_groups
except Exception as _e:  # pragma: no cover
    warnings.warn(f"quantwave.abstract introspection unavailable: {_e}")

    class _DummyNS: pass
    abstract = _DummyNS()

# Bulk-compute feature matrix: registers the `df.ta.all()` / `lf.ta.all()`
# DataFrame/LazyFrame namespaces (built on the abstract-API registry above)
# and exposes the `qw.feature_matrix()` top-level alias.
if _has_polars:
    try:
        from .bulk import feature_matrix as feature_matrix
    except Exception as _e:  # pragma: no cover
        warnings.warn(f"quantwave.bulk (df.ta.all()) unavailable: {_e}")


_raw_atr = globals().get("atr")
_raw_ta_atr = globals().get("ta_atr")
_raw_atr_wilder = globals().get("atr_wilder")
_raw_max = globals().get("ta_max") or globals().get("max")
_raw_min = globals().get("ta_min") or globals().get("min")
_raw_sma = globals().get("sma")
_raw_ema = globals().get("ema")
_raw_rocp = globals().get("rocp")


def sma(*args, **kwargs):
    """Simple Moving Average (SMA).

Accepts arguments in either order:
  - sma(period, series, warmup="partial")
  - sma(series, period, warmup="partial")
"""
    if len(args) >= 2:
        if isinstance(args[0], (int, float)) and not hasattr(args[0], "__len__"):
            return _raw_sma(*args, **kwargs)
        if hasattr(args[0], "__len__") and isinstance(args[1], (int, float)) and not hasattr(args[1], "__len__"):
            new_args = (args[1], args[0]) + args[2:]
            return _raw_sma(*new_args, **kwargs)
    if _raw_sma is not None:
        return _raw_sma(*args, **kwargs)
    raise RuntimeError("native sma function unavailable")


def ema(*args, **kwargs):
    """Exponential Moving Average (EMA).

Accepts arguments in either order:
  - ema(period, series, warmup="partial")
  - ema(series, period, warmup="partial")
"""
    if len(args) >= 2:
        if isinstance(args[0], (int, float)) and not hasattr(args[0], "__len__"):
            return _raw_ema(*args, **kwargs)
        if hasattr(args[0], "__len__") and isinstance(args[1], (int, float)) and not hasattr(args[1], "__len__"):
            new_args = (args[1], args[0]) + args[2:]
            return _raw_ema(*new_args, **kwargs)
    if _raw_ema is not None:
        return _raw_ema(*args, **kwargs)
    raise RuntimeError("native ema function unavailable")


def atr(*args, **kwargs):
    """Average True Range (Wilder RMA smoothed by default, or EMA).

Note on smoothing and seed conventions:
  - `atr` defaults to Wilder's RMA (`smoothing="wilder"`).
  - Standalone `atr` defaults to the TA-Lib seed convention (`seed="talib"`): True Range
    is undefined on bar 0 (no prior close), accumulates `period` valid TRs from bars 1..period,
    and emits the first ATR at bar `period` (index `period`, so `period` leading NaNs).
  - To use the classic Wilder (1978) textbook formula, pass `seed="wilder1978"` (or `seed="classic"`):
    `TR[0] = High[0] - Low[0]`, seeds at bar `period - 1` (index `period - 1`) as the simple
    mean of `TR[0..period-1]`, with subsequent bars recursing as `(ATR_prev * (period - 1) + TR) / period`.
  - In contrast, `supertrend`'s internal ATR defaults to EMA smoothing (`smoothing="ema"`).
  - Use `smoothing="ema"` on `atr` or `smoothing="wilder"` on `supertrend` to align them.

Accepts arguments in either order:
  - atr(period, high, low, close, smoothing="wilder", seed="talib")
  - atr(high, low, close, period=14, smoothing="wilder", seed="talib")
  - atr(high=..., low=..., close=..., period=..., smoothing="wilder", seed="talib")
"""
    smoothing = kwargs.pop("smoothing", "wilder")
    seed = kwargs.pop("seed", None)
    warmup = kwargs.pop("warmup", None)
    pass_kwargs = {"smoothing": smoothing}
    if seed is not None:
        pass_kwargs["seed"] = seed
    if warmup is not None:
        pass_kwargs["warmup"] = warmup

    if len(args) == 5:
        if isinstance(args[0], (int, float)) and not hasattr(args[0], "__len__"):
            return _raw_atr(int(args[0]), args[1], args[2], args[3], smoothing=args[4], **pass_kwargs)
        if isinstance(args[3], (int, float)) and not hasattr(args[3], "__len__"):
            return _raw_atr(int(args[3]), args[0], args[1], args[2], smoothing=args[4], **pass_kwargs)
    elif len(args) == 4:
        if isinstance(args[0], (int, float)) and not hasattr(args[0], "__len__"):
            return _raw_atr(int(args[0]), args[1], args[2], args[3], **pass_kwargs)
        if isinstance(args[3], (int, float)) and not hasattr(args[3], "__len__"):
            return _raw_atr(int(args[3]), args[0], args[1], args[2], **pass_kwargs)
    elif len(args) == 3:
        period = kwargs.pop("period", kwargs.pop("timeperiod", 14))
        return _raw_atr(int(period), args[0], args[1], args[2], **pass_kwargs)
    elif not args and kwargs:
        period = kwargs.pop("period", kwargs.pop("timeperiod", 14))
        return _raw_atr(int(period), kwargs["high"], kwargs["low"], kwargs["close"], **pass_kwargs)
    if _raw_atr is not None:
        return _raw_atr(*args, **pass_kwargs, **kwargs)
    raise RuntimeError("native atr function unavailable")


def ta_atr(*args, **kwargs):
    """Average True Range (Wilder RMA smoothing, TA-Lib standard).

Accepts arguments in either order:
  - ta_atr(timeperiod, high, low, close, seed="talib")
  - ta_atr(high, low, close, timeperiod=14, seed="talib")
"""
    seed = kwargs.pop("seed", None)
    pass_kwargs = {}
    if seed is not None:
        pass_kwargs["seed"] = seed

    if len(args) == 4:
        if isinstance(args[0], (int, float)) and not hasattr(args[0], "__len__"):
            return _raw_ta_atr(int(args[0]), args[1], args[2], args[3], **pass_kwargs)
        if isinstance(args[3], (int, float)) and not hasattr(args[3], "__len__"):
            return _raw_ta_atr(int(args[3]), args[0], args[1], args[2], **pass_kwargs)
    elif len(args) == 3:
        period = kwargs.get("timeperiod", kwargs.get("period", 14))
        return _raw_ta_atr(int(period), args[0], args[1], args[2], **pass_kwargs)
    elif not args and kwargs:
        period = kwargs.get("timeperiod", kwargs.get("period", 14))
        return _raw_ta_atr(int(period), kwargs["high"], kwargs["low"], kwargs["close"], **pass_kwargs)
    if _raw_ta_atr is not None:
        return _raw_ta_atr(*args, **pass_kwargs, **kwargs)
    raise RuntimeError("native ta_atr function unavailable")


def atr_wilder(*args, **kwargs):
    """Average True Range (Wilder RMA smoothing).

Accepts arguments in either order:
  - atr_wilder(timeperiod, high, low, close)
  - atr_wilder(high, low, close, timeperiod=14)
"""
    return ta_atr(*args, **kwargs)


def rocp(*args, **kwargs):
    """Rate of Change Percentage (as a decimal ratio: (p - p_prev) / p_prev).

Unlike `roc` which returns percentage points (100 * (p - p_prev) / p_prev),
`rocp` returns the fractional ratio (p - p_prev) / p_prev.

Accepts arguments in either order:
  - rocp(period, series)
  - rocp(series, period=1)
  - rocp(series, period)
"""
    if len(args) == 2:
        if isinstance(args[0], (int, float)) and not hasattr(args[0], "__len__"):
            return _raw_rocp(int(args[0]), args[1])
        if hasattr(args[0], "__len__") and isinstance(args[1], (int, float)) and not hasattr(args[1], "__len__"):
            return _raw_rocp(int(args[1]), args[0])
    elif len(args) == 1:
        period = kwargs.get("period", kwargs.get("timeperiod", 1))
        return _raw_rocp(int(period), args[0])
    elif not args and kwargs:
        period = kwargs.get("period", kwargs.get("timeperiod", 1))
        series = kwargs.get("series", kwargs.get("real", kwargs.get("close")))
        if series is not None:
            return _raw_rocp(int(period), series)
    if _raw_rocp is not None:
        return _raw_rocp(*args, **kwargs)
    raise RuntimeError("native rocp function unavailable")


def apply_with_gap_reset(indicator_factory, *series_args, max_gap: int = 5):
    """Apply a streaming indicator across series, resetting internal state when a gap

    (more than `max_gap` consecutive NaN/missing bars) occurs.

    This ensures stateful indicators (EMA, SuperTrend, ATR, etc.) restart with a
    fresh warmup after extended halts or data holes (rule S14).

    Parameters:
        indicator_factory: Callable returning an indicator instance with a `.next(*inputs)` method.
        *series_args: One or more input price series (e.g. close, or high, low, close).
        max_gap: Maximum allowed consecutive missing/NaN bars before resetting the indicator (default: 5).

    Returns:
        List of computed indicator outputs.
    """
    if not series_args:
        raise ValueError("at least one series must be provided")
    n = len(series_args[0])
    for s in series_args[1:]:
        if len(s) != n:
            raise ValueError(f"input series lengths must match: {n} vs {len(s)}")

    import math
    indicator = indicator_factory()
    out = []
    consecutive_nans = 0
    needs_reset = False

    for i in range(n):
        vals = [s[i] for s in series_args]
        has_nan = any(v is None or (isinstance(v, (int, float)) and math.isnan(v)) for v in vals)

        if has_nan:
            consecutive_nans += 1
            if consecutive_nans > max_gap:
                needs_reset = True
            try:
                res = indicator.next(*(float("nan") if (v is None or (isinstance(v, (int, float)) and math.isnan(v))) else float(v) for v in vals))
            except Exception:
                res = float("nan")
            out.append(res)
        else:
            if needs_reset:
                indicator = indicator_factory()
                needs_reset = False
            consecutive_nans = 0
            res = indicator.next(*(float(v) for v in vals))
            out.append(res)

    return out


def max(*args, **kwargs):
    """Rolling Maximum (MAX) over a lookback window.

Accepts arguments in either order:
  - max(series, timeperiod=14)
  - max(timeperiod, series)
"""
    if len(args) == 2:
        if isinstance(args[0], (int, float)) and not hasattr(args[0], "__len__"):
            return _raw_max(args[1], int(args[0]))
        if isinstance(args[1], (int, float)) and not hasattr(args[1], "__len__"):
            return _raw_max(args[0], int(args[1]))
    elif len(args) == 1:
        timeperiod = kwargs.get("timeperiod", kwargs.get("period", 14))
        return _raw_max(args[0], int(timeperiod))
    if _raw_max is not None:
        return _raw_max(*args, **kwargs)
    raise RuntimeError("native max function unavailable")


def min(*args, **kwargs):
    """Rolling Minimum (MIN) over a lookback window.

Accepts arguments in either order:
  - min(series, timeperiod=14)
  - min(timeperiod, series)
"""
    if len(args) == 2:
        if isinstance(args[0], (int, float)) and not hasattr(args[0], "__len__"):
            return _raw_min(args[1], int(args[0]))
        if isinstance(args[1], (int, float)) and not hasattr(args[1], "__len__"):
            return _raw_min(args[0], int(args[1]))
    elif len(args) == 1:
        timeperiod = kwargs.get("timeperiod", kwargs.get("period", 14))
        return _raw_min(args[0], int(timeperiod))
    if _raw_min is not None:
        return _raw_min(*args, **kwargs)
    raise RuntimeError("native min function unavailable")


def cross_sectional_rank(data, descending: bool = False, percentile: bool = True):
    """Cross-sectional rank (percentile of values across a 1D sequence).

Parameters:
  data: 1D iterable of numerical scores/values.
  descending: If False (default), lowest value gets lowest rank (0.0). If True,
    highest value gets lowest rank.
  percentile: If True (default), normalizes ranks to (0.0, 1.0] as ``rank / count``.
    If False, returns raw ranks.
"""
    import numpy as np

    arr = np.asarray(data, dtype=float)
    n = len(arr)
    if n == 0:
        return np.array([], dtype=float)
    sorter = np.argsort(arr)
    ranks = np.empty(n, dtype=float)
    if descending:
        ranks[sorter] = np.arange(n, 0, -1, dtype=float)
    else:
        ranks[sorter] = np.arange(1, n + 1, dtype=float)
    if percentile:
        return ranks / float(n)
    return ranks


globals()["atr"] = atr
globals()["ta_atr"] = ta_atr
globals()["atr_wilder"] = atr_wilder
globals()["max"] = max
globals()["min"] = min
globals()["ta_max"] = max
globals()["ta_min"] = min
globals()["cross_sectional_rank"] = cross_sectional_rank
globals()["sma"] = sma
globals()["ema"] = ema


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
    "warmup_rows",
    "trim_warmup",
    "WarmupWarning",
    "get_indicator_signature",
    "build_feature_matrix",
    "feature_column_names",
    "FeatureSpec",
    "RECOMMENDED_PRESET",
    "boundary_info",
    "conventions",
    "convention_slugs",
    "ConventionNote",
    "categories",
    "indicators_by_category",
    "category",
    "get_functions",
    "get_function_groups",
    "abstract",
    "feature_matrix",
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
    "datasets",
    "options_india",  # legacy compat (will warn in future)
    "PerformanceMetrics",
    "BacktestStats",
    "cross_sectional_rank",
    "max",
    "min",
    "rocp",
    "Rocp",
    "apply_with_gap_reset",
]
