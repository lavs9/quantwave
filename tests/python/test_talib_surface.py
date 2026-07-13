"""Classic quantwave.talib array API delegates faithfully to the Polars .ta surface.

The .ta plugins are parity-tested against talib-rs in Rust, so verifying that the
talib wrappers reproduce the .ta output exactly is sufficient for correctness.
"""

import numpy as np
import polars as pl
import pytest

import quantwave  # noqa: F401 - registers pl.col().ta
from quantwave import talib


@pytest.fixture
def ohlcv():
    rng = np.random.RandomState(7)
    close = np.cumsum(rng.randn(120)) + 100.0
    high = close + rng.rand(120)
    low = close - rng.rand(120)
    open_ = close + rng.randn(120) * 0.2
    volume = np.abs(rng.randn(120)) * 1e5 + 1e4
    return open_, high, low, close, volume


def test_surface_is_complete():
    fns = talib.list_functions()
    assert fns == sorted(fns)
    assert len(fns) >= 100
    for name in ("RSI", "MACD", "SMA", "EMA", "ATR", "ADX", "BBANDS", "STOCH", "CDLDOJI"):
        assert name in fns, f"{name} missing from talib surface"
        assert callable(getattr(talib, name))


def test_single_input_matches_ta(ohlcv):
    _, _, _, close, _ = ohlcv
    got = talib.RSI(close, timeperiod=14)
    want = pl.DataFrame({"c": close}).select(pl.col("c").ta.rsi(timeperiod=14)).to_series().to_numpy()
    assert np.allclose(got, want, equal_nan=True)


def test_ohlc_input_order_matches_ta(ohlcv):
    _, high, low, close, _ = ohlcv
    got = talib.ATR(high, low, close, timeperiod=14)
    want = (
        pl.DataFrame({"h": high, "l": low, "c": close})
        .select(pl.col("c").ta.atr("h", "l", timeperiod=14))
        .to_series()
        .to_numpy()
    )
    assert np.allclose(got, want, equal_nan=True)


def test_volume_input_position(ohlcv):
    _, high, low, close, volume = ohlcv
    # AD is classic talib order (high, low, close, volume); self=close, volume is an extra col.
    got = talib.AD(high, low, close, volume)
    assert got.shape == close.shape
    assert np.isfinite(got[-1])


def test_multi_output_returns_tuple(ohlcv):
    _, _, _, close, _ = ohlcv
    out = talib.MACD(close)
    assert isinstance(out, tuple) and len(out) == 3
    macd, signal, hist = out
    assert macd.shape == signal.shape == hist.shape == close.shape


def test_candlestick_four_inputs(ohlcv):
    open_, high, low, close, _ = ohlcv
    doji = talib.CDLDOJI(open_, high, low, close)
    assert doji.shape == close.shape


def test_wrong_arity_raises(ohlcv):
    _, _, _, close, _ = ohlcv
    with pytest.raises(TypeError):
        talib.ATR(close)  # ATR needs 3 arrays


def test_bare_call_uses_defaults(ohlcv):
    _, _, _, close, _ = ohlcv
    # STDDEV requires period+nbdev in the .ta method; the wrapper fills talib defaults.
    out = talib.STDDEV(close)
    assert out.shape == close.shape
