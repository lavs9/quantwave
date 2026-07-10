"""TA-Lib name map coverage (quantwave-5ipk.7)."""

from __future__ import annotations

import json
from pathlib import Path

from quantwave._talib_map_generated import TALIB_MAP_COUNT, TALIB_SLUG_TO_NAME

ROOT = Path(__file__).resolve().parents[2]
HARVEST = ROOT / "docs" / "generated" / "talib_map.json"


def test_talib_map_covers_parity_harvest():
    data = json.loads(HARVEST.read_text(encoding="utf-8"))
    expected = set(data["map"])
    assert set(TALIB_SLUG_TO_NAME) == expected
    assert TALIB_MAP_COUNT == len(expected)
    assert len(expected) >= 100


def test_talib_list_functions_nonempty():
    from quantwave import talib

    names = talib.list_functions()
    assert len(names) >= 30
    assert "RSI" in names or "CDLDOJI" in names