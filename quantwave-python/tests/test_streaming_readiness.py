"""Tests for streaming readiness API (quantwave-h6xe)."""

import quantwave as qw


def test_track_streaming_explicit_warmup():
    cls = qw.streaming_class("rsi")
    wrapped = qw.track_streaming(cls(14), warmup_bars_count=3)
    for i in range(1, 4):
        wrapped.next(100.0 + i)
        assert wrapped.bars_consumed == i
        assert wrapped.is_ready == (i >= 3)
    assert wrapped.is_ready


def test_wrap_streaming_by_name():
    cls = qw.streaming_class("rsi")
    wrapped = qw.wrap_streaming(cls(14), name="rsi")
    for _ in range(13):
        wrapped.next(100.0)
        assert not wrapped.is_ready
    wrapped.next(100.0)
    assert wrapped.is_ready