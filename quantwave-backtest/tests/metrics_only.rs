#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::borrow_deref_ref,
    clippy::field_reassign_with_default
)]
use polars::prelude::*;
use quantwave_backtest::{BacktestConfig, BacktestEngine};

#[test]
fn test_metrics_only_matches_full_backtest_metrics() {
    let df = df!(
        "timestamp" => &[1i64, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        "close" => &[10.0, 10.5, 9.8, 11.2, 11.5, 12.0, 11.8, 12.5, 13.0, 12.8],
        "signal" => &[0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]
    )
    .unwrap();

    let config = BacktestConfig::default();
    let engine = BacktestEngine::new(config);

    let full_report = engine.backtest_with_report(df.clone().lazy()).unwrap();
    let metrics_only = engine.run_metrics_only(df.lazy()).unwrap();

    assert_eq!(full_report.metrics.num_trades, metrics_only.num_trades);
    assert!((full_report.metrics.cagr - metrics_only.cagr).abs() < 1e-9);
    assert!((full_report.metrics.total_return - metrics_only.total_return).abs() < 1e-9);
    assert!((full_report.metrics.win_rate - metrics_only.win_rate).abs() < 1e-9);
    assert!((full_report.metrics.profit_factor - metrics_only.profit_factor).abs() < 1e-9);
}

#[test]
fn test_metrics_only_zero_trades_flat() {
    let df = df!(
        "timestamp" => &[1i64, 2, 3],
        "close" => &[10.0, 10.5, 9.8],
        "signal" => &[0.0, 0.0, 0.0]
    )
    .unwrap();

    let engine = BacktestEngine::new(BacktestConfig::default());
    let metrics = engine.run_metrics_only(df.lazy()).unwrap();

    assert_eq!(metrics.num_trades, 0.0);
    assert!((metrics.total_return).abs() < 1e-9);
}

#[test]
fn test_metrics_only_multi_symbol_parity() {
    let df = df!(
        "timestamp" => &[1i64, 1, 2, 2],
        "symbol" => &["A", "B", "A", "B"],
        "close" => &[10.0, 20.0, 12.0, 18.0],
        "signal" => &[1.0, 1.0, 0.0, 0.0]
    )
    .unwrap();

    let mut config = BacktestConfig::default();
    config.symbol_col = Some("symbol".to_string());
    let engine = BacktestEngine::new(config);

    let full_report = engine.backtest_with_report(df.clone().lazy()).unwrap();
    let metrics_only = engine.run_metrics_only(df.lazy()).unwrap();

    println!(
        "Full report total_return: {}",
        full_report.metrics.total_return
    );
    println!("Metrics only total_return: {}", metrics_only.total_return);
    assert_eq!(full_report.metrics.num_trades, metrics_only.num_trades);
    assert!((full_report.metrics.total_return - metrics_only.total_return).abs() < 1e-9);
}
