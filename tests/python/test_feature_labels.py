"""Tests for ML label helpers: forward returns and triple-barrier labeling."""

from __future__ import annotations

import pytest

polars = pytest.importorskip("polars")
pl = polars

from quantwave.feature_labels import forward_returns, triple_barrier


# ---------------------------------------------------------------------------
# Triple-barrier gold fixtures
# ---------------------------------------------------------------------------
# Definition under test (see feature_labels.py docstring for the full spec):
#   From bar t, look forward over bars t+1 .. t+max_holding (inclusive window).
#   upper = price[t] * (1 + pt); lower = price[t] * (1 - sl).
#   First bar in the window where price >= upper -> label +1, touch_kind "pt".
#   First bar in the window where price <= lower -> label -1, touch_kind "sl".
#   If a single bar satisfies both (degenerate), pt wins (upper barrier priority).
#   If neither barrier is touched anywhere in the window -> label 0, touch_kind
#   "time", touch_idx == t + max_holding (the vertical/time barrier).
#   A touch landing exactly on bar t + max_holding still counts as a pt/sl touch,
#   not a time-out (tie policy: the window is closed/inclusive on the right).
#   touch_idx is the bar index *within the symbol's own series* (0-based, local
#   to the group when `by` is used).


def test_triple_barrier_pt_touched_first():
    # Rises past +pt (upper=105) at k=3 before ever reaching -sl (lower=95, seen
    # at k=4, which is later) -> label +1, touch_kind "pt", touch_idx 3.
    prices = [100.0, 101.0, 102.0, 108.0, 90.0, 95.0]
    df = pl.DataFrame({"close": prices})
    out = triple_barrier(df, price="close", pt=0.05, sl=0.05, max_holding=4)

    assert out["label"][0] == 1
    assert out["touch_idx"][0] == 3
    assert out["touch_kind"][0] == "pt"


def test_triple_barrier_sl_touched_first():
    # Falls to -sl (lower=95) at k=3 before ever reaching +pt (upper=105, only
    # reached at k=4) -> label -1, touch_kind "sl", touch_idx 3.
    prices = [100.0, 99.0, 97.0, 93.0, 110.0]
    df = pl.DataFrame({"close": prices})
    out = triple_barrier(df, price="close", pt=0.05, sl=0.05, max_holding=4)

    assert out["label"][0] == -1
    assert out["touch_idx"][0] == 3
    assert out["touch_kind"][0] == "sl"


def test_triple_barrier_time_barrier_when_neither_touched():
    # Neither +/-10% barrier is touched within max_holding=4 bars -> label 0,
    # touch_kind "time", touch_idx == t + max_holding == 4.
    prices = [100.0, 101.0, 102.0, 103.0, 104.0]
    df = pl.DataFrame({"close": prices})
    out = triple_barrier(df, price="close", pt=0.10, sl=0.10, max_holding=4)

    assert out["label"][0] == 0
    assert out["touch_idx"][0] == 4
    assert out["touch_kind"][0] == "time"


def test_triple_barrier_tie_at_max_holding_counts_as_touch():
    # Flat until the very last bar of the window, which lands exactly on
    # t + max_holding and clears the pt barrier. Documented tie policy: this
    # counts as a pt touch, NOT a time-out, because the window is inclusive.
    prices = [100.0, 100.0, 100.0, 100.0, 106.0]
    df = pl.DataFrame({"close": prices})
    out = triple_barrier(df, price="close", pt=0.05, sl=0.05, max_holding=4)

    assert out["label"][0] == 1
    assert out["touch_idx"][0] == 4
    assert out["touch_kind"][0] == "pt"


def test_triple_barrier_incomplete_window_is_null():
    # The last bar has no future bars at all, so no verdict can be reached
    # (not even a time-barrier verdict, since we can't confirm max_holding
    # bars elapsed without a touch). Labeled null, matching forward_returns'
    # "insufficient future data -> null" convention.
    prices = [100.0, 101.0, 102.0]
    df = pl.DataFrame({"close": prices})
    out = triple_barrier(df, price="close", pt=0.10, sl=0.10, max_holding=4)

    assert out["label"][2] is None
    assert out["touch_idx"][2] is None
    assert out["touch_kind"][2] is None


def test_triple_barrier_scalar_over_full_series():
    # Sanity: every row gets a verdict of the expected shape/type for a longer
    # series with mixed outcomes.
    prices = [100.0, 100.0, 106.0, 106.0, 94.0, 94.0, 94.0, 94.0]
    df = pl.DataFrame({"close": prices})
    out = triple_barrier(df, price="close", pt=0.05, sl=0.05, max_holding=3)

    assert out.height == len(prices)
    assert set(out.columns) >= {"label", "touch_idx", "touch_kind"}
    # t=0: window k=1..3 -> prices[2]=106 hits pt first (k=2)
    assert out["label"][0] == 1
    assert out["touch_kind"][0] == "pt"
    assert out["touch_idx"][0] == 2


