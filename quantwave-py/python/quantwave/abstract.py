"""TA-Lib ``abstract``-style introspection façade over quantwave's indicators.

Mirrors ``talib.abstract.Function`` so screeners, no-code UIs, optimizers, and
migration tools can drive every indicator generically — discover inputs, parameters
(with defaults), and outputs, then call it — without hardcoding per-indicator
signatures.

    from quantwave.abstract import Function
    f = Function("RSI")
    f.input_names       # ['close']
    f.parameters        # {'timeperiod': 14}
    f.output_names      # ['rsi']
    f(close, timeperiod=14)                      # -> np.ndarray
    Function("MACD")(close)                       # multi-output -> tuple of arrays
    Function("ATR")({"high": h, "low": l, "close": c})

Discovery:
    import quantwave as qw
    qw.get_functions()          # list[str] of all indicator names
    qw.get_function_groups()    # {group: [names]}
"""

from __future__ import annotations

import inspect
from typing import Any, Dict, List, Optional, Sequence

from ._metadata import metadata as _metadata
from .talib import _REQUIRED_DEFAULTS, _Signature, _method_by_norm, _norm, _resolve_kwargs


def _resolve_method(name: str) -> Optional[str]:
    """Map an indicator name / TA-Lib name to its Polars ``.ta`` method."""
    return _method_by_norm().get(_norm(name))


