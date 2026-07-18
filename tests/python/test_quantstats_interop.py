"""TDD vertical slices for quantwave.quantstats_interop (p2k0.7)."""

from __future__ import annotations

import math
from dataclasses import dataclass
from datetime import datetime, timezone

import pytest

pl = pytest.importorskip("polars")

from quantwave import quantstats_interop as qsi
from quantwave.backtest import BacktestConfig, BacktestEngine

IST = "Asia/Kolkata"


# --- helpers -----------------------------------------------------------


@dataclass
class _FakeResult:
    """Minimal stand-in exposing only what backtest_returns needs."""

    equity_curve: "pl.DataFrame"


def _known_equity_df(ts_seconds: list[int], equity: list[float]) -> "pl.DataFrame":
    return pl.DataFrame({"ts": ts_seconds, "equity": equity})


def _daily_backtest_result(n: int = 40):
    """Deterministic, always-long, zero-cost daily-bar backtest.

    With signal=1.0 for every bar and zero commission/slippage, the engine
    holds a constant position and constant cash, so per-bar equity returns
    are exactly the per-bar close-price returns — a fully known, reproducible
    series useful for cross-validating against an independent implementation
    (QuantStats).
    """
    closes = [100.0 * (1.0 + 0.02 * math.sin(i / 3.0) + 0.001 * i) for i in range(n)]
    ts = [1_700_000_000 + i * 86400 for i in range(n)]
    df = pl.DataFrame({"timestamp": ts, "close": closes, "signal": [1.0] * n})
    config = BacktestConfig(commission_bps=0.0, slippage_bps=0.0)
    engine = BacktestEngine(config)
    return engine.run(df)


# --- 1. native returns (no optional deps) -------------------------------


def test_backtest_returns_matches_hand_computed_pct_change():
    ts = [1_700_000_000 + i * 86400 for i in range(5)]
    equity = [100_000.0, 100_100.0, 99_800.0, 101_000.0, 101_500.0]
    equity_df = _known_equity_df(ts, equity)

    result = qsi.backtest_returns(equity_df, freq="1d")

    expected = [
        equity[i] / equity[i - 1] - 1.0 for i in range(1, len(equity))
    ]
    assert result["return"].to_list() == pytest.approx(expected)
    # First row (no prior observation) is dropped.
    assert result.height == len(equity) - 1


def test_backtest_returns_first_row_dropped_and_tail_convention():
    ts = [1_700_000_000 + i * 86400 for i in range(4)]
    equity = [100_000.0, 105_000.0, 99_000.0, 110_000.0]
    equity_df = _known_equity_df(ts, equity)

    result = qsi.backtest_returns(equity_df, freq="1d")

    # Last return corresponds to the last equity transition.
    assert result["return"].to_list()[-1] == pytest.approx(110_000.0 / 99_000.0 - 1.0)
    # ts column carries forward the *later* timestamp of each transition.
    result_ts = result["ts"].to_list()
    assert len(result_ts) == 3
    assert result_ts[-1].tzinfo is not None


def test_backtest_returns_accepts_result_like_object_via_duck_typing():
    ts = [1_700_000_000 + i * 86400 for i in range(3)]
    equity = [100_000.0, 100_500.0, 100_200.0]
    fake = _FakeResult(equity_curve=_known_equity_df(ts, equity))

    result = qsi.backtest_returns(fake, freq="1d")

    assert result.height == 2
    assert result["return"].to_list() == pytest.approx(
        [100_500.0 / 100_000.0 - 1.0, 100_200.0 / 100_500.0 - 1.0]
    )


def test_backtest_returns_on_real_backtest_result():
    bt_result = _daily_backtest_result(n=10)
    result = qsi.backtest_returns(bt_result, freq="1d")

    equity = bt_result.equity_curve["equity"].to_list()
    expected = [equity[i] / equity[i - 1] - 1.0 for i in range(1, len(equity))]
    assert result["return"].to_list() == pytest.approx(expected)


def test_backtest_returns_missing_columns_raises_value_error():
    bad = pl.DataFrame({"ts": [1, 2, 3], "close": [1.0, 2.0, 3.0]})
    with pytest.raises(ValueError, match="equity"):
        qsi.backtest_returns(bad)


def test_backtest_returns_wrong_type_raises_type_error():
    with pytest.raises(TypeError):
        qsi.backtest_returns(object())


# --- 2. golden cross-validation against QuantStats ----------------------


