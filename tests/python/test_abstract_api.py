"""TA-Lib abstract-API introspection registry (quantwave-p2k0.1).

get_functions / get_function_groups + quantwave.abstract.Function, verified to
dispatch identically to the underlying Polars `.ta` implementation.
"""

import numpy as np
import polars as pl
import pytest

import quantwave as qw
from quantwave.abstract import Function


@pytest.fixture
def ohlcv():
    rng = np.random.RandomState(11)
    close = np.cumsum(rng.randn(150)) + 100.0
    high = close + rng.rand(150)
    low = close - rng.rand(150)
    open_ = close + rng.randn(150) * 0.2
    volume = np.abs(rng.randn(150)) * 1e5 + 1e4
    return open_, high, low, close, volume


def test_get_functions_nonempty_sorted():
    fns = qw.get_functions()
    assert len(fns) >= 200
    assert fns == sorted(fns)
    assert len(fns) == len(set(fns))


def test_groups_union_equals_functions_no_dupes():
    # Acceptance #2: union of get_function_groups() == get_functions(); no duplicates.
    groups = qw.get_function_groups()
    flat = [name for names in groups.values() for name in names]
    assert len(flat) == len(set(flat)), "an indicator appears in >1 group"
    assert set(flat) == set(qw.get_functions())


def test_function_introspection():
    rsi = Function("RSI")
    assert rsi.input_names == ["close"]
    assert rsi.parameters == {"timeperiod": 14}
    assert rsi.output_names == ["rsi"]
    macd = Function("MACD")
    assert macd.output_names == ["macd", "signal", "histogram"]
    assert set(macd.parameters) == {"fast", "slow", "signal"}
    assert "name" in macd.info and "group" in macd.info


@pytest.mark.parametrize("name", ["RSI", "SMA", "ATR", "MACD", "STOCH"])
def test_abstract_parity_with_ta(name, ohlcv):
    # Acceptance #3: abstract.Function(name)(df, **defaults) == direct .ta call.
    open_, high, low, close, volume = ohlcv
    f = Function(name)
    frame = {"open": open_, "high": high, "low": low, "close": close, "volume": volume}
    inputs = {k: frame[k] for k in f.input_names}
    got = f(inputs)

    # Build the equivalent direct .ta call (map public input names -> internal roles).
    spec = f._spec
    df = pl.DataFrame(
        {role: frame[iname] for iname, role in zip(f.input_names, spec.input_roles)}
    )
    method = getattr(pl.col(spec.self_role).ta, f._method)
    call_kwargs = {k: v for k, v in f.parameters.items() if v is not None}
    expr = method(*spec.extra_roles, **call_kwargs)
    out = df.select(expr.alias("_out"))
    if out.to_series().dtype == pl.Struct:
        want = tuple(
            out.unnest("_out")[fld].to_numpy() for fld in out.to_series().struct.fields
        )
        assert isinstance(got, tuple) and len(got) == len(want)
        for g, w in zip(got, want):
            assert np.allclose(g, w, equal_nan=True)
    else:
        assert np.allclose(got, out.to_series().to_numpy(), equal_nan=True)


def test_positional_and_dict_inputs_agree(ohlcv):
    _, high, low, close, _ = ohlcv
    f = Function("ATR")
    via_pos = f(high, low, close, timeperiod=14)  # canonical H,L,C order
    via_dict = f({"high": high, "low": low, "close": close}, timeperiod=14)
    assert np.allclose(via_pos, via_dict, equal_nan=True)


def test_missing_input_column_raises(ohlcv):
    _, _, _, close, _ = ohlcv
    with pytest.raises(KeyError):
        Function("ATR")({"close": close})  # missing high, low
