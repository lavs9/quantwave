//! First-class order types with deterministic OHLC fills (quantwave-9gk7).
//!
//! This mirrors [`crate::stops`]: every fill is a **pure function of one bar's
//! OHLC plus a fixed rule**, so the batch and streaming engines agree by
//! construction — the parity moat is preserved without a second engine or any
//! "parity tier". Same-bar ambiguity (a bracket whose take-profit *and*
//! stop-loss are both touched) is resolved by the **conservative / pessimistic
//! convention** — stop-loss before take-profit — exactly as
//! [`crate::stops::evaluate_stops`] already does.
//!
//! With only OHLC the true intrabar path is unknowable, so this is an
//! approximation — the industry-standard one. Tick / lower-timeframe fidelity is
//! deliberately out of scope (see `planning/ORDER_MODE_EXECUTION_ADR.md`).

use serde::{Deserialize, Serialize};

/// Order direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

/// Entry order type. Prices are absolute levels.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    /// Fill at this bar's open.
    Market,
    /// Rest until price trades to `price` (buy: at/below; sell: at/above).
    Limit { price: f64 },
    /// Breakout trigger (buy: at/above `trigger`; sell: at/below).
    Stop { trigger: f64 },
    /// `Stop` that, once triggered, becomes a `Limit` at `limit`.
    StopLimit { trigger: f64, limit: f64 },
}

/// A submitted order: a direction, a type, and a quantity (in units).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub side: Side,
    pub kind: OrderType,
    pub qty: f64,
}

impl Order {
    pub fn new(side: Side, kind: OrderType, qty: f64) -> Self {
        Self { side, kind, qty }
    }
}

/// Why/how an order filled (for the trade record).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillKind {
    Market,
    Limit,
    Stop,
    StopLimit,
}

/// A resolved fill: the raw price before slippage/commission and its cause.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fill {
    pub price: f64,
    pub kind: FillKind,
}

/// One bar's prices. `open` is required (market/stop fills reference it);
/// `high`/`low` fall back to `close` when absent upstream.
#[derive(Debug, Clone, Copy)]
pub struct ExecBar {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

/// Resolve whether `order` (in direction `side`) fills against `bar`, and at what
/// price. `None` means the order rests (no fill this bar). Deterministic in the
/// bar's OHLC alone — no future data, no intrabar path assumption beyond the
/// documented gap/price-improvement rules.
pub fn fill_order(side: Side, order: OrderType, bar: ExecBar) -> Option<Fill> {
    match (side, order) {
        (_, OrderType::Market) => Some(Fill {
            price: bar.open,
            kind: FillKind::Market,
        }),

        // Buy limit rests at/below market: fills if the bar trades down to `price`.
        (Side::Buy, OrderType::Limit { price }) => {
            if bar.open <= price {
                Some(Fill {
                    price: bar.open,
                    kind: FillKind::Limit,
                }) // gap through → price improvement
            } else if bar.low <= price {
                Some(Fill {
                    price,
                    kind: FillKind::Limit,
                })
            } else {
                None
            }
        }
        // Sell limit rests at/above market: fills if the bar trades up to `price`.
        (Side::Sell, OrderType::Limit { price }) => {
            if bar.open >= price {
                Some(Fill {
                    price: bar.open,
                    kind: FillKind::Limit,
                })
            } else if bar.high >= price {
                Some(Fill {
                    price,
                    kind: FillKind::Limit,
                })
            } else {
                None
            }
        }

        // Buy stop (breakout up): triggers when the bar trades up to `trigger`.
        (Side::Buy, OrderType::Stop { trigger }) => {
            if bar.open >= trigger {
                Some(Fill {
                    price: bar.open,
                    kind: FillKind::Stop,
                }) // gap up over the stop
            } else if bar.high >= trigger {
                Some(Fill {
                    price: trigger,
                    kind: FillKind::Stop,
                })
            } else {
                None
            }
        }
        // Sell stop (breakdown): triggers when the bar trades down to `trigger`.
        (Side::Sell, OrderType::Stop { trigger }) => {
            if bar.open <= trigger {
                Some(Fill {
                    price: bar.open,
                    kind: FillKind::Stop,
                })
            } else if bar.low <= trigger {
                Some(Fill {
                    price: trigger,
                    kind: FillKind::Stop,
                })
            } else {
                None
            }
        }

        // Buy stop-limit: once triggered (>= trigger), a buy limit at `limit`.
        (Side::Buy, OrderType::StopLimit { trigger, limit }) => {
            let triggered = bar.open >= trigger || bar.high >= trigger;
            if !triggered {
                return None;
            }
            if bar.open >= trigger && bar.open <= limit {
                Some(Fill {
                    price: bar.open,
                    kind: FillKind::StopLimit,
                })
            } else if bar.low <= limit {
                Some(Fill {
                    price: limit,
                    kind: FillKind::StopLimit,
                })
            } else {
                None // triggered but the limit was never reached this bar
            }
        }
        // Sell stop-limit: once triggered (<= trigger), a sell limit at `limit`.
        (Side::Sell, OrderType::StopLimit { trigger, limit }) => {
            let triggered = bar.open <= trigger || bar.low <= trigger;
            if !triggered {
                return None;
            }
            if bar.open <= trigger && bar.open >= limit {
                Some(Fill {
                    price: bar.open,
                    kind: FillKind::StopLimit,
                })
            } else if bar.high >= limit {
                Some(Fill {
                    price: limit,
                    kind: FillKind::StopLimit,
                })
            } else {
                None
            }
        }
    }
}

/// Which leg of a bracket/OCO exit fired.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitLeg {
    StopLoss,
    TakeProfit,
}

