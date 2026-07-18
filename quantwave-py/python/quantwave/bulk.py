"""Bulk-compute feature matrix: ``df.ta.all()`` / ``lf.ta.all()``.

Registers new Polars **DataFrame** and **LazyFrame** namespaces named ``"ta"``
(via ``pl.api.register_dataframe_namespace`` / ``register_lazyframe_namespace``).
These are separate plugin registries from the existing Expr ``.ta`` namespace
(:mod:`quantwave._ta_namespace`) registered via ``register_expr_namespace`` — no
conflict, both coexist as ``pl.col(...).ta.sma(...)`` and ``df.ta.all()``.

The bulk method drives every batch-capable indicator generically through the
``quantwave.abstract.Function`` introspection registry:
for each function whose required input columns are present on the frame, it
builds a single lazy ``with_columns`` expression list (so Polars can plan and
parallelize the whole batch), collecting once. Streaming-only indicators and
indicators whose inputs aren't satisfiable by the frame are skipped with a
recorded reason rather than raised.

    import quantwave as qw

    features, manifest = df.ta.all()                 # DataFrame -> DataFrame
    features, manifest = df.lazy().ta.all()           # LazyFrame -> LazyFrame
    features, columns = qw.feature_matrix(df)         # top-level alias
"""

from __future__ import annotations

import time
from typing import Any, Dict, Iterable, List, Optional, Tuple, Union

import polars as pl

from .abstract import Function, get_function_groups, get_functions
from .talib import _Signature

# Internal role placeholders that have no natural OHLCV column mapping
# (e.g. ``beta``/``correl`` take two arbitrary real-valued series).
_ARBITRARY_SERIES_ROLES = {"in1", "in2", "other"}

FrameT = Union[pl.DataFrame, pl.LazyFrame]


def _is_arbitrary_series(input_names: List[str]) -> bool:
    return any(n in _ARBITRARY_SERIES_ROLES for n in input_names)


def _resolve_candidates(
    include: Optional[Iterable[str]],
    exclude: Optional[Iterable[str]],
    groups: Optional[Iterable[str]],
) -> List[str]:
    """Apply include/exclude/groups set algebra over the full function universe."""
    candidates = set(get_functions())

    if groups is not None:
        group_map = get_function_groups()
        allowed: set[str] = set()
        for g in groups:
            allowed |= set(group_map.get(g, []))
        candidates &= allowed

    if include is not None:
        candidates &= set(include)

    if exclude is not None:
        candidates -= set(exclude)

    return sorted(candidates)


def _build_raw_expr(func: Function, overrides: Dict[str, Any]) -> pl.Expr:
    """Build the unaliased ``pl.Expr`` for one indicator (scalar or struct output)."""
    method_name = func._method  # resolved batch (.ta) method; caller has verified non-None
    spec = _Signature(method_name)
    roles = spec.input_roles
    input_names = func.input_names
    # Mirror abstract.Function._collect_inputs's own fallback for role-count mismatches.
    if len(input_names) != len(roles):
        input_names = roles

    # `roles` are the .ta method's internal input-slot labels (frequently, but not
    # always, equal to the canonical OHLCV names); `input_names` are the frame's
    # real column names, positionally aligned with `roles`. Map role -> real column
    # so we reference the caller's actual frame columns when building the Expr.
    role_to_real = {role: iname for iname, role in zip(input_names, roles)}
    self_real = role_to_real.get(spec.self_role, spec.self_role)
    extra_real = [role_to_real.get(r, r) for r in spec.extra_roles]

    call_kwargs = {**func.parameters, **overrides}
    method = getattr(pl.col(self_real).ta, method_name)
    return method(*extra_real, **call_kwargs)


def _finalize_expr(
    name: str, expr: pl.Expr, dtype: pl.DataType
) -> Tuple[List[pl.Expr], List[str]]:
    """Alias a raw indicator Expr into its final output column(s).

    Determined from the Expr's *actual resolved dtype* (not indicator metadata,
    which can overcount outputs for some functions) — Struct dtype means a
    multi-output indicator, unnested to prefixed ``f"{name}_{field}"`` columns
    (deterministic, registry-prefixed collision policy); anything else is a
    single column named ``name``.
    """
    if isinstance(dtype, pl.Struct):
        exprs: List[pl.Expr] = []
        cols: List[str] = []
        for field in dtype.fields:
            col = f"{name}_{field.name}"
            exprs.append(expr.struct.field(field.name).alias(col))
            cols.append(col)
        return exprs, cols
    return [expr.alias(name)], [name]


