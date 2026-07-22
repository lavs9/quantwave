//! Order-driven simulation core (quantwave-9gk7 integration).
//!
//! Wires the deterministic fill primitives in [`crate::orders`] into a stepping
//! simulation. A strategy emits [`Order`]s per bar; each is resolved against the
//! bar's OHLC by [`orders::fill_order`] (a pure function of that one bar), so the
//! loop is **parity-safe by construction** — folding over all bars (batch) and
//! stepping one bar at a time (streaming) are the same computation.
//!
//! ## Position model (this first slice)
//! Flat-or-single-position, matching the exposure engine's "no intra-trade
//! resizing" model:
//! - an order that fills while **flat** opens a position;
//! - an **opposite-side** order that fills while in a position **closes it**
//!   (records a [`Trade`]); the closing order's qty is not netted — it is an exit
//!   signal;
//! - a **same-side** order while already in a position is ignored (no pyramiding).
//!
//! Deferred to later 9gk7 / bbhb slices: partial fills, pyramiding/averaging,
//! same-bar flips, bracket/OCO wiring, and the Python DataFrame/streaming
//! entry points. Costs/slippage reuse [`ExecutionModel`].

use crate::orders::{ExecBar, Order, Side, fill_order};
use crate::{EquityPoint, ExecutionModel, Trade};
use chrono::{DateTime, Utc};

struct OpenTrade {
    side: i8, // 1 long, -1 short
    qty: f64,
    entry_raw_px: f64,
    entry_fill_px: f64,
    entry_commission: f64,
    entry_ts: DateTime<Utc>,
}

/// Stepping order-execution simulator. Feed it one bar (+ that bar's orders) at
/// a time via [`OrderSim::step`]; call [`OrderSim::finish`] to flatten any open
/// position at the final close and collect the trades.
pub struct OrderSim<'a> {
    exec: &'a ExecutionModel,
    cash: f64,
    open: Option<OpenTrade>,
    trade_id: u32,
    trades: Vec<Trade>,
    last_close: f64,
    last_ts: Option<DateTime<Utc>>,
}

impl<'a> OrderSim<'a> {
    pub fn new(exec: &'a ExecutionModel) -> Self {
        let cash = match exec {
            ExecutionModel::Simple(cm) => cm.initial_cash,
            ExecutionModel::HighFidelity { .. } => 100_000.0,
        };
        Self {
            exec,
            cash,
            open: None,
            trade_id: 0,
            trades: Vec::new(),
            last_close: 0.0,
            last_ts: None,
        }
    }

    fn position(&self) -> f64 {
        match &self.open {
            Some(t) => t.side as f64 * t.qty,
            None => 0.0,
        }
    }

    /// Buy raises cash outflow (price*qty + commission); sell raises inflow
    /// (price*qty - commission). Returns the slippage-adjusted fill price.
    fn apply_cash(&mut self, is_buy: bool, qty: f64, raw_px: f64) -> (f64, f64) {
        let fill_px = self.exec.slippage_price(raw_px, qty, is_buy, None);
        let commission = self.exec.commission_for(qty, fill_px);
        if is_buy {
            self.cash -= fill_px * qty + commission;
        } else {
            self.cash += fill_px * qty - commission;
        }
        (fill_px, commission)
    }

    fn close_open(&mut self, exit_raw_px: f64, exit_bar_ts: DateTime<Utc>) {
        let Some(t) = self.open.take() else {
            return;
        };
        // Long exit = sell; short cover = buy.
        let is_buy = t.side == -1;
        let (exit_fill, exit_commission) = self.apply_cash(is_buy, t.qty, exit_raw_px);
        let gross = if t.side == 1 {
            (exit_fill - t.entry_fill_px) * t.qty
        } else {
            (t.entry_fill_px - exit_fill) * t.qty
        };
        let costs = t.entry_commission + exit_commission;
        self.trades.push(Trade {
            trade_id: self.trade_id,
            symbol: None,
            side: t.side,
            entry_ts: t.entry_ts,
            entry_price: t.entry_raw_px,
            entry_fill_price: t.entry_fill_px,
            exit_ts: Some(exit_bar_ts),
            exit_price: Some(exit_raw_px),
            exit_fill_price: Some(exit_fill),
            pnl_gross: gross,
            costs,
            pnl_net: gross - costs,
            quantity: t.qty,
            entry_metadata: None,
        });
        self.trade_id += 1;
    }

    /// Feed one bar and the orders resting/submitted for it. Returns the equity
    /// snapshot after processing. Orders are resolved in the given order.
    pub fn step(&mut self, ts: DateTime<Utc>, bar: ExecBar, orders: &[Order]) -> EquityPoint {
        self.last_close = bar.close;
        self.last_ts = Some(ts);
        for order in orders {
            let Some(fill) = fill_order(order.side, order.kind, bar) else {
                continue;
            };
            match &self.open {
                None => {
                    // Open a new position.
                    let is_buy = order.side == Side::Buy;
                    let (fill_px, commission) = self.apply_cash(is_buy, order.qty, fill.price);
                    self.open = Some(OpenTrade {
                        side: if is_buy { 1 } else { -1 },
                        qty: order.qty,
                        entry_raw_px: fill.price,
                        entry_fill_px: fill_px,
                        entry_commission: commission,
                        entry_ts: ts,
                    });
                }
                Some(t) => {
                    let opposite = (t.side == 1 && order.side == Side::Sell)
                        || (t.side == -1 && order.side == Side::Buy);
                    if opposite {
                        self.close_open(fill.price, ts);
                    }
                    // same-side order while in a position: ignored (no pyramiding)
                }
            }
        }
        EquityPoint {
            ts,
            symbol: None,
            equity: self.cash + self.position() * bar.close,
            cash: self.cash,
            position: self.position(),
            close: bar.close,
        }
    }

