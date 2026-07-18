"""Bulk-compute feature: ``df.ta.all()`` / ``lf.ta.all()`` / ``qw.feature_matrix()``
(quantwave-p2k0.2).

Registers new DataFrame/LazyFrame ``"ta"`` namespaces (distinct from the existing
Expr ``.ta`` namespace) that compute every batch-capable, input-satisfiable
indicator in a single lazy pass and return ``(frame, manifest)``.
"""

from __future__ import annotations

import random
import time

import numpy as np
import polars as pl
import pytest

import quantwave as qw
from quantwave.abstract import Function


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------


@pytest.fixture
def ohlcv_df() -> pl.DataFrame:
    rng = np.random.RandomState(7)
    n = 300
    close = np.cumsum(rng.randn(n)) + 100.0
    high = close + rng.rand(n)
    low = close - rng.rand(n)
    open_ = close + rng.randn(n) * 0.2
    volume = np.abs(rng.randn(n)) * 1e5 + 1e4
    return pl.DataFrame(
        {
            "open": open_,
            "high": high,
            "low": low,
            "close": close,
            "volume": volume,
        }
    )


@pytest.fixture
def ohlc_no_volume_df(ohlcv_df: pl.DataFrame) -> pl.DataFrame:
    return ohlcv_df.drop("volume")


def _batch_capable_names() -> list[str]:
    names = []
    for name in qw.get_functions():
        if Function(name)._method is not None:
            names.append(name)
    return names


def _input_satisfiable_names(columns: set[str]) -> list[str]:
    names = []
    for name in _batch_capable_names():
        f = Function(name)
        if all(c in columns for c in f.input_names):
            names.append(name)
    return names


# ---------------------------------------------------------------------------
# 1. Basic computation + manifest shape
# ---------------------------------------------------------------------------


def test_all_computes_expected_minimum_columns(ohlcv_df: pl.DataFrame):
    expected_names = _input_satisfiable_names(set(ohlcv_df.columns))

    result, manifest = ohlcv_df.ta.all()

    assert isinstance(result, pl.DataFrame)
    assert manifest["computed"], "manifest.computed should be non-empty"
    assert set(manifest["computed"]) == set(expected_names)
    assert len(manifest["columns"]) >= len(expected_names)

    # every new column actually landed on the returned frame
    for col in manifest["columns"]:
        assert col in result.columns

    # skipped entries all carry a reason
    for entry in manifest["skipped"]:
        assert entry["reason"]
        assert entry["name"]

    # streaming-only + missing/placeholder-input functions accounted for
    all_names = set(qw.get_functions())
    accounted = set(manifest["computed"]) | {e["name"] for e in manifest["skipped"]}
    assert accounted == all_names


def test_all_lazyframe_namespace(ohlcv_df: pl.DataFrame):
    result, manifest = ohlcv_df.lazy().ta.all()
    assert isinstance(result, pl.LazyFrame)
    collected = result.collect()
    assert manifest["computed"]
    for col in manifest["columns"]:
        assert col in collected.columns


# ---------------------------------------------------------------------------
# 2. Equality against per-indicator Function() calls
# ---------------------------------------------------------------------------


def test_all_matches_individual_function_calls(ohlcv_df: pl.DataFrame):
    result, manifest = ohlcv_df.ta.all()

    rng = random.Random(42)
    sample = rng.sample(manifest["computed"], k=min(20, len(manifest["computed"])))

    for name in sample:
        f = Function(name)
        expected = f(ohlcv_df, **f.parameters)
        if isinstance(expected, tuple):
            for field, exp_arr in zip(f.output_names, expected):
                col = f"{name}_{field}"
                assert col in result.columns, f"missing column {col}"
                got = result[col].to_numpy()
                assert np.allclose(got, exp_arr, equal_nan=True), name
        else:
            assert name in result.columns, f"missing column {name}"
            got = result[name].to_numpy()
            assert np.allclose(got, expected, equal_nan=True), name


# ---------------------------------------------------------------------------
# 3. include / exclude / groups set algebra
# ---------------------------------------------------------------------------


def test_include_restricts_to_named_set(ohlcv_df: pl.DataFrame):
    expected_names = set(_input_satisfiable_names(set(ohlcv_df.columns)))
    include = sorted(expected_names)[:5]

    _, manifest = ohlcv_df.ta.all(include=include)

    assert set(manifest["computed"]) == set(include) & expected_names


def test_exclude_removes_named_set(ohlcv_df: pl.DataFrame):
    expected_names = set(_input_satisfiable_names(set(ohlcv_df.columns)))
    exclude = sorted(expected_names)[:5]

    _, manifest = ohlcv_df.ta.all(exclude=exclude)

    assert set(manifest["computed"]) == expected_names - set(exclude)