def test_quantstats_metrics_matches_engine_sharpe_and_max_drawdown():
    qs = pytest.importorskip("quantstats")
    pd = pytest.importorskip("pandas")

    bt_result = _daily_backtest_result(n=60)
    engine_metrics = bt_result.metrics()

    returns = qsi.to_quantstats(bt_result, freq="1d")
    assert isinstance(returns, pd.Series)

    # Must run without raising.
    report = qsi.quantstats_metrics(bt_result, freq="1d")
    assert report is not None

    qs_sharpe = qs.stats.sharpe(returns, rf=0.0, periods=252)
    qs_max_dd = abs(qs.stats.max_drawdown(returns))

    assert qs_sharpe == pytest.approx(engine_metrics["sharpe_ratio"], abs=1e-6)
    assert qs_max_dd == pytest.approx(engine_metrics["max_drawdown_pct"], abs=1e-6)


def test_to_quantstats_missing_pandas_raises_clear_import_error(monkeypatch):
    import builtins

    real_import = builtins.__import__

    def _blocked_import(name, *args, **kwargs):
        if name == "pandas":
            raise ImportError("simulated missing pandas")
        return real_import(name, *args, **kwargs)

    monkeypatch.setattr(builtins, "__import__", _blocked_import)

    ts = [1_700_000_000 + i * 86400 for i in range(3)]
    equity = [100_000.0, 100_500.0, 100_200.0]
    equity_df = _known_equity_df(ts, equity)

    with pytest.raises(ImportError, match="pip install pandas"):
        qsi.to_quantstats(equity_df)


def test_quantstats_metrics_missing_quantstats_raises_clear_import_error(monkeypatch):
    import builtins

    real_import = builtins.__import__

    def _blocked_import(name, *args, **kwargs):
        if name == "quantstats":
            raise ImportError("simulated missing quantstats")
        return real_import(name, *args, **kwargs)

    monkeypatch.setattr(builtins, "__import__", _blocked_import)

    ts = [1_700_000_000 + i * 86400 for i in range(3)]
    equity = [100_000.0, 100_500.0, 100_200.0]
    equity_df = _known_equity_df(ts, equity)

    with pytest.raises(ImportError, match="pip install quantstats"):
        qsi.quantstats_metrics(equity_df)


# --- 3. frequency / tz edges ---------------------------------------------


def test_backtest_returns_is_tz_aware_ist():
    ts = [1_700_000_000 + i * 86400 for i in range(3)]
    equity = [100_000.0, 100_100.0, 100_300.0]
    equity_df = _known_equity_df(ts, equity)

    result = qsi.backtest_returns(equity_df, freq="1d")

    result_ts = result["ts"].to_list()
    assert all(t.tzinfo is not None for t in result_ts)
    # IST is UTC+5:30, and "1d" resampling truncates to the IST calendar-day
    # boundary, so the returned timestamps land at IST midnight.
    assert result_ts[0].utcoffset().total_seconds() == 5.5 * 3600
    assert (result_ts[0].hour, result_ts[0].minute) == (0, 0)


def test_daily_resample_keeps_last_equity_per_day():
    # Two intraday bars per calendar day (12h apart); "1d" resample should
    # collapse each day down to its last observed equity before diffing.
    ts = [
        1_700_000_000,
        1_700_000_000 + 12 * 3600,
        1_700_000_000 + 24 * 3600,
        1_700_000_000 + 36 * 3600,
    ]
    equity = [100_000.0, 100_050.0, 100_200.0, 100_100.0]
    equity_df = _known_equity_df(ts, equity)

    daily = qsi.backtest_returns(equity_df, freq="1d")
    # Day 1 last equity = 100_050.0, Day 2 last equity = 100_100.0
    assert daily["return"].to_list() == pytest.approx(
        [100_100.0 / 100_050.0 - 1.0]
    )


def test_intraday_raw_freq_keeps_every_bar():
    ts = [
        1_700_000_000,
        1_700_000_000 + 12 * 3600,
        1_700_000_000 + 24 * 3600,
        1_700_000_000 + 36 * 3600,
    ]
    equity = [100_000.0, 100_050.0, 100_200.0, 100_100.0]
    equity_df = _known_equity_df(ts, equity)

    raw = qsi.backtest_returns(equity_df, freq="raw")
    assert raw.height == 3
    expected = [
        100_050.0 / 100_000.0 - 1.0,
        100_200.0 / 100_050.0 - 1.0,
        100_100.0 / 100_200.0 - 1.0,
    ]
    assert raw["return"].to_list() == pytest.approx(expected)


def test_hourly_resample_freq():
    # Two bars in the same hour, one bar in the next hour.
    ts = [1_700_000_000, 1_700_000_000 + 600, 1_700_000_000 + 3600]
    equity = [100_000.0, 100_010.0, 100_020.0]
    equity_df = _known_equity_df(ts, equity)

    hourly = qsi.backtest_returns(equity_df, freq="1h")
    assert hourly.height == 1
    assert hourly["return"].to_list() == pytest.approx([100_020.0 / 100_010.0 - 1.0])
