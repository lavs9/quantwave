#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::borrow_deref_ref,
    clippy::field_reassign_with_default
)]
//! Pooled-book metrics (quantwave-qzpi.8).
//!
//! `cargo nextest run -p quantwave-backtest portfolio_metrics`

use approx::assert_relative_eq;
use polars::prelude::*;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, CostModel, ExecutionModel, PerformanceMetrics, PortfolioMode,
};

#[test]
fn test_shared_capital_metrics_use_single_initial_cash() {
    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), vec![1i64, 1, 2, 2]),
        Column::new("symbol".into(), vec!["A", "B", "A", "B"]),
        Column::new("close".into(), vec![100.0, 100.0, 110.0, 110.0]),
        Column::new("signal".into(), vec![1.0, 1.0, 1.0, 1.0]),
    ])
    .unwrap();

    let config = BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 50_000.0,
        }),
        symbol_col: Some("symbol".to_string()),
        portfolio_mode: PortfolioMode::SharedCapital,
        ..Default::default()
    };

    let report = BacktestEngine::new(config)
        .backtest_with_report(df.lazy())
        .unwrap();
    let m = &report.metrics;

    assert_relative_eq!(
        m.final_equity,
        report.result.stats["final_equity"],
        epsilon = 1e-6
    );
    assert_relative_eq!(
        m.total_return,
        (m.final_equity - 50_000.0) / 50_000.0,
        epsilon = 1e-8
    );
    assert!(m.num_trades >= 0.0);
}

#[test]
fn test_metrics_from_result_matches_report() {
    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), (0..10i64).collect::<Vec<_>>()),
        Column::new(
            "close".into(),
            (0..10).map(|i| 100.0 + i as f64).collect::<Vec<_>>(),
        ),
        Column::new(
            "signal".into(),
            vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
        ),
    ])
    .unwrap();

    let report = BacktestEngine::with_default_costs()
        .backtest_with_report(df.lazy())
        .unwrap();
    let from_result = PerformanceMetrics::from_result(&report.result);
    // NaN-aware: this run has no losing trades, so `profit_factor` is NaN by
    // design (quantwave-s3iu) and derived `==` would compare it unequal to itself.
    assert!(
        from_result.eq_including_nan(&report.metrics),
        "from_result = {from_result:?}\nreport.metrics = {:?}",
        report.metrics
    );
}
