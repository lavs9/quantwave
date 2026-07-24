"""Python exposure of the existing Rust risk_model / rebalance_policy configs
via ``.bt.backtest()`` / ``.bt.backtest_with_report()`` / ``.bt.backtest_metrics()``
and ``.bt.portfolio_backtest()`` (quantwave-bbhb).
"""

import polars as pl
import quantwave  # noqa: F401  (registers .bt)


def _trend_df(n: int = 40) -> pl.LazyFrame:
    closes = [100.0 + i * 1.5 for i in range(n)]
    signal = [1.0] * n
    return pl.DataFrame(
        {
            "timestamp": list(range(n)),
            "close": closes,
            "signal": signal,
        }
    ).lazy()


def test_position_limit_caps_exposure_vs_no_risk_model():
    df = _trend_df()

    uncapped = df.bt.backtest_with_report(
        commission_bps=0.0,
        slippage_bps=0.0,
        initial_cash=100_000.0,
    )
    capped = df.bt.backtest_with_report(
        commission_bps=0.0,
        slippage_bps=0.0,
        initial_cash=100_000.0,
        risk_model={"position_limit": {"max_abs_exposure": 5.0}},
    )

    uncapped_trades = uncapped.result.trades
    capped_trades = capped.result.trades
    assert uncapped_trades.height > 0
    assert capped_trades.height > 0

    # Uncapped sizing (default 1 unit * size_multiplier fallback... engine default
    # sizing is not overlay-scaled) should differ from the position-limit-clamped size.
    uncapped_qty = uncapped_trades["quantity"].abs().max()
    capped_qty = capped_trades["quantity"].abs().max()
    assert capped_qty <= 5.0 + 1e-9
    assert capped_qty <= uncapped_qty


def test_risk_model_none_matches_default_behavior():
    df = _trend_df()

    baseline = df.bt.backtest_with_report(
        commission_bps=5.0, slippage_bps=2.0, initial_cash=100_000.0
    )
    explicit_none = df.bt.backtest_with_report(
        commission_bps=5.0,
        slippage_bps=2.0,
        initial_cash=100_000.0,
        risk_model=None,
    )

    assert baseline.metrics() == explicit_none.metrics()
    assert baseline.result.trades.equals(explicit_none.result.trades)
    assert baseline.result.equity_curve.equals(explicit_none.result.equity_curve)


def test_backtest_raw_risk_model_none_matches_default():
    df = _trend_df()

    baseline = df.bt.backtest(commission_bps=5.0, slippage_bps=2.0)
    explicit_none = df.bt.backtest(commission_bps=5.0, slippage_bps=2.0, risk_model=None)

    assert baseline.metrics() == explicit_none.metrics()
    assert baseline.trades.equals(explicit_none.trades)


def test_backtest_metrics_risk_model_none_matches_default():
    df = _trend_df()

    baseline = df.bt.backtest_metrics(commission_bps=5.0, slippage_bps=2.0)
    explicit_none = df.bt.backtest_metrics(
        commission_bps=5.0, slippage_bps=2.0, risk_model=None
    )

    assert baseline == explicit_none


def _multi_symbol_df() -> pl.LazyFrame:
    n_bars = 12
    symbols = ["A", "B"]
    rows = {"timestamp": [], "symbol": [], "close": [], "signal": []}
    for t in range(n_bars):
        for i, sym in enumerate(symbols):
            rows["timestamp"].append(t)
            rows["symbol"].append(sym)
            # Oscillating signal so a "rebalance every bar" default produces
            # more trades than a calendar policy that only checks every N bars.
            rows["close"].append(100.0 + i * 10 + t * 0.5)
            rows["signal"].append(1.0 if (t + i) % 2 == 0 else -1.0)
    return pl.DataFrame(rows).lazy()


def test_rebalance_policy_calendar_changes_trade_count():
    df = _multi_symbol_df()

    default_report = df.bt.portfolio_backtest(
        symbol_col="symbol", commission_bps=0.0, slippage_bps=0.0
    )
    calendar_report = df.bt.portfolio_backtest(
        symbol_col="symbol",
        commission_bps=0.0,
        slippage_bps=0.0,
        rebalance_policy={"calendar": {"every_n_bars": 6}},
    )

    default_trades = default_report.result.trades.height
    calendar_trades = calendar_report.result.trades.height
    assert default_trades > 0
    assert calendar_trades != default_trades


def test_rebalance_policy_none_matches_default():
    df = _multi_symbol_df()

    baseline = df.bt.portfolio_backtest(
        symbol_col="symbol", commission_bps=0.0, slippage_bps=0.0
    )
    explicit_none = df.bt.portfolio_backtest(
        symbol_col="symbol",
        commission_bps=0.0,
        slippage_bps=0.0,
        rebalance_policy=None,
    )

    assert baseline.metrics() == explicit_none.metrics()
    assert baseline.result.trades.equals(explicit_none.result.trades)


def test_risk_model_unknown_key_raises():
    df = _trend_df()
    try:
        df.bt.backtest(risk_model={"not_a_real_overlay": {}})
    except (ValueError, TypeError) as e:
        assert "unknown key" in str(e)
    else:
        raise AssertionError("expected an error for an unknown risk_model key")


def test_rebalance_policy_unknown_key_raises():
    df = _multi_symbol_df()
    try:
        df.bt.portfolio_backtest(
            symbol_col="symbol", rebalance_policy={"not_a_real_policy": {}}
        )
    except (ValueError, TypeError) as e:
        assert "unknown key" in str(e)
    else:
        raise AssertionError("expected an error for an unknown rebalance_policy key")
