"""Shared-capital portfolio backtest Python smoke (quantwave-qzpi.9)."""

import polars as pl
import quantwave


def test_portfolio_backtest_two_symbol_smoke():
    df = pl.DataFrame(
        {
            "timestamp": [1, 1, 2, 2, 3, 3],
            "symbol": ["A", "B", "A", "B", "A", "B"],
            "close": [100.0, 50.0, 101.0, 51.0, 102.0, 52.0],
            "signal": [0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
        }
    ).lazy()

    report = df.bt.portfolio_backtest(
        symbol_col="symbol",
        commission_bps=0.0,
        slippage_bps=0.0,
        initial_cash=100_000.0,
    )
    metrics = report.metrics()
    assert metrics["final_equity"] != 100_000.0 or metrics["num_trades"] > 0
    assert report.result.stats()["initial_cash"] == 100_000.0


def test_portfolio_backtest_differs_from_independent():
    df = pl.DataFrame(
        {
            "timestamp": [1, 1, 2, 2],
            "symbol": ["A", "B", "A", "B"],
            "close": [100.0, 100.0, 110.0, 90.0],
            "signal": [1.0, 1.0, 1.0, 1.0],
        }
    ).lazy()

    shared = df.bt.portfolio_backtest(
        commission_bps=0.0,
        slippage_bps=0.0,
        portfolio_mode="shared_capital",
    )
    independent = df.bt.backtest_with_report(
        symbol_col="symbol",
        commission_bps=0.0,
        slippage_bps=0.0,
    )
    assert shared.metrics()["final_equity"] != independent.metrics()["final_equity"]

def test_portfolio_backtest_five_symbols_stress():
    df = pl.DataFrame(
        {
            "timestamp": [1] * 5 + [2] * 5 + [3] * 5,
            "symbol": ["A", "B", "C", "D", "E"] * 3,
            "close": [100.0, 101.0, 102.0, 103.0, 104.0] * 3,
            "signal": [1.0, 0.0, -1.0, 1.0, 0.0] * 3,
        }
    ).lazy()

    shared = df.bt.portfolio_backtest(
        symbol_col="symbol",
        commission_bps=0.0,
        slippage_bps=0.0,
        portfolio_mode="shared_capital",
        initial_cash=100_000.0,
    )
    metrics = shared.metrics()
    assert metrics["num_trades"] > 0