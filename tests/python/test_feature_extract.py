"""TDD tests for the unified ML feature extraction surface (``feature_extract.extract``).

Leakage safety is the primary guarantee under test: every feature at row t must be a
function of information at rows <= t only. The leakage-property test (below) mutates
all rows strictly after some index t with extreme values and asserts the recomputed
feature rows at indices <= t are bit-identical — a feature that changes proves
lookahead into the mutated future.
"""

from __future__ import annotations

import math

import pytest

pl = pytest.importorskip("polars")

from quantwave import datasets
from quantwave.feature_extract import extract

FEATURE_SETS = ("ta_core", "ehlers", "regimes", "rolling_stats")


def _base_df(rows: int = 260, seed: int = 7):
    return datasets.synthetic(seed=seed, rows=rows)


def _mutate_after(df: "pl.DataFrame", t: int, cols=("open", "high", "low", "close", "volume")):
    """Return a copy of ``df`` with all rows at index > t set to extreme values."""
    idx_col = "__test_idx__"
    tagged = df.with_row_index(idx_col)
    exprs = [
        pl.when(pl.col(idx_col) > t)
        .then(pl.lit(1.0e9))
        .otherwise(pl.col(c))
        .alias(c)
        for c in cols
        if c in df.columns
    ]
    return tagged.with_columns(exprs).drop(idx_col)


def _values_equal(va, vb) -> bool:
    """NaN-aware, struct-aware equality (equal_nan semantics for nested dicts too)."""
    if va is None and vb is None:
        return True
    if isinstance(va, float) and isinstance(vb, float):
        if math.isnan(va) and math.isnan(vb):
            return True
        return va == vb
    if isinstance(va, dict) and isinstance(vb, dict):
        if va.keys() != vb.keys():
            return False
        return all(_values_equal(va[k], vb[k]) for k in va)
    return va == vb


def _assert_prefix_identical(a: "pl.DataFrame", b: "pl.DataFrame", names, upto: int, label: str):
    for name in names:
        va_list = a[name][: upto + 1].to_list()
        vb_list = b[name][: upto + 1].to_list()
        assert len(va_list) == len(vb_list)
        for i, (va, vb) in enumerate(zip(va_list, vb_list)):
            assert _values_equal(va, vb), (
                f"{label}/{name} leaked future information: row {i} (<= t={upto}) "
                f"changed from {va!r} to {vb!r} after mutating rows > t"
            )


# ---------------------------------------------------------------------------
# 1. Leakage property — the core guarantee, run over every feature set.
# ---------------------------------------------------------------------------


@pytest.mark.parametrize("feature_set", FEATURE_SETS)
def test_leakage_property_single_feature_set(feature_set):
    df = _base_df()
    t = 150

    features_a, names_a, _ = extract(df, feature_sets=(feature_set,))
    mutated = _mutate_after(df, t)
    features_b, names_b, _ = extract(mutated, feature_sets=(feature_set,))

    assert names_a == names_b
    assert names_a, f"{feature_set} produced no feature columns to check"
    _assert_prefix_identical(features_a, features_b, names_a, t, feature_set)


def test_leakage_property_all_feature_sets_combined():
    df = _base_df()
    t = 150

    features_a, names_a, _ = extract(df, feature_sets=FEATURE_SETS)
    mutated = _mutate_after(df, t)
    features_b, names_b, _ = extract(mutated, feature_sets=FEATURE_SETS)

    assert names_a == names_b
    _assert_prefix_identical(features_a, features_b, names_a, t, "combined")


def test_leakage_property_with_grouping():
    df = datasets.synthetic(seed=13, rows=180, symbols=["AAA", "BBB"])
    t = 90

    features_a, names_a, _ = extract(df, feature_sets=FEATURE_SETS, by="symbol")
    mutated = _mutate_after(df, t)
    features_b, names_b, _ = extract(mutated, feature_sets=FEATURE_SETS, by="symbol")

    assert names_a == names_b
    _assert_prefix_identical(features_a, features_b, names_a, t, "grouped")


