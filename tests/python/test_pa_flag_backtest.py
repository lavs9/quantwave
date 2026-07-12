import polars as pl
import pytest
import quantwave as qw
from quantwave.pa_flag_strategy import build_pa_flag_signals

def test_pa_flag_backtest_bt_namespace_import():
    highs = [100.5, 101.2, 102.8, 104.1, 106.5, 108.9, 110.2, 109.8, 108.5, 107.9,
             107.2, 106.8, 107.5, 108.1, 107.8, 108.3, 109.1, 111.5, 110.8, 112.2]
    lows = [99.5, 100.1, 101.5, 102.8, 104.2, 105.8, 107.0, 106.5, 105.2, 104.8,
            104.1, 103.9, 104.8, 105.5, 105.0, 105.8, 106.5, 108.0, 107.2, 108.9]
    df = pl.DataFrame({
        "bar": list(range(len(highs))),
        "high": highs,
        "low": lows,
        "close": [(h + l) / 2 for h, l in zip(highs, lows)],
    }).lazy()

    signal_lf = build_pa_flag_signals(df)
    assert hasattr(signal_lf, "bt")
    assert "signal" in signal_lf.collect().columns

def test_pa_flag_backtest_num_trades_positive():
    highs = [100.5, 101.2, 102.8, 104.1, 106.5, 108.9, 110.2, 109.8, 108.5, 107.9,
             107.2, 106.8, 107.5, 108.1, 107.8, 108.3, 109.1, 111.5, 110.8, 112.2]
    lows = [99.5, 100.1, 101.5, 102.8, 104.2, 105.8, 107.0, 106.5, 105.2, 104.8,
            104.1, 103.9, 104.8, 105.5, 105.0, 105.8, 106.5, 108.0, 107.2, 108.9]
    df = pl.DataFrame({
        "bar": list(range(len(highs))),
        "high": highs,
        "low": lows,
        "close": [(h + l) / 2 for h, l in zip(highs, lows)],
    }).lazy()

    signal_lf = build_pa_flag_signals(df)
    report = signal_lf.bt.backtest_with_report(
        signal="signal",
        close_col="close",
        timestamp_col="bar"
    )
    assert report.metrics()["num_trades"] >= 1

def test_pa_flag_backtest_regime_filter_reduces_trades():
    highs = [100.5, 101.2, 102.8, 104.1, 106.5, 108.9, 110.2, 109.8, 108.5, 107.9,
             107.2, 106.8, 107.5, 108.1, 107.8, 108.3, 109.1, 111.5, 110.8, 112.2]
    lows = [99.5, 100.1, 101.5, 102.8, 104.2, 105.8, 107.0, 106.5, 105.2, 104.8,
            104.1, 103.9, 104.8, 105.5, 105.0, 105.8, 106.5, 108.0, 107.2, 108.9]
    df = pl.DataFrame({
        "bar": list(range(len(highs))),
        "high": highs,
        "low": lows,
        "close": [(h + l) / 2 for h, l in zip(highs, lows)],
    }).lazy()

    # Create a filter that blocks entry
    signal_lf = build_pa_flag_signals(df).with_columns(
        strict_regime=pl.lit(False)
    )
    
    report_unfiltered = signal_lf.bt.backtest_with_report(
        signal="signal",
        close_col="close",
        timestamp_col="bar"
    )
    
    report_filtered = signal_lf.bt.backtest_with_report(
        signal="signal",
        close_col="close",
        timestamp_col="bar",
        entry_filter_col="strict_regime"
    )
    
    assert report_filtered.metrics()["num_trades"] < report_unfiltered.metrics()["num_trades"]

def test_pa_flag_backtest_pole_sizing_changes_quantity():
    highs = [100.5, 101.2, 102.8, 104.1, 106.5, 108.9, 110.2, 109.8, 108.5, 107.9,
             107.2, 106.8, 107.5, 108.1, 107.8, 108.3, 109.1, 111.5, 110.8, 112.2]
    lows = [99.5, 100.1, 101.5, 102.8, 104.2, 105.8, 107.0, 106.5, 105.2, 104.8,
            104.1, 103.9, 104.8, 105.5, 105.0, 105.8, 106.5, 108.0, 107.2, 108.9]
    df = pl.DataFrame({
        "bar": list(range(len(highs))),
        "high": highs,
        "low": lows,
        "close": [(h + l) / 2 for h, l in zip(highs, lows)],
    }).lazy()

    signal_lf = build_pa_flag_signals(df).with_columns(
        size_one=pl.lit(1.0)
    )
    
    report_unscaled = signal_lf.bt.backtest_with_report(
        signal="signal",
        close_col="close",
        timestamp_col="bar",
        size_multiplier_col="size_one"
    )
    
    report_scaled = signal_lf.bt.backtest_with_report(
        signal="signal",
        close_col="close",
        timestamp_col="bar",
        size_multiplier_col="pole_length_atr"
    )
    
    assert report_unscaled.result.trades["quantity"][0] != report_scaled.result.trades["quantity"][0]

def test_pa_flag_backtest_metrics_keys():
    highs = [100.5, 101.2, 102.8, 104.1, 106.5, 108.9, 110.2, 109.8, 108.5, 107.9,
             107.2, 106.8, 107.5, 108.1, 107.8, 108.3, 109.1, 111.5, 110.8, 112.2]
    lows = [99.5, 100.1, 101.5, 102.8, 104.2, 105.8, 107.0, 106.5, 105.2, 104.8,
            104.1, 103.9, 104.8, 105.5, 105.0, 105.8, 106.5, 108.0, 107.2, 108.9]
    df = pl.DataFrame({
        "bar": list(range(len(highs))),
        "high": highs,
        "low": lows,
        "close": [(h + l) / 2 for h, l in zip(highs, lows)],
    }).lazy()

    signal_lf = build_pa_flag_signals(df)
    
    report = signal_lf.bt.backtest_with_report(
        signal="signal",
        close_col="close",
        timestamp_col="bar"
    )
    
    metrics = report.metrics()
    expected_keys = {
        'num_trades', 'win_rate', 'profit_factor', 'max_drawdown_pct', 
        'cagr', 'sharpe_ratio', 'sortino_ratio', 'total_return', 
        'final_equity', 'avg_trade_pnl'
    }
    assert set(metrics.keys()) == expected_keys
