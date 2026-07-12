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


def test_backtest_missing_column_raises_key_error():
    df = pl.DataFrame({"timestamp": [1], "close": [100.0]})
    with pytest.raises(KeyError, match="signal"):
        BacktestEngine.with_default_costs().run(df)


def test_backtest_wrong_timestamp_dtype_raises_type_error():
    df = pl.DataFrame(
        {
            "timestamp": ["not_a_ts"],
            "close": [100.0],
            "signal": [1.0],
        }
    )
    with pytest.raises(TypeError, match="timestamp"):
        BacktestEngine.with_default_costs().run(df)


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


# --- quantwave-cr6v.5: Polars LazyFrame `.bt` namespace ---


def test_bt_namespace_exists():
    lf = _single_trade_df().lazy()
    assert hasattr(lf, "bt")
    assert hasattr(lf.bt, "backtest")
    assert hasattr(lf.bt, "backtest_with_report")
    assert hasattr(lf.bt, "sweep")
    assert hasattr(lf.bt, "walk_forward")
    assert hasattr(lf.bt, "cross_sectional_backtest")


def test_bt_backtest_lazyframe():
    result = (
        _single_trade_df()
        .lazy()
        .bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    )
    assert result.trades.height == 1
    assert result.stats()["num_trades"] == pytest.approx(1.0)


