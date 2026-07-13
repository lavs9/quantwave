"""Tests for Python DX polish: errors, version, talib, categories, boundaries."""

import re

import pytest

import quantwave as qw
from quantwave import talib


class TestVersion:
    def test_version_is_semver_like(self):
        assert isinstance(qw.__version__, str)
        assert re.match(r"^\d+\.\d+\.\d+", qw.__version__)


class TestErrors:
    def test_hierarchy(self):
        assert issubclass(qw.InternalError, qw.QuantwaveError)
        assert issubclass(qw.IndicatorNotFoundError, qw.QuantwaveError)
        assert issubclass(qw.InvalidParameterError, qw.QuantwaveError)
        assert issubclass(qw.ParityError, qw.QuantwaveError)
        assert issubclass(qw.ParityError, AssertionError)
        assert issubclass(qw.StreamingError, qw.QuantwaveError)

    def test_unknown_indicator_raises_indicator_not_found(self):
        with pytest.raises(qw.IndicatorNotFoundError):
            qw.assert_parity("not_a_real_indicator_xyz", {}, [1.0, 2.0, 3.0])


class TestCategories:
    def test_categories_nonempty(self):
        cats = qw.categories()
        assert len(cats) > 0
        assert "Momentum" in cats or "Classic" in cats

    def test_indicators_by_category_consistent(self):
        by_cat = qw.indicators_by_category()
        all_meta = {m.name for m in qw.list_metadata()}
        assert sum(len(v) for v in by_cat.values()) == len(all_meta)
        assert "rsi" in by_cat.get("Momentum", [])

    def test_category_lookup_case_insensitive(self):
        classic = qw.category("classic")
        assert len(classic) > 0
        assert qw.category("CLASSIC") == classic


class TestBoundaryInfo:
    def test_rsi_scalar_semantics(self):
        info = qw.boundary_info("rsi")
        assert info is not None
        assert "NaN" in info.warmup_behavior

    def test_unknown_returns_none(self):
        assert qw.boundary_info("not_a_real_indicator_xyz") is None

    def test_sr_monitor_event_semantics(self):
        info = qw.boundary_info("sr_monitor")
        assert info is not None
        assert "event" in info.warmup_behavior.lower() or "struct" in info.warmup_behavior.lower()


class TestTalib:
    def test_list_functions(self):
        names = talib.list_functions()
        assert "RSI" in names
        assert "MACD" in names
        assert names == sorted(names)

    def test_rsi_callable(self):
        assert callable(talib.RSI)