# ---------------------------------------------------------------------------
# 2. feature_names contract.
# ---------------------------------------------------------------------------


def test_feature_names_exactly_equals_added_columns():
    df = _base_df()
    before_cols = set(df.columns)

    features_df, feature_names, _ = extract(df, feature_sets=FEATURE_SETS)

    added_cols = set(features_df.columns) - before_cols
    assert set(feature_names) == added_cols
    assert len(feature_names) == len(set(feature_names)), "duplicate feature names"


# ---------------------------------------------------------------------------
# 3. Multi-symbol isolation.
# ---------------------------------------------------------------------------


def test_multi_symbol_isolation():
    rows = 220
    df_multi = datasets.synthetic(seed=11, rows=rows, symbols=["AAA", "BBB"])

    features_multi, names_multi, _ = extract(df_multi, feature_sets=FEATURE_SETS, by="symbol")

    for sym in ("AAA", "BBB"):
        df_single = df_multi.filter(pl.col("symbol") == sym).drop("symbol")
        features_single, names_single, _ = extract(df_single, feature_sets=FEATURE_SETS)

        assert names_single == names_multi

        sub_multi = features_multi.filter(pl.col("symbol") == sym)
        assert sub_multi.height == features_single.height

        for name in names_single:
            a_list = sub_multi[name].to_list()
            b_list = features_single[name].to_list()
            for i, (va, vb) in enumerate(zip(a_list, b_list)):
                assert _values_equal(va, vb), (
                    f"symbol {sym!r} feature {name!r} row {i} reflects cross-symbol "
                    f"data: grouped={va!r} isolated={vb!r}"
                )


# ---------------------------------------------------------------------------
# 4. metadata contract.
# ---------------------------------------------------------------------------


def test_metadata_contract():
    df = _base_df()
    features_df, feature_names, metadata = extract(df, feature_sets=FEATURE_SETS)

    union_names = []
    for fs in FEATURE_SETS:
        assert fs in metadata
        info = metadata[fs]
        assert "feature_names" in info
        assert "warmup" in info
        assert info["feature_names"], f"{fs} has empty feature_names"
        union_names.extend(info["feature_names"])

    # ta_core, ehlers, and rolling_stats are windowed -> nonzero warmup required.
    for fs in ("ta_core", "ehlers", "rolling_stats"):
        assert metadata[fs]["warmup"] > 0, f"{fs} warmup should be > 0"

    assert set(union_names) == set(feature_names)


def test_metadata_requested_subset_only():
    df = _base_df()
    _, feature_names, metadata = extract(df, feature_sets=("rolling_stats",))

    assert set(metadata.keys()) >= {"rolling_stats"}
    for fs in ("ta_core", "ehlers", "regimes"):
        assert fs not in metadata
    assert set(metadata["rolling_stats"]["feature_names"]) == set(feature_names)


# ---------------------------------------------------------------------------
# 5. Integration smoke — sklearn optional, must skip cleanly if absent.
# ---------------------------------------------------------------------------


def test_integration_smoke_sklearn_fit():
    sklearn = pytest.importorskip("sklearn")
    from sklearn.linear_model import LinearRegression

    df = _base_df(rows=400)
    features_df, feature_names, metadata = extract(df, feature_sets=FEATURE_SETS)

    warmup = max(info["warmup"] for info in metadata.values() if isinstance(info, dict) and "warmup" in info)
    trimmed = features_df.slice(warmup, features_df.height - warmup)

    x = trimmed.select(feature_names).to_numpy()
    y = trimmed["close"].to_numpy()

    assert not bool((x != x).any()), "NaNs remain in feature matrix after warmup trim"

    model = LinearRegression()
    model.fit(x, y)
    assert model.coef_.shape[0] == len(feature_names)
