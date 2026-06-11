"""TDD vertical slices for quantwave.backtest (quantwave-cr6v.4)."""

import pytest

polars = pytest.importorskip("polars")
pl = polars

from quantwave import backtest
from quantwave.backtest import BacktestConfig, BacktestEngine, BacktestReport


def test_backtest_module_import():
    assert hasattr(backtest, "BacktestEngine")
    assert hasattr(backtest, "BacktestConfig")
    assert hasattr(backtest, "BacktestResult")
    assert hasattr(backtest, "BacktestReport")


def _single_trade_df():
    """6 bars: enter bar 1, exit bar 4 (matches Rust steel-thread test)."""
    return pl.DataFrame(
        {
            "timestamp": [1_700_000_000 + i * 3600 for i in range(6)],
            "close": [100.0, 101.0, 102.5, 103.0, 102.0, 101.0],
            "signal": [0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
        }
    )


def test_backtest_engine_run_single_trade():
    engine = BacktestEngine.with_default_costs()
    result = engine.run(_single_trade_df())

    assert result.trades.height == 1
    stats = result.stats()
    assert stats["num_trades"] == pytest.approx(1.0)
    assert stats["final_equity"] > stats["initial_cash"]


def test_backtest_result_dataframe_schema():
    engine = BacktestEngine.with_default_costs()
    result = engine.run(_single_trade_df())

    trade_cols = set(result.trades.columns)
    assert {"trade_id", "side", "entry_ts", "entry_price", "exit_ts", "pnl_net"}.issubset(
        trade_cols
    )

    equity_cols = set(result.equity_curve.columns)
    assert {"ts", "equity", "cash", "position", "close"}.issubset(equity_cols)
    assert result.equity_curve.height == 6


def test_backtest_metrics_dict_keys():
    engine = BacktestEngine.with_default_costs()
    result = engine.run(_single_trade_df())
    metrics = result.metrics()

    expected = {
        "num_trades",
        "win_rate",
        "profit_factor",
        "max_drawdown_pct",
        "cagr",
        "sharpe_ratio",
        "sortino_ratio",
        "total_return",
        "final_equity",
        "avg_trade_pnl",
    }
    assert expected == set(metrics.keys())
    assert metrics["num_trades"] == pytest.approx(1.0)


def test_backtest_with_report():
    config = BacktestConfig(commission_bps=0.0, slippage_bps=0.0)
    engine = BacktestEngine(config)
    report = engine.backtest_with_report(_single_trade_df())

    assert isinstance(report, BacktestReport)
    assert report.result.trades.height == 1
    report_metrics = report.metrics()
    assert report_metrics["num_trades"] == pytest.approx(1.0)
    assert report_metrics["final_equity"] == pytest.approx(
        report.result.stats()["final_equity"]
    )