/// Resolve a bracket / OCO exit (a take-profit `tp` and stop-loss `sl` level) for
/// an open position against one bar. Uses the **pessimistic** convention: on a
/// same-bar double-touch the **stop-loss** is assumed to fire first — identical
/// to [`crate::stops::evaluate_stops`]. Returns `(leg, exit_price)` or `None`.
pub fn resolve_bracket(is_long: bool, tp: f64, sl: f64, bar: ExecBar) -> Option<(ExitLeg, f64)> {
    if is_long {
        if bar.low <= sl {
            Some((ExitLeg::StopLoss, sl))
        } else if bar.high >= tp {
            Some((ExitLeg::TakeProfit, tp))
        } else {
            None
        }
    } else if bar.high >= sl {
        Some((ExitLeg::StopLoss, sl))
    } else if bar.low <= tp {
        Some((ExitLeg::TakeProfit, tp))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bar(open: f64, high: f64, low: f64, close: f64) -> ExecBar {
        ExecBar {
            open,
            high,
            low,
            close,
        }
    }

    #[test]
    fn market_fills_at_open() {
        let f = fill_order(Side::Buy, OrderType::Market, bar(100.0, 102.0, 98.0, 101.0)).unwrap();
        assert_eq!(f.kind, FillKind::Market);
        assert_eq!(f.price, 100.0);
    }

    #[test]
    fn buy_limit_touch_fills_at_limit() {
        // open above limit, low dips to it → fill at the limit.
        let f = fill_order(
            Side::Buy,
            OrderType::Limit { price: 95.0 },
            bar(100.0, 101.0, 94.0, 99.0),
        )
        .unwrap();
        assert_eq!(f.kind, FillKind::Limit);
        assert_eq!(f.price, 95.0);
    }

    #[test]
    fn buy_limit_gap_through_fills_at_open() {
        // open already below the limit → price improvement, fill at open.
        let f = fill_order(
            Side::Buy,
            OrderType::Limit { price: 95.0 },
            bar(93.0, 96.0, 92.0, 94.0),
        )
        .unwrap();
        assert_eq!(f.price, 93.0);
    }

    #[test]
    fn buy_limit_no_touch_rests() {
        assert!(
            fill_order(
                Side::Buy,
                OrderType::Limit { price: 90.0 },
                bar(100.0, 101.0, 95.0, 99.0)
            )
            .is_none()
        );
    }

    #[test]
    fn sell_limit_mirrors_buy() {
        let f = fill_order(
            Side::Sell,
            OrderType::Limit { price: 105.0 },
            bar(100.0, 106.0, 99.0, 101.0),
        )
        .unwrap();
        assert_eq!(f.price, 105.0);
        assert!(
            fill_order(
                Side::Sell,
                OrderType::Limit { price: 110.0 },
                bar(100.0, 106.0, 99.0, 101.0)
            )
            .is_none()
        );
    }

    #[test]
    fn buy_stop_triggers_at_level_or_open_gap() {
        // high reaches the stop → fill at the stop level.
        let f = fill_order(
            Side::Buy,
            OrderType::Stop { trigger: 105.0 },
            bar(100.0, 106.0, 99.0, 104.0),
        )
        .unwrap();
        assert_eq!(f.kind, FillKind::Stop);
        assert_eq!(f.price, 105.0);
        // open gaps above the stop → fill at open.
        let g = fill_order(
            Side::Buy,
            OrderType::Stop { trigger: 105.0 },
            bar(107.0, 108.0, 106.0, 107.5),
        )
        .unwrap();
        assert_eq!(g.price, 107.0);
        // never reaches → rests.
        assert!(
            fill_order(
                Side::Buy,
                OrderType::Stop { trigger: 105.0 },
                bar(100.0, 103.0, 99.0, 101.0)
            )
            .is_none()
        );
    }

    #[test]
    fn stop_limit_triggered_but_limit_unreached_rests() {
        // trigger 105 is hit (high 106) but limit 104 is never traded (low 105.5) → no fill.
        assert!(
            fill_order(
                Side::Buy,
                OrderType::StopLimit {
                    trigger: 105.0,
                    limit: 104.0
                },
                bar(100.5, 106.0, 105.5, 105.8)
            )
            .is_none()
        );
    }

    #[test]
    fn stop_limit_triggered_and_filled_at_limit() {
        let f = fill_order(
            Side::Buy,
            OrderType::StopLimit {
                trigger: 105.0,
                limit: 104.0,
            },
            bar(100.0, 106.0, 103.5, 104.5),
        )
        .unwrap();
        assert_eq!(f.kind, FillKind::StopLimit);
        assert_eq!(f.price, 104.0);
    }

    #[test]
    fn bracket_pessimistic_stop_before_target_on_double_touch() {
        // long: bar touches BOTH tp (110) and sl (94) → SL wins (pessimistic).
        let (leg, px) = resolve_bracket(true, 110.0, 94.0, bar(100.0, 111.0, 93.0, 105.0)).unwrap();
        assert_eq!(leg, ExitLeg::StopLoss);
        assert_eq!(px, 94.0);
    }

    #[test]
    fn bracket_take_profit_only() {
        let (leg, px) = resolve_bracket(true, 110.0, 94.0, bar(100.0, 111.0, 99.0, 108.0)).unwrap();
        assert_eq!(leg, ExitLeg::TakeProfit);
        assert_eq!(px, 110.0);
    }

    #[test]
    fn bracket_short_mirrors_and_none_when_untouched() {
        // short: SL above (106) touched → stop.
        let (leg, _) = resolve_bracket(false, 90.0, 106.0, bar(100.0, 107.0, 95.0, 101.0)).unwrap();
        assert_eq!(leg, ExitLeg::StopLoss);
        // neither level touched → rests.
        assert!(resolve_bracket(true, 110.0, 90.0, bar(100.0, 105.0, 95.0, 101.0)).is_none());
    }
}
