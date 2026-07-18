"""Alternative bar construction — Renko (quantwave-p2k0.9, slice 1)."""

import polars as pl
import pytest

import quantwave as qw


def test_renko_fixed_box_uptrend():
    b = qw.bars.renko([10.0, 10.5, 11.0, 12.4, 13.0], box_size=1.0)
    assert b.columns == ["open", "close", "direction"]
    assert b.height == 3
    assert b["direction"].to_list() == [1, 1, 1]
    assert b["open"].to_list() == [10.0, 11.0, 12.0]
    assert b["close"].to_list() == [11.0, 12.0, 13.0]


def test_renko_reversal_non_retracing():
    # Drozda Def. 1: reversal bricks are adjacent below the band, not retracing
    # the last up-brick — up to 12 gives [10,11],[11,12]; reversal to 9 gives
    # [11,10],[10,9] → dirs [1,1,-1,-1].
    b = qw.bars.renko([10.0, 12.0, 11.5, 10.0, 9.0], box_size=1.0)
    assert b["direction"].to_list() == [1, 1, -1, -1]
    assert b["close"].to_list()[-1] == 9.0


def test_renko_no_brick_within_box():
    b = qw.bars.renko([10.0, 10.9, 10.1, 10.99], box_size=1.0)
    assert b.height == 0


def test_renko_span_invariant_on_frame():
    df = qw.datasets.synthetic(seed=3, rows=300)
    b = qw.bars.renko(df, box_size=2.0)
    spans = (b["close"] - b["open"]).abs().round(6)
    assert spans.n_unique() == 1 and spans[0] == pytest.approx(2.0)
    assert set(b["direction"].unique().to_list()) <= {-1, 1}


def test_renko_from_dataframe_uses_close():
    df = pl.DataFrame({"close": [10.0, 11.0, 12.0]})
    assert qw.bars.renko(df, box_size=1.0).height == 2


def test_renko_atr_box():
    df = qw.datasets.synthetic(seed=5, rows=200)
    b = qw.bars.renko(df, box_size="atr", atr_period=14, multiplier=1.0)
    assert b.height > 0
    spans = (b["close"] - b["open"]).abs().round(6)
    assert spans.n_unique() == 1  # single fixed ATR-derived box


def test_renko_invalid_box_raises():
    with pytest.raises(Exception):
        qw.bars.renko([10.0, 11.0], box_size=0.0)
    with pytest.raises(ValueError):
        qw.bars.renko([10.0, 11.0], box_size="bogus")


def test_range_bars_single_bar():
    b = qw.bars.range_bars([10.0, 10.5, 11.0, 12.0], range_size=2.0)
    assert b.columns == ["open", "high", "low", "close"]
    assert b.height == 1
    assert b.row(0) == (10.0, 12.0, 10.0, 12.0)


def test_range_bars_span_both_directions():
    b = qw.bars.range_bars([10.0, 11.0, 9.0], range_size=2.0)
    assert b.height == 1
    assert b["high"][0] == 11.0 and b["low"][0] == 9.0 and b["close"][0] == 9.0


def test_range_bars_no_bar_within_range():
    assert qw.bars.range_bars([10.0, 10.9, 10.1, 10.5], range_size=2.0).height == 0


def test_range_bars_span_invariant():
    df = qw.datasets.synthetic(seed=4, rows=300)
    b = qw.bars.range_bars(df, range_size=3.0)
    spans = b["high"] - b["low"]
    assert (spans >= 3.0 - 1e-9).all()


def test_range_bars_atr_and_errors():
    df = qw.datasets.synthetic(seed=6, rows=200)
    assert qw.bars.range_bars(df, range_size="atr", multiplier=2.0).height > 0
    with pytest.raises(Exception):
        qw.bars.range_bars([10.0, 11.0], range_size=0.0)
    with pytest.raises(ValueError):
        qw.bars.range_bars([10.0, 11.0], range_size="bogus")
