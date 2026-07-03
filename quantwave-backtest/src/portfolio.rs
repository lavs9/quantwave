//! Shared-capital portfolio simulation (quantwave-qzpi.6–10).
//!
//! Single cash pool across symbols; bar-by-bar processing at each timestamp.
//! See `planning/SHARED_CAPITAL_PORTFOLIO_ADR.md`.

use crate::{
    apply_signal_modifiers, stops, BacktestConfig, BacktestError, BacktestResult, EquityPoint,
    ExecutionDelay, ExecutionModel, InitialRiskPositionSizer, StopConfig, StrategySignal, Trade,
};
use chrono::{DateTime, Utc};
use quantwave_core::traits::Next;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// How multi-symbol runs allocate capital across symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PortfolioMode {
    /// Each symbol gets its own `initial_cash` book (legacy default).
    #[default]
    IndependentBooks,
    /// Single cash pool shared across all symbols.
    SharedCapital,
}

/// Budget split when opening positions in shared-capital mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum PortfolioAllocator {
    /// `equity / N` for N symbols with non-zero entry intent.
    #[default]
    EqualWeight,
    /// `equity × (|signal| / Σ|signal|)`.
    SignalWeighted,
}

/// Bar with symbol tag for shared-capital streaming parity.
#[derive(Debug, Clone, PartialEq)]
pub struct PortfolioBar {
    pub ts: DateTime<Utc>,
    pub symbol: String,
    pub close: f64,
    pub high: Option<f64>,
    pub low: Option<f64>,
}

struct SymbolBar {
    symbol: String,
    close: f64,
    high: Option<f64>,
    low: Option<f64>,
    raw_signal: f64,
    meta: Option<HashMap<String, f64>>,
}

pub(crate) struct TimestampGroup {
    ts: DateTime<Utc>,
    bars: Vec<SymbolBar>,
}

#[derive(Clone)]
struct SymbolBook {
    exposure: f64,
    entry_price: f64,
    entry_ts: Option<DateTime<Utc>>,
    entry_metadata: Option<HashMap<String, f64>>,
    stop_state: stops::StopPositionState,
    trade_id: u32,
}

/// Allocate signed units for a new entry given portfolio equity and peer intents.
fn allocate_entry_units(
    allocator: PortfolioAllocator,
    raw_desired: f64,
    price: f64,
    equity: f64,
    peer_intents: &[(f64, f64)], // (abs_weight, price) for each entering symbol
    my_weight: f64,
) -> f64 {
    if !price.is_finite() || price <= 0.0 || equity <= 0.0 || raw_desired == 0.0 {
        return 0.0;
    }
    let sign = if raw_desired > 0.0 { 1.0 } else { -1.0 };
    let budget = match allocator {
        PortfolioAllocator::EqualWeight => {
            let n = peer_intents.len().max(1) as f64;
            equity / n
        }
        PortfolioAllocator::SignalWeighted => {
            let total: f64 = peer_intents.iter().map(|(w, _)| w).sum();
            if total <= f64::EPSILON {
                equity / peer_intents.len().max(1) as f64
            } else {
                equity * (my_weight / total)
            }
        }
    };
    let units_from_budget = budget / price;
    let cap = raw_desired.abs();
    sign * units_from_budget.min(cap)
}

fn mark_to_market_equity(cash: f64, books: &HashMap<String, SymbolBook>, prices: &HashMap<String, f64>) -> f64 {
    let mut eq = cash;
    for (sym, book) in books {
        if book.exposure != 0.0 {
            if let Some(&px) = prices.get(sym) {
                eq += book.exposure * px;
            }
        }
    }
    eq
}

