"""
Unified ML feature extraction surface.

Combines three existing, independently-tested extractors into one wide,
leakage-safe feature matrix:

- ``ta_core``: the classic/DSP batch indicator surface via ``df.ta.all()``
  (:mod:`quantwave.bulk`).
- ``ehlers``: Ehlers cycle/trend extractors (hurst, cyber_cycle,
  griffiths_dominant_cycle, trendflex, instantaneous_trendline) via the
  ``_apply_feature`` machinery in :mod:`quantwave.features`.
- ``regimes``: the bull/bear HMM regime label, also via
  :mod:`quantwave.features`.

Plus a fourth, new set:

- ``rolling_stats``: trailing-window mean/std/min/max of close and of
  log-returns, and a rolling return z-score, as native Polars expressions.

Every feature produced by every set is causal by construction: batch
indicators only look backward over their configured window, the Ehlers/regime
extractors are themselves streaming algorithms replayed in order, and
``rolling_stats`` uses only ``rolling_*`` (trailing, ``center=False``)
expressions and single-step ``shift(1)`` — never a forward fill or a
centered window. Optional per-symbol grouping (``by=...``) processes each
group as a fully independent sub-frame, so no feature for symbol A can ever
be computed from symbol B's rows.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Dict, List, Optional, Sequence, Tuple

from .features import FeatureSpec, _apply_feature

if TYPE_CHECKING:
    import polars as pl


def _pl():
    """Late-bind polars so ``import quantwave`` works without the optional extra."""
    import polars as pl

    return pl


def _ensure_ta_all_registered() -> None:
    """Import :mod:`quantwave.bulk` for its ``df.ta.all()`` registration side effect."""
    from . import bulk  # noqa: F401


# Ehlers cycle/trend extractors, mirroring quantwave.features.RECOMMENDED_PRESET
# minus the regime label (split out below as its own feature set).
_EHLERS_SPECS: List[FeatureSpec] = [
    FeatureSpec("hurst", {"period": 100}),
    FeatureSpec("cyber_cycle", {"length": 30}),
    FeatureSpec("griffiths_dominant_cycle", {"lower": 6, "upper": 50, "length": 30}),
    FeatureSpec("trendflex", {"length": 30}),
    FeatureSpec("instantaneous_trendline", {}),
]

_REGIME_SPECS: List[FeatureSpec] = [
    FeatureSpec("regime_hmm", {}),
]

_ROLLING_WINDOWS: Tuple[int, ...] = (5, 10, 20, 50)

_KNOWN_FEATURE_SETS: Tuple[str, ...] = ("ta_core", "ehlers", "regimes", "rolling_stats")

_DEFAULT_FEATURE_SETS: Tuple[str, ...] = _KNOWN_FEATURE_SETS


def _leading_invalid_count(series: "pl.Series") -> int:
    """Count leading null/NaN rows in ``series`` (warmup proxy for one column)."""
    pl = _pl()
    valid = series.is_not_null()
    if series.dtype in (pl.Float32, pl.Float64):
        valid = valid & series.is_not_nan()
    true_idx = valid.arg_true()
    return int(true_idx[0]) if len(true_idx) else series.len()


def _check_no_collisions(existing_cols: Sequence[str], new_names: Sequence[str], feature_set: str) -> None:
    collisions = sorted(set(existing_cols) & set(new_names))
    if collisions:
        raise ValueError(
            f"feature set {feature_set!r} would overwrite existing column(s) {collisions}; "
            "rename the input columns or drop the conflicting feature set"
        )


def _add_ta_core(sub: "pl.DataFrame") -> Tuple["pl.DataFrame", List[str], int]:
    """Attach the classic/DSP batch indicator surface via ``df.ta.all()``."""
    _ensure_ta_all_registered()
    existing_cols = sub.columns
    result, manifest = sub.ta.all()
    cols = list(manifest["columns"])
    _check_no_collisions(existing_cols, cols, "ta_core")
    warmup = 0
    for c in cols:
        warmup = max(warmup, _leading_invalid_count(result[c]))
    return result, cols, warmup


def _add_spec_group(
    sub: "pl.DataFrame", specs: Sequence[FeatureSpec], close_col: str, feature_set: str
) -> Tuple["pl.DataFrame", List[str], int]:
    """Attach columns from :mod:`quantwave.features`' ``_apply_feature`` machinery."""
    pl = _pl()
    closes = sub[close_col].cast(pl.Float64).to_list()
    columns: Dict[str, List[float]] = {}
    for spec in specs:
        columns.update(_apply_feature(closes, spec))
    names = list(columns.keys())
    _check_no_collisions(sub.columns, names, feature_set)
    if names:
        sub = sub.with_columns([pl.Series(name, columns[name]) for name in names])
    warmup = 0
    for name in names:
        warmup = max(warmup, _leading_invalid_count(sub[name]))
    return sub, names, warmup


