"""Tests for quantwave.research (conditional-outcome query layer).

Covers: the Wilson confidence-interval formula against a known textbook
reference value, an obviously-correlated condition/outcome case, the
min-sample-size guard (no fabricated numbers below threshold), a
zero-matching-rows case (division-by-zero guard), and the chronological
stability split producing two roughly-half-sized sub-results.
"""

import pytest

polars = pytest.importorskip("polars")
pl = polars

from quantwave import research


# ---------------------------------------------------------------------------
# Wilson interval: known reference value.
# ---------------------------------------------------------------------------


def test_wilson_interval_known_reference_n100_x50():
    # Textbook example: n=100, x=50 successes (p_hat=0.5), z=1.96.
    # Widely-cited Wilson 95% bounds for this case are approximately
    # (0.404, 0.596) -- see e.g. Wilson (1927) / standard stats references.
    low, high = research.wilson_interval(50, 100)
    assert low == pytest.approx(0.404, abs=0.001)
    assert high == pytest.approx(0.596, abs=0.001)


def test_wilson_interval_symmetric_around_center_for_p_half():
    low, high = research.wilson_interval(50, 100)
    # p_hat=0.5 is the one case where the Wilson center is (numerically)
    # extremely close to 0.5, so the interval should be nearly symmetric.
    assert (low + high) / 2 == pytest.approx(0.5, abs=0.01)


def test_wilson_interval_zero_n_is_maximally_uninformative():
    low, high = research.wilson_interval(0, 0)
    assert low == 0.0
    assert high == 1.0


def test_wilson_interval_near_zero_proportion_stays_in_bounds():
    # p_hat near 0 -- the naive Wald interval can go negative; Wilson must not.
    low, high = research.wilson_interval(1, 200)
    assert 0.0 <= low <= high <= 1.0


def test_wilson_interval_near_one_proportion_stays_in_bounds():
    low, high = research.wilson_interval(199, 200)
    assert 0.0 <= low <= high <= 1.0


# ---------------------------------------------------------------------------
# conditional_outcome: correlated condition/outcome case.
# ---------------------------------------------------------------------------


def _correlated_df(n: int = 100) -> pl.DataFrame:
    # condition true on even rows, outcome deterministically true whenever
    # condition is true (and false otherwise) -- an obvious "edge".
    cond = [(i % 2 == 0) for i in range(n)]
    outcome = [c for c in cond]  # outcome mirrors condition exactly
    return pl.DataFrame({"cond": cond, "outcome": outcome})


def test_conditional_outcome_deterministic_edge_near_one_tight_ci():
    df = _correlated_df(100)
    result = research.conditional_outcome(
        df,
        outcome=pl.col("outcome"),
        condition=pl.col("cond"),
        min_samples=30,
    )
    assert result["computed"] is True
    assert result["n"] == 50
    assert result["point_estimate"] == pytest.approx(1.0)
    assert result["wilson_ci_low"] > 0.85  # tight interval hugging 1.0
    assert result["wilson_ci_high"] == pytest.approx(1.0)


def test_conditional_outcome_below_min_samples_does_not_fabricate():
    df = _correlated_df(20)  # only 10 eligible rows (cond true on evens)
    result = research.conditional_outcome(
        df,
        outcome=pl.col("outcome"),
        condition=pl.col("cond"),
        min_samples=30,
    )
    assert result["n"] == 10
    assert result["computed"] is False
    assert result["min_samples_met"] is False
    assert result["point_estimate"] is None
    assert result["wilson_ci_low"] is None
    assert result["wilson_ci_high"] is None


def test_conditional_outcome_zero_matching_rows_no_crash():
    df = pl.DataFrame({"cond": [False, False, False], "outcome": [True, False, True]})
    result = research.conditional_outcome(
        df,
        outcome=pl.col("outcome"),
        condition=pl.col("cond"),
        min_samples=1,
    )
    assert result["n"] == 0
    assert result["computed"] is False
    assert result["point_estimate"] is None
    assert result["first_half"] is None
    assert result["second_half"] is None


# ---------------------------------------------------------------------------
# Stability split: two roughly-half-sized sub-results.
# ---------------------------------------------------------------------------


def test_conditional_outcome_stability_split_halves():
    df = _correlated_df(100)  # 50 eligible rows
    result = research.conditional_outcome(
        df,
        outcome=pl.col("outcome"),
        condition=pl.col("cond"),
        min_samples=10,
    )
    assert result["computed"] is True
    first, second = result["first_half"], result["second_half"]
    assert first is not None and second is not None
    assert first["n"] + second["n"] == result["n"]
    assert abs(first["n"] - second["n"]) <= 1
    # The synthetic edge is deterministic, so it should hold in both halves.
    assert first["computed"] is True
    assert second["computed"] is True
    assert first["point_estimate"] == pytest.approx(1.0)
    assert second["point_estimate"] == pytest.approx(1.0)


def test_conditional_outcome_stability_split_below_min_samples_per_half():
    # 50 eligible rows total but min_samples=30 -> whole sample passes,
    # but each half (25 rows) does not -- halves should report
    # computed=False without crashing.
    df = _correlated_df(100)
    result = research.conditional_outcome(
        df,
        outcome=pl.col("outcome"),
        condition=pl.col("cond"),
        min_samples=30,
    )
    assert result["computed"] is True
    assert result["first_half"]["computed"] is False
    assert result["second_half"]["computed"] is False
