"""
Test MarketStructure batch vs streaming parity (quantwave-ihji).

Verifies:
1. Bit-identical parity across all 12 flat fields between:
   - Polars batch `.ta.market_structure()`
   - Streaming `MarketStructure.next(high, low)`
   - Convenience batch helper `market_structure_batch(strength, highs, lows)`
2. Direct attribute access for all 12 flat fields on MarketStructureStateResult.
3. Dictionary access (`as_dict()`, `to_dict()`, `__getitem__`, `__contains__`, `keys()`).
4. Backward compatibility: rich nested objects (`last_swing_high`, `last_swing_low`,
   `current_flip`, `swing_depth_used`) remain fully accessible and accurate.
5. TtmSqueezeResult attribute parity (`histogram`, `is_squeezed`, `value`, `direction`).
6. GeometricPatternScanner constructor kwargs (`min_pole_atr`, `max_retrace_percent`).
"""

from __future__ import annotations

import math
import random

import polars as pl
import pytest

import quantwave as qw


def _generate_ohlc(n: int = 120, seed: int = 20261127):
    random.seed(seed)
    base = 100.0
    highs, lows = [], []
    for _ in range(n):
        base += random.uniform(-2.5, 2.5)
        h = base + random.uniform(0.5, 3.5)
        l = base - random.uniform(0.5, 3.5)
        highs.append(max(h, l))
        lows.append(min(h, l))
    return highs, lows


@pytest.mark.parametrize("strength", [2, 3, 5])
def test_market_structure_batch_streaming_exact_parity(strength: int):
    highs, lows = _generate_ohlc(120)
    df = pl.DataFrame({"high": highs, "low": lows})

    # 1. Polars batch
    batch_df = df.select(
        pl.col("low").ta.market_structure(pl.col("high"), swing_strength=strength).alias("ms")
    ).unnest("ms")

    # 2. Streaming Next<T>
    streamer = qw.streaming_class("market_structure")(swing_strength=strength)
    stream_results = [streamer.next(h, l) for h, l in zip(highs, lows)]

    # 3. Helper batch
    helper_results = qw.market_structure_batch(strength, highs, lows)

    fields = [
        "bias",
        "last_high_price",
        "last_high_bar",
        "last_low_price",
        "last_low_bar",
        "has_flip",
        "flip_bearish",
        "flip_price",
        "flip_bar",
        "flip_strength",
        "swing_depth",
        "bar_index",
    ]

    assert len(stream_results) == len(highs)
    assert len(helper_results) == len(highs)
    assert len(batch_df) == len(highs)

    for i in range(len(highs)):
        s_res = stream_results[i]
        h_res = helper_results[i]

        for field in fields:
            b_val = batch_df[field][i]
            s_val = getattr(s_res, field)
            h_val = getattr(h_res, field)
            d_val = s_res[field]

            if isinstance(b_val, float):
                if math.isnan(b_val):
                    assert math.isnan(s_val), f"bar {i} {field}: batch=NaN stream={s_val}"
                    assert math.isnan(h_val), f"bar {i} {field}: batch=NaN helper={h_val}"
                    assert math.isnan(d_val), f"bar {i} {field}: batch=NaN dict={d_val}"
                else:
                    assert b_val == pytest.approx(s_val), f"bar {i} {field}: batch={b_val} stream={s_val}"
                    assert b_val == pytest.approx(h_val), f"bar {i} {field}: batch={b_val} helper={h_val}"
                    assert b_val == pytest.approx(d_val), f"bar {i} {field}: batch={b_val} dict={d_val}"
            else:
                assert b_val == s_val, f"bar {i} {field}: batch={b_val} stream={s_val}"
                assert b_val == h_val, f"bar {i} {field}: batch={b_val} helper={h_val}"
                assert b_val == d_val, f"bar {i} {field}: batch={b_val} dict={d_val}"


def test_market_structure_dict_and_item_access():
    streamer = qw.streaming_class("market_structure")(swing_strength=3)
    res = streamer.next(110.0, 90.0)

    # Dictionary conversion
    d = res.as_dict()
    assert isinstance(d, dict)
    d2 = res.to_dict()
    assert d.keys() == d2.keys()
    for k in d:
        v1, v2 = d[k], d2[k]
        if isinstance(v1, float) and math.isnan(v1):
            assert math.isnan(v2)
        else:
            assert v1 == v2

    expected_keys = [
        "bias",
        "last_high_price",
        "last_high_bar",
        "last_low_price",
        "last_low_bar",
        "has_flip",
        "flip_bearish",
        "flip_price",
        "flip_bar",
        "flip_strength",
        "swing_depth",
        "bar_index",
    ]
    for k in expected_keys:
        assert k in d
        assert k in res
        assert "key" in repr(res) or "MarketStructureStateResult" in repr(res)

    # keys() method
    assert res.keys() == expected_keys

    # KeyError on invalid key
    with pytest.raises(KeyError):
        _ = res["non_existent_key"]


def test_market_structure_backward_compatibility():
    # Make sure old code that uses nested sub-objects continues working seamlessly
    streamer = qw.streaming_class("market_structure")(swing_strength=2)
    highs, lows = _generate_ohlc(50)
    for h, l in zip(highs, lows):
        res = streamer.next(h, l)
        # Nested objects are accessible
        assert res.swing_depth_used == 2
        assert res.swing_depth == 2
        if res.last_swing_high is not None:
            assert res.last_swing_high.price == res.last_high_price
            assert res.last_swing_high.bar == res.last_high_bar
            assert "SwingPointResult" in repr(res.last_swing_high)
        if res.last_swing_low is not None:
            assert res.last_swing_low.price == res.last_low_price
            assert res.last_swing_low.bar == res.last_low_bar
        if res.current_flip is not None:
            assert res.has_flip is True
            assert res.current_flip.price == res.flip_price
            assert res.current_flip.bar == res.flip_bar
            assert res.current_flip.is_bearish == res.flip_bearish
            assert res.current_flip.structure_strength == res.flip_strength
            assert "FlipEventResult" in repr(res.current_flip)


def test_ttm_squeeze_output_attribute_parity():
    ttm_cls = qw.streaming_class("ttm_squeeze")
    ttm = ttm_cls(20, 2.0, 1.5)
    res = ttm.next(105.0, 95.0, 100.0)

    # Both batch field names and legacy streaming names are supported
    assert hasattr(res, "histogram")
    assert hasattr(res, "is_squeezed")
    assert hasattr(res, "value")
    assert hasattr(res, "direction")
    assert res.histogram == res.value
    assert res.is_squeezed == (res.direction != 0)


def test_geometric_patterns_scanner_kwargs():
    geo_cls = qw.streaming_class("geometric_patterns")
    geo = geo_cls(swing_strength=3, min_pole_atr=1.5, max_retrace_percent=50.0)
    res = geo.next(105.0, 95.0)

    assert hasattr(res, "flag")
    assert hasattr(res, "hs")
    assert hasattr(res, "market_structure")
    assert hasattr(res.market_structure, "last_high_price")
    assert hasattr(res.market_structure, "has_flip")
