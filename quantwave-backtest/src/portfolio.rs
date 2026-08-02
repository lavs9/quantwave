//! Shared-capital portfolio simulation (quantwave-qzpi.6–10).
//!
//! Single cash pool across symbols; bar-by-bar processing at each timestamp.
//! See `planning/SHARED_CAPITAL_PORTFOLIO_ADR.md`.

use crate::{
    BacktestConfig, BacktestError, BacktestResult, EquityPoint, ExecutionDelay, ExecutionModel,
    InitialRiskPositionSizer, StopConfig, StrategySignal, Trade, apply_signal_modifiers, stops,
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

/// Decides, per bar, whether the shared-capital simulator re-evaluates
/// target weights this bar (opening/closing/flipping positions per the
/// current signals), or holds all existing positions unchanged and defers
/// the decision to a later bar (quantwave-nbrx).
///
/// Stop-loss / take-profit / trailing exits (`StopConfig`) are a separate
/// risk-management concern and are always evaluated regardless of this
/// policy — only *signal-driven* entries/exits/flips are gated.
///
/// `None` (the default) rebalances every bar, which is today's behavior —
/// runs without a policy are byte-identical to pre-policy output.
///
/// Concepts (clean-room; no code copied from any source): the
/// calendar / threshold(drift) / signal-change / turnover-budget trigger
/// taxonomy is a standard staple of the portfolio-rebalancing literature —
/// see Perold & Sharpe (1988), "Dynamic Strategies for Asset Allocation",
/// J. Portfolio Management, for the calendar-vs-threshold rebalancing
/// trade-off, and Donohue & Yip (2003), "Optimal Portfolio Rebalancing with
/// Transaction Costs", J. Portfolio Management, for turnover/no-trade-band
/// rebalancing. QuantJourney-bt (inspiration only, not consulted for
/// implementation) exposes an analogous calendar/drift rebalance-trigger
/// split; only the general concept (not any code or API shape) is reused
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RebalancePolicy {
    /// Rebalance every `every_n_bars` bars, counted from the start of the
    /// run (bar 0 always rebalances). `0` and `1` both behave like "every
    /// bar" (today's default).
    Calendar { every_n_bars: usize },
    /// Rebalance only when some symbol's mark-to-market weight
    /// (`position notional / portfolio equity`) has drifted more than
    /// `threshold` (a fraction of equity, e.g. `0.05` = 5 percentage
    /// points) away from the weight it would be assigned if it were
    /// (re)entered fresh this bar under the configured `PortfolioAllocator`.
    Drift { threshold: f64 },
    /// Rebalance only when the raw strategy signal (pre-sizer, pre-modifier)
    /// for some symbol differs from the raw signal in effect at the last
    /// rebalance — i.e. only on a genuine target-weight change (new symbol,
    /// sign flip, or magnitude change).
    Signal,
    /// Skip a rebalance whose estimated turnover — the sum, across symbols,
    /// of `|target_weight - current_weight|` for every symbol that would
    /// need a close/open/flip this bar — would fall below `min_turnover`
    /// (a fraction of equity). Caps how often small weight changes churn
    /// the book.
    Turnover { min_turnover: f64 },
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

/// Hypothetical mark-to-market weight (`notional / equity`, signed) a symbol
/// would receive if it were (re)entered fresh this bar under `allocator`,
/// given every symbol with a non-zero desired signal this bar as its peer
/// set. Used only for `RebalancePolicy::Drift` / `Turnover` decisions — it
/// does not execute a trade or mutate any book. Pure function of
/// `desired_map` / `price` / `equity`, so it is deterministic and
/// lookahead-free (data available at the current bar only), keeping batch
/// and streaming in lockstep.
fn hypothetical_target_weight(
    allocator: PortfolioAllocator,
    desired_map: &HashMap<String, f64>,
    sym: &str,
    price: f64,
    equity: f64,
) -> f64 {
    if equity <= 0.0 || !price.is_finite() || price <= 0.0 {
        return 0.0;
    }
    let desired = desired_map.get(sym).copied().unwrap_or(0.0);
    if desired == 0.0 {
        return 0.0;
    }
    let sign = if desired > 0.0 { 1.0 } else { -1.0 };
    let active: Vec<f64> = desired_map
        .values()
        .filter(|v| **v != 0.0)
        .map(|v| v.abs())
        .collect();
    let budget = match allocator {
        PortfolioAllocator::EqualWeight => equity / active.len().max(1) as f64,
        PortfolioAllocator::SignalWeighted => {
            let total: f64 = active.iter().sum();
            if total <= f64::EPSILON {
                equity / active.len().max(1) as f64
            } else {
                equity * (desired.abs() / total)
            }
        }
    };
    let units = (budget / price).min(desired.abs());
    sign * units * price / equity
}

/// Decide whether this bar re-evaluates target weights (signal-driven
/// entries/exits/flips), per `policy`. `bar_index` counts bars from the
/// start of the run (for `Calendar`). `last_rebalance_signal` is the raw
/// signal map as of the last bar that *did* rebalance (for `Signal`); `None`
/// before the first rebalance. Reads only `books` / `prices` / `desired_map`
/// / `eq` as computed up to and including the current bar — no lookahead.
#[allow(clippy::too_many_arguments)]
fn should_rebalance(
    policy: Option<RebalancePolicy>,
    allocator: PortfolioAllocator,
    bar_index: usize,
    books: &HashMap<String, SymbolBook>,
    prices: &HashMap<String, f64>,
    desired_map: &HashMap<String, f64>,
    raw_signal_map: &HashMap<String, f64>,
    last_rebalance_signal: &Option<HashMap<String, f64>>,
    eq: f64,
) -> bool {
    match policy {
        None => true,
        Some(RebalancePolicy::Calendar { every_n_bars }) => {
            let n = every_n_bars.max(1);
            bar_index.is_multiple_of(n)
        }
        Some(RebalancePolicy::Signal) => match last_rebalance_signal {
            None => true,
            Some(prev) => {
                prev.len() != raw_signal_map.len()
                    || raw_signal_map
                        .iter()
                        .any(|(sym, v)| prev.get(sym).copied().unwrap_or(0.0) != *v)
            }
        },
        Some(RebalancePolicy::Drift { threshold }) => {
            if eq <= 0.0 {
                return false;
            }
            desired_map.keys().chain(books.keys()).any(|sym| {
                let price = prices.get(sym).copied().unwrap_or(0.0);
                let current_weight = books
                    .get(sym)
                    .map(|b| b.exposure * price / eq)
                    .unwrap_or(0.0);
                let target_weight =
                    hypothetical_target_weight(allocator, desired_map, sym, price, eq);
                (target_weight - current_weight).abs() > threshold
            })
        }
        Some(RebalancePolicy::Turnover { min_turnover }) => {
            if eq <= 0.0 {
                return true;
            }
            let mut turnover = 0.0;
            let mut seen: std::collections::HashSet<&String> = std::collections::HashSet::new();
            for sym in desired_map.keys().chain(books.keys()) {
                if !seen.insert(sym) {
                    continue;
                }
                let price = prices.get(sym).copied().unwrap_or(0.0);
                let current_weight = books
                    .get(sym)
                    .map(|b| b.exposure * price / eq)
                    .unwrap_or(0.0);
                let target_weight =
                    hypothetical_target_weight(allocator, desired_map, sym, price, eq);
                turnover += (target_weight - current_weight).abs();
            }
            turnover >= min_turnover
        }
    }
}

fn mark_to_market_equity(
    cash: f64,
    books: &HashMap<String, SymbolBook>,
    prices: &HashMap<String, f64>,
) -> f64 {
    let mut eq = cash;
    for (sym, book) in books {
        if book.exposure != 0.0
            && let Some(&px) = prices.get(sym)
        {
            eq += book.exposure * px;
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
    rebalance_policy: Option<RebalancePolicy>,
) -> (
    Vec<Trade>,
    HashMap<String, Vec<EquityPoint>>,
    Vec<EquityPoint>,
) {
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
    // Rebalance-policy state (quantwave-nbrx): tracked across bars so the
    // decision is a deterministic function of data up to and including the
    // current bar, applied identically in the batch and streaming paths
    // (both funnel through this one function).
    let mut last_rebalance_signal: Option<HashMap<String, f64>> = None;

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

    for (bar_index, group) in groups.iter().enumerate() {
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
                record_exit(&mut cash, sym, &snapshot, ts, stop_exit.exit_price);
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
        let mut raw_signal_map: HashMap<String, f64> = HashMap::new();
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
            raw_signal_map.insert(bar.symbol.clone(), bar.raw_signal);
            let _ = idx;
        }

        let eq = mark_to_market_equity(cash, &books, &prices);

        // Rebalance-policy gate (quantwave-nbrx): a deterministic function of
        // data up to and including this bar, applied identically whether we
        // got here from the batch or the streaming caller — both call this
        // one `simulate_shared_capital`. When `false`, all signal-driven
        // entries/exits/flips below are skipped for this bar; stop-loss
        // checks above are unaffected (separate concern, always active).
        let do_rebalance = should_rebalance(
            rebalance_policy,
            allocator,
            bar_index,
            &books,
            &prices,
            &desired_map,
            &raw_signal_map,
            &last_rebalance_signal,
            eq,
        );

        if do_rebalance {
            last_rebalance_signal = Some(raw_signal_map.clone());

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
                    let peers: Vec<(f64, f64)> =
                        entry_peers.iter().map(|(_, w, p)| (*w, *p)).collect();
                    let my_weight = desired_raw.abs();
                    let allocated =
                        allocate_entry_units(allocator, desired_raw, close, eq, &peers, my_weight);
                    if allocated != 0.0 {
                        trade_id += 1;
                        *book = open_position(&mut cash, trade_id, allocated, ts, close, meta);
                    }
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

    // Deterministic trade order: symbol books are stored in a HashMap, so exit
    // records can be pushed in a run-to-run-varying order. Sort by a stable
    // content key so batch and streaming (both via this fn) and repeated runs
    // are reproducible (entry timestamp → symbol → trade id).
    trades.sort_by(|a, b| {
        a.entry_ts
            .cmp(&b.entry_ts)
            .then_with(|| a.symbol.cmp(&b.symbol))
            .then_with(|| a.trade_id.cmp(&b.trade_id))
    });

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
/// Shift each timestamp group's signals to the group they should be executed from.
///
/// Group *gi* executes the signal observed at group `signal_bar_index(gi, delay)`,
/// matched per symbol. Symbols with no signal in the source group go flat (0.0),
/// which mirrors the batch path's `delayed_signals` default.
fn apply_group_execution_delay(groups: &mut [TimestampGroup], delay: ExecutionDelay) {
    if delay == ExecutionDelay::SameBar {
        return;
    }
    let sources: Vec<HashMap<String, (f64, Option<HashMap<String, f64>>)>> = (0..groups.len())
        .map(|gi| match crate::signal_bar_index(gi, delay) {
            Some(si) => groups[si]
                .bars
                .iter()
                .map(|b| (b.symbol.clone(), (b.raw_signal, b.meta.clone())))
                .collect(),
            None => HashMap::new(),
        })
        .collect();

    for (group, source) in groups.iter_mut().zip(sources) {
        for bar in &mut group.bars {
            match source.get(&bar.symbol) {
                Some((sig, meta)) => {
                    bar.raw_signal = *sig;
                    bar.meta = meta.clone();
                }
                None => {
                    bar.raw_signal = 0.0;
                    bar.meta = None;
                }
            }
        }
    }
}

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
        groups_map.entry(bar.ts).or_default().push(SymbolBar {
            symbol: bar.symbol.clone(),
            close: bar.close,
            high: bar.high,
            low: bar.low,
            raw_signal: sig.exposure,
            meta: sig.metadata.clone(),
        });
    }
    let mut groups: Vec<TimestampGroup> = groups_map
        .into_iter()
        .map(|(ts, mut bars)| {
            bars.sort_by(|a, b| a.symbol.cmp(&b.symbol));
            TimestampGroup { ts, bars }
        })
        .collect();

    let exec = &config.execution_model;
    let sizer = &config.position_sizer;
    let delay = config.execution_delay;

    // Apply the execution delay per timestamp group, exactly as the batch path
    // does in `BacktestEngine::run_shared_capital_multi_symbol`. `simulate_shared_capital`
    // consumes already-delayed signals, so this shift is what keeps batch↔streaming
    // parity under `ExecutionDelay::NextBar` (quantwave-zmjw — previously the
    // streaming path silently ignored the delay, which was invisible while
    // `SameBar` was the default).
    apply_group_execution_delay(&mut groups, delay);
    let stops = &config.stop_config;
    let allocator = config.portfolio_allocator;
    let rebalance_policy = config.rebalance_policy;

    let (trades, per_symbol_equity, portfolio_eq) = simulate_shared_capital(
        &groups,
        exec,
        sizer,
        delay,
        stops,
        allocator,
        rebalance_policy,
    );

    crate::BacktestEngine::assemble_shared_capital_result(
        &config,
        trades,
        per_symbol_equity,
        portfolio_eq,
    )
}

#[cfg(test)]
mod rebalance_policy_tests {
    //! Direct unit tests for `should_rebalance` (quantwave-nbrx): each
    //! trigger checked against hand-crafted books/signals so the expected
    //! rebalance bars are known exactly, without needing to hand-derive
    //! multi-bar cash-flow arithmetic. Batch↔streaming parity and the
    //! default-unchanged guarantee are covered at the integration-test
    //! level (see `tests/portfolio_rebalance_policy.rs`), since both paths
    //! share this exact function.
    use super::*;

    fn book(exposure: f64) -> SymbolBook {
        SymbolBook {
            exposure,
            entry_price: 100.0,
            entry_ts: None,
            entry_metadata: None,
            stop_state: stops::StopPositionState::default(),
            trade_id: 1,
        }
    }

    fn map1(sym: &str, v: f64) -> HashMap<String, f64> {
        let mut m = HashMap::new();
        m.insert(sym.to_string(), v);
        m
    }

    #[test]
    fn none_policy_always_rebalances() {
        for bar in 0..5 {
            assert!(should_rebalance(
                None,
                PortfolioAllocator::EqualWeight,
                bar,
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &None,
                100_000.0,
            ));
        }
    }

    #[test]
    fn calendar_rebalances_only_on_multiples_of_n() {
        let policy = Some(RebalancePolicy::Calendar { every_n_bars: 3 });
        let expected = [true, false, false, true, false, false, true];
        for (bar, want) in expected.iter().enumerate() {
            let got = should_rebalance(
                policy,
                PortfolioAllocator::EqualWeight,
                bar,
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &HashMap::new(),
                &None,
                100_000.0,
            );
            assert_eq!(got, *want, "bar {bar}: expected {want}, got {got}");
        }
    }

    #[test]
    fn calendar_zero_or_one_behaves_like_every_bar() {
        for every_n_bars in [0usize, 1usize] {
            let policy = Some(RebalancePolicy::Calendar { every_n_bars });
            for bar in 0..4 {
                assert!(should_rebalance(
                    policy,
                    PortfolioAllocator::EqualWeight,
                    bar,
                    &HashMap::new(),
                    &HashMap::new(),
                    &HashMap::new(),
                    &HashMap::new(),
                    &None,
                    100_000.0,
                ));
            }
        }
    }

    #[test]
    fn signal_rebalances_only_when_raw_signal_changes() {
        let policy = Some(RebalancePolicy::Signal);
        // First call: no prior rebalance signal -> always rebalance.
        let sig_a = map1("AAA", 1.0);
        assert!(should_rebalance(
            policy,
            PortfolioAllocator::EqualWeight,
            0,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &sig_a,
            &None,
            100_000.0,
        ));

        // Unchanged signal vs last rebalance -> skip.
        let last = Some(sig_a.clone());
        assert!(!should_rebalance(
            policy,
            PortfolioAllocator::EqualWeight,
            1,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &sig_a,
            &last,
            100_000.0,
        ));

        // Changed magnitude/sign -> rebalance.
        let sig_b = map1("AAA", -1.0);
        assert!(should_rebalance(
            policy,
            PortfolioAllocator::EqualWeight,
            2,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &sig_b,
            &last,
            100_000.0,
        ));

        // New symbol appears -> rebalance even if the shared symbol's value is
        // unchanged.
        let mut sig_c = sig_a.clone();
        sig_c.insert("BBB".to_string(), 1.0);
        assert!(should_rebalance(
            policy,
            PortfolioAllocator::EqualWeight,
            3,
            &HashMap::new(),
            &HashMap::new(),
            &HashMap::new(),
            &sig_c,
            &last,
            100_000.0,
        ));
    }

    #[test]
    fn drift_rebalances_when_weight_deviates_past_threshold() {
        let policy_tight = Some(RebalancePolicy::Drift { threshold: 0.05 });
        let policy_loose = Some(RebalancePolicy::Drift { threshold: 0.9 });
        let mut books = HashMap::new();
        // Currently fully invested in AAA (weight 1.0 given equity 10_000 and
        // exposure*price = 100 * 100 = 10_000).
        books.insert("AAA".to_string(), book(100.0));
        let mut prices = HashMap::new();
        prices.insert("AAA".to_string(), 100.0);
        prices.insert("BBB".to_string(), 100.0);
        // BBB now also wants in: with EqualWeight and 2 active symbols, AAA's
        // hypothetical fresh target weight drops to ~0.5, a 0.5 drift from
        // its current ~1.0 weight.
        let mut desired = HashMap::new();
        desired.insert("AAA".to_string(), 1_000_000.0);
        desired.insert("BBB".to_string(), 1_000_000.0);

        assert!(should_rebalance(
            policy_tight,
            PortfolioAllocator::EqualWeight,
            5,
            &books,
            &prices,
            &desired,
            &HashMap::new(),
            &None,
            10_000.0,
        ));
        assert!(!should_rebalance(
            policy_loose,
            PortfolioAllocator::EqualWeight,
            5,
            &books,
            &prices,
            &desired,
            &HashMap::new(),
            &None,
            10_000.0,
        ));
    }

    #[test]
    fn drift_no_change_stays_below_threshold() {
        // AAA alone, signal unchanged: hypothetical fresh target == current
        // weight (both derive from the same full-equity budget), so drift is
        // ~0 regardless of threshold.
        let policy = Some(RebalancePolicy::Drift { threshold: 0.001 });
        let mut books = HashMap::new();
        books.insert("AAA".to_string(), book(100.0));
        let mut prices = HashMap::new();
        prices.insert("AAA".to_string(), 100.0);
        let mut desired = HashMap::new();
        desired.insert("AAA".to_string(), 1_000_000.0);

        assert!(!should_rebalance(
            policy,
            PortfolioAllocator::EqualWeight,
            5,
            &books,
            &prices,
            &desired,
            &HashMap::new(),
            &None,
            10_000.0,
        ));
    }

    #[test]
    fn turnover_skips_small_changes_and_allows_large_ones() {
        let mut books = HashMap::new();
        books.insert("AAA".to_string(), book(100.0));
        let mut prices = HashMap::new();
        prices.insert("AAA".to_string(), 100.0);
        prices.insert("BBB".to_string(), 100.0);
        let mut desired = HashMap::new();
        desired.insert("AAA".to_string(), 1_000_000.0);
        desired.insert("BBB".to_string(), 1_000_000.0);

        // Estimated turnover here is ~0.5 (AAA's weight moving from ~1.0
        // toward ~0.5 as BBB joins, plus BBB's own ~0.5 entry) -> a high
        // budget should skip it, a low budget should allow it.
        let high_budget = Some(RebalancePolicy::Turnover { min_turnover: 5.0 });
        let low_budget = Some(RebalancePolicy::Turnover { min_turnover: 0.01 });

        assert!(!should_rebalance(
            high_budget,
            PortfolioAllocator::EqualWeight,
            5,
            &books,
            &prices,
            &desired,
            &HashMap::new(),
            &None,
            10_000.0,
        ));
        assert!(should_rebalance(
            low_budget,
            PortfolioAllocator::EqualWeight,
            5,
            &books,
            &prices,
            &desired,
            &HashMap::new(),
            &None,
            10_000.0,
        ));
    }
}
