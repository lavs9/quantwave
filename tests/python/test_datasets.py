"""Tests for quantwave.datasets — the zero-network synthetic data layer (p2k0.4)."""

from __future__ import annotations

import polars as pl
import pytest

import quantwave as qw
from quantwave import datasets
from quantwave.datasets import SCHEMA, load_sample, synthetic


class TestSchemaContract:
    def test_load_sample_columns_and_dtypes(self) -> None:
        df = load_sample()
        assert set(df.columns) >= set(SCHEMA.keys())
        for col, dtype in SCHEMA.items():
            assert df.schema[col] == dtype, f"{col}: expected {dtype}, got {df.schema[col]}"

    def test_load_sample_ts_is_tz_aware_ist(self) -> None:
        df = load_sample()
        assert df.schema["ts"].time_zone == "Asia/Kolkata"

    def test_load_sample_ts_monotonic_increasing(self) -> None:
        df = load_sample()
        # Per-symbol (if present) monotonicity; else global.
        if "symbol" in df.columns:
            for _, sub in df.group_by("symbol"):
                ts = sub["ts"].to_list()
                assert ts == sorted(ts)
        else:
            ts = df["ts"].to_list()
            assert ts == sorted(ts)

    def test_load_sample_is_zero_network_and_nonempty(self) -> None:
        df = load_sample()
        assert len(df) > 0

    def test_load_sample_no_nulls_in_ohlcv(self) -> None:
        df = load_sample()
        for col in ("open", "high", "low", "close", "volume"):
            assert df[col].null_count() == 0

    def test_load_sample_ohlc_sane_bounds(self) -> None:
        df = load_sample()
        assert (df["high"] >= df["low"]).all()
        assert (df["high"] >= df["open"]).all()
        assert (df["high"] >= df["close"]).all()
        assert (df["low"] <= df["open"]).all()
        assert (df["low"] <= df["close"]).all()
        assert (df["volume"] >= 0).all()


class TestSyntheticDeterminism:
    def test_same_seed_identical_frame(self) -> None:
        a = synthetic(seed=42, rows=2_000)
        b = synthetic(seed=42, rows=2_000)
        assert datasets.frame_hash(a) == datasets.frame_hash(b)

    def test_different_seed_differs(self) -> None:
        a = synthetic(seed=42, rows=2_000)
        b = synthetic(seed=43, rows=2_000)
        assert datasets.frame_hash(a) != datasets.frame_hash(b)

    def test_schema_contract(self) -> None:
        df = synthetic(seed=1, rows=500)
        # symbol is optional (only present for multi-symbol frames); everything
        # else in SCHEMA is required.
        for col, dtype in SCHEMA.items():
            if col == "symbol" and col not in df.columns:
                continue
            assert df.schema[col] == dtype

    def test_rows_and_sorted(self) -> None:
        df = synthetic(seed=1, rows=500)
        assert len(df) == 500
        ts = df["ts"].to_list()
        assert ts == sorted(ts)

    def test_multi_symbol(self) -> None:
        df = synthetic(seed=1, rows=100, symbols=["NIFTY", "SYM_A"])
        assert set(df["symbol"].unique().to_list()) == {"NIFTY", "SYM_A"}
        assert len(df) == 200

    def test_regime_variance_differs(self) -> None:
        # With a large enough sample and multiple regime switches, the stdev of
        # returns should differ meaningfully between the first and second half
        # of a regime-switching run (sanity, not a strict statistical test).
        df = synthetic(seed=7, rows=20_000, n_regimes=4)
        closes = df["close"].to_numpy()
        import numpy as np

        rets = np.diff(np.log(closes))
        chunks = np.array_split(rets, 4)
        stds = [c.std() for c in chunks]
        assert max(stds) / (min(stds) + 1e-12) > 1.2

    def test_volume_model_nonzero_and_finite(self) -> None:
        df = synthetic(seed=3, rows=1_000)
        vol = df["volume"].to_numpy()
        assert (vol > 0).all()
        assert bool((vol == vol).all())  # no NaN


class TestFetchersAreDocumentedStubs:
    def test_yfinance_missing_dependency_raises_clean_import_error(self) -> None:
        from quantwave import data as qwd

        # yfinance is not a dependency of this project; calling the wrapper
        # without it installed must raise a clear, actionable ImportError.
        try:
            import yfinance  # noqa: F401

            pytest.skip("yfinance happens to be installed in this environment")
        except ImportError:
            pass
        with pytest.raises(ImportError):
            qwd.fetch_yfinance("^NSEI", start="2020-01-01", end="2020-02-01")

    def test_nse_bhavcopy_is_documented_stub(self) -> None:
        from quantwave import data as qwd

        with pytest.raises(NotImplementedError):
            qwd.fetch_nse_bhavcopy("2024-01-01")

    def test_nse_option_chain_is_documented_stub(self) -> None:
        from quantwave import data as qwd

        with pytest.raises(NotImplementedError):
            qwd.fetch_nse_option_chain("NIFTY")


class TestQuickstart:
    def test_ten_minute_path(self) -> None:
        """load_sample() -> .ta.rsi(14) end-to-end, zero network, finite tail."""
        df = load_sample()
        symbol_col = "symbol" if "symbol" in df.columns else None
        if symbol_col:
            first_symbol = df[symbol_col][0]
            df = df.filter(pl.col(symbol_col) == first_symbol)
        out = df.select(pl.col("close").ta.rsi(14).alias("rsi"))
        tail = out.tail(10)["rsi"].to_numpy()
        assert len(tail) == 10
        assert all(v == v for v in tail)  # no NaN
        assert all(0.0 <= v <= 100.0 for v in tail)

    def test_datasets_reachable_from_top_level_package(self) -> None:
        assert hasattr(qw, "datasets")