def test_bt_backtest_custom_columns():
    df = pl.DataFrame(
        {
            "ts": [1, 2, 3, 4, 5, 6],
            "px": [100.0, 101.0, 102.5, 103.0, 102.0, 101.0],
            "exposure": [0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
        }
    )
    result = df.lazy().bt.backtest(
        signal="exposure",
        timestamp_col="ts",
        close_col="px",
        commission_bps=0.0,
        slippage_bps=0.0,
    )
    assert result.trades.height == 1


def test_bt_backtest_with_report():
    report = (
        _single_trade_df()
        .lazy()
        .bt.backtest_with_report(commission_bps=0.0, slippage_bps=0.0)
    )
    assert report.result.trades.height == 1

def test_bt_backtest_metrics_lazyframe():
    df = _single_trade_df().lazy()
    metrics = df.bt.backtest_metrics(commission_bps=0.0, slippage_bps=0.0)
    assert isinstance(metrics, dict)
    assert "num_trades" in metrics
    assert metrics["num_trades"] == 1.0


def test_bt_backtest_filter_and_multiplier():
    df = pl.DataFrame(
        {
            "timestamp": [1, 2, 3, 4, 5, 6],
            "close": [100.0, 100.0, 101.0, 102.0, 101.0, 100.0],
            "signal": [0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
            "regime_ok": [True, False, True, True, True, True],
            "size_mult": [1.0, 1.0, 0.5, 0.5, 1.0, 1.0],
        }
    )
    result = df.lazy().bt.backtest(
        entry_filter_col="regime_ok",
        size_multiplier_col="size_mult",
        commission_bps=0.0,
        slippage_bps=0.0,
    )
    assert result.trades.height == 1


def test_bt_backtest_stop_loss_exits():
    """2% SL exits long when close breaches stop (signal may stay 1)."""
    df = pl.DataFrame(
        {
            "timestamp": [1_700_100_000 + i * 3600 for i in range(5)],
            "close": [100.0, 100.0, 99.0, 97.0, 98.0],
            "signal": [0.0, 1.0, 1.0, 1.0, 0.0],
        }
    )
    result = df.lazy().bt.backtest(
        commission_bps=0.0,
        slippage_bps=0.0,
        stop_loss_pct=0.02,
    )
    assert result.trades.height == 1
    assert result.trades["exit_price"][0] == pytest.approx(97.0)


def test_bt_backtest_take_profit_exits():
    df = pl.DataFrame(
        {
            "timestamp": [1_700_200_000 + i * 3600 for i in range(5)],
            "close": [100.0, 100.0, 101.0, 103.0, 104.0],
            "signal": [0.0, 1.0, 1.0, 1.0, 1.0],
        }
    )
    result = df.lazy().bt.backtest(
        commission_bps=0.0,
        slippage_bps=0.0,
        take_profit_pct=0.03,
    )
    assert result.trades.height == 1
    assert result.trades["exit_price"][0] == pytest.approx(103.0)


def test_bt_backtest_trailing_stop_ratchets():
    df = pl.DataFrame(
        {
            "timestamp": [1_700_300_000 + i * 3600 for i in range(4)],
            "close": [100.0, 110.0, 104.0, 100.0],
            "signal": [0.0, 1.0, 1.0, 0.0],
        }
    )
    result = df.lazy().bt.backtest(
        commission_bps=0.0,
        slippage_bps=0.0,
        trailing_stop_pct=0.05,
    )
    assert result.trades.height == 1
    assert result.trades["exit_price"][0] == pytest.approx(104.0)


def test_bt_backtest_struct_signal_exposure():
    df = pl.DataFrame(
        {
            "timestamp": [1_900_100_000 + i for i in range(4)],
            "close": [100.0, 100.0, 105.0, 104.0],
            "signal": [
                {"exposure": 0.0},
                {"exposure": 1.0},
                {"exposure": 1.0},
                {"exposure": 0.0},
            ],
        }
    )
    result = df.lazy().bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    assert result.trades.height == 1
    assert result.trades["pnl_net"][0] == pytest.approx(4.0)


def test_bt_backtest_struct_signal_pole_height():
    df = pl.DataFrame(
        {
            "timestamp": [1_900_200_000 + i for i in range(4)],
            "close": [100.0, 100.0, 102.0, 101.0],
            "signal": [
                {"long": False, "pole_height": 0.0},
                {"long": True, "pole_height": 8.0},
                {"long": True, "pole_height": 8.0},
                {"long": False, "pole_height": 0.0},
            ],
        }
    )
    result = df.lazy().bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    assert result.trades.height == 1
    assert result.trades["quantity"][0] == pytest.approx(2.0)


def test_bt_backtest_short_pnl_on_decline():
    df = pl.DataFrame(
        {
            "timestamp": [1_800_100_000 + i for i in range(5)],
            "close": [100.0, 100.0, 98.0, 95.0, 96.0],
            "signal": [0.0, -1.0, -1.0, 0.0, 0.0],
        }
    )
    result = df.lazy().bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    assert result.trades.height == 1
    assert result.trades["side"][0] == -1
    assert result.trades["pnl_net"][0] == pytest.approx(5.0)


def test_bt_backtest_long_short_flip():
    df = pl.DataFrame(
        {
            "timestamp": [1_800_300_000 + i for i in range(5)],
            "close": [100.0, 100.0, 102.0, 101.0, 99.0],
            "signal": [0.0, 1.0, 1.0, -1.0, 0.0],
        }
    )
    result = df.lazy().bt.backtest(commission_bps=0.0, slippage_bps=0.0)
    assert result.trades.height == 2
    assert result.trades["side"].to_list() == [1, -1]


def test_bt_backtest_t1_delays_entry_one_bar():
    """T+1: signal on bar 1 fills at bar 2 close (102.5)."""
    result_t0 = (
        _single_trade_df()
        .lazy()
        .bt.backtest(commission_bps=0.0, slippage_bps=0.0, execution_delay="same_bar")
    )
    result_t1 = (
        _single_trade_df()
        .lazy()
        .bt.backtest(commission_bps=0.0, slippage_bps=0.0, execution_delay="next_bar")
    )
    assert result_t0.trades.height == 1
    assert result_t1.trades.height == 1
    t0_entry = result_t0.trades["entry_ts"][0]
    t1_entry = result_t1.trades["entry_ts"][0]
    assert t1_entry > t0_entry
    assert result_t1.trades["entry_price"][0] == pytest.approx(102.5)


def test_bt_backtest_multi_symbol_smoke():
    df = pl.DataFrame(
        {
            "timestamp": [
                1_700_010_000,
                1_700_010_000,
                1_700_010_001,
                1_700_010_001,
                1_700_010_002,
                1_700_010_002,
            ],
            "symbol": ["AAA", "BBB", "AAA", "BBB", "AAA", "BBB"],
            "close": [100.0, 50.0, 101.0, 51.0, 102.0, 52.0],
            "signal": [0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
        }
    )
    result = df.lazy().bt.backtest(
        symbol_col="symbol",
        commission_bps=0.0,
        slippage_bps=0.0,
    )
    assert result.trades.height == 2
    assert "symbol" in result.trades.columns
    assert result.stats()["num_symbols"] == pytest.approx(2.0)


# --- quantwave-cr6v.12: param sweep helper ---


def _sweep_base_df():
    return pl.DataFrame(
        {
            "timestamp": [1_700_000_000 + i * 3600 for i in range(6)],
            "close": [100.0, 101.0, 102.5, 103.0, 102.0, 101.0],
            "signal_early": [0.0, 1.0, 1.0, 1.0, 0.0, 0.0],
            "signal_late": [0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
            "signal_flat": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        }
    )


def test_bt_sweep_returns_param_metrics_df():
    sweep_df = _sweep_base_df().lazy().bt.sweep(
        param_name="threshold",
        param_values=[0.5, 1.0, 2.0],
        signal_cols=["signal_early", "signal_late", "signal_flat"],
        commission_bps=0.0,
        slippage_bps=0.0,
    )

    assert sweep_df.height == 3
    assert "threshold" in sweep_df.columns
    assert "num_trades" in sweep_df.columns
    assert "final_equity" in sweep_df.columns
    assert sweep_df["threshold"].to_list() == [0.5, 1.0, 2.0]
    assert sweep_df["num_trades"].to_list() == pytest.approx([1.0, 1.0, 0.0])


def test_bt_sweep_variants_differ_in_final_equity():
    sweep_df = _sweep_base_df().lazy().bt.sweep(
        param_name="entry_bar",
        param_values=[1.0, 2.0],
        signal_cols=["signal_early", "signal_late"],
        commission_bps=0.0,
        slippage_bps=0.0,
    )

    equities = sweep_df["final_equity"].to_list()
    assert equities[0] != pytest.approx(equities[1])


def test_bt_sweep_metrics_columns_complete():
    sweep_df = _sweep_base_df().lazy().bt.sweep(
        param_name="threshold",
        param_values=[0.5],
        signal_cols=["signal_early"],
        commission_bps=0.0,
        slippage_bps=0.0,
    )

    expected_metrics = {
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
    assert expected_metrics.issubset(set(sweep_df.columns))


# --- quantwave-cr6v.14–16: P2 walk-forward, cross-sectional, live bridge ---


def test_bt_walk_forward_returns_folds():
    n = 80
    df = pl.DataFrame(
        {
            "timestamp": [1_700_000_000 + i * 3600 for i in range(n)],
            "close": [100.0 + i * 0.1 for i in range(n)],
            "signal": [1.0 if (i // 15) % 2 == 0 else 0.0 for i in range(n)],
        }
    ).lazy()
    wf_df = df.bt.walk_forward(
        train_bars=20,
        test_bars=15,
        commission_bps=0.0,
        slippage_bps=0.0,
    )
    assert wf_df.height >= 2
    assert "fold_id" in wf_df.columns
    assert "num_trades" in wf_df.columns


def test_bt_cross_sectional_panel_smoke():
    df = pl.DataFrame(
        {
            "timestamp": [1, 1, 1, 2, 2, 2],
            "symbol": ["A", "B", "C", "A", "B", "C"],
            "close": [10.0, 10.0, 10.0, 11.0, 11.0, 11.0],
            "factor": [3.0, 2.0, 1.0, 3.0, 2.0, 1.0],
        }
    )
    report = df.lazy().bt.cross_sectional_backtest(
        factor_col="factor",
        top_frac=0.34,
        bottom_frac=0.34,
        commission_bps=0.0,
        slippage_bps=0.0,
    )
    assert report.metrics()["final_equity"] != 100_000.0


def test_bt_cross_sectional_zscore_python():
    # Construct a synthetic panel
    timestamps = [1, 1, 1, 1, 2, 2, 2, 2]
    symbols = ["A", "B", "C", "D", "A", "B", "C", "D"]
    closes = [10.0, 10.0, 10.0, 10.0, 11.0, 11.0, 11.0, 11.0]
    scores = [4.0, 3.0, 2.0, 1.0, 4.0, 3.0, 2.0, 1.0]
    
    df = pl.DataFrame({
        "timestamp": timestamps,
        "symbol": symbols,
        "close": closes,
        "score": scores
    })
    
    report = df.lazy().bt.cross_sectional_backtest(
        factor_col="score",
        transform="zscore",
        top_frac=0.25,
        bottom_frac=0.25,
        commission_bps=0.0,
        slippage_bps=0.0
    )
    assert report.metrics()["num_trades"] >= 0.0


def test_bt_cross_sectional_winsorize_python():
    timestamps = [1, 1, 1, 2, 2, 2]
    symbols = ["A", "B", "C", "A", "B", "C"]
    closes = [10.0, 10.0, 10.0, 11.0, 11.0, 11.0]
    scores = [100.0, 4.0, 3.0, 2.0, 1.0, -100.0]
    
    df = pl.DataFrame({
        "timestamp": timestamps,
        "symbol": symbols,
        "close": closes,
        "score": scores
    })
    
    report = df.lazy().bt.cross_sectional_backtest(
        factor_col="score",
        transform="winsorize",
        top_frac=0.3,
        bottom_frac=0.3,
        commission_bps=0.0,
        slippage_bps=0.0
    )
    assert report.metrics()["num_trades"] >= 0.0



def test_bt_walk_forward_optimize_python():
    # Make a simple builder fn
    def build_fn(lf, params):
        p = params["thresh"]
        return lf.with_columns(
            pl.when(pl.col("close") > p).then(1.0).otherwise(-1.0).alias("signal")
        )

    # Synthetic data
    n = 60
    timestamps = list(range(n))
    # Fold 1 train (0..19): close is 101, 102... -> high threshold (120) makes it -1 always, 110 makes it -1 always. Let's make it so that 110 captures some uptrend, 120 doesn't.
    # Actually, let's just use synthetic data where threshold 110 works perfectly on train, and fails on OOS.
    closes = [100.0 + i * (1.0 if i < 30 else -1.0) for i in range(n)]
    
    df = pl.DataFrame({
        "timestamp": timestamps,
        "close": closes,
    })
    
    res = df.lazy().bt.walk_forward_optimize(
        param_grid={"thresh": [110.0, 120.0]},
        build_fn=build_fn,
        objective="total_return",
        train_bars=20,
        test_bars=10,
        commission_bps=0.0,
        slippage_bps=0.0,
        overfit_threshold=0.0, # ensure overfit is triggered
    )
    
    assert res.height == 4 # (60 - 20) / 10 = 4 folds
    assert "best_thresh" in res.columns
    assert "train_metric" in res.columns
    assert "oos_metric" in res.columns
    assert "overfit_flag" in res.columns
    
    # Check that best parameter is selected and varies across folds (or is consistent)
    best_params = res["best_thresh"].to_list()
    assert len(best_params) == 4
    
    # Check overfit flag is boolean
    assert res["overfit_flag"].dtype == pl.Boolean
    
    # Overfit should trigger if train > oos
    train_metrics = res["train_metric"].to_list()
    oos_metrics = res["oos_metric"].to_list()
    overfits = res["overfit_flag"].to_list()
    
    for train, oos, overfit in zip(train_metrics, oos_metrics, overfits):
        assert overfit == (train - oos > 0.0)
