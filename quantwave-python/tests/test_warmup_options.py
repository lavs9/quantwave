"""Tests for warmup semantics (976r) and options namespace (05q7)."""

import warnings

import pytest

import quantwave as qw
from quantwave import options


class TestWarmupBars:
    def test_rsi_warmup_from_metadata(self):
        assert qw.warmup_bars("rsi", {"period": 14}) == 14

    def test_macd_default_warmup(self):
        assert qw.warmup_bars("macd") == 26

    def test_unknown_indicator_returns_zero(self):
        assert qw.warmup_bars("not_a_real_indicator_xyz") == 0

    def test_params_override_optional_defaults(self):
        assert qw.warmup_bars("ema", {"period": 50}) == 50

    def test_metadata_exposes_warmup_field(self):
        meta = qw.metadata("atr")
        assert meta is not None
        assert meta.warmup_bars == 14

    def test_wrap_streaming_readiness_uses_warmup(self):
        cls = qw.streaming_class("rsi")
        if cls is None:
            pytest.skip("RSI streaming class not available in this build")
        wrapped = qw.wrap_streaming(cls(14), name="rsi")
        for i in range(13):
            wrapped.next(100.0 + i)
            assert not wrapped.is_ready
        wrapped.next(113.0)
        assert wrapped.is_ready
        assert wrapped.bars_consumed == 14


class TestOptionsNamespace:
    def test_options_module_callable(self):
        price = options.bs_call_price(100.0, 100.0, 0.07, 0.25, 0.2)
        assert price > 0

    def test_options_not_in_indicator_list(self):
        names = qw.indicators()
        for sym in (
            "bs_call_price", "max_pain", "nse_lot_size", "implied_vol", "chain_pcr",
        ):
            assert sym not in names
            assert not qw.is_indicator(sym)

    def test_top_level_options_deprecated(self):
        with warnings.catch_warnings(record=True) as caught:
            warnings.simplefilter("always")
            fn = qw.bs_call_price
            assert callable(fn)
        assert any(
            issubclass(w.category, DeprecationWarning) and "quantwave.options" in str(w.message)
            for w in caught
        )

    def test_options_india_legacy_compat(self):
        assert hasattr(qw.options_india, "bs_call_price")
        assert qw.options_india.bs_call_price is options.bs_call_price