/// Core shared-capital simulator (batch + streaming).
pub(crate) fn simulate_shared_capital(
    groups: &[TimestampGroup],
    exec: &ExecutionModel,
    sizer: &Option<InitialRiskPositionSizer>,
    _delay: ExecutionDelay,
    stops: &StopConfig,
    allocator: PortfolioAllocator,
) -> (Vec<Trade>, HashMap<String, Vec<EquityPoint>>, Vec<EquityPoint>) {
    let initial_cash = match exec {
        ExecutionModel::Simple(cm) => cm.initial_cash,
        ExecutionModel::HighFidelity { .. } => 100_000.0,
    };
    let mut cash = initial_cash;
    let mut books: HashMap<String, SymbolBook> = HashMap::new();
    let mut trade_id: u32 = 0;
    let mut trades: Vec<Trade> = Vec::new();
    let mut per_symbol_equity: HashMap<String, Vec<EquityPoint>> = HashMap::new();
    let mut portfolio_curve: Vec<EquityPoint> = Vec::with_capacity(groups.len());

    let mut record_exit = |cash: &mut f64,
                           sym: &str,
                           book: &SymbolBook,
                           exit_bar_ts: DateTime<Utc>,
                           exit_close: f64| {
        let qty = book.exposure.abs();
        let side = if book.exposure > 0.0 { 1 } else { -1 };
        let is_buy = side == -1;
        let fill_price = exec.slippage_price(exit_close, qty, is_buy, None);
        let notional = fill_price * qty;
        let cost = exec.commission_for(qty, fill_price);
        let gross_pnl = if side == 1 {
            (fill_price - book.entry_price) * qty
        } else {
            (book.entry_price - fill_price) * qty
        };
        let net_pnl = gross_pnl - cost;
        if side == 1 {
            *cash += notional - cost;
        } else {
            *cash -= notional + cost;
        }
        trades.push(Trade {
            trade_id: book.trade_id,
            symbol: Some(sym.to_string()),
            side,
            entry_ts: book.entry_ts.unwrap_or(exit_bar_ts),
            entry_price: book.entry_price,
            entry_fill_price: book.entry_price,
            exit_ts: Some(exit_bar_ts),
            exit_price: Some(exit_close),
            exit_fill_price: Some(fill_price),
            pnl_gross: gross_pnl,
            costs: cost,
            pnl_net: net_pnl,
            quantity: qty,
            entry_metadata: book.entry_metadata.clone(),
        });
    };

    let open_position = |cash: &mut f64,
                         tid: u32,
                         desired: f64,
                         fill_ts: DateTime<Utc>,
                         fill_close: f64,
                         meta: Option<HashMap<String, f64>>|
     -> SymbolBook {
        let qty = desired.abs();
        let is_long = desired > 0.0;
        let fill_price = exec.slippage_price(fill_close, qty, is_long, None);
        let notional = fill_price * qty;
        let cost = exec.commission_for(qty, fill_price);
        if is_long {
            *cash -= notional + cost;
        } else {
            *cash += notional - cost;
        }
        let exposure = if is_long { qty } else { -qty };
        let mut stop_state = stops::StopPositionState::default();
        if let Some(pct) = stops.trailing_stop_pct {
            stop_state.trailing_stop_level =
                Some(stops::trailing_level_at_entry(fill_price, is_long, pct));
        }
        SymbolBook {
            exposure,
            entry_price: fill_price,
            entry_ts: Some(fill_ts),
            entry_metadata: meta,
            stop_state,
            trade_id: tid,
        }
    };

    for group in groups {
        let ts = group.ts;
        let mut prices: HashMap<String, f64> = HashMap::new();
        for bar in &group.bars {
            prices.insert(bar.symbol.clone(), bar.close);
        }

        // Stop checks per symbol
        for bar in &group.bars {
            let sym = &bar.symbol;
            let close = bar.close;
            if !close.is_finite() {
                continue;
            }
            let Some(book) = books.get_mut(sym) else {
                continue;
            };
            if book.exposure == 0.0 || !stops.has_stops() {
                continue;
            }
            let is_long = book.exposure > 0.0;
            let ohlc = stops::OhlcBar {
                close,
                high: bar.high,
                low: bar.low,
            };
            if let Some(stop_exit) =
                stops::evaluate_stops(stops, ohlc, is_long, book.entry_price, &mut book.stop_state)
            {
                let snapshot = book.clone();
                record_exit(
                    &mut cash,
                    sym,
                    &snapshot,
                    ts,
                    stop_exit.exit_price,
                );
                *book = SymbolBook {
                    exposure: 0.0,
                    entry_price: 0.0,
                    entry_ts: None,
                    entry_metadata: None,
                    stop_state: stops::StopPositionState::default(),
                    trade_id: snapshot.trade_id,
                };
            }
        }

        // Collect entry intents for this bar (after stops)
        let mut desired_map: HashMap<String, f64> = HashMap::new();
        let mut meta_map: HashMap<String, Option<HashMap<String, f64>>> = HashMap::new();
        for (idx, bar) in group.bars.iter().enumerate() {
            let raw = apply_signal_modifiers(bar.raw_signal, None, None);
            let sized = if let Some(s) = sizer {
                let eq = mark_to_market_equity(cash, &books, &prices);
                s.compute_sized_exposure(raw, &bar.meta, bar.close, eq)
            } else {
                raw
            };
            desired_map.insert(bar.symbol.clone(), sized);
            meta_map.insert(bar.symbol.clone(), bar.meta.clone());
            let _ = idx;
        }

        let eq = mark_to_market_equity(cash, &books, &prices);
        let mut entry_peers: Vec<(String, f64, f64)> = Vec::new(); // sym, weight, price
        for (sym, &desired) in &desired_map {
            if desired == 0.0 {
                continue;
            }
            let in_pos = books.get(sym).map(|b| b.exposure != 0.0).unwrap_or(false);
            if !in_pos {
                entry_peers.push((sym.clone(), desired.abs(), prices[sym]));
            }
        }

        // Execute signals per symbol (deterministic symbol order)
        let mut syms: Vec<&String> = desired_map.keys().collect();
        syms.sort();
        for sym in syms {
            let close = prices[sym];
            if !close.is_finite() {
                continue;
            }
            let desired_raw = desired_map[sym];
            let meta = meta_map[sym].clone();

            let book = books.entry(sym.clone()).or_insert(SymbolBook {
                exposure: 0.0,
                entry_price: 0.0,
                entry_ts: None,
                entry_metadata: None,
                stop_state: stops::StopPositionState::default(),
                trade_id: 0,
            });

            if desired_raw == 0.0 && book.exposure != 0.0 {
                let snapshot = book.clone();
                record_exit(&mut cash, sym, &snapshot, ts, close);
                *book = SymbolBook {
                    exposure: 0.0,
                    entry_price: 0.0,
                    entry_ts: None,
                    entry_metadata: None,
                    stop_state: stops::StopPositionState::default(),
                    trade_id: snapshot.trade_id,
                };
                continue;
            }

            if desired_raw == 0.0 {
                continue;
            }

            let want_long = desired_raw > 0.0;
            let in_long = book.exposure > 0.0;
            let in_short = book.exposure < 0.0;
            let flip = (want_long && in_short) || (!want_long && in_long);

            if flip {
                let snapshot = book.clone();
                record_exit(&mut cash, sym, &snapshot, ts, close);
                *book = SymbolBook {
                    exposure: 0.0,
                    entry_price: 0.0,
                    entry_ts: None,
                    entry_metadata: None,
                    stop_state: stops::StopPositionState::default(),
                    trade_id: snapshot.trade_id,
                };
            }

            if book.exposure == 0.0 {
                let peers: Vec<(f64, f64)> = entry_peers
                    .iter()
                    .map(|(_, w, p)| (*w, *p))
                    .collect();
                let my_weight = desired_raw.abs();
                let allocated = allocate_entry_units(
                    allocator,
                    desired_raw,
                    close,
                    eq,
                    &peers,
                    my_weight,
                );
                if allocated != 0.0 {
                    trade_id += 1;
                    *book = open_position(&mut cash, trade_id, allocated, ts, close, meta);
                }
            }
        }

        // Record equity points
        let total_eq = mark_to_market_equity(cash, &books, &prices);
        for bar in &group.bars {
            let sym = &bar.symbol;
            let book = books.get(sym);
            let (pos, sym_cash, sym_eq) = match book {
                Some(b) if b.exposure != 0.0 => {
                    let pos_val = b.exposure * bar.close;
                    (b.exposure, cash, cash + pos_val)
                }
                _ => (0.0, cash, cash),
            };
            per_symbol_equity
                .entry(sym.clone())
                .or_default()
                .push(EquityPoint {
                    ts,
                    symbol: Some(sym.clone()),
                    equity: sym_eq,
                    cash: sym_cash,
                    position: pos,
                    close: bar.close,
                });
        }
        portfolio_curve.push(EquityPoint {
            ts,
            symbol: None,
            equity: total_eq,
            cash,
            position: books.values().map(|b| b.position_value()).sum(),
            close: 0.0,
        });
    }

    // Terminal open trades
    if let Some(last) = groups.last() {
        let ts = last.ts;
        for (sym, book) in &books {
            if book.exposure != 0.0 {
                let close = last
                    .bars
                    .iter()
                    .find(|b| &b.symbol == sym)
                    .map(|b| b.close)
                    .unwrap_or(0.0);
                let qty = book.exposure.abs();
                let side = if book.exposure > 0.0 { 1 } else { -1 };
                let gross = if side == 1 {
                    (close - book.entry_price) * qty
                } else {
                    (book.entry_price - close) * qty
                };
                if let Some(ets) = book.entry_ts {
                    trades.push(Trade {
                        trade_id: book.trade_id,
                        symbol: Some(sym.clone()),
                        side,
                        entry_ts: ets,
                        entry_price: book.entry_price,
                        entry_fill_price: book.entry_price,
                        exit_ts: None,
                        exit_price: Some(close),
                        exit_fill_price: None,
                        pnl_gross: gross,
                        costs: 0.0,
                        pnl_net: gross,
                        quantity: qty,
                        entry_metadata: book.entry_metadata.clone(),
                    });
                }
            }
            let _ = ts;
        }
    }

    (trades, per_symbol_equity, portfolio_curve)
}

