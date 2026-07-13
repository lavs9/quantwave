"""TA-Lib compatible interface (classic array-in / array-out API).

Provides uppercase function names and parameter aliases familiar from the classic
``talib`` Python library, backed by quantwave's pure-Rust TA-Lib implementations
exposed through the Polars ``.ta`` expression namespace. Because
those plugins are parity-tested against ``talib-rs`` in Rust, delegating to them
yields the reference values for free; this module only translates the classic
positional-array call convention into the ``.ta`` call. The Polars ``.ta``
expression namespace is the underlying implementation.

Example:
    from quantwave import talib as ta

    rsi = ta.RSI(close, timeperiod=14)                 # -> np.ndarray
    macd, signal, hist = ta.MACD(close)                # struct -> tuple of arrays
    atr = ta.ATR(high, low, close, timeperiod=14)
    doji = ta.CDLDOJI(open_, high, low, close)

Discovery:
    talib.list_functions()   # all available TA-Lib-style names (sorted)
"""

from __future__ import annotations

import inspect
from typing import Any, Callable, Dict, List, Optional

from quantwave._talib_map_generated import TALIB_SLUG_TO_NAME as _TALIB_MAP

# numpy is imported lazily (inside the wrappers) so that `import quantwave` — and the
# rest of quantwave.talib's discovery API (list_functions) — work without numpy
# installed. Calling a talib function does require numpy (classic array in/out).

# Canonical TA-Lib price-input order. Multi-input functions receive their arrays in
# this order (subset), matching classic talib (e.g. ATR(high, low, close)).
_OHLCV = ["open", "high", "low", "close", "volume"]

# Classic talib kwarg -> candidate ``.ta`` parameter names (first match on the
# target method wins). The plugins already mirror talib names for most functions,
# so this only bridges the handful that differ (e.g. macd fast/slow/signal).
_PARAM_ALIASES: Dict[str, List[str]] = {
    "timeperiod": ["timeperiod", "period", "length", "window"],
    "fastperiod": ["fastperiod", "fast", "fastlimit"],
    "slowperiod": ["slowperiod", "slow", "slowlimit"],
    "signalperiod": ["signalperiod", "signal", "signal_period"],
    "fastk_period": ["fastk_period", "fastk"],
    "slowk_period": ["slowk_period", "slowk"],
    "slowd_period": ["slowd_period", "slowd"],
    "fastd_period": ["fastd_period", "fastd"],
    "nbdevup": ["nbdevup"],
    "nbdevdn": ["nbdevdn"],
    "nbdev": ["nbdev"],
    "matype": ["matype"],
    "vfactor": ["vfactor", "vcoef"],
}

# Defaults for numeric params that a ``.ta`` method requires (no signature default)
# but classic talib defaults — so bare calls like ``talib.STDDEV(close)`` still work.
_REQUIRED_DEFAULTS: Dict[str, Any] = {
    "period": 5,
    "nbdev": 1.0,
    "minperiod": 2,
    "maxperiod": 30,
    "matype": 0,
}


def _ta_namespace_class():
    """The registered ``pl.col().ta`` namespace class (importing quantwave binds it)."""
    from quantwave._ta_namespace import TaNamespace  # noqa: WPS433

    return TaNamespace


def _norm(name: str) -> str:
    return name.replace("_", "").lower()


def _method_by_norm() -> Dict[str, str]:
    ns = _ta_namespace_class()
    out: Dict[str, str] = {}
    for name in dir(ns):
        if name.startswith("_"):
            continue
        if callable(getattr(ns, name, None)):
            out.setdefault(_norm(name), name)
    return out


class _Signature:
    """Parsed ``.ta`` method signature: input roles + numeric params."""

    __slots__ = ("method", "self_role", "extra_roles", "input_roles", "numeric", "required")

    def __init__(self, method_name: str):
        ns = _ta_namespace_class()
        sig = inspect.signature(getattr(ns, method_name))
        extra: List[str] = []
        numeric: List[str] = []
        required: List[str] = []
        for p in sig.parameters.values():
            if p.name == "self":
                continue
            if "str" in str(p.annotation):
                extra.append(p.name)
            else:
                numeric.append(p.name)
                if p.default is inspect.Parameter.empty:
                    required.append(p.name)
        self.method = method_name
        self.extra_roles = extra  # in signature order (how .ta expects the columns)
        self.numeric = numeric
        self.required = required
        if not extra:
            # single-input: the sole series is `self`
            self.self_role = "real"
            self.input_roles = ["real"]
        elif all(r in _OHLCV for r in extra):
            # OHLCV function: self is the canonical role not already an extra
            self.self_role = "close" if "close" not in extra else "open"
            self.input_roles = sorted(set(extra) | {self.self_role}, key=_OHLCV.index)
        else:
            # math / multi-real (add(in2), sub(in2), ...): self is the first real input
            self.self_role = "in1"
            self.input_roles = ["in1"] + extra


