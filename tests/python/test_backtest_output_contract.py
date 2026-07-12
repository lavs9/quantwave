import pytest
import polars as pl
import quantwave.bt_polars

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


