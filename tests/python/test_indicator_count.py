"""Indicator count single source of truth (quantwave-9gek.3)."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
import sys

sys.path.insert(0, str(ROOT / "quantwave-python" / "python"))
import quantwave as qw  # noqa: E402
METADATA = ROOT / "metadata_export.json"


def canonical_count() -> int:
    data = json.loads(METADATA.read_text(encoding="utf-8"))
    return len(data)


def test_indicators_matches_metadata_export() -> None:
    count = canonical_count()
    names = qw.indicators()
    assert len(names) == count, f"expected {count}, got {len(names)}"
    assert not any(n.endswith("Result") for n in names)
    assert not any(n.endswith("Protocol") for n in names)