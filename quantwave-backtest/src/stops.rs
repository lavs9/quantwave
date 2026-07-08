//! Shared stop-loss / take-profit / trailing evaluation (quantwave-m8z5).
//!
//! Supports close-only (legacy) and OHLC touched-exit modes.
//! Sources: polars-backtest `touched_exit`, RaptorBT stop semantics (clean-room).

use serde::{Deserialize, Serialize};

/// How stop/target levels are evaluated against each bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StopEvaluationMode {
    /// Legacy: all checks use bar close (and trailing ratchets on close).
    #[default]
    CloseOnly,
    /// Intrabar: SL/trailing use low (long) / high (short); TP uses high (long) / low (short);
    /// trailing ratchets on bar high (long) / low (short). SL checked before TP when both touch.
    OhlcTouched,
}

/// Fixed / trailing stop and take-profit knobs (RaptorBT-inspired, clean-room).
///
/// Percentages are fractions of entry price for long positions (e.g. `0.02` = 2%).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StopConfig {
    /// Fixed stop-loss below entry (long): exit when price breaches entry * (1 - pct).
    pub stop_loss_pct: Option<f64>,
    /// Fixed take-profit above entry: exit when price breaches entry * (1 + pct).
    pub take_profit_pct: Option<f64>,
    /// Trailing stop from peak (long uses bar high in OHLC mode, close otherwise).
    pub trailing_stop_pct: Option<f64>,
    /// How stops are evaluated against each bar. Default `CloseOnly` preserves v0.6 behavior.
    #[serde(default)]
    pub stop_evaluation: StopEvaluationMode,
}

impl StopConfig {
    pub fn has_stops(&self) -> bool {
        self.stop_loss_pct.is_some()
            || self.take_profit_pct.is_some()
            || self.trailing_stop_pct.is_some()
    }
}

/// OHLC slice for one bar (missing high/low fall back to close).
#[derive(Debug, Clone, Copy)]
pub struct OhlcBar {
    pub close: f64,
    pub high: Option<f64>,
    pub low: Option<f64>,
}

impl OhlcBar {
    fn high_px(&self) -> f64 {
        self.high.unwrap_or(self.close)
    }

    fn low_px(&self) -> f64 {
        self.low.unwrap_or(self.close)
    }
}