# ---------------------------------------------------------------------------
# forward_returns correctness
# ---------------------------------------------------------------------------


def test_forward_returns_values_match_hand_computation():
    prices = [100.0, 102.0, 101.0, 105.0, 110.0, 108.0]
    df = pl.DataFrame({"close": prices})
    out = forward_returns(df, horizons=(1, 3), price="close")

    n = len(prices)
    for h in (1, 3):
        col = out[f"fwd_ret_{h}"].to_list()
        for t in range(n):
            if t + h < n:
                expected = (prices[t + h] - prices[t]) / prices[t]
                assert col[t] == pytest.approx(expected)
            else:
                assert col[t] is None


def test_forward_returns_last_h_rows_null():
    prices = [1.0, 2.0, 3.0, 4.0, 5.0]
    df = pl.DataFrame({"close": prices})
    out = forward_returns(df, horizons=(1, 2, 4), price="close")

    assert out["fwd_ret_1"][-1] is None
    assert out["fwd_ret_2"][-1] is None
    assert out["fwd_ret_2"][-2] is None
    assert out["fwd_ret_4"][0] == pytest.approx((5.0 - 1.0) / 1.0)
    assert out["fwd_ret_4"][1:].null_count() == len(prices) - 1


# ---------------------------------------------------------------------------
# Multi-symbol (`by`) boundary correctness — no cross-symbol leakage
# ---------------------------------------------------------------------------


def _two_symbol_df():
    # Symbol A: gentle ramp ending at 104 (5 bars).
    # Symbol B: starts at a wildly different level (1000) so any leakage
    # across the boundary is numerically obvious.
    a_prices = [100.0, 101.0, 102.0, 103.0, 104.0]
    b_prices = [1000.0, 950.0, 1100.0, 900.0, 980.0]
    return pl.DataFrame(
        {
            "symbol": ["A"] * len(a_prices) + ["B"] * len(b_prices),
            "close": a_prices + b_prices,
        }
    )


def test_forward_returns_no_cross_symbol_leakage():
    df = _two_symbol_df()
    out = forward_returns(df, horizons=(1, 2), price="close", by="symbol")

    a_rows = out.filter(pl.col("symbol") == "A")
    b_rows = out.filter(pl.col("symbol") == "B")

    # Last row(s) of A must be null for both horizons, not filled from B.
    assert a_rows["fwd_ret_1"][-1] is None
    assert a_rows["fwd_ret_2"][-1] is None
    assert a_rows["fwd_ret_2"][-2] is None

    # First row of B computed purely from B's own prices.
    expected = (1100.0 - 1000.0) / 1000.0
    assert b_rows["fwd_ret_2"][0] == pytest.approx(expected)

    # Last row of B is null (no future data at all, regardless of symbol).
    assert b_rows["fwd_ret_1"][-1] is None


def test_triple_barrier_no_cross_symbol_leakage():
    df = _two_symbol_df()
    # pt/sl huge enough that A's own prices never trigger a touch, so if the
    # implementation leaked into B's 1000-level prices it would trigger
    # spuriously (or produce a wildly different index).
    out = triple_barrier(df, price="close", pt=5.0, sl=5.0, max_holding=3, by="symbol")

    a_rows = out.filter(pl.col("symbol") == "A")
    b_rows = out.filter(pl.col("symbol") == "B")

    # A's rows never touch a 500%-away barrier within its own series ->
    # every row is either "time" (label 0) or null (incomplete window), and
    # touch_idx must stay within A's own local index range [0, 4].
    for i in range(len(a_rows)):
        kind = a_rows["touch_kind"][i]
        assert kind in ("time", None)
        idx = a_rows["touch_idx"][i]
        if idx is not None:
            assert idx <= 4

    # B's first row touch_idx must be a local (small) index, not offset by
    # A's length (which would indicate leakage/global indexing).
    b_touch_idx = b_rows["touch_idx"][0]
    if b_touch_idx is not None:
        assert b_touch_idx <= 4


# ---------------------------------------------------------------------------
# Determinism
# ---------------------------------------------------------------------------


def test_forward_returns_deterministic():
    df = _two_symbol_df()
    out1 = forward_returns(df, horizons=(1, 2), price="close", by="symbol")
    out2 = forward_returns(df, horizons=(1, 2), price="close", by="symbol")
    assert out1.equals(out2)


def test_triple_barrier_deterministic():
    df = _two_symbol_df()
    out1 = triple_barrier(df, price="close", pt=0.05, sl=0.05, max_holding=3, by="symbol")
    out2 = triple_barrier(df, price="close", pt=0.05, sl=0.05, max_holding=3, by="symbol")
    assert out1.equals(out2)
