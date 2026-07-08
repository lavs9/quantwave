"""Benchmark harness self-tests (quantwave-9gek.2)."""

from __future__ import annotations

import json
from pathlib import Path

from benchmarks.data import OhlcvConfig, frame_hash, generate_ohlcv
from benchmarks.python_comparisons import correctness_precheck

ROOT = Path(__file__).resolve().parents[2]
RESULTS = ROOT / "benchmarks" / "results" / "latest.json"


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


def test_correctness_precheck_runs() -> None:
    df = generate_ohlcv(OhlcvConfig(rows=2_000, seed=7))
    result = correctness_precheck(df)
    assert result["passed"] is True
    assert "checks" in result


def test_latency_only_when_instrumented() -> None:
    if not RESULTS.exists():
        return
    data = json.loads(RESULTS.read_text(encoding="utf-8"))
    lat = data.get("latency", {})
    if lat and "source" in lat:
        assert lat["source"] == "per_tick_instrumented"
        assert "sma_20_mean_ns" in lat