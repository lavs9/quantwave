#!/usr/bin/env python3
"""Harvest TA-Lib parity-tested indicator slugs from Rust proptest files (quantwave-5ipk.7)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PARITY_FILES = (
    ROOT / "quantwave-core" / "tests" / "test_all_talib_parity.rs",
    ROOT / "quantwave-core" / "tests" / "test_missing_talib_parity.rs",
)

# test_macdext_parity_auto -> macdext; test_cdldoji_parity_auto -> cdldoji
TEST_FN_RE = re.compile(r"fn\s+test_([a-z0-9_]+)_parity")


def slug_to_talib(slug: str) -> str:
    """Map quantwave slug to TA-Lib uppercase export name."""
    if slug.startswith("cdl"):
        # cdldoji -> CDLDOJI, cdl3inside -> CDL3INSIDE
        return slug.upper()
    # macdext -> MACDEXT, ht_trendline -> HT_TRENDLINE
    return slug.upper()


def harvest() -> dict[str, str]:
    slugs: set[str] = set()
    for path in PARITY_FILES:
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        for m in TEST_FN_RE.finditer(text):
            slug = m.group(1)
            if slug.endswith("_auto"):
                slug = slug[: -len("_auto")]
            slugs.add(slug)
    return {slug: slug_to_talib(slug) for slug in sorted(slugs)}


def main() -> int:
    mapping = harvest()
    out = ROOT / "docs" / "generated" / "talib_map.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    import json

    payload = {
        "source": "harvested from test_all_talib_parity.rs + test_missing_talib_parity.rs",
        "count": len(mapping),
        "map": mapping,
    }
    out.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"harvested {len(mapping)} TA-Lib slugs -> {out}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())