    /// Flatten any open position at the last seen close (terminal MTM, no extra
    /// cost — matches `run_simulation`) and return the recorded trades.
    pub fn finish(mut self) -> Vec<Trade> {
        if let (Some(_), Some(ts)) = (&self.open, self.last_ts) {
            let last_close = self.last_close;
            self.close_open(last_close, ts);
        }
        self.trades
    }
}

/// Batch driver: fold [`OrderSim`] over a full OHLC series + per-bar orders.
/// `next_orders(i)` returns the orders for bar `i`. Returns `(trades, equity)`.
#[allow(clippy::too_many_arguments)]
pub fn run_order_simulation(
    timestamps: &[DateTime<Utc>],
    opens: &[f64],
    highs: &[f64],
    lows: &[f64],
    closes: &[f64],
    mut next_orders: impl FnMut(usize) -> Vec<Order>,
    exec: &ExecutionModel,
) -> (Vec<Trade>, Vec<EquityPoint>) {
    let n = closes.len();
    let mut sim = OrderSim::new(exec);
    let mut equity = Vec::with_capacity(n);
    for i in 0..n {
        let bar = ExecBar {
            open: opens[i],
            high: highs[i],
            low: lows[i],
            close: closes[i],
        };
        equity.push(sim.step(timestamps[i], bar, &next_orders(i)));
    }
    let trades = sim.finish();
    (trades, equity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CostModel;
    use crate::orders::OrderType;

    fn ts(i: i64) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(1_700_000_000 + i * 3600, 0).unwrap()
    }

    fn zero_cost() -> ExecutionModel {
        ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        })
    }

    // (open, high, low, close) bars.
    const BARS: &[(f64, f64, f64, f64)] = &[
        (100.0, 101.0, 99.0, 100.5),
        (100.5, 103.0, 100.0, 102.0), // limit buy @ 100 could fill (low 100)
        (102.0, 105.0, 101.0, 104.0),
        (104.0, 106.0, 98.0, 99.0), // sell stop / exit territory
        (99.0, 100.0, 97.0, 98.0),
    ];

    fn series() -> (Vec<DateTime<Utc>>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let t: Vec<_> = (0..BARS.len() as i64).map(ts).collect();
        let o = BARS.iter().map(|b| b.0).collect();
        let h = BARS.iter().map(|b| b.1).collect();
        let l = BARS.iter().map(|b| b.2).collect();
        let c = BARS.iter().map(|b| b.3).collect();
        (t, o, h, l, c)
    }

    #[test]
    fn market_open_then_opposite_close_records_trade() {
        let (t, o, h, l, c) = series();
        let exec = zero_cost();
        // Buy market at bar 0 (fills at open 100), sell market at bar 3 (open 104).
        let orders = |i: usize| match i {
            0 => vec![Order::new(Side::Buy, OrderType::Market, 10.0)],
            3 => vec![Order::new(Side::Sell, OrderType::Market, 10.0)],
            _ => vec![],
        };
        let (trades, equity) = run_order_simulation(&t, &o, &h, &l, &c, orders, &exec);
        assert_eq!(trades.len(), 1);
        let tr = &trades[0];
        assert_eq!(tr.side, 1);
        assert_eq!(tr.entry_fill_price, 100.0);
        assert_eq!(tr.exit_fill_price, Some(104.0));
        assert!((tr.pnl_net - 40.0).abs() < 1e-9); // (104-100)*10
        assert_eq!(equity.len(), 5);
    }

    #[test]
    fn limit_buy_fills_at_limit_when_touched() {
        let (t, o, h, l, c) = series();
        let exec = zero_cost();
        // Resting limit buy @ 100 submitted each bar until filled; bar 1 low=100 fills.
        let orders = |_i: usize| {
            vec![Order::new(
                Side::Buy,
                OrderType::Limit { price: 100.0 },
                5.0,
            )]
        };
        let (trades, _eq) = run_order_simulation(&t, &o, &h, &l, &c, orders, &exec);
        // Opens at bar 0 (open 100 <= limit -> fills at open 100), never a same-side
        // add after; terminal flatten closes it -> exactly one trade.
        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].entry_fill_price, 100.0);
    }

    #[test]
    fn fold_equals_incremental_stepping_parity() {
        let (t, o, h, l, c) = series();
        let exec = zero_cost();
        let orders_vec: Vec<Vec<Order>> = (0..BARS.len())
            .map(|i| match i {
                0 => vec![Order::new(Side::Buy, OrderType::Market, 7.0)],
                3 => vec![Order::new(Side::Sell, OrderType::Market, 7.0)],
                _ => vec![],
            })
            .collect();

        // Batch fold.
        let (batch_trades, batch_eq) =
            run_order_simulation(&t, &o, &h, &l, &c, |i| orders_vec[i].clone(), &exec);

        // Manual incremental stepping through the same core.
        let mut sim = OrderSim::new(&exec);
        let mut stream_eq = Vec::new();
        for i in 0..BARS.len() {
            let bar = ExecBar {
                open: o[i],
                high: h[i],
                low: l[i],
                close: c[i],
            };
            stream_eq.push(sim.step(t[i], bar, &orders_vec[i]));
        }
        let stream_trades = sim.finish();

        assert_eq!(batch_eq, stream_eq);
        assert_eq!(batch_trades.len(), stream_trades.len());
        for (a, b) in batch_trades.iter().zip(stream_trades.iter()) {
            assert_eq!(a.pnl_net, b.pnl_net);
            assert_eq!(a.entry_fill_price, b.entry_fill_price);
            assert_eq!(a.exit_fill_price, b.exit_fill_price);
        }
    }
}
