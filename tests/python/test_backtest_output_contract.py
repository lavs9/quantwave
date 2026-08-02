import math

import pytest
import polars as pl
import quantwave.bt_polars

def _single_win_trade_df():
    """Exactly one winning trade, no losers — the quantwave-s3iu repro.

    Ratios computed off this run are noise; sortino/profit_factor are undefined.
    """
    return pl.DataFrame(
        {
            "timestamp": [1_700_000_000 + i * 3600 for i in range(5)],
            "close": [100.0, 101.0, 102.0, 103.0, 104.0],
            "signal": [0.0, 1.0, 1.0, 0.0, 0.0],
        }
    )

def _loss_trade_df():
    """Trade that loses money to create drawdown."""
    return pl.DataFrame(
        {
            "timestamp": [1_700_000_000 + i * 3600 for i in range(6)],
            "close": [100.0, 99.0, 98.0, 95.0, 96.0, 97.0],
            "signal": [0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
        }
    )

def _win_trade_df():
    """Trade that makes money."""
    return pl.DataFrame(
        {
            "timestamp": [1_700_000_000 + i * 3600 for i in range(6)],
            "close": [100.0, 101.0, 102.5, 103.0, 102.0, 101.0],
            "signal": [0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
        }
    )

def test_backtest_contract_metrics_keys():
    df = _loss_trade_df()
    result = df.lazy().bt.backtest_with_report(commission_bps=0.0, slippage_bps=0.0)
    
    metrics = result.metrics()
    expected_keys = {
        "total_return",
        "cagr",
        "sharpe_ratio",
        "sortino_ratio",
        "max_drawdown_pct",
        "win_rate",
        "profit_factor",
        "num_trades",
        "avg_trade_pnl",
        "final_equity",
    }
    
    assert set(metrics.keys()) == expected_keys

def test_backtest_contract_drawdown_sign():
    df = _loss_trade_df()
    result = df.lazy().bt.backtest_with_report(commission_bps=0.0, slippage_bps=0.0)
    metrics = result.metrics()
    
    # max_drawdown_pct MUST be a positive fraction
    mdd = metrics["max_drawdown_pct"]
    assert mdd >= 0.0
    assert mdd <= 1.0 # Assuming it didn't go below 0 equity

def test_backtest_contract_win_rate():
    df = _win_trade_df()
    result = df.lazy().bt.backtest_with_report(commission_bps=0.0, slippage_bps=0.0)
    metrics = result.metrics()
    assert 0.0 <= metrics["win_rate"] <= 1.0

def test_backtest_contract_stats_keys():
    df = _loss_trade_df()
    result = df.lazy().bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    stats = result.stats()
    
    # Required stable keys
    expected_subset = {
        "initial_cash",
        "num_trades",
        "net_pnl",
        "final_equity",
        "total_return",
    }
    
    assert expected_subset.issubset(set(stats.keys()))

def test_backtest_contract_trades_schema():
    df = _win_trade_df()
    result = df.lazy().bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    
    trades = result.trades
    
    assert list(trades.columns) == [
        "trade_id", "side", "entry_ts", "entry_price", "entry_fill_price",
        "exit_ts", "exit_price", "exit_fill_price", "quantity", "pnl_net"
    ]
    
    # Dtypes
    assert trades.schema["side"] in [pl.Int64, pl.Float64, pl.Int32, pl.Float32, pl.Int8]
    # Check side in {-1, 1}
    sides = trades["side"].unique().to_list()
    assert set(sides).issubset({1.0, -1.0, 1, -1})
    
    # timestamps should be integers (epoch seconds)
    assert trades.schema["entry_ts"] in [pl.Int64, pl.Int32]
    assert trades.schema["exit_ts"] in [pl.Int64, pl.Int32]

def test_backtest_contract_equity_curve_schema():
    df = _win_trade_df()
    result = df.lazy().bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    
    ec = result.equity_curve
    assert list(ec.columns) == ["ts", "equity", "cash", "position", "close"]

def test_backtest_contract_trades_schema_empty():
    # Force empty trades by returning flat signals
    df = pl.DataFrame({
        "timestamp": [1_700_000_000, 1_700_003_600],
        "close": [100.0, 101.0],
        "signal": [0.0, 0.0]
    })
    result = df.lazy().bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    
    trades = result.trades
    assert list(trades.columns) == [
        "trade_id", "side", "entry_ts", "entry_price", "entry_fill_price",
        "exit_ts", "exit_price", "exit_fill_price", "quantity", "pnl_net"
    ]
    assert len(trades) == 0


# --- quantwave-s3iu: undefined ratios are NaN, and thin samples are flagged ---

def test_undefined_ratios_are_nan_not_inf():
    """No losing trades / no downside -> undefined, not 'infinitely good'."""
    df = _single_win_trade_df()
    result = df.lazy().bt.backtest_with_report(commission_bps=0.0, slippage_bps=0.0)
    metrics = result.metrics()

    pf = metrics["profit_factor"]
    sortino = metrics["sortino_ratio"]

    assert math.isnan(pf), f"profit_factor should be NaN, got {pf}"
    assert not math.isinf(pf), "profit_factor must not be inf"
    assert math.isnan(sortino), f"sortino_ratio should be NaN, got {sortino}"
    assert not math.isinf(sortino), "sortino_ratio must not be inf"

def test_metrics_contract_still_exactly_ten_keys_with_nan_values():
    """NaN ratios must not change the key set."""
    df = _single_win_trade_df()
    result = df.lazy().bt.backtest_with_report(commission_bps=0.0, slippage_bps=0.0)
    assert len(set(result.metrics().keys())) == 10

def test_diagnostics_flags_thin_sample():
    df = _single_win_trade_df()
    result = df.lazy().bt.backtest_with_report(commission_bps=0.0, slippage_bps=0.0)

    diag = result.diagnostics()
    assert diag["low_sample_size"] is True
    assert diag["min_trades_for_reliable_ratios"] == 30
    assert diag["num_trades"] < 30
    assert set(diag["undefined_metrics"]) == {"profit_factor", "sortino_ratio"}
    assert len(diag["warnings"]) == 2
    assert any("30" in w for w in diag["warnings"])

def test_diagnostics_is_additive_not_on_metrics():
    """The diagnostics surface must live off `.metrics()`, which stays 10 keys."""
    df = _single_win_trade_df()
    result = df.lazy().bt.backtest_with_report(commission_bps=0.0, slippage_bps=0.0)

    assert "diagnostics" not in set(result.metrics().keys())
    assert "low_sample_size" not in set(result.metrics().keys())
    assert "diagnostics" in result.extended_metrics()