def _add_rolling_stats(
    sub: "pl.DataFrame", close_col: str, windows: Sequence[int] = _ROLLING_WINDOWS
) -> Tuple["pl.DataFrame", List[str], int]:
    """Trailing rolling stats of close and log-returns (leakage-safe: trailing only)."""
    pl = _pl()
    close = pl.col(close_col).cast(pl.Float64)
    log_ret = (close / close.shift(1)).log()

    exprs: List["pl.Expr"] = []
    names: List[str] = []
    for w in windows:
        window_specs = [
            (f"roll_mean_{w}", close.rolling_mean(w)),
            (f"roll_std_{w}", close.rolling_std(w)),
            (f"roll_min_{w}", close.rolling_min(w)),
            (f"roll_max_{w}", close.rolling_max(w)),
            (f"logret_mean_{w}", log_ret.rolling_mean(w)),
            (f"logret_std_{w}", log_ret.rolling_std(w)),
            (f"logret_zscore_{w}", (log_ret - log_ret.rolling_mean(w)) / log_ret.rolling_std(w)),
        ]
        for name, expr in window_specs:
            exprs.append(expr.alias(name))
            names.append(name)

    _check_no_collisions(sub.columns, names, "rolling_stats")
    sub = sub.with_columns(exprs)
    warmup = 0
    for name in names:
        warmup = max(warmup, _leading_invalid_count(sub[name]))
    return sub, names, warmup


def _process_group(
    sub: "pl.DataFrame", feature_sets: Sequence[str], close_col: str
) -> Tuple["pl.DataFrame", Dict[str, List[str]], Dict[str, int]]:
    """Apply every requested feature set to one fully independent sub-frame."""
    names_by_set: Dict[str, List[str]] = {}
    warmup_by_set: Dict[str, int] = {}

    for fs in feature_sets:
        if fs == "ta_core":
            sub, names, warmup = _add_ta_core(sub)
        elif fs == "ehlers":
            sub, names, warmup = _add_spec_group(sub, _EHLERS_SPECS, close_col, fs)
        elif fs == "regimes":
            sub, names, warmup = _add_spec_group(sub, _REGIME_SPECS, close_col, fs)
        elif fs == "rolling_stats":
            sub, names, warmup = _add_rolling_stats(sub, close_col)
        else:  # pragma: no cover - guarded earlier in extract()
            raise ValueError(f"Unknown feature set: {fs!r}")
        names_by_set[fs] = names
        warmup_by_set[fs] = warmup

    return sub, names_by_set, warmup_by_set


