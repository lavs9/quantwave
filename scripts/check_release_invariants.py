#!/usr/bin/env python3
"""Assert release.yml enforces pre-publish artifact gates (quantwave-ruh0.4)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
RELEASE = ROOT / ".github" / "workflows" / "release.yml"

REQUIRED_SNIPPETS = [
    "verify_wheel_tags.py",
    "wheel_smoke_test.py",
    "pypa/gh-action-pypi-publish",
    "verify-python-wheel",
    "publish-python",
    "needs: [build-python-wheels, verify-python-wheel]",
    "id-token: write",
]


def main() -> int:
    if not RELEASE.exists():
        print(f"check_release_invariants: missing {RELEASE}", file=sys.stderr)
        return 1
    text = RELEASE.read_text(encoding="utf-8")
    missing = [s for s in REQUIRED_SNIPPETS if s not in text]
    if missing:
        print("check_release_invariants: release.yml missing required gates:", file=sys.stderr)
        for item in missing:
            print(f"  - {item}", file=sys.stderr)
        return 1
    if "twine upload" in text:
        print("check_release_invariants: release.yml must not use twine upload", file=sys.stderr)
        return 1
    if re.search(r"secrets\.PYPI_API_TOKEN|TWINE_PASSWORD", text):
        print("check_release_invariants: release.yml must not reference PyPI API tokens", file=sys.stderr)
        return 1
    print("check_release_invariants: OK")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())