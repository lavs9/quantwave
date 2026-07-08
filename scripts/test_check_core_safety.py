#!/usr/bin/env python3
"""Tests for check_core_safety.py (quantwave-ruh0.3 TDD)."""

from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCRIPT = ROOT / "scripts" / "check_core_safety.py"


def _load():
    spec = importlib.util.spec_from_file_location("core_safety", SCRIPT)
    mod = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(mod)
    return mod


class CoreSafetyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = _load()

    def test_main_repo_passes(self):
        self.assertEqual(self.mod.main(), 0)

    def test_unsafe_in_production_fails(self):
        with tempfile.TemporaryDirectory() as td:
            src = Path(td) / "src"
            src.mkdir()
            (src / "evil.rs").write_text("pub fn bad() { unsafe { core::ptr::null::<i8>(); } }\n", encoding="utf-8")
            original = self.mod.CORE_SRC
            original_report = self.mod.REPORT
            try:
                self.mod.CORE_SRC = src
                self.mod.REPORT = Path(td) / "core_safety.json"
                data = self.mod.scan()
                self.assertGreater(data["forbidden_violations"], 0)
                self.assertEqual(self.mod.main(), 1)
            finally:
                self.mod.CORE_SRC = original
                self.mod.REPORT = original_report


if __name__ == "__main__":
    raise SystemExit(unittest.main())