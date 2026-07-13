"""
ML feature matrix helpers.

Builds wide, zero-lookahead feature DataFrames from OHLCV using the same
batch extractors that power Rust proptests and ``lf.ta().features.*``.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING, Any, Dict, List, Sequence, Union

if TYPE_CHECKING:
    import polars as pl


def _pl():
    """Late-bind polars so ``import quantwave`` works without the optional extra."""
    import polars as pl

    return pl


def _ta():
    """Late-bind to avoid circular import with ``__init__.py`` ta population."""
    from quantwave import ta

    return ta


@dataclass(frozen=True)
class FeatureSpec:
    """One feature column group to add to a matrix."""

    name: str
    params: Dict[str, Any]


# Default preset aligned with ml_feature_backtest_parity / ml_feature_stability notebooks.
RECOMMENDED_PRESET: List[FeatureSpec] = [
    FeatureSpec("hurst", {"period": 100}),
    FeatureSpec("cyber_cycle", {"length": 30}),
    FeatureSpec("griffiths_dominant_cycle", {"lower": 6, "upper": 50, "length": 30}),
    FeatureSpec("trendflex", {"length": 30}),
    FeatureSpec("instantaneous_trendline", {}),
    FeatureSpec("regime_hmm", {}),
]


def _closes_from_input(
    data: Union["pl.DataFrame", "pl.LazyFrame", Sequence[float]],
    close_col: str,
) -> tuple[List[float], "pl.DataFrame | None"]:
    pl = _pl()
    if isinstance(data, pl.LazyFrame):
        data = data.collect()
    if isinstance(data, pl.DataFrame):
        if close_col not in data.columns:
            raise ValueError(f"close column '{close_col}' not in DataFrame")
        closes = data[close_col].cast(pl.Float64).to_list()
        return closes, data
    closes = [float(x) for x in data]
    return closes, None


def _apply_feature(
    closes: List[float],
    spec: FeatureSpec,
) -> Dict[str, List[float]]:
    n = len(closes)
    out: Dict[str, List[float]] = {}

    if spec.name == "hurst":
        period = int(spec.params.get("period", 100))
        rows = _ta().hurst_features(period, closes)
        out["hurst_persistence"] = [r.persistence for r in rows]
        out["hurst_regime"] = [
            float(r.regime_label if r.regime_label != -99 else 0) for r in rows
        ]

    elif spec.name == "cyber_cycle":
        length = int(spec.params.get("length", 30))
        rows = _ta().cyber_cycle_features(length, closes)
        out["cyber_cycle"] = [r.cycle for r in rows]
        out["cyber_trigger"] = [r.trigger for r in rows]
        out["cyber_momentum"] = [r.cycle_momentum for r in rows]
        out["cyber_signal"] = [r.trigger_signal for r in rows]

    elif spec.name == "griffiths_dominant_cycle":
        lower = int(spec.params.get("lower", 6))
        upper = int(spec.params.get("upper", 50))
        length = int(spec.params.get("length", 30))
        rows = _ta().griffiths_dominant_cycle_features(lower, upper, length, closes)
        out["griffiths_dc"] = [r.dominant_cycle for r in rows]

    elif spec.name == "trendflex":
        length = int(spec.params.get("length", 30))
        rows = _ta().trendflex_features(length, closes)
        out["trendflex"] = [r.trendflex for r in rows]

    elif spec.name == "instantaneous_trendline":
        rows = _ta().instantaneous_trendline_features(closes)
        out["itrend"] = [r.trend for r in rows]
        out["itrend_strength"] = [r.strength for r in rows]

    elif spec.name == "regime_hmm":
        hmm = _ta().BullBearHMM.bull_bear()
        labels: List[float] = []
        prev = closes[0] if closes else 0.0
        for i, c in enumerate(closes):
            ret = 0.0 if i == 0 else (c - prev) / prev if prev else 0.0
            prev = c
            regime = hmm.next(ret)
            # Match Polars regime_features encoding: 1=Bull, 2=Bear, 0=other
            label = getattr(regime, "value", None)
            if label is None:
                name = str(regime).lower()
                if "bull" in name:
                    label = 1
                elif "bear" in name:
                    label = 2
                else:
                    label = 0
            labels.append(float(label))
        out["regime_label"] = labels

    else:
        raise ValueError(f"Unknown feature '{spec.name}'")

    for k, v in out.items():
        if len(v) != n:
            raise RuntimeError(f"Feature {spec.name}/{k} length {len(v)} != {n}")

    return out


def build_feature_matrix(
    data: Union["pl.DataFrame", "pl.LazyFrame", Sequence[float]],
    *,
    close_col: str = "close",
    features: Union[str, Sequence[Union[str, FeatureSpec, Dict[str, Any]]]] = "recommended",
    drop_warmup: bool = False,
    warmup_bars: int | None = None,
) -> "pl.DataFrame":
    """Build a wide feature DataFrame from close prices (batch / zero-lookahead).

    Args:
        data: LazyFrame, DataFrame, or list of close prices.
        close_col: Column name when ``data`` is a DataFrame.
        features: ``"recommended"`` or a list of feature specs / names.
        drop_warmup: If True, drop leading rows (see ``warmup_bars``).
        warmup_bars: Rows to drop when ``drop_warmup``; default max period heuristic.

    Returns:
        DataFrame with ``close_col`` (if input was DataFrame) plus feature columns.

    Example:
        >>> import quantwave as qw
        >>> df = qw.build_feature_matrix(ohlcv_df, features="recommended")
    """
    closes, base_df = _closes_from_input(data, close_col)

    if features == "recommended":
        specs = list(RECOMMENDED_PRESET)
    else:
        specs = []
        for item in features:
            if isinstance(item, FeatureSpec):
                specs.append(item)
            elif isinstance(item, str):
                specs.append(FeatureSpec(item, {}))
            elif isinstance(item, dict):
                d = dict(item)
                name = d.pop("name", None) or d.pop("feature", None)
                if not name:
                    raise ValueError("feature dict requires 'name' key")
                specs.append(FeatureSpec(name, d))
            else:
                raise TypeError(f"Unsupported feature spec: {item!r}")

    columns: Dict[str, List[float]] = {}
    if base_df is not None:
        columns[close_col] = closes

    for spec in specs:
        columns.update(_apply_feature(closes, spec))

    df = _pl().DataFrame(columns)

    if drop_warmup:
        n_drop = warmup_bars
        if n_drop is None:
            n_drop = 0
            for spec in specs:
                if spec.name == "hurst":
                    n_drop = max(n_drop, int(spec.params.get("period", 100)))
                elif spec.name == "griffiths_dominant_cycle":
                    n_drop = max(n_drop, int(spec.params.get("length", 30)))
                elif spec.name in ("cyber_cycle", "trendflex"):
                    n_drop = max(n_drop, int(spec.params.get("length", 30)))
        if n_drop > 0:
            df = df.slice(n_drop, df.height - n_drop)

    return df


def feature_column_names(
    features: Union[str, Sequence[Union[str, FeatureSpec]]] = "recommended",
) -> List[str]:
    """Return output column names for a feature preset (for ML pipeline wiring)."""
    if features == "recommended":
        specs = RECOMMENDED_PRESET
    else:
        specs = [
            s if isinstance(s, FeatureSpec) else FeatureSpec(s, {})
            for s in features
        ]
    names: List[str] = []
    for spec in specs:
        names.extend(_apply_feature([100.0] * 3, spec).keys())
    return names