def _run_all(
    frame: FrameT,
    *,
    include: Optional[Iterable[str]] = None,
    exclude: Optional[Iterable[str]] = None,
    groups: Optional[Iterable[str]] = None,
    params: Optional[Dict[str, Dict[str, Any]]] = None,
    timing: bool = False,
) -> Tuple[FrameT, Dict[str, Any]]:
    is_lazy = isinstance(frame, pl.LazyFrame)
    lf = frame if is_lazy else frame.lazy()
    available_columns = set(lf.collect_schema().names())

    params = params or {}
    candidates = _resolve_candidates(include, exclude, groups)

    computed: List[str] = []
    skipped: List[Dict[str, str]] = []
    raw_exprs: List[Tuple[str, pl.Expr]] = []

    for name in candidates:
        func = Function(name)
        if func._method is None:
            skipped.append(
                {"name": name, "reason": "streaming-only: no batch (.ta) implementation"}
            )
            continue

        input_names = func.input_names
        if _is_arbitrary_series(input_names):
            skipped.append(
                {
                    "name": name,
                    "reason": f"requires {len(input_names)} arbitrary series",
                }
            )
            continue

        missing = [c for c in input_names if c not in available_columns]
        if missing:
            skipped.append(
                {"name": name, "reason": f"missing input columns: {missing}"}
            )
            continue

        overrides = params.get(name, {})
        raw_exprs.append((name, _build_raw_expr(func, overrides)))
        computed.append(name)

    # Resolve each survivor's actual output dtype (scalar vs struct) in one cheap,
    # lazy schema-only pass — no data is materialized here.
    probe_aliases = [f"__ta_all_probe__{name}" for name, _ in raw_exprs]
    if raw_exprs:
        probe_schema = lf.select(
            [expr.alias(alias) for (_, expr), alias in zip(raw_exprs, probe_aliases)]
        ).collect_schema()
    else:
        probe_schema = {}

    all_exprs: List[pl.Expr] = []
    columns: List[str] = []
    plan: List[Tuple[str, List[pl.Expr]]] = []
    for (name, expr), alias in zip(raw_exprs, probe_aliases):
        exprs, cols = _finalize_expr(name, expr, probe_schema[alias])
        all_exprs.extend(exprs)
        columns.extend(cols)
        plan.append((name, exprs))

    timing_map: Dict[str, float] = {}
    if timing:
        # Per-indicator timing requires per-indicator collection, trading batch
        # parallelism for a wall-clock breakdown — only paid when explicitly asked.
        for name, exprs in plan:
            start = time.perf_counter()
            lf.select(exprs).collect()
            timing_map[name] = time.perf_counter() - start

    result_lf = lf.with_columns(all_exprs) if all_exprs else lf
    result: FrameT = result_lf if is_lazy else result_lf.collect()

    manifest: Dict[str, Any] = {
        "computed": computed,
        "skipped": skipped,
        "columns": columns,
    }
    if timing:
        manifest["timing"] = timing_map

    return result, manifest


@pl.api.register_dataframe_namespace("ta")
class TaAllDataFrameNamespace:
    """Bulk indicator namespace on ``pl.DataFrame`` (``df.ta.all()``).

    Distinct from the Expr ``pl.col(...).ta`` namespace — this drives every
    batch-capable indicator generically via the abstract-API registry.
    """

    def __init__(self, df: pl.DataFrame) -> None:
        self._df = df

    def all(
        self,
        include: Optional[Iterable[str]] = None,
        exclude: Optional[Iterable[str]] = None,
        groups: Optional[Iterable[str]] = None,
        params: Optional[Dict[str, Dict[str, Any]]] = None,
        timing: bool = False,
    ) -> Tuple[pl.DataFrame, Dict[str, Any]]:
        """Compute every batch-capable, input-satisfiable indicator.

        Returns ``(frame_with_new_columns, manifest)`` where ``manifest`` is
        ``{"computed": [names], "skipped": [{"name", "reason"}], "columns": [...]}``
        (plus ``"timing": {name: seconds}`` when ``timing=True``).
        """
        return _run_all(
            self._df,
            include=include,
            exclude=exclude,
            groups=groups,
            params=params,
            timing=timing,
        )


@pl.api.register_lazyframe_namespace("ta")
class TaAllLazyFrameNamespace:
    """Bulk indicator namespace on ``pl.LazyFrame`` (``lf.ta.all()``)."""

    def __init__(self, lf: pl.LazyFrame) -> None:
        self._lf = lf

    def all(
        self,
        include: Optional[Iterable[str]] = None,
        exclude: Optional[Iterable[str]] = None,
        groups: Optional[Iterable[str]] = None,
        params: Optional[Dict[str, Dict[str, Any]]] = None,
        timing: bool = False,
    ) -> Tuple[pl.LazyFrame, Dict[str, Any]]:
        """Lazy counterpart of :meth:`TaAllDataFrameNamespace.all`; does not collect."""
        return _run_all(
            self._lf,
            include=include,
            exclude=exclude,
            groups=groups,
            params=params,
            timing=timing,
        )


def feature_matrix(
    df: pl.DataFrame,
    *,
    include: Optional[Iterable[str]] = None,
    exclude: Optional[Iterable[str]] = None,
    groups: Optional[Iterable[str]] = None,
    params: Optional[Dict[str, Dict[str, Any]]] = None,
    timing: bool = False,
) -> Tuple[pl.DataFrame, List[str]]:
    """Top-level alias for ``df.ta.all()`` returning ``(df, feature_column_names)``.

    Same keyword arguments as :meth:`TaAllDataFrameNamespace.all`; drops the
    richer manifest in favor of the flat list of new feature column names for
    quick ML pipeline wiring.
    """
    result, manifest = _run_all(
        df,
        include=include,
        exclude=exclude,
        groups=groups,
        params=params,
        timing=timing,
    )
    return result, manifest["columns"]
