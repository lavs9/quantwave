#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::borrow_deref_ref,
    clippy::field_reassign_with_default
)]
//! Shared-capital portfolio simulation (quantwave-qzpi.7).
//!
//! `cargo nextest run -p quantwave-backtest portfolio_shared_capital`

use approx::assert_relative_eq;
use polars::prelude::*;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, CostModel, ExecutionModel, PortfolioAllocator, PortfolioMode,
};

fn shared_capital_config(signal_col: &str) -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: signal_col.to_string(),
        symbol_col: Some("symbol".to_string()),
        portfolio_mode: PortfolioMode::SharedCapital,
        portfolio_allocator: PortfolioAllocator::EqualWeight,
        ..Default::default()
    }
}

fn make_two_symbol_df() -> DataFrame {
    let timestamps = vec![
        1_700_010_000i64,
        1_700_010_000,
        1_700_010_001,
        1_700_010_001,
        1_700_010_002,
        1_700_010_002,
        1_700_010_003,
        1_700_010_003,
        1_700_010_004,
        1_700_010_004,
    ];
    let symbols = vec![
        "AAA", "BBB", "AAA", "BBB", "AAA", "BBB", "AAA", "BBB", "AAA", "BBB",
    ];
    let closes = vec![
        100.0, 50.0, 101.0, 51.0, 102.0, 52.0, 103.0, 53.0, 104.0, 54.0,
    ];
    let signals = vec![0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

    DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("symbol".into(), symbols),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap()
}

fn portfolio_equity_series(result: &quantwave_backtest::BacktestResult) -> Vec<f64> {
    let eq = result.equity_curve.column("equity").unwrap().f64().unwrap();
    let sym = result.equity_curve.column("symbol").unwrap().str().unwrap();
    eq.into_iter()
        .zip(&*sym)
        .filter_map(|(e, s)| if s.is_none() { Some(e.unwrap()) } else { None })
        .collect()
}

#[test]
fn test_shared_capital_single_pool_initial_cash() {
    let df = make_two_symbol_df();
    let engine = BacktestEngine::new(shared_capital_config("signal"));
    let result = engine.run(df.lazy()).expect("shared-capital run");

    assert_relative_eq!(
        *result.stats.get("initial_cash").unwrap(),
        100_000.0,
        epsilon = 1e-6
    );
    assert_relative_eq!(
        *result.stats.get("portfolio_mode").unwrap(),
        1.0,
        epsilon = 1e-9
    );
}

#[test]
fn test_shared_capital_equity_not_sum_of_independent_books() {
    let df = make_two_symbol_df();
    let shared = BacktestEngine::new(shared_capital_config("signal"))
        .run(df.clone().lazy())
        .expect("shared run");

    let independent_cfg = BacktestConfig {
        symbol_col: Some("symbol".to_string()),
        portfolio_mode: PortfolioMode::IndependentBooks,
        ..shared_capital_config("signal")
    };
    let independent = BacktestEngine::new(independent_cfg)
        .run(df.lazy())
        .expect("independent run");

    let pooled = portfolio_equity_series(&shared);
    let indep_sum = portfolio_equity_series(&independent);

    assert_eq!(pooled.len(), indep_sum.len());
    let mut differs = false;
    for (p, i) in pooled.iter().zip(indep_sum.iter()) {
        if (p - i).abs() > 1.0 {
            differs = true;
            break;
        }
    }
    assert!(
        differs,
        "shared-capital pooled equity should diverge from sum of independent books"
    );
    assert_relative_eq!(
        *independent.stats.get("initial_cash").unwrap(),
        200_000.0,
        epsilon = 1e-6
    );
}

#[test]
fn test_shared_capital_independent_mode_regression() {
    let df = make_two_symbol_df();
    let cfg = BacktestConfig {
        symbol_col: Some("symbol".to_string()),
        portfolio_mode: PortfolioMode::IndependentBooks,
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: "signal".to_string(),
        ..Default::default()
    };
    let result = BacktestEngine::new(cfg).run(df.lazy()).unwrap();
    assert_relative_eq!(
        *result.stats.get("initial_cash").unwrap(),
        200_000.0,
        epsilon = 1e-6
    );
    assert_eq!(result.trades.height(), 3);
}
fn make_five_symbol_df() -> DataFrame {
    let mut timestamps = Vec::new();
    let mut symbols = Vec::new();
    let mut closes = Vec::new();
    let mut signals = Vec::new();

    let syms = ["A", "B", "C", "D", "E"];
    for i in 0..10 {
        for s in syms.iter() {
            timestamps.push(1_700_010_000 + i as i64);
            symbols.push(*s);
            closes.push(100.0 + (i as f64));
            signals.push(if i % 2 == 0 { 1.0 } else { 0.0 });
        }
    }

    DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("symbol".into(), symbols),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap()
}

#[test]
fn test_shared_capital_five_symbols_stress() {
    let df = make_five_symbol_df();
    let engine = BacktestEngine::new(shared_capital_config("signal"));
    let result = engine.run(df.lazy()).expect("five symbol run");

    assert_relative_eq!(
        *result.stats.get("initial_cash").unwrap(),
        100_000.0,
        epsilon = 1e-6
    );
    assert_eq!(result.trades.height(), 25);
}

fn make_three_symbol_df() -> DataFrame {
    let mut timestamps = Vec::new();
    let mut symbols = Vec::new();
    let mut closes = Vec::new();
    let mut signals = Vec::new();

    let syms = ["X", "Y", "Z"];
    for i in 0..8 {
        for (s_idx, s) in syms.iter().enumerate() {
            timestamps.push(1_700_020_000 + i as i64);
            symbols.push(*s);
            closes.push(100.0 + s_idx as f64 + i as f64 * 0.5);
            signals.push(match (i + s_idx) % 3 {
                0 => 0.0,
                1 => 1.0,
                _ => 0.5,
            });
        }
    }

    DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("symbol".into(), symbols),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap()
}

#[test]
fn test_shared_capital_three_symbols_stress() {
    let df = make_three_symbol_df();
    let result = BacktestEngine::new(shared_capital_config("signal"))
        .run(df.lazy())
        .expect("three symbol run");

    assert_relative_eq!(
        *result.stats.get("initial_cash").unwrap(),
        100_000.0,
        epsilon = 1e-6
    );
    assert_relative_eq!(
        *result.stats.get("num_symbols").unwrap(),
        3.0,
        epsilon = 1e-9
    );
    assert!(result.trades.height() > 0);
    let pooled = portfolio_equity_series(&result);
    assert_eq!(pooled.len(), 8);
}

fn trade_quantity(result: &quantwave_backtest::BacktestResult, symbol: &str) -> f64 {
    let trades = &result.trades;
    let sym_col = trades.column("symbol").unwrap().str().unwrap();
    let qty_col = trades.column("quantity").unwrap().f64().unwrap();
    for (sym, qty) in sym_col.into_iter().zip(&*qty_col) {
        if sym == Some(symbol) {
            return qty.unwrap();
        }
    }
    0.0
}

#[test]
fn test_signal_weighted_allocator_splits_by_signal_strength() {
    // Small pool: budget binds before unit cap. Signals 1.0 vs 3.0 → 1:3 budget split.
    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), vec![1i64, 1, 2, 2]),
        Column::new("symbol".into(), vec!["AAA", "BBB", "AAA", "BBB"]),
        Column::new("close".into(), vec![100.0, 100.0, 100.0, 100.0]),
        Column::new("signal".into(), vec![1.0, 3.0, 0.0, 0.0]),
    ])
    .unwrap();

    let base = BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100.0,
        }),
        signal_col: "signal".to_string(),
        symbol_col: Some("symbol".to_string()),
        portfolio_mode: PortfolioMode::SharedCapital,
        ..Default::default()
    };
    let equal_cfg = BacktestConfig {
        portfolio_allocator: PortfolioAllocator::EqualWeight,
        ..base.clone()
    };
    let weighted_cfg = BacktestConfig {
        portfolio_allocator: PortfolioAllocator::SignalWeighted,
        ..base
    };

    let equal = BacktestEngine::new(equal_cfg)
        .run(df.clone().lazy())
        .expect("equal weight");
    let weighted = BacktestEngine::new(weighted_cfg)
        .run(df.lazy())
        .expect("signal weighted");

    let eq_a = trade_quantity(&equal, "AAA");
    let eq_b = trade_quantity(&equal, "BBB");
    let wt_a = trade_quantity(&weighted, "AAA");
    let wt_b = trade_quantity(&weighted, "BBB");

    assert_relative_eq!(eq_a, 0.5, epsilon = 1e-6);
    assert_relative_eq!(eq_b, 0.5, epsilon = 1e-6);

    assert_relative_eq!(wt_a, 0.25, epsilon = 1e-6);
    assert_relative_eq!(wt_b, 0.75, epsilon = 1e-6);
    assert!(wt_b > eq_b);
}
