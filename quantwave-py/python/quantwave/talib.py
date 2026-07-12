"""
TA-Lib compatible interface.

Provides uppercase function names and parameter aliases familiar from the
classic ``talib`` library. Functions are thin wrappers around the native
``quantwave._quantwave`` implementations.

Example:
    from quantwave import talib as ta

    rsi = ta.RSI(close, timeperiod=14)
    upper, middle, lower = ta.BBANDS(close, timeperiod=20)

Discovery:
    talib.list_functions()  # all available TA-Lib-style names
"""

from __future__ import annotations

from typing import Any, Callable, Dict, List, Optional, Sequence

from quantwave._talib_map_generated import TALIB_SLUG_TO_NAME as _TALIB_MAP

# TA-Lib param name -> quantwave param name
_PARAM_ALIASES = {
    "timeperiod": "period",
    "fastperiod": "fast",
    "slowperiod": "slow",
    "signalperiod": "signal",
    "fastk_period": "fastk",
    "slowk_period": "slowk",
    "slowd_period": "slowd",
    "nbdevup": "std_dev",
    "nbdevdn": "std_dev",
}


def _resolve_native(slug: str) -> Optional[Callable[..., Any]]:
    try:
        from . import _quantwave
    except ImportError:
        return None
    fn = getattr(_quantwave, slug, None)
    return fn if callable(fn) else None


def _make_talib_wrapper(slug: str, talib_name: str) -> Optional[Callable[..., Any]]:
    native = _resolve_native(slug)
    if native is None:
        return None

    def wrapper(*args, **kwargs):
        kw = dict(kwargs)
        for talib_key, qw_key in _PARAM_ALIASES.items():
            if talib_key in kw and qw_key not in kw:
                kw[qw_key] = kw.pop(talib_key)
        # Positional: first arg is typically the price series
        if args:
            if len(args) == 1:
                return native(series=args[0], **kw)
            return native(*args, **kw)
        return native(**kw)

    wrapper.__name__ = talib_name
    wrapper.__qualname__ = talib_name
    wrapper.__doc__ = (
        f"TA-Lib compatible wrapper for quantwave ``{slug}``.\n\n"
        f"See ``quantwave.metadata('{slug}')`` and ``quantwave.boundary_info('{slug}')``."
    )
    return wrapper


_loaded: Dict[str, Callable[..., Any]] = {}

for _slug, _alias in _TALIB_MAP.items():
    _fn = _make_talib_wrapper(_slug, _alias)
    if _fn is not None:
        _loaded[_alias] = _fn
        globals()[_alias] = _fn


def list_functions() -> List[str]:
    """Return sorted TA-Lib-style function names available in this build."""
    return sorted(_loaded.keys())


def __getattr__(name: str) -> Callable[..., Any]:
    """Lazy-resolve uppercase names not pre-bound at import (partial builds)."""
    key = name.upper()
    if key in _loaded:
        return _loaded[key]
    slug = name.lower()
    if slug in _TALIB_MAP:
        fn = _make_talib_wrapper(slug, _TALIB_MAP[slug])
        if fn is not None:
            _loaded[_TALIB_MAP[slug]] = fn
            globals()[_TALIB_MAP[slug]] = fn
            return fn
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


__all__ = list_functions()