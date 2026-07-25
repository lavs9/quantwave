"""Order-driven backtest via `.bt.order_backtest` (quantwave-bbhb).

Covers the additive PyO3 `order_backtest_py` binding + `.bt.order_backtest`
namespace method: explicit per-bar orders (market/limit/stop/stop_limit)
resolved deterministically against OHLC bars, mirroring
`quantwave-backtest::order_exec::run_order_simulation`.
"""

import pytest

polars = pytest.importorskip("polars")
pl = polars

import quantwave  # noqa: F401  (registers LazyFrame.bt)


def _bars() -> pl.DataFrame:
    # (open, high, low, close) per bar; index 0..4.
    return pl.DataFrame(
        {
            "timestamp": [0, 1, 2, 3, 4],
            "open": [100.0, 100.5, 102.0, 104.0, 99.0],
            "high": [101.0, 103.0, 105.0, 106.0, 100.0],
            "low": [99.0, 100.0, 101.0, 98.0, 97.0],
            "close": [100.5, 102.0, 104.0, 99.0, 98.0],
        }
    )


def _order(
    bar_index: int,
    side: str,
    kind: str,
    qty: float,
    price: float | None = None,
    trigger: float | None = None,
    take_profit: float | None = None,
    stop_loss: float | None = None,
) -> dict:
    return {
        "bar_index": bar_index,
        "side": side,
        "type": kind,
        "qty": qty,
        "price": price,
        "trigger": trigger,
        "take_profit": take_profit,
        "stop_loss": stop_loss,
    }


def test_market_buy_then_market_sell_yields_one_trade_with_expected_pnl():
    bars = _bars()
    orders = pl.DataFrame(
        [
            _order(0, "buy", "market", 10.0),
            _order(3, "sell", "market", 10.0),
        ]
    )

    trades, equity = bars.lazy().bt.order_backtest(
        orders, commission_bps=0.0, slippage_bps=0.0
    )

    assert trades.height == 1
    row = trades.row(0, named=True)
    assert row["side"] == 1
    assert row["entry_fill_price"] == pytest.approx(100.0)  # bar 0 open
    assert row["exit_fill_price"] == pytest.approx(104.0)  # bar 3 open
    assert row["quantity"] == pytest.approx(10.0)
    assert row["pnl_net"] == pytest.approx((104.0 - 100.0) * 10.0)

    assert equity.height == bars.height
    # Columns match the shape of `.bt.backtest()`'s trades/equity output.
    assert set(trades.columns) == {
        "trade_id",
        "side",
        "entry_ts",
        "entry_price",
        "entry_fill_price",
        "exit_ts",
        "exit_price",
        "exit_fill_price",
        "quantity",
        "pnl_net",
    }
    assert set(equity.columns) == {"ts", "equity", "cash", "position", "close"}


def test_resting_limit_fills_at_the_limit():
    bars = _bars()
    # Buy limit @ 100 resubmitted every bar: bar 0 open (100.0) <= 100 -> fills
    # at open immediately (price improvement / gap-through convention).
    orders = pl.DataFrame([_order(i, "buy", "limit", 5.0, price=100.0) for i in range(5)])

    trades, _equity = bars.lazy().bt.order_backtest(
        orders, commission_bps=0.0, slippage_bps=0.0
    )

    assert trades.height == 1
    row = trades.row(0, named=True)
    assert row["entry_fill_price"] == pytest.approx(100.0)


def test_resting_limit_touch_fills_at_the_limit_not_open():
    bars = _bars()
    # Buy limit @ 100.25 only submitted at bar 1: open 100.5 > 100.25 but
    # low 100.0 <= 100.25 -> fills at the limit level itself (not open).
    orders = pl.DataFrame([_order(1, "buy", "limit", 5.0, price=100.25)])

    trades, _equity = bars.lazy().bt.order_backtest(
        orders, commission_bps=0.0, slippage_bps=0.0
    )

    assert trades.height == 1
    assert trades.row(0, named=True)["entry_fill_price"] == pytest.approx(100.25)


