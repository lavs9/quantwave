"""Tier 2: feature matrix, monte_carlo, Rust .bt alignment smoke tests."""

import polars as pl
import pytest

import quantwave as qw


class TestBuildFeatureMatrix:
    def test_recommended_from_list(self):
        closes = [100.0 + i * 0.1 for i in range(120)]
        df = qw.build_feature_matrix(closes, features="recommended")
        assert df.height == 120
        assert "hurst_persistence" in df.columns
        assert "cyber_cycle" in df.columns
        assert "regime_label" in df.columns

    def test_from_dataframe(self):
        df_in = pl.DataFrame({"close": [100.0 + i for i in range(80)]})
        out = qw.build_feature_matrix(df_in, features=["hurst"])
        assert "close" in out.columns
        assert "hurst_persistence" in out.columns

    def test_drop_warmup(self):
        closes = [100.0 + i * 0.1 for i in range(120)]
        full = qw.build_feature_matrix(closes, features="recommended")
        trimmed = qw.build_feature_matrix(
            closes, features="recommended", drop_warmup=True, warmup_bars=50
        )
        assert trimmed.height == full.height - 50

    def test_feature_column_names(self):
        names = qw.feature_column_names("recommended")
        assert "hurst_persistence" in names
        assert "cyber_cycle" in names


class TestMonteCarlo:
    def test_trade_bootstrap_on_bt_namespace(self):
        df = pl.DataFrame({
            "timestamp": list(range(20)),
            "close": [100.0 + i * 0.5 for i in range(20)],
            "signal": [0.0, 1.0, 1.0, 1.0, 1.0, 0.0] + [0.0] * 14,
        })
        summary = df.lazy().bt.monte_carlo(
            commission_bps=0.0,
            slippage_bps=0.0,
            n_simulations=100,
            seed=1,
            mode="trade_bootstrap",
        )
        assert summary["n_simulations"] == 100
        assert summary["n_trades_sampled"] >= 1
        assert "p50_final_equity" in summary

    def test_return_paths_mode(self):
        df = pl.DataFrame({
            "timestamp": list(range(30)),
            "close": [100.0 + (i % 5) * 0.2 for i in range(30)],
            "signal": [1.0 if i % 4 == 0 else 0.0 for i in range(30)],
        })
        summary = df.lazy().bt.monte_carlo(
            commission_bps=0.0,
            slippage_bps=0.0,
            n_simulations=50,
            seed=2,
            mode="return_paths",
            n_bars_forward=10,
        )
        assert "var_95" in summary
        assert "cvar_95" in summary