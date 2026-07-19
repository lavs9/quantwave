"""Harmonic pattern detection (quantwave-p2k0.9, slice 4).

Patterns and their Fibonacci-ratio definitions are the work of Scott M. Carney
(Harmonic Trading Vols. 1-2 / HarmonicTrader.com); see qw.patterns for attribution.
"""

import polars as pl
import pytest

import quantwave as qw

STRENGTH = 2
SPACING = 6


def series_from_pivots(pivots, strength=STRENGTH, spacing=SPACING, eps=0.01):
    """OHLC frame whose confirmed swings land on the given (price, is_high) pivots."""
    start = strength + 1
    bars = [start + i * spacing for i in range(len(pivots))]
    n = start + (len(pivots) - 1) * spacing + strength + 2
    highs, lows = [], []
    for i in range(n):
        if i <= bars[0]:
            mid = pivots[0][0]
        elif i >= bars[-1]:
            mid = pivots[-1][0]
        else:
            k = next(j for j, b in enumerate(bars) if b > i)
            b0, b1 = bars[k - 1], bars[k]
            v0, v1 = pivots[k - 1][0], pivots[k][0]
            mid = v0 + (v1 - v0) * (i - b0) / (b1 - b0)
        highs.append(mid + eps)
        lows.append(mid - eps)
    for idx, (price, is_high) in enumerate(pivots):
        b = bars[idx]
        if is_high:
            highs[b], lows[b] = price, price - 2 * eps
        else:
            highs[b], lows[b] = price + 2 * eps, price
    return pl.DataFrame({"high": highs, "low": lows})


def detect(pivots):
    return qw.patterns.harmonic(series_from_pivots(pivots), swing_strength=STRENGTH)


def test_bullish_abcd_exact():
    pats = detect([(110.0, True), (100.0, False), (106.18, True), (96.18, False)])
    assert pats.columns[:3] == ["id", "kind", "is_bull"]
    abcd = pats.filter(pl.col("kind") == "abcd")
    assert abcd.height == 1
    row = abcd.row(0, named=True)
    assert row["is_bull"] is True
    assert row["score"] > 0.9
    assert row["cd_ab"] == pytest.approx(1.0, abs=0.03)
    assert row["bc_ab"] == pytest.approx(0.618, abs=0.03)
    assert row["x_bar"] is None  # AB=CD has no X


def test_alternate_abcd_1_27():
    pats = detect([(110.0, True), (100.0, False), (106.18, True), (93.48, False)])
    alt = pats.filter(pl.col("kind") == "alternate_abcd")
    assert alt.height == 1
    assert alt.row(0, named=True)["cd_ab"] == pytest.approx(1.27, abs=0.04)


def test_bullish_5_0_exact():
    pats = detect(
        [
            (100.0, False),
            (110.0, True),
            (93.82, False),
            (126.18, True),
            (110.0, False),
        ]
    )
    five = pats.filter(pl.col("kind") == "5-0")
    assert five.height == 1
    row = five.row(0, named=True)
    assert row["is_bull"] is True
    assert row["x_bar"] is not None
    assert row["cd_bc"] == pytest.approx(0.5, abs=0.03)
    assert row["score"] > 0.9


def test_perturbed_abcd_rejected():
    # CD = 1.5 x AB is neither AB=CD (1.0) nor Alternate (1.27/1.618) within 10%.
    pats = detect([(110.0, True), (100.0, False), (106.18, True), (91.18, False)])
    assert pats.filter(pl.col("kind").is_in(["abcd", "alternate_abcd"])).height == 0


def test_bad_retrace_rejected():
    # C retraces AB by only 0.2 (< 0.382 gate) -> not an AB=CD candidate.
    pats = detect([(110.0, True), (100.0, False), (102.0, True), (92.0, False)])
    assert pats.filter(pl.col("kind") == "abcd").height == 0


def test_bearish_abcd_detected():
    pats = detect([(90.0, False), (100.0, True), (93.82, False), (103.82, True)])
    abcd = pats.filter(pl.col("kind") == "abcd")
    assert abcd.height == 1
    assert abcd.row(0, named=True)["is_bull"] is False


def test_accepts_high_low_pair_and_attribution():
    df = series_from_pivots(
        [(110.0, True), (100.0, False), (106.18, True), (96.18, False)]
    )
    pats = qw.patterns.harmonic(
        (df["high"].to_list(), df["low"].to_list()), swing_strength=2
    )
    assert pats.height >= 1
    assert "Carney" in qw.patterns.HARMONIC_ATTRIBUTION


def test_errors():
    with pytest.raises(Exception):
        qw.patterns.harmonic(([1.0, 2.0], [1.0]))  # length mismatch
    with pytest.raises(ValueError):
        qw.patterns.harmonic(pl.DataFrame({"high": [1.0], "low": [0.0]}), swing_strength=0)
