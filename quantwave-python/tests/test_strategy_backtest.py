"""TDD vertical slices for strategy_backtest notebook (quantwave-cr6v.7)."""

from __future__ import annotations

import pytest

polars = pytest.importorskip("polars")
pl = polars
np = pytest.importorskip("numpy")

import quantwave as qw  # noqa: F401 — registers LazyFrame.bt


def _deterministic_ohlcv(n: int = 1000) -> pl.DataFrame:
    """Same deterministic synthetic series as strategy_backtest.py (no RNG)."""
    from datetime import datetime, timedelta

    t = np.arange(n, dtype=np.float64)
    base = 150.0 + 12.0 * np.sin(t * 0.04) + 0.05 * t
    noise = 2.0 * np.sin(t * 0.17) + 0.8 * np.sin(t * 0.53)
    close = base + noise
    ts0 = datetime(2023, 1, 1)
    timestamps = [ts0 + timedelta(hours=int(i)) for i in range(n)]
    return pl.DataFrame(
        {
            "time": timestamps,
            "open": close - 0.2,
            "high": close + 0.5,
            "low": close - 0.5,
            "close": close,
            "volume": 2000.0 + 500.0 * np.abs(np.sin(t * 0.11)),
        }
    )


def _apply_supertrend(df: pl.DataFrame) -> pl.DataFrame:
    st = qw.streaming_class("supertrend")(period=10, multiplier=3.0)
    st_vals = []
    dirs = []
    for row in df.iter_rows(named=True):
        r = st.next(row["high"], row["low"], row["close"])
        st_vals.append(r.value)
        dirs.append(float(r.direction))
    return df.with_columns(
        [
            pl.Series("supertrend", st_vals),
            pl.Series("supertrend_dir", dirs),
        ]
    )


def _exposure_from_supertrend_dir(df: pl.DataFrame) -> pl.DataFrame:
    return df.with_columns(
        pl.when(pl.col("supertrend_dir") > 0)
        .then(1.0)
        .otherwise(0.0)
        .alias("exposure")
    )


def test_exposure_from_supertrend_direction():
    df = pl.DataFrame({"supertrend_dir": [1.0, -1.0, 1.0, 0.0]})
    out = _exposure_from_supertrend_dir(df)
    assert out["exposure"].to_list() == [1.0, 0.0, 1.0, 0.0]


def test_supertrend_backtest_num_trades_positive():
    df = _apply_supertrend(_deterministic_ohlcv())
    signal_df = _exposure_from_supertrend_dir(df).select(["time", "close", "exposure"])
    report = signal_df.lazy().bt.backtest_with_report(
        signal="exposure",
        timestamp_col="time",
        commission_bps=0.0,
        slippage_bps=0.0,
    )
    assert report.result.trades.height > 0


def test_supertrend_backtest_metrics_not_placeholder():
    df = _apply_supertrend(_deterministic_ohlcv())
    signal_df = _exposure_from_supertrend_dir(df).select(["time", "close", "exposure"])
    metrics = (
        signal_df.lazy()
        .bt.backtest_with_report(
            signal="exposure",
            timestamp_col="time",
            commission_bps=0.0,
            slippage_bps=0.0,
        )
        .metrics()
    )
    for key in ("sharpe_ratio", "max_drawdown_pct", "win_rate", "num_trades"):
        assert key in metrics
        assert metrics[key] == metrics[key]  # not NaN


def test_deterministic_backtest_reproducible():
    df = _apply_supertrend(_deterministic_ohlcv(400))
    signal_df = _exposure_from_supertrend_dir(df).select(["time", "close", "exposure"])

    def run_once():
        return (
            signal_df.lazy()
            .bt.backtest_with_report(
                signal="exposure",
                timestamp_col="time",
                commission_bps=0.0,
                slippage_bps=0.0,
            )
            .metrics()
        )

    m1 = run_once()
    m2 = run_once()
    assert m1["final_equity"] == m2["final_equity"]
    assert m1["num_trades"] == m2["num_trades"]