"""Benchmark harness self-tests (quantwave-9gek.2)."""

from __future__ import annotations

from benchmarks.data import OhlcvConfig, frame_hash, generate_ohlcv


def test_deterministic_frame_hash() -> None:
    a = generate_ohlcv(OhlcvConfig(rows=10_000, seed=42))
    b = generate_ohlcv(OhlcvConfig(rows=10_000, seed=42))
    c = generate_ohlcv(OhlcvConfig(rows=10_000, seed=43))
    assert frame_hash(a) == frame_hash(b)
    assert frame_hash(a) != frame_hash(c)


def test_ohlcv_columns() -> None:
    df = generate_ohlcv(OhlcvConfig(rows=100, seed=1))
    assert set(df.columns) >= {"open", "high", "low", "close", "volume", "symbol"}
    assert len(df) == 100