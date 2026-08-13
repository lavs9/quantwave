"""Benchmark harness self-tests (quantwave-9gek.2)."""

from __future__ import annotations

import json
from pathlib import Path

import pytest

import benchmarks.data
from benchmarks.data import OhlcvConfig, frame_hash, generate_ohlcv
from benchmarks.python_comparisons import correctness_precheck

ROOT = Path(__file__).resolve().parents[2]
RESULTS = ROOT / "benchmarks" / "results" / "latest.json"


# Pinned digests for the synthetic generator. These are the whole point of
# quantwave-5yjg: frame_hash exists to prove two benchmark runs measured the
# same data, so the generator must be reproducible across environments, not
# merely within one process. Comparing two frames built back-to-back (as this
# file used to) passes on any numpy build and therefore proves nothing.
#
# If one of these fails, the data stream changed. That invalidates comparisons
# against every previously published benchmark number -- treat it as a
# deliberate baseline reset, not a test to re-bless.
GOLDEN_FRAME_HASHES = {
    (10_000, 42): "ac483b7c2835037ca7616b63da3ba160df66ed5e685fce9fccaf9e6479fddb0b",
    (10_000, 43): "80e928722501bd56e69f32f9814e94c08b49f5436417d3d95701c81902375285",
    (2_000, 7): "0f3a1b2808db35724020506311a4ffa4eb621e78d8307522b9f7493c1e79473a",
    (100, 1): "ced9d39321966a0159f64f01a31dc10c0a49323552f9195f4258941c3c9c0ac6",
}


@pytest.mark.parametrize(("rows", "seed"), sorted(GOLDEN_FRAME_HASHES))
def test_frame_hash_matches_pinned_golden(rows: int, seed: int) -> None:
    """The generator is reproducible across numpy/polars versions, not just runs."""
    digest = frame_hash(generate_ohlcv(OhlcvConfig(rows=rows, seed=seed)))
    assert digest == GOLDEN_FRAME_HASHES[(rows, seed)], (
        f"synthetic data stream changed for rows={rows} seed={seed}: "
        f"expected {GOLDEN_FRAME_HASHES[(rows, seed)]}, got {digest}. "
        "Published benchmark numbers are no longer comparable to new ones."
    )


def test_generator_does_not_use_numpy_random() -> None:
    """Guard the actual root cause of quantwave-5yjg.

    numpy's own policy (NEP 19) allows ``Generator`` streams to change between
    feature releases, so any reintroduction of ``np.random`` silently makes the
    data environment-dependent again. The golden hashes above would catch it,
    but only on a machine whose numpy happens to differ -- this catches it
    everywhere.
    """
    source = Path(benchmarks.data.__file__).read_text(encoding="utf-8")
    code = "\n".join(
        line for line in source.splitlines() if not line.strip().startswith("#")
    )
    _, _, body = code.partition('"""')
    _, _, body = body.partition('"""')  # skip the module docstring, which discusses it
    assert "np.random" not in body and "numpy.random" not in body, (
        "benchmarks/data.py must not draw from numpy.random -- Generator streams "
        "are not stable across numpy versions (NEP 19)"
    )


def test_frame_values_are_independent_of_requested_row_count() -> None:
    """Row ``i`` must depend only on (seed, column, i).

    A shared sequential stream would renumber every value when the row count
    changes, so a 100k smoke run and a 1M nightly run would not be prefixes of
    one another and could not be compared at all.
    """
    small = generate_ohlcv(OhlcvConfig(rows=100, seed=7))
    large = generate_ohlcv(OhlcvConfig(rows=5_000, seed=7))
    for col in ("open", "high", "low", "close", "volume"):
        assert small[col].to_list() == large[col].to_list()[:100], f"{col} shifted"
    assert small["symbol"].to_list() == large["symbol"].to_list()[:100]


def test_deterministic_frame_hash() -> None:
    a = generate_ohlcv(OhlcvConfig(rows=10_000, seed=42))
    b = generate_ohlcv(OhlcvConfig(rows=10_000, seed=42))
    c = generate_ohlcv(OhlcvConfig(rows=10_000, seed=43))
    assert frame_hash(a) == frame_hash(b)
    assert frame_hash(a) != frame_hash(c)


def test_frame_hash_distinguishes_column_identity() -> None:
    """Renaming or reordering columns must change the digest.

    Hashing only concatenated value bytes let a retyped or reordered frame
    collide with the original, which would defeat the "same data" claim.
    """
    df = generate_ohlcv(OhlcvConfig(rows=500, seed=3))
    swapped = df.rename({"open": "close", "close": "open"}).select(df.columns)
    assert frame_hash(df) != frame_hash(swapped)


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