def _resolve_kwargs(numeric_params: List[str], kwargs: Dict[str, Any]) -> Dict[str, Any]:
    """Map classic talib kwargs onto the target method's numeric parameter names."""
    resolved: Dict[str, Any] = {}
    numset = set(numeric_params)
    for key, val in kwargs.items():
        if key in numset:
            resolved[key] = val
            continue
        for cand in _PARAM_ALIASES.get(key, [key]):
            if cand in numset:
                resolved[cand] = val
                break
        else:
            # Unknown/incompatible kwarg for this function — ignore rather than crash,
            # matching talib's lenient defaulting behavior.
            continue
    return resolved


def _make_wrapper(talib_name: str, method_name: str) -> Callable[..., Any]:
    import polars as pl

    spec = _Signature(method_name)

    def wrapper(*arrays, **kwargs):
        import numpy as np  # lazy: classic talib API is numpy-based

        if len(arrays) != len(spec.input_roles):
            raise TypeError(
                f"{talib_name} expects {len(spec.input_roles)} input array(s) "
                f"{tuple(spec.input_roles)}, got {len(arrays)}"
            )
        data = {
            role: np.asarray(arr, dtype=float)
            for role, arr in zip(spec.input_roles, arrays)
        }
        df = pl.DataFrame(data)
        method = getattr(pl.col(spec.self_role).ta, method_name)
        call_kwargs = _resolve_kwargs(spec.numeric, kwargs)
        # Fill any still-missing required params with classic-talib defaults.
        for req in spec.required:
            if req not in call_kwargs and req in _REQUIRED_DEFAULTS:
                call_kwargs[req] = _REQUIRED_DEFAULTS[req]
        expr = method(*spec.extra_roles, **call_kwargs)
        out = df.select(expr.alias("_out"))
        series = out.to_series()
        if series.dtype == pl.Struct:
            nested = out.unnest("_out")
            return tuple(nested[f].to_numpy() for f in series.struct.fields)
        return series.to_numpy()

    wrapper.__name__ = talib_name
    wrapper.__qualname__ = talib_name
    wrapper.__doc__ = (
        f"TA-Lib compatible wrapper for quantwave ``{method_name}`` "
        f"(delegates to ``pl.col(...).ta.{method_name}``).\n\n"
        f"Inputs (positional arrays, in order): {tuple(spec.input_roles)}.\n"
        f"Optional params: {tuple(spec.numeric)}.\n"
        f"See ``quantwave.metadata('{method_name}')`` for details."
    )
    return wrapper


def _build() -> Dict[str, Callable[..., Any]]:
    """Bind every talib name that maps to an available ``.ta`` method."""
    by_norm = _method_by_norm()
    loaded: Dict[str, Callable[..., Any]] = {}
    for slug, talib_name in _TALIB_MAP.items():
        method = by_norm.get(_norm(slug))
        if method is None:
            continue
        try:
            loaded[talib_name] = _make_wrapper(talib_name, method)
        except Exception:  # pragma: no cover - a signature we can't model; skip it
            continue
    return loaded


_loaded: Dict[str, Callable[..., Any]] = _build()

# Bind as module attributes for `talib.RSI` static access.
for _name, _fn in _loaded.items():
    globals()[_name] = _fn


def list_functions() -> List[str]:
    """Return sorted TA-Lib-style function names available in this build."""
    return sorted(_loaded.keys())


def get_functions() -> List[str]:
    """Alias for :func:`list_functions` (classic talib name)."""
    return list_functions()


def __getattr__(name: str) -> Callable[..., Any]:
    """Lazy uppercase lookup for names not pre-bound."""
    fn = _loaded.get(name) or _loaded.get(name.upper())
    if fn is not None:
        return fn
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


__all__ = list_functions()
