"""TDD vertical slices for ml_feature_backtest_parity notebook (quantwave-cr6v.6)."""

from __future__ import annotations

import pytest

polars = pytest.importorskip("polars")
pl = polars
np = pytest.importorskip("numpy")

from quantwave import (
    BullBearHmm,
    CyberCycleFeatureExtractor,
    GriffithsDominantCycleFeatureExtractor,
    HurstFeatureExtractor,
)
import quantwave  # noqa: F401 — registers LazyFrame.bt


def _generate_synthetic_ohlcv(n: int = 320):
    from datetime import datetime, timedelta, timezone

    closes = []
    price = 100.0
    ts0 = datetime(2026, 5, 1, 9, 30, tzinfo=timezone(timedelta(hours=5, minutes=30)))

    for i in range(n):
        seg = i // 80
        t = i % 80
        if seg == 0:
            drift = 0.18
            wave = 1.8 * np.sin(t * 0.09)
            noise = 0.08 * np.sin(i * 0.7)
            price = price + drift + wave * 0.02 + noise
        elif seg == 1:
            local_mean = 118.0 if i < 120 else 115.5
            pull = 0.18 * (local_mean - price)
            wave = 2.4 * np.sin(t * 0.22)
            noise = 0.12 * np.sin(i * 1.3)
            price = price + pull + wave * 0.03 + noise
        elif seg == 2:
            drift = 0.04
            wave = 3.6 * np.sin(t * 0.31 + 1.2)
            noise = 0.25 * np.sin(i * 0.9)
            price = price + drift + wave * 0.04 + noise
        else:
            drift = 0.005
            wave = 0.6 * np.sin(t * 0.11)
            noise = 0.04 * np.sin(i * 1.7)
            price = price + drift + wave * 0.01 + noise
        closes.append(price)

    closes_arr = np.array(closes, dtype=np.float64)
    timestamps = [ts0 + timedelta(hours=i) for i in range(n)]
    return pl.DataFrame(
        {
            "timestamp": timestamps,
            "close": closes_arr,
        }
    )


def _compute_exposure_and_meta(hurst_p, cyber_m, griff_dc, regime_l):
    regime_ok = regime_l in (0.0, 1.0)
    persistence_ok = hurst_p > 0.53
    mom_ok = abs(cyber_m) > 0.004
    cycle_ok = 8.0 < griff_dc < 46.0
    if regime_ok and persistence_ok and mom_ok and cycle_ok:
        exposure = float(np.clip(hurst_p * 1.65, 0.45, 1.95))
        meta = {
            "hurst_persistence": float(hurst_p),
            "cyber_momentum": float(cyber_m),
            "dominant_cycle": float(griff_dc),
            "regime_label": float(regime_l),
            "sizing_basis": 1.0,
        }
    else:
        exposure = 0.0
        meta = None
    return exposure, meta


def _batch_signal_df(ohlcv: pl.DataFrame) -> pl.DataFrame:
    closes = ohlcv["close"].to_numpy()
    hurst_ext = HurstFeatureExtractor(20)
    cyber_ext = CyberCycleFeatureExtractor(14)
    griff_ext = GriffithsDominantCycleFeatureExtractor(8, 42, 28)
    regime_ext = BullBearHmm.bull_bear()

    exposures = []
    for p in closes:
        h = hurst_ext.next(float(p))
        c = cyber_ext.next(float(p))
        g = griff_ext.next(float(p))
        r = regime_ext.next(float(p))
        exp, _ = _compute_exposure_and_meta(
            h.persistence, c.cycle_momentum, g.dominant_cycle, float(r)
        )
        exposures.append(exp)

    return ohlcv.with_columns(pl.Series("exposure", exposures)).select(
        ["timestamp", "close", "exposure"]
    )


class FeatureToSignal:
    def __init__(self):
        self.hurst = HurstFeatureExtractor(20)
        self.cyber = CyberCycleFeatureExtractor(14)
        self.griff = GriffithsDominantCycleFeatureExtractor(8, 42, 28)
        self.regime = BullBearHmm.bull_bear()

    def next(self, bar):
        close = float(bar["close"])
        h = self.hurst.next(close)
        c = self.cyber.next(close)
        g = self.griff.next(close)
        r = self.regime.next(close)
        exposure, meta = _compute_exposure_and_meta(
            h.persistence, c.cycle_momentum, g.dominant_cycle, float(r)
        )
        return {"exposure": exposure, "metadata": meta}


def _streaming_signal_df(ohlcv: pl.DataFrame) -> pl.DataFrame:
    gen = FeatureToSignal()
    exposures = []
    for close in ohlcv["close"].to_list():
        sig = gen.next({"close": close})
        exposures.append(sig["exposure"])
    return ohlcv.with_columns(pl.Series("exposure", exposures)).select(
        ["timestamp", "close", "exposure"]
    )


def _run_rust_backtest(signal_df: pl.DataFrame):
    return (
        signal_df.lazy()
        .bt.backtest(
            signal="exposure",
            commission_bps=0.0,
            slippage_bps=0.0,
        )
    )


def test_batch_path_uses_rust_engine():
    batch_df = _batch_signal_df(_generate_synthetic_ohlcv())
    result = _run_rust_backtest(batch_df)
    assert result.trades.height >= 0
    assert result.equity_curve.height == batch_df.height


def test_batch_streaming_equity_parity_within_ug9t_tolerance():
    ohlcv = _generate_synthetic_ohlcv()
    batch_result = _run_rust_backtest(_batch_signal_df(ohlcv))
    stream_result = _run_rust_backtest(_streaming_signal_df(ohlcv))

    b_eq = batch_result.equity_curve["equity"].to_list()
    s_eq = stream_result.equity_curve["equity"].to_list()
    assert len(b_eq) == len(s_eq)

    max_abs_diff = max(abs(b - s) for b, s in zip(b_eq, s_eq))
    assert max_abs_diff <= 1e-8


def test_batch_streaming_trade_count_exact_match():
    ohlcv = _generate_synthetic_ohlcv()
    batch_result = _run_rust_backtest(_batch_signal_df(ohlcv))
    stream_result = _run_rust_backtest(_streaming_signal_df(ohlcv))
    assert batch_result.trades.height == stream_result.trades.height
    assert batch_result.trades.height >= 1


def test_batch_metrics_sharpe_and_max_drawdown():
    batch_result = _run_rust_backtest(_batch_signal_df(_generate_synthetic_ohlcv()))
    metrics = batch_result.metrics()
    assert "sharpe_ratio" in metrics
    assert "max_drawdown_pct" in metrics
    assert metrics["num_trades"] >= 1.0