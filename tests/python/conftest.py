"""Pytest path setup for root-level Python integration tests."""

from __future__ import annotations

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
for sub in ("tests/python", "quantwave-python/python"):
    p = str(ROOT / sub)
    if p not in sys.path:
        sys.path.insert(0, p)