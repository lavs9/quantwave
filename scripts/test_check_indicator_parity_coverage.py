#!/usr/bin/env python3
"""Tests for check_indicator_parity_coverage.py (quantwave-ruh0.1 TDD)."""

from __future__ import annotations

import importlib.util
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCRIPT = ROOT / "scripts" / "check_indicator_parity_coverage.py"


def _load_module():
    spec = importlib.util.spec_from_file_location("parity_check", SCRIPT)
    mod = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(mod)
    return mod


class ParityCoverageCheckTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = _load_module()

    def test_exemption_without_reason_fails_schema(self):
        with tempfile.TemporaryDirectory() as td:
            bad = Path(td) / "parity_exemptions.toml"
            bad.write_text('[[exemption]]\nslug = "dummy"\nreason = "short"\n', encoding="utf-8")
            original = self.mod.EXEMPTIONS
            try:
                self.mod.EXEMPTIONS = bad
                with self.assertRaises(ValueError):
                    self.mod._load_exemptions()
            finally:
                self.mod.EXEMPTIONS = original

    def test_uncovered_slug_fails_check(self):
        with tempfile.TemporaryDirectory() as td:
            tmp = Path(td)
            registry = tmp / "metadata_registry.rs"
            registry.write_text(
                'RegisteredMetadata { slug: "dummy_gap", meta: &DUMMY_METADATA, struct_name: "Dummy", source_file: "dummy_gap" }\n',
                encoding="utf-8",
            )
            ind_dir = tmp / "indicators"
            ind_dir.mkdir(parents=True)
            (ind_dir / "dummy_gap.rs").write_text(
                "pub const DUMMY_METADATA: IndicatorMetadata = IndicatorMetadata { gold_standard_file: \"\" };\n",
                encoding="utf-8",
            )
            exemptions = tmp / "parity_exemptions.toml"
            exemptions.write_text("", encoding="utf-8")

            original_registry = self.mod.REGISTRY
            original_indicators = self.mod.INDICATORS
            original_regimes = self.mod.REGIMES
            original_core = self.mod.CORE
            original_exemptions = self.mod.EXEMPTIONS
            original_report = self.mod.REPORT
            try:
                self.mod.REGISTRY = registry
                self.mod.INDICATORS = ind_dir
                self.mod.REGIMES = tmp / "regimes"
                self.mod.CORE = tmp
                self.mod.EXEMPTIONS = exemptions
                self.mod.REPORT = tmp / "parity_coverage.json"
                data = self.mod.collect_coverage()
                self.assertIn("dummy_gap", data["gaps"])
                self.assertTrue(data["failures"])
            finally:
                self.mod.REGISTRY = original_registry
                self.mod.INDICATORS = original_indicators
                self.mod.REGIMES = original_regimes
                self.mod.CORE = original_core
                self.mod.EXEMPTIONS = original_exemptions
                self.mod.REPORT = original_report

    def test_main_repo_check_passes(self):
        result = self.mod.main()
        self.assertEqual(result, 0, "expected all registered indicators covered or exempt")


if __name__ == "__main__":
    raise SystemExit(unittest.main())