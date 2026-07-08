"""Wheel tag verifier tests (quantwave-9gek.1)."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts" / "verify_wheel_tags.py"
BROKEN = ROOT / "dist" / "quantwave-0.6.0-py3-none-macosx_11_0_arm64.whl"
FIXED = ROOT / "dist" / ".abi3-test" / "quantwave-0.6.0-cp39-abi3-macosx_11_0_arm64.whl"


def _run(*args: str) -> int:
    return subprocess.run(
        [sys.executable, str(SCRIPT), *args],
        cwd=ROOT,
        capture_output=True,
        text=True,
    ).returncode


def test_rejects_broken_py3_none_wheel() -> None:
    if not BROKEN.exists():
        return  # skip when old artifact not present locally
    assert _run(str(BROKEN)) != 0


def test_accepts_abi3_wheel_when_built() -> None:
    if not FIXED.exists():
        return
    assert _run(str(FIXED)) == 0