def test_groups_restricts_to_named_groups(ohlcv_df: pl.DataFrame):
    groups_map = qw.get_function_groups()
    group_name = "Volatility"
    assert group_name in groups_map

    expected_names = set(_input_satisfiable_names(set(ohlcv_df.columns)))
    expected_in_group = expected_names & set(groups_map[group_name])

    _, manifest = ohlcv_df.ta.all(groups=[group_name])

    assert set(manifest["computed"]) == expected_in_group


def test_include_exclude_groups_combine(ohlcv_df: pl.DataFrame):
    groups_map = qw.get_function_groups()
    expected_names = set(_input_satisfiable_names(set(ohlcv_df.columns)))
    group_name = "Momentum"
    in_group = expected_names & set(groups_map[group_name])
    exclude = sorted(in_group)[:2]

    _, manifest = ohlcv_df.ta.all(groups=[group_name], exclude=exclude)

    assert set(manifest["computed"]) == in_group - set(exclude)


# ---------------------------------------------------------------------------
# 4. Missing-column handling
# ---------------------------------------------------------------------------


def test_missing_volume_column_skips_volume_indicators(ohlc_no_volume_df: pl.DataFrame):
    volume_names = {
        name
        for name in _batch_capable_names()
        if "volume" in Function(name).input_names
    }
    assert volume_names, "expected at least one volume-dependent batch indicator"

    result, manifest = ohlc_no_volume_df.ta.all()

    skipped_by_name = {e["name"]: e["reason"] for e in manifest["skipped"]}
    for name in volume_names:
        assert name in skipped_by_name
        assert "volume" in skipped_by_name[name]
        assert name not in manifest["computed"]

    # nothing raised, and computation still happened for satisfiable indicators
    assert manifest["computed"]
    assert "volume" not in result.columns or "volume" not in ohlc_no_volume_df.columns


def test_beta_correl_skipped_as_arbitrary_series(ohlcv_df: pl.DataFrame):
    _, manifest = ohlcv_df.ta.all()
    skipped_by_name = {e["name"]: e["reason"] for e in manifest["skipped"]}
    for name in ("beta", "correl"):
        assert name in skipped_by_name
        assert "arbitrary series" in skipped_by_name[name]


# ---------------------------------------------------------------------------
# 5. Perf smoke
# ---------------------------------------------------------------------------


def test_all_perf_smoke_100k_rows():
    rng = np.random.RandomState(3)
    n = 100_000
    close = np.cumsum(rng.randn(n)) + 100.0
    high = close + rng.rand(n)
    low = close - rng.rand(n)
    open_ = close + rng.randn(n) * 0.2
    volume = np.abs(rng.randn(n)) * 1e5 + 1e4
    df = pl.DataFrame(
        {"open": open_, "high": high, "low": low, "close": close, "volume": volume}
    )

    # `geometric_patterns` has a pre-existing (pathological, non-linear-looking)
    # cost in its underlying implementation at this row count — confirmed via
    # direct `Function("geometric_patterns")(...)` calls outside of df.ta.all(),
    # i.e. not something introduced by the bulk-compute path. Excluded here so
    # the smoke test measures df.ta.all()'s own overhead rather than that one
    # indicator's known algorithmic cost.
    exclude = ["geometric_patterns"]

    start = time.perf_counter()
    result, manifest = df.ta.all(exclude=exclude)
    elapsed = time.perf_counter() - start

    assert result.height == n
    assert manifest["computed"]
    print(f"\n[perf] df.ta.all() on {n} rows: {elapsed:.3f}s, "
          f"{len(manifest['computed'])} indicators computed")


# ---------------------------------------------------------------------------
# feature_matrix top-level alias
# ---------------------------------------------------------------------------


def test_feature_matrix_alias(ohlcv_df: pl.DataFrame):
    result, columns = qw.feature_matrix(ohlcv_df)
    assert isinstance(result, pl.DataFrame)
    assert isinstance(columns, list)
    assert columns
    for col in columns:
        assert col in result.columns


def test_feature_matrix_alias_honors_kwargs(ohlcv_df: pl.DataFrame):
    expected_names = set(_input_satisfiable_names(set(ohlcv_df.columns)))
    include = sorted(expected_names)[:3]
    result, columns = qw.feature_matrix(ohlcv_df, include=include)
    assert columns
    # every returned column should be attributable to one of the included indicators
    for col in columns:
        base = col.split("_")[0]
        assert any(col == name or col.startswith(f"{name}_") for name in include)


# ---------------------------------------------------------------------------
# timing=True manifest
# ---------------------------------------------------------------------------


def test_timing_manifest(ohlcv_df: pl.DataFrame):
    _, manifest = ohlcv_df.ta.all(include=["sma", "rsi", "atr"], timing=True)
    assert "timing" in manifest
    for name in manifest["computed"]:
        assert name in manifest["timing"]
        assert manifest["timing"][name] >= 0
