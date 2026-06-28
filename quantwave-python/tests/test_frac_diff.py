"""Fractional differentiation (quantwave-wnd9)."""

import pytest

import quantwave as qw


def test_frac_diff_streaming():
    fd = qw.FracDiff(0.4, 0.05)
    out = [fd.next(float(x)) for x in [100.0, 101.0, 102.0, 101.5, 103.0]]
    assert out[0] != out[0]  # NaN
    assert out[-1] == pytest.approx(43.696, rel=1e-3)


def test_frac_diff_batch_vector():
    closes = [100.0, 101.0, 102.0, 101.5, 103.0, 104.0]
    out = qw.fracdiff(0.4, 0.05, closes)
    assert len(out) == len(closes)
    assert out[-1] == pytest.approx(44.092, rel=1e-3)


def test_frac_diff_metadata():
    meta = qw.metadata("frac_diff")
    assert meta is not None
    assert "d" in meta.optional_params or "d" in meta.required_params