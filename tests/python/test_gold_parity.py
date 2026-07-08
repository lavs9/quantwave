"""Gold-standard numerical parity for Python streaming API (quantwave-5ipk.2)."""

from __future__ import annotations

from pathlib import Path

import pytest

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tests" / "python"))
sys.path.insert(0, str(ROOT / "quantwave-python" / "python"))

from gold_parity import GOLD_DIR, run_streaming_parity  # noqa: E402
from gold_parity_registry import (  # noqa: E402
    GOLD_PARITY_CASES,
    GOLD_PARITY_DEFERRED,
)

@pytest.mark.parametrize(
    "case",
    GOLD_PARITY_CASES,
    ids=[c.fixture for c in GOLD_PARITY_CASES],
)
def test_streaming_matches_gold_vector(case) -> None:
    run_streaming_parity(case)


def test_gold_fixture_inventory() -> None:
    """Every on-disk gold JSON is either tested or explicitly deferred."""
    on_disk = {p.stem for p in GOLD_DIR.glob("*.json")}
    covered = {c.fixture for c in GOLD_PARITY_CASES}
    deferred = {stem for stem, _ in GOLD_PARITY_DEFERRED}
    missing = on_disk - covered - deferred
    assert not missing, f"gold fixtures without parity plan: {sorted(missing)}"
    assert len(covered) >= 25, f"expected ≥25 streaming cases, got {len(covered)}"


def test_deferred_fixtures_documented() -> None:
    for stem, reason in GOLD_PARITY_DEFERRED:
        assert (GOLD_DIR / f"{stem}.json").exists()
        assert reason