def test_stop_limit_order_uses_trigger_and_price_columns():
    bars = _bars()
    # Buy stop-limit at bar 2: trigger 104.5 (high 105 triggers), limit 104.8
    # (low never below limit after trigger within the bar) -> fills at limit.
    orders = pl.DataFrame(
        [_order(2, "buy", "stop_limit", 5.0, price=104.8, trigger=104.5)]
    )

    trades, _equity = bars.lazy().bt.order_backtest(
        orders, commission_bps=0.0, slippage_bps=0.0
    )

    assert trades.height == 1
    assert trades.row(0, named=True)["entry_fill_price"] == pytest.approx(104.8)


def test_missing_column_raises_key_error():
    bars = _bars()
    bad_orders = pl.DataFrame({"bar_index": [0], "side": ["buy"], "qty": [1.0]})
    with pytest.raises(KeyError):
        bars.lazy().bt.order_backtest(bad_orders)


def test_bad_order_type_raises_value_error():
    bars = _bars()
    orders = pl.DataFrame([_order(0, "buy", "not_a_type", 1.0)])
    with pytest.raises(ValueError):
        bars.lazy().bt.order_backtest(orders)


def test_out_of_range_bar_index_raises_value_error():
    bars = _bars()
    orders = pl.DataFrame([_order(99, "buy", "market", 1.0)])
    with pytest.raises(ValueError):
        bars.lazy().bt.order_backtest(orders)


def test_bracket_stop_loss_exits_position():
    bars = _bars()
    # Buy market bar 0 @ 100 with bracket tp=110 / sl=99.5. sl first touched on
    # bar 3 (low 98) -> exit at 99.5.
    orders = pl.DataFrame(
        [_order(0, "buy", "market", 10.0, take_profit=110.0, stop_loss=99.5)]
    )
    trades, _equity = bars.lazy().bt.order_backtest(
        orders, commission_bps=0.0, slippage_bps=0.0
    )
    assert trades.height == 1
    row = trades.row(0, named=True)
    assert row["exit_price"] == pytest.approx(99.5)
    assert row["pnl_net"] == pytest.approx((99.5 - 100.0) * 10.0)


def test_bracket_take_profit_exits_position():
    bars = _bars()
    # Buy market bar 0 @ 100 with bracket tp=104.5 / sl=95. tp touched bar 2 (high 105).
    orders = pl.DataFrame(
        [_order(0, "buy", "market", 10.0, take_profit=104.5, stop_loss=95.0)]
    )
    trades, _equity = bars.lazy().bt.order_backtest(
        orders, commission_bps=0.0, slippage_bps=0.0
    )
    assert trades.height == 1
    row = trades.row(0, named=True)
    assert row["exit_price"] == pytest.approx(104.5)
    assert row["pnl_net"] == pytest.approx((104.5 - 100.0) * 10.0)


def test_bracket_same_bar_double_touch_is_pessimistic():
    bars = _bars()
    # Buy market bar 2 @ 102 with bracket tp=105.5 / sl=99.5. Bar 3 touches BOTH
    # (high 106, low 98) -> stop-loss wins (pessimistic convention).
    orders = pl.DataFrame(
        [_order(2, "buy", "market", 10.0, take_profit=105.5, stop_loss=99.5)]
    )
    trades, _equity = bars.lazy().bt.order_backtest(
        orders, commission_bps=0.0, slippage_bps=0.0
    )
    assert trades.height == 1
    assert trades.row(0, named=True)["exit_price"] == pytest.approx(99.5)


def test_bracket_requires_both_legs():
    bars = _bars()
    orders = pl.DataFrame(
        [_order(0, "buy", "market", 10.0, take_profit=110.0)]  # stop_loss omitted
    )
    with pytest.raises(ValueError, match="bracket requires both"):
        bars.lazy().bt.order_backtest(orders)