class Function:
    """A single indicator, introspectable and callable (talib.abstract.Function)."""

    def __init__(self, name: str):
        self._name = name
        self._method = _resolve_method(name)
        self._meta = _metadata(name) or (
            _metadata(self._method) if self._method else None
        )
        self._spec = _Signature(self._method) if self._method else None
        self._defaults = self._param_defaults()

    def _param_defaults(self) -> Dict[str, Any]:
        if self._method is None:
            # No batch (.ta) implementation — fall back to metadata param names.
            if self._meta is None:
                return {}
            names = list(getattr(self._meta, "required_params", []) or [])
            opt = getattr(self._meta, "optional_params", {}) or {}
            return {**{n: None for n in names}, **dict(opt)}
        sig = inspect.signature(getattr(_ta_ns(), self._method))
        out: Dict[str, Any] = {}
        for p in sig.parameters.values():
            if p.name == "self" or "str" in str(p.annotation):
                continue
            if p.default is not inspect.Parameter.empty:
                out[p.name] = p.default
            else:
                # Required by the .ta signature — source the authoritative default
                # from the curated (Rust-exported) metadata before the generic
                # classic-talib guess table. Only stays None if neither knows it.
                out[p.name] = self._meta_default(p.name)
        return out

    def _meta_default(self, param: str) -> Any:
        """Best known default for a param the ``.ta`` signature leaves required.

        Prefers the Rust-exported ``optional_params`` (keyed by the raw ``.ta``
        param name) over the 5-key classic-talib guess table, so a generic caller
        (``df.ta.all()``, screeners) gets a real value instead of ``None``. Reads
        the generated entry directly rather than the merged metadata, because a
        hand overlay may blank ``optional_params`` on collision."""
        for src in (self._generated_params(), self._merged_optional()):
            val = src.get(param)
            if val is not None:
                return val
        return _REQUIRED_DEFAULTS.get(param)

    def _generated_params(self) -> Dict[str, Any]:
        # Faithful raw-name param defaults from the Rust export (not the
        # required-filtered / alias-renamed optional_params).
        try:
            from ._metadata_generated import PARAM_DEFAULTS
        except ImportError:
            return {}
        for key in (self._name, self._name.lower(), self._method):
            if key and key in PARAM_DEFAULTS:
                return PARAM_DEFAULTS[key]
        return {}

    def _merged_optional(self) -> Dict[str, Any]:
        return (getattr(self._meta, "optional_params", None) or {}) if self._meta else {}

    # --- talib.abstract-style introspection surface ---

    @property
    def name(self) -> str:
        return self._name

    @property
    def input_names(self) -> List[str]:
        meta_inputs = (
            list(self._meta.data_inputs)
            if self._meta is not None and getattr(self._meta, "data_inputs", None)
            else []
        )
        spec_inputs = list(self._spec.input_roles) if self._spec is not None else []
        # Curated data_inputs can undercount multi-series functions (beta/correl
        # take a second `other` series); trust the actual .ta signature when it
        # declares more inputs, so a generic caller feeds the right count.
        if len(spec_inputs) > len(meta_inputs):
            return spec_inputs
        return meta_inputs or spec_inputs

    @property
    def output_names(self) -> List[str]:
        if self._meta is not None and getattr(self._meta, "outputs", None):
            return list(self._meta.outputs)
        return [self._name]

    @property
    def parameters(self) -> Dict[str, Any]:
        return dict(self._defaults)

    @property
    def group(self) -> Optional[str]:
        return getattr(self._meta, "category", None) if self._meta else None

    @property
    def info(self) -> Dict[str, Any]:
        return {
            "name": self._name,
            "group": self.group,
            "input_names": self.input_names,
            "parameters": self.parameters,
            "output_names": self.output_names,
            "warmup": getattr(self._meta, "warmup_bars", None) if self._meta else None,
        }

    def __repr__(self) -> str:
        return (
            f"Function('{self._name}', inputs={self.input_names}, "
            f"parameters={self.parameters}, outputs={self.output_names})"
        )

    # --- dispatch ---

    def __call__(self, *args, **params):
        """Compute the indicator. Accepts a Polars/dict frame of named inputs, or
        positional arrays in canonical (open, high, low, close, volume) order."""
        if self._method is None or self._spec is None:
            raise NotImplementedError(
                f"{self._name!r} has no batch (.ta) implementation; use "
                f"quantwave.streaming_class({self._name!r}) for streaming."
            )
        import numpy as np  # lazy: array in/out
        import polars as pl

        data = self._collect_inputs(args, np)
        method = getattr(pl.col(self._spec.self_role).ta, self._method)
        # Start from effective defaults (incl. required-param fallbacks), override
        # with caller-supplied params.
        call_kwargs = {k: v for k, v in self._defaults.items() if v is not None}
        call_kwargs.update(_resolve_kwargs(self._spec.numeric, params))
        expr = method(*self._spec.extra_roles, **call_kwargs)
        out = pl.DataFrame(data).select(expr.alias("_out"))
        series = out.to_series()
        if series.dtype == pl.Struct:
            nested = out.unnest("_out")
            return tuple(nested[f].to_numpy() for f in series.struct.fields)
        return series.to_numpy()

    def _collect_inputs(self, args: Sequence[Any], np) -> Dict[str, Any]:
        # `roles` are the internal .ta column names (e.g. 'real' for single-input
        # series); `input_names` are the public names (e.g. 'close'). They align
        # positionally in canonical order — map one to the other.
        roles = self._spec.input_roles
        input_names = self.input_names
        if len(input_names) != len(roles):
            input_names = roles
        # Named inputs: a single dict or a DataFrame-like (has columns).
        if len(args) == 1 and _is_named_frame(args[0]):
            frame = args[0]
            cols = _frame_columns(frame)
            missing = [n for n in input_names if n not in cols]
            if missing:
                raise KeyError(f"{self._name}: missing input column(s) {missing}")
            return {
                role: np.asarray(_frame_col(frame, iname), dtype=float)
                for iname, role in zip(input_names, roles)
            }
        # Positional arrays in canonical role order.
        if len(args) != len(roles):
            raise TypeError(
                f"{self._name} expects {len(roles)} input array(s) {tuple(input_names)}, "
                f"got {len(args)}"
            )
        return {r: np.asarray(a, dtype=float) for r, a in zip(roles, args)}


def _ta_ns():
    from ._ta_namespace import TaNamespace

    return TaNamespace


def _is_named_frame(obj: Any) -> bool:
    if isinstance(obj, dict):
        return True
    return hasattr(obj, "columns") and hasattr(obj, "__getitem__")


def _frame_columns(frame: Any) -> List[str]:
    if isinstance(frame, dict):
        return list(frame.keys())
    return list(frame.columns)


def _frame_col(frame: Any, name: str):
    return frame[name]


def get_functions() -> List[str]:
    """All indicator names (TA-Lib ``get_functions`` equivalent), sorted."""
    from . import indicators

    return sorted(indicators())


def get_function_groups() -> Dict[str, List[str]]:
    """Map indicator group/category -> sorted names.

    Derived from :func:`get_functions` so the union of all groups is exactly the
    function list (no extras, no duplicates)."""
    groups: Dict[str, List[str]] = {}
    for fn in get_functions():
        meta = _metadata(fn)
        grp = getattr(meta, "category", None) or "Other"
        groups.setdefault(grp, []).append(fn)
    for names in groups.values():
        names.sort()
    return groups
