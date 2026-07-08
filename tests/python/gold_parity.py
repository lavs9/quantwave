"""Helpers for gold-standard Python parity tests."""

from __future__ import annotations

import json
import math
from pathlib import Path
from typing import Any

import pytest

from gold_parity_registry import GoldParityCase

ROOT = Path(__file__).resolve().parents[2]
GOLD_DIR = ROOT / "quantwave-core" / "tests" / "gold_standard"


def load_fixture(stem: str) -> dict[str, Any]:
    path = GOLD_DIR / f"{stem}.json"
    if not path.exists():
        pytest.skip(f"gold fixture missing: {path.name}")
    return json.loads(path.read_text(encoding="utf-8"))


def _is_nan(x: float) -> bool:
    return isinstance(x, float) and math.isnan(x)


def assert_close(actual: float, expected: float | None, *, rtol: float, idx: int) -> None:
    if expected is None:
        assert _is_nan(actual), f"expected NaN at index {idx}, got {actual}"
        return
    if _is_nan(actual):
        pytest.fail(f"expected {expected} at index {idx}, got NaN")
    assert actual == pytest.approx(expected, rel=rtol, abs=rtol)


def _make_streaming(case: GoldParityCase):
    import quantwave as qw

    cls = qw.streaming_class(case.streaming_name)
    if case.args is not None:
        return cls(*case.args)
    if case.kwargs is not None:
        return cls(**case.kwargs)
    return cls()


def run_streaming_parity(case: GoldParityCase) -> None:
    data = load_fixture(case.fixture)
    inst = _make_streaming(case)
    rtol = case.rtol

    if case.kind == "scalar":
        for i, x in enumerate(data["input"]):
            assert_close(inst.next(float(x)), _opt(data["expected"][i]), rtol=rtol, idx=i)
        return

    if case.kind == "vec":
        for i, x in enumerate(data["input"]):
            got = inst.next(float(x))
            exp_row = data["expected"][i]
            assert len(got) == len(exp_row)
            for j, (a, e) in enumerate(zip(got, exp_row)):
                assert_close(float(a), _opt(e), rtol=rtol, idx=i * 1000 + j)
        return

    if case.kind == "close_series":
        exp_key = case.field or "expected"
        for i, x in enumerate(data["close"]):
            assert_close(float(inst.next(float(x))), float(data[exp_key][i]), rtol=rtol, idx=i)
        return

    if case.kind == "cycle_trend":
        for i, x in enumerate(data["input"]):
            res = inst.next(float(x))
            exp_row = data["expected"][i]
            assert_close(float(res.cycle), _opt(exp_row[0]), rtol=rtol, idx=i)
            assert_close(float(res.trend), _opt(exp_row[1]), rtol=rtol, idx=i + 10_000)
        return

    if case.kind == "tuple":
        for i, x in enumerate(data["input"]):
            res = inst.next(float(x))
            exp0, exp1 = data["expected"][i]
            assert_close(float(res.filt), _opt(exp0), rtol=rtol, idx=i)
            assert_close(float(res.voss), _opt(exp1), rtol=rtol, idx=i)
        return

    if case.kind == "loops":
        for i, pair in enumerate(data["input"]):
            p, v = pair
            res = inst.next(float(p), float(v))
            exp0, exp1 = data["expected"][i]
            assert_close(float(res.price_rms), _opt(exp0), rtol=rtol, idx=i)
            assert_close(float(res.vol_rms), _opt(exp1), rtol=rtol, idx=i)
        return

    if case.kind == "oc":
        for i, pair in enumerate(data["input"]):
            o, c = pair
            assert_close(float(inst.next(float(o), float(c))), _opt(data["expected"][i]), rtol=rtol, idx=i)
        return

    if case.kind == "donchian":
        for i in range(len(data["highs"])):
            res = inst.next(float(data["highs"][i]), float(data["lows"][i]))
            assert_close(float(res.middle), float(data["expected_middle"][i]), rtol=rtol, idx=i)
        return

    if case.kind == "keltner":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]), float(data["close"][i]))
            assert_close(float(res.middle), float(data["expected_middle"][i]), rtol=rtol, idx=i)
        return

    if case.kind == "atr_ts":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]), float(data["close"][i]))
            assert_close(float(res.stop), float(data["expected_stop"][i]), rtol=rtol, idx=i)
            assert int(res.direction) == int(data["expected_dir"][i])
        return

    if case.kind == "vortex":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]), float(data["close"][i]))
            assert_close(float(res.plus), float(data["expected_plus"][i]), rtol=rtol, idx=i)
            assert_close(float(res.minus), float(data["expected_minus"][i]), rtol=rtol, idx=i)
        return

    if case.kind == "ttm_squeeze":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]), float(data["close"][i]))
            assert_close(float(res.value), float(data["expected_histogram"][i]), rtol=rtol, idx=i)
            assert bool(res.direction) == bool(data["expected_squeezed"][i])
        return

    if case.kind == "wavetrend":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]), float(data["close"][i]))
            assert_close(float(res.wt1), float(data["expected_wt1"][i]), rtol=rtol, idx=i)
            assert_close(float(res.wt2), float(data["expected_wt2"][i]), rtol=rtol, idx=i)
        return

    if case.kind == "supertrend":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]), float(data["close"][i]))
            assert_close(float(res.value), float(data["expected_st"][i]), rtol=rtol, idx=i)
            assert int(res.direction) == int(data["expected_dir"][i])
        return

    if case.kind == "heikin_ashi":
        for i in range(len(data["open"])):
            res = inst.next(
                float(data["open"][i]),
                float(data["high"][i]),
                float(data["low"][i]),
                float(data["close"][i]),
            )
            assert_close(float(res.open), float(data["expected_open"][i]), rtol=rtol, idx=i)
            assert_close(float(res.high), float(data["expected_high"][i]), rtol=rtol, idx=i)
            assert_close(float(res.low), float(data["expected_low"][i]), rtol=rtol, idx=i)
            assert_close(float(res.close), float(data["expected_close"][i]), rtol=rtol, idx=i)
        return

    if case.kind == "ichimoku":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]))
            assert_close(float(res.tenkan), float(data["expected_tenkan"][i]), rtol=rtol, idx=i)
            assert_close(float(res.kijun), float(data["expected_kijun"][i]), rtol=rtol, idx=i)
            assert_close(float(res.senkou_a), float(data["expected_senkou_a"][i]), rtol=rtol, idx=i)
            assert_close(float(res.senkou_b), float(data["expected_senkou_b"][i]), rtol=rtol, idx=i)
        return

    if case.kind == "fractals":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]))
            assert bool(res.bearish) == bool(data["expected_bearish"][i])
            assert bool(res.bullish) == bool(data["expected_bullish"][i])
        return

    if case.kind == "pivot_points":
        for i in range(len(data["high"])):
            res = inst.next(float(data["high"][i]), float(data["low"][i]), float(data["close"][i]))
            assert_close(float(res.p), float(data["expected_p"][i]), rtol=rtol, idx=i)
            assert_close(float(res.r1), float(data["expected_r1"][i]), rtol=rtol, idx=i)
            assert_close(float(res.s1), float(data["expected_s1"][i]), rtol=rtol, idx=i)
        return

    pytest.fail(f"unknown gold parity kind: {case.kind}")


def _opt(value: Any) -> float | None:
    if value is None:
        return None
    return float(value)