impl SymbolBook {
    fn position_value(&self) -> f64 {
        self.exposure
    }
}

/// Build timestamp groups from a sorted long-format DataFrame.
pub(crate) fn build_timestamp_groups(
    signal_vals: &[f64],
    signal_metas: &[Option<HashMap<String, f64>>],
    symbols: &[String],
    timestamps: &[DateTime<Utc>],
    closes: &[f64],
    highs: Option<&[f64]>,
    lows: Option<&[f64]>,
) -> Vec<TimestampGroup> {
    let mut groups: Vec<TimestampGroup> = Vec::new();
    let mut i = 0usize;
    while i < timestamps.len() {
        let ts = timestamps[i];
        let mut bars = Vec::new();
        while i < timestamps.len() && timestamps[i] == ts {
            bars.push(SymbolBar {
                symbol: symbols[i].clone(),
                close: closes[i],
                high: highs.and_then(|h| h.get(i).copied()),
                low: lows.and_then(|l| l.get(i).copied()),
                raw_signal: signal_vals[i],
                meta: signal_metas[i].clone(),
            });
            i += 1;
        }
        bars.sort_by(|a, b| a.symbol.cmp(&b.symbol));
        groups.push(TimestampGroup { ts, bars });
    }
    groups
}

/// Run shared-capital streaming simulation for batch↔streaming parity.
pub fn run_shared_capital_streaming_simulation<G>(
    bars: &[PortfolioBar],
    mut generator: G,
    config: BacktestConfig,
) -> Result<BacktestResult, BacktestError>
where
    G: for<'a> Next<&'a PortfolioBar, Output = StrategySignal>,
{
    if bars.is_empty() {
        return Err(BacktestError::InvalidInput("empty bars".into()));
    }
    if config.symbol_col.is_none() {
        return Err(BacktestError::InvalidInput(
            "shared-capital streaming requires symbol_col".into(),
        ));
    }

    let mut groups_map: BTreeMap<DateTime<Utc>, Vec<SymbolBar>> = BTreeMap::new();
    for bar in bars {
        let sig = generator.next(bar);
        groups_map
            .entry(bar.ts)
            .or_default()
            .push(SymbolBar {
                symbol: bar.symbol.clone(),
                close: bar.close,
                high: bar.high,
                low: bar.low,
                raw_signal: sig.exposure,
                meta: sig.metadata.clone(),
            });
    }
    let groups: Vec<TimestampGroup> = groups_map
        .into_iter()
        .map(|(ts, mut bars)| {
            bars.sort_by(|a, b| a.symbol.cmp(&b.symbol));
            TimestampGroup { ts, bars }
        })
        .collect();

    let exec = &config.execution_model;
    let sizer = &config.position_sizer;
    let delay = config.execution_delay;
    let stops = &config.stop_config;
    let allocator = config.portfolio_allocator;

    let (trades, per_symbol_equity, portfolio_eq) = simulate_shared_capital(
        &groups,
        exec,
        sizer,
        delay,
        stops,
        allocator,
    );

    crate::BacktestEngine::assemble_shared_capital_result(
        &config,
        trades,
        per_symbol_equity,
        portfolio_eq,
    )
}