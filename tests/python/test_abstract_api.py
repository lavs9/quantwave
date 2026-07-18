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


# --- schema completeness (quantwave-n4w8) -----------------------------------
# Regression guards for the introspection gap found by live verification: a
# generic caller (df.ta.all(), screeners) reads Function.parameters and
# input_names to drive every indicator, so a None default or an undercounted
# input silently crashes or mis-feeds the call. These assert the whole
# batch-capable set, not a happy-path sample.


def _batch_capable():
    for name in qw.get_functions():
        f = Function(name)
        if f._method is not None:  # has a .ta batch implementation
            yield name, f


def test_no_batch_capable_function_has_none_param_default():
    offenders = {
        name: [k for k, v in f.parameters.items() if v is None]
        for name, f in _batch_capable()
    }
    offenders = {k: v for k, v in offenders.items() if v}
    assert offenders == {}, f"None param defaults would crash df.ta.all(): {offenders}"


@pytest.mark.parametrize("name", ["beta", "correl"])
def test_multiseries_functions_declare_both_inputs(name):
    # beta/correl correlate two series; the schema must not undercount to one.
    assert len(Function(name).input_names) == 2


def test_bbands_default_tracks_the_callable_not_the_registry():
    # Guards the design rule: introspection reports the .ta callable's own
    # default (TA-Lib's 5), not the curated registry's textbook 20.
    assert Function("bbands").parameters["timeperiod"] == 5


def test_batch_capable_functions_callable_with_their_defaults(ohlcv):
    # Every batch-capable indicator must actually run when driven by exactly the
    # inputs + defaults it advertises — the end-to-end contract df.ta.all() relies
    # on. Feed each function by its own declared input_names (roles like 'in1' /
    # 'other' for multi-series functions get a valid price series too).
    open_, high, low, close, volume = ohlcv
    named = {"open": open_, "high": high, "low": low, "close": close, "volume": volume}
    failures = {}
    for name, f in _batch_capable():
        frame = {iname: named.get(iname, close) for iname in f.input_names}
        try:
            f(frame, **{k: v for k, v in f.parameters.items() if v is not None})
        except Exception as exc:  # noqa: BLE001 — collect all, report together
            failures[name] = f"{type(exc).__name__}: {exc}"
    assert failures == {}, f"batch-capable indicators failed on default call: {failures}"


def test_output_names_arity_matches_actual_ta_output():
    """output_names must match the real .ta output arity (quantwave-jni1): a
    consumer reading output_names to unpack columns breaks if metadata over- or
    under-declares outputs. Exercises every batch-capable indicator."""
    import numpy as np

    n = 80
    frame = {
        "open": np.arange(n, dtype=float) + 1,
        "high": np.arange(n, dtype=float) + 2,
        "low": np.arange(n, dtype=float),
        "close": np.arange(n, dtype=float) + 1,
        "volume": np.abs(np.arange(n, dtype=float)) + 1e3,
    }
    mism = {}
    for name, f in _batch_capable():
        got_frame = {iname: frame.get(iname, frame["close"]) for iname in f.input_names}
        try:
            out = f(got_frame, **{k: v for k, v in f.parameters.items() if v is not None})
        except Exception:
            continue
        actual = len(out) if isinstance(out, tuple) else 1
        if actual != len(f.output_names):
            mism[name] = (len(f.output_names), actual)
    assert mism == {}, f"output_names arity != actual .ta output: {mism}"