/// Mutable trailing-stop state while a position is open.
#[derive(Debug, Clone, Default)]
pub struct StopPositionState {
    pub trailing_stop_level: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopExitKind {
    TakeProfit,
    StopLoss,
    TrailingStop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StopExit {
    pub kind: StopExitKind,
    /// Raw exit price before slippage (stop level when touched in OHLC mode).
    pub exit_price: f64,
}

/// Initial trailing level at position entry.
pub fn trailing_level_at_entry(entry_price: f64, is_long: bool, trail_pct: f64) -> f64 {
    if is_long {
        entry_price * (1.0 - trail_pct)
    } else {
        entry_price * (1.0 + trail_pct)
    }
}

/// Ratchet trailing stop using the current bar, updating `state`.
pub fn ratchet_trailing(
    stop_config: &StopConfig,
    bar: OhlcBar,
    is_long: bool,
    state: &mut StopPositionState,
) {
    let Some(trail_pct) = stop_config.trailing_stop_pct else {
        return;
    };
    let ratchet_px = match stop_config.stop_evaluation {
        StopEvaluationMode::CloseOnly => bar.close,
        StopEvaluationMode::OhlcTouched => {
            if is_long {
                bar.high_px()
            } else {
                bar.low_px()
            }
        }
    };
    if is_long {
        let new_level = ratchet_px * (1.0 - trail_pct);
        state.trailing_stop_level = Some(match state.trailing_stop_level {
            Some(prev) => prev.max(new_level),
            None => new_level,
        });
    } else {
        let new_level = ratchet_px * (1.0 + trail_pct);
        state.trailing_stop_level = Some(match state.trailing_stop_level {
            Some(prev) => prev.min(new_level),
            None => new_level,
        });
    }
}

/// Evaluate stops for an open position. Returns `Some(StopExit)` when a stop fires.
pub fn evaluate_stops(
    stop_config: &StopConfig,
    bar: OhlcBar,
    is_long: bool,
    entry_price: f64,
    state: &mut StopPositionState,
) -> Option<StopExit> {
    if !stop_config.has_stops() {
        return None;
    }

    ratchet_trailing(stop_config, bar, is_long, state);

    let close = bar.close;
    let high = bar.high_px();
    let low = bar.low_px();
    let mode = stop_config.stop_evaluation;

    if is_long {
        evaluate_stops_long(stop_config, mode, close, high, low, entry_price, state)
    } else {
        evaluate_stops_short(stop_config, mode, close, high, low, entry_price, state)
    }
}

fn evaluate_stops_long(
    stop_config: &StopConfig,
    mode: StopEvaluationMode,
    close: f64,
    high: f64,
    low: f64,
    entry_price: f64,
    state: &StopPositionState,
) -> Option<StopExit> {
    match mode {
        StopEvaluationMode::CloseOnly => {
            if let Some(tp) = stop_config.take_profit_pct
                && close >= entry_price * (1.0 + tp)
            {
                return Some(StopExit {
                    kind: StopExitKind::TakeProfit,
                    exit_price: close,
                });
            }
            let mut effective_stop = f64::NEG_INFINITY;
            if let Some(sl) = stop_config.stop_loss_pct {
                effective_stop = entry_price * (1.0 - sl);
            }
            if let Some(level) = state.trailing_stop_level {
                effective_stop = effective_stop.max(level);
            }
            if effective_stop > f64::NEG_INFINITY && close <= effective_stop {
                let kind = if stop_config
                    .stop_loss_pct
                    .is_some_and(|sl| close <= entry_price * (1.0 - sl))
                {
                    StopExitKind::StopLoss
                } else {
                    StopExitKind::TrailingStop
                };
                return Some(StopExit {
                    kind,
                    exit_price: close,
                });
            }
        }
        StopEvaluationMode::OhlcTouched => {
            // Conservative intrabar ordering: SL / trailing before TP.
            if let Some(sl) = stop_config.stop_loss_pct {
                let level = entry_price * (1.0 - sl);
                if low <= level {
                    return Some(StopExit {
                        kind: StopExitKind::StopLoss,
                        exit_price: level,
                    });
                }
            }
            if let Some(level) = state.trailing_stop_level
                && low <= level
            {
                return Some(StopExit {
                    kind: StopExitKind::TrailingStop,
                    exit_price: level,
                });
            }
            if let Some(tp) = stop_config.take_profit_pct {
                let level = entry_price * (1.0 + tp);
                if high >= level {
                    return Some(StopExit {
                        kind: StopExitKind::TakeProfit,
                        exit_price: level,
                    });
                }
            }
        }
    }
    None
}

fn evaluate_stops_short(
    stop_config: &StopConfig,
    mode: StopEvaluationMode,
    close: f64,
    high: f64,
    low: f64,
    entry_price: f64,
    state: &StopPositionState,
) -> Option<StopExit> {
    match mode {
        StopEvaluationMode::CloseOnly => {
            if let Some(tp) = stop_config.take_profit_pct
                && close <= entry_price * (1.0 - tp)
            {
                return Some(StopExit {
                    kind: StopExitKind::TakeProfit,
                    exit_price: close,
                });
            }
            let mut effective_stop = f64::INFINITY;
            if let Some(sl) = stop_config.stop_loss_pct {
                effective_stop = entry_price * (1.0 + sl);
            }
            if let Some(level) = state.trailing_stop_level {
                effective_stop = effective_stop.min(level);
            }
            if effective_stop < f64::INFINITY && close >= effective_stop {
                let kind = if stop_config
                    .stop_loss_pct
                    .is_some_and(|sl| close >= entry_price * (1.0 + sl))
                {
                    StopExitKind::StopLoss
                } else {
                    StopExitKind::TrailingStop
                };
                return Some(StopExit {
                    kind,
                    exit_price: close,
                });
            }
        }
        StopEvaluationMode::OhlcTouched => {
            if let Some(sl) = stop_config.stop_loss_pct {
                let level = entry_price * (1.0 + sl);
                if high >= level {
                    return Some(StopExit {
                        kind: StopExitKind::StopLoss,
                        exit_price: level,
                    });
                }
            }
            if let Some(level) = state.trailing_stop_level
                && high >= level
            {
                return Some(StopExit {
                    kind: StopExitKind::TrailingStop,
                    exit_price: level,
                });
            }
            if let Some(tp) = stop_config.take_profit_pct {
                let level = entry_price * (1.0 - tp);
                if low <= level {
                    return Some(StopExit {
                        kind: StopExitKind::TakeProfit,
                        exit_price: level,
                    });
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ohlc_stop_loss_wick_exits_at_level() {
        let cfg = StopConfig {
            stop_loss_pct: Some(0.02),
            stop_evaluation: StopEvaluationMode::OhlcTouched,
            ..Default::default()
        };
        let mut state = StopPositionState::default();
        let bar = OhlcBar {
            close: 99.0,
            high: Some(100.0),
            low: Some(96.0),
        };
        let exit = evaluate_stops(&cfg, bar, true, 100.0, &mut state).unwrap();
        assert_eq!(exit.kind, StopExitKind::StopLoss);
        assert!((exit.exit_price - 98.0).abs() < 1e-9);
    }

    #[test]
    fn close_only_stop_waits_for_close_breach() {
        let cfg = StopConfig {
            stop_loss_pct: Some(0.02),
            stop_evaluation: StopEvaluationMode::CloseOnly,
            ..Default::default()
        };
        let mut state = StopPositionState::default();
        let bar = OhlcBar {
            close: 99.0,
            high: Some(100.0),
            low: Some(96.0),
        };
        assert!(evaluate_stops(&cfg, bar, true, 100.0, &mut state).is_none());
    }
}