def extract(
    df: "pl.DataFrame",
    feature_sets: Sequence[str] = _DEFAULT_FEATURE_SETS,
    by: Optional[str] = None,
    horizon: Optional[int] = None,
    close_col: str = "close",
) -> Tuple["pl.DataFrame", List[str], Dict[str, Any]]:
    """Build a unified, leakage-safe ML feature matrix.

    Every feature at row t is computed from rows <= t only:

    - ``ta_core`` uses ``df.ta.all()``, the trailing-window classic/DSP
      indicator surface.
    - ``ehlers`` and ``regimes`` replay the streaming Ehlers cycle/trend and
      HMM regime extractors bar-by-bar via
      :mod:`quantwave.features`'s ``_apply_feature``.
    - ``rolling_stats`` uses only trailing (``center=False``) rolling
      expressions and single-step ``shift(1)`` — never a forward fill or a
      centered window.

    Args:
        df: Input DataFrame or LazyFrame (collected internally) containing at
            least ``close_col``, plus whatever OHLCV columns the requested
            ``ta_core`` indicators need (columns they require but don't find
            are skipped, not errored — see ``df.ta.all()``'s manifest).
        feature_sets: Which feature groups to compute, in output order.
            One or more of ``"ta_core"``, ``"ehlers"``, ``"regimes"``,
            ``"rolling_stats"``.
        by: Optional grouping column (e.g. ``"symbol"``). When given, every
            feature set is computed independently per group value — no
            feature for one group's rows is ever derived from another
            group's rows. Row order in the returned frame matches the input
            frame's order regardless of how groups are interleaved.
        horizon: Optional forward-looking bar count reserved for the
            caller's own label construction (e.g. "predict return over the
            next `horizon` bars"). Not used to compute any feature here —
            doing so would introduce lookahead — and is only echoed back in
            ``metadata["horizon"]`` for the caller's convenience.
        close_col: Name of the close-price column.

    Returns:
        A 3-tuple ``(features_df, feature_names, metadata)``:

        - ``features_df``: the input columns plus every added feature
          column.
        - ``feature_names``: the list of added feature column names, in the
          order the feature sets were requested (and, within a set, the
          order that set's columns were produced). Only columns usable as
          model features appear here: ``df.select(feature_names).to_numpy()``
          is guaranteed to be a numeric matrix. Struct-valued columns are
          excluded (see ``metadata["excluded_features"]``); they remain in
          ``features_df`` and can be unnested by the caller.
        - ``metadata``: ``{feature_set_name: {"feature_names": [...],
          "warmup": int}, ...}`` for each requested feature set, plus
          ``"warmup"`` (the max across sets), ``"by"``, ``"horizon"``,
          ``"n_rows"``, and ``"excluded_features"`` (a ``{column: dtype}``
          map of columns held back from ``feature_names``). ``warmup`` is the number of leading rows (per set)
          that contain a null/NaN in at least one of that set's columns —
          i.e. the number of rows a caller should trim before training.

    Example:
        >>> import quantwave as qw
        >>> from quantwave.feature_extract import extract
        >>> features_df, feature_names, metadata = extract(
        ...     ohlcv_df, feature_sets=("ta_core", "rolling_stats"), by="symbol"
        ... )
    """
    pl = _pl()
    if isinstance(df, pl.LazyFrame):
        df = df.collect()
    if close_col not in df.columns:
        raise ValueError(f"close column {close_col!r} not in DataFrame")
    if by is not None and by not in df.columns:
        raise ValueError(f"group column {by!r} not in DataFrame")

    requested = list(feature_sets)
    if not requested:
        raise ValueError("feature_sets must be non-empty")
    unknown = sorted(set(requested) - set(_KNOWN_FEATURE_SETS))
    if unknown:
        raise ValueError(
            f"Unknown feature set(s): {unknown}; known sets are {list(_KNOWN_FEATURE_SETS)}"
        )

    idx_col = "__qw_extract_row_idx__"
    indexed = df.with_row_index(idx_col)

    group_keys: List[Any]
    if by is None:
        group_keys = [None]
    else:
        group_keys = indexed[by].unique(maintain_order=True).to_list()

    processed_frames: List["pl.DataFrame"] = []
    names_by_set: Dict[str, List[str]] = {fs: [] for fs in requested}
    warmup_by_set: Dict[str, int] = {fs: 0 for fs in requested}

    for key in group_keys:
        sub = indexed if key is None else indexed.filter(pl.col(by) == key)
        sub_result, sub_names, sub_warmup = _process_group(sub, requested, close_col)
        processed_frames.append(sub_result)
        for fs in requested:
            if not names_by_set[fs]:
                names_by_set[fs] = sub_names[fs]
            warmup_by_set[fs] = max(warmup_by_set[fs], sub_warmup[fs])

    result = processed_frames[0] if len(processed_frames) == 1 else pl.concat(
        processed_frames, how="vertical"
    )
    result = result.sort(idx_col).drop(idx_col)

    # `feature_names` is a promise that `df.select(feature_names).to_numpy()` is a
    # numeric matrix. Struct-valued columns (the geometric-pattern detectors) break
    # that: a single Struct column forces numpy to fall back to dtype=object for the
    # WHOLE matrix, so every other column loses its numeric-ness too and even a NaN
    # check raises. They stay in `features_df` -- callers who want them can unnest
    # them -- but they are not model features (quantwave-3vin).
    dtypes = dict(zip(result.columns, result.dtypes))
    excluded: Dict[str, str] = {}

    def _is_model_feature(name: str) -> bool:
        dtype = dtypes.get(name)
        if dtype is not None and isinstance(dtype, pl.Struct):
            excluded[name] = str(dtype)
            return False
        return True

    names_by_set = {fs: [n for n in names_by_set[fs] if _is_model_feature(n)] for fs in requested}

    feature_names: List[str] = []
    for fs in requested:
        feature_names.extend(names_by_set[fs])

    metadata: Dict[str, Any] = {
        fs: {"feature_names": names_by_set[fs], "warmup": warmup_by_set[fs]} for fs in requested
    }
    metadata["excluded_features"] = excluded
    metadata["warmup"] = max(warmup_by_set.values()) if warmup_by_set else 0
    metadata["by"] = by
    metadata["horizon"] = horizon
    metadata["n_rows"] = result.height

    return result, feature_names, metadata
