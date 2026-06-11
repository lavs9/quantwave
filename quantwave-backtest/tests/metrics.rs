//! Integration tests for `PerformanceMetrics` (quantwave-cr6v.1).
//!
//! One vertical TDD slice per test; run with:
//! `cargo nextest run -p quantwave-backtest metrics`

use approx::assert_relative_eq;
use polars::prelude::*;
use quantwave_backtest::{
    backtest_simple_bool_signal, BacktestConfig, BacktestEngine, BacktestResult, CostModel,
    ExecutionModel, PerformanceMetrics,
};
use std::collections::HashMap;
use std::path::PathBuf;

/// Synthetic flat run: zero signal, constant price → no trades, flat equity.
fn make_flat_equity_result() -> quantwave_backtest::BacktestResult {
    let n: usize = 5;
    let ts: Vec<i64> = (0..n).map(|i| 1_700_000_100 + i as i64).collect();
    let closes = vec![100.0; n];
    let signals = vec![0.0; n];

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), ts),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap();

    backtest_simple_bool_signal(df, "signal").expect("flat run should succeed")
}

#[test]
fn test_metrics_zero_trades_flat_equity() {
    let result = make_flat_equity_result();

    assert_eq!(result.trades.height(), 0, "precondition: no trades");

    let metrics = PerformanceMetrics::from_result(&result);

    assert_relative_eq!(metrics.num_trades, 0.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.win_rate, 0.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.profit_factor, 0.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.max_drawdown_pct, 0.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.cagr, 0.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.sharpe_ratio, 0.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.sortino_ratio, 0.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.total_return, 0.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.final_equity, 100_000.0, epsilon = 1e-4);
    assert_relative_eq!(metrics.avg_trade_pnl, 0.0, epsilon = 1e-9);
}

/// Two round-trips with zero costs: +10 win then -10 loss → exact trade aggregates.
fn make_two_trade_win_loss_result() -> quantwave_backtest::BacktestResult {
    let ts: Vec<i64> = (0..7).map(|i| 1_700_000_200 + i as i64).collect();
    // Trade 1: enter @100 (bar 1), exit @110 (bar 3). Trade 2: enter @100 (bar 4), exit @90 (bar 6).
    let closes = vec![100.0, 100.0, 105.0, 110.0, 100.0, 95.0, 90.0];
    let signals = vec![0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0];

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), ts),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap();

    let config = BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: "signal".to_string(),
        ..Default::default()
    };

    BacktestEngine::new(config)
        .run(df.lazy())
        .expect("two-trade run should succeed")
}

#[test]
fn test_metrics_win_rate_and_profit_factor() {
    let result = make_two_trade_win_loss_result();

    assert_eq!(result.trades.height(), 2, "precondition: exactly two trades");

    let pnls: Vec<f64> = result
        .trades
        .column("pnl_net")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    assert_relative_eq!(pnls[0], 10.0, epsilon = 1e-6);
    assert_relative_eq!(pnls[1], -10.0, epsilon = 1e-6);

    let metrics = PerformanceMetrics::from_result(&result);

    assert_relative_eq!(metrics.num_trades, 2.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.win_rate, 0.5, epsilon = 1e-9);
    assert_relative_eq!(metrics.profit_factor, 1.0, epsilon = 1e-9);
    assert_relative_eq!(metrics.avg_trade_pnl, 0.0, epsilon = 1e-6);
}

/// Hand-built equity curve: peak 110k → trough 99k → 10% max drawdown, no trades.
fn make_known_drawdown_result() -> BacktestResult {
    let n = 6;
    let ts: Vec<i64> = (0..n).map(|i| 1_700_000_300 + i as i64).collect();
    let equity = vec![100_000.0, 105_000.0, 110_000.0, 105_000.0, 99_000.0, 102_000.0];

    let equity_curve = DataFrame::new(vec![
        Column::new("ts".into(), ts),
        Column::new("equity".into(), equity.clone()),
        Column::new("position".into(), vec![0.0; n]),
    ])
    .unwrap();

    let trades = DataFrame::new(vec![
        Column::new("trade_id".into(), Vec::<u32>::new()),
        Column::new("side".into(), Vec::<i8>::new()),
        Column::new("entry_ts".into(), Vec::<i64>::new()),
        Column::new("entry_price".into(), Vec::<f64>::new()),
        Column::new("pnl_net".into(), Vec::<f64>::new()),
    ])
    .unwrap();

    let initial = equity[0];
    let final_eq = *equity.last().unwrap();
    let mut stats = HashMap::new();
    stats.insert("initial_cash".to_string(), initial);
    stats.insert("final_equity".to_string(), final_eq);
    stats.insert("total_return".to_string(), (final_eq - initial) / initial);
    stats.insert("num_trades".to_string(), 0.0);

    BacktestResult {
        trades,
        equity_curve,
        stats,
    }
}

#[test]
fn test_metrics_max_drawdown_known_curve() {
    let result = make_known_drawdown_result();

    // Peak 110_000, trough 99_000 after peak → (110_000 - 99_000) / 110_000 = 0.10
    let expected_dd = 11_000.0 / 110_000.0;

    let metrics = PerformanceMetrics::from_result(&result);

    assert_relative_eq!(metrics.max_drawdown_pct, expected_dd, epsilon = 1e-9);
}

fn make_result_from_equity(equity: &[f64], ts_base: i64) -> BacktestResult {
    let n = equity.len();
    let ts: Vec<i64> = (0..n).map(|i| ts_base + i as i64).collect();
    let equity_curve = DataFrame::new(vec![
        Column::new("ts".into(), ts),
        Column::new("equity".into(), equity.to_vec()),
        Column::new("position".into(), vec![0.0; n]),
    ])
    .unwrap();
    let trades = DataFrame::new(vec![
        Column::new("trade_id".into(), Vec::<u32>::new()),
        Column::new("side".into(), Vec::<i8>::new()),
        Column::new("entry_ts".into(), Vec::<i64>::new()),
        Column::new("entry_price".into(), Vec::<f64>::new()),
        Column::new("pnl_net".into(), Vec::<f64>::new()),
    ])
    .unwrap();
    let initial = equity[0];
    let final_eq = *equity.last().unwrap();
    let mut stats = HashMap::new();
    stats.insert("initial_cash".to_string(), initial);
    stats.insert("final_equity".to_string(), final_eq);
    stats.insert("total_return".to_string(), (final_eq - initial) / initial);
    stats.insert("num_trades".to_string(), 0.0);
    BacktestResult {
        trades,
        equity_curve,
        stats,
    }
}

#[test]
fn test_metrics_cagr_annualized() {
    // 252 daily bars, 10% endpoint growth → CAGR = (1.10)^(252/252) - 1 = 10%.
    let n = 252;
    let initial: f64 = 100_000.0;
    let final_target: f64 = 110_000.0;
    let equity: Vec<f64> = (0..n)
        .map(|i| initial + (final_target - initial) * (i as f64 / (n as f64 - 1.0)))
        .collect();

    let result = make_result_from_equity(&equity, 1_700_001_000);
    let metrics = PerformanceMetrics::from_result(&result);

    assert_relative_eq!(metrics.cagr, 0.10, epsilon = 1e-9);
    assert_relative_eq!(metrics.total_return, 0.10, epsilon = 1e-9);
}

#[test]
fn test_metrics_sharpe_positive_drift() {
    // All-positive per-bar returns with slight variation → Sharpe > 0.
    let mut equity = vec![100_000.0];
    let daily_returns = [0.002, 0.0025, 0.0018, 0.0022, 0.0021];
    for &r in daily_returns.iter().cycle().take(60) {
        equity.push(equity.last().unwrap() * (1.0 + r));
    }

    let result = make_result_from_equity(&equity, 1_700_002_000);
    let metrics = PerformanceMetrics::from_result(&result);

    assert!(metrics.sharpe_ratio > 0.0, "sharpe={}", metrics.sharpe_ratio);
}

#[test]
fn test_metrics_sortino_downside_only() {
    // Mixed returns including downside → finite Sortino.
    let mut equity = vec![100_000.0];
    let daily_returns = [0.003, -0.001, 0.002, -0.002, 0.004];
    for &r in daily_returns.iter().cycle().take(50) {
        equity.push(equity.last().unwrap() * (1.0 + r));
    }

    let result = make_result_from_equity(&equity, 1_700_003_000);
    let metrics = PerformanceMetrics::from_result(&result);

    assert!(metrics.sortino_ratio.is_finite());
    assert!(metrics.sortino_ratio > 0.0);
}

#[test]
fn test_backtest_result_metrics_method() {
    let result = make_two_trade_win_loss_result();
    assert_eq!(result.trades.height(), 2);
    let direct = PerformanceMetrics::from_result(&result);
    let via_method = result.metrics();
    assert_eq!(direct, via_method);
}

#[test]
fn test_backtest_with_report_returns_report() {
    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), (0..7i64).map(|i| 1_700_000_200 + i).collect::<Vec<_>>()),
        Column::new(
            "close".into(),
            vec![100.0, 100.0, 105.0, 110.0, 100.0, 95.0, 90.0],
        ),
        Column::new(
            "signal".into(),
            vec![0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0],
        ),
    ])
    .unwrap();

    let config = BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: "signal".to_string(),
        ..Default::default()
    };

    let report = BacktestEngine::new(config)
        .backtest_with_report(df.lazy())
        .expect("report should succeed");

    assert_eq!(report.result.trades.height(), 2);
    assert_relative_eq!(report.metrics.win_rate, 0.5, epsilon = 1e-9);
    assert_relative_eq!(report.metrics.profit_factor, 1.0, epsilon = 1e-9);
    assert_eq!(
        report.result.metrics(),
        report.metrics,
        "BacktestReport metrics must match BacktestResult::metrics()"
    );
}

/// 20-bar flip strategy (zero costs): win +6 then loss -8.
fn make_gold_standard_flip_result() -> BacktestResult {
    let n = 20;
    let ts: Vec<i64> = (0..n).map(|i| 1_700_004_000 + i as i64).collect();
    let closes = vec![
        100.0, 100.0, 100.0, 102.0, 104.0, 106.0, 106.0, 106.0, 106.0, 106.0, 100.0,
        98.0, 96.0, 94.0, 92.0, 92.0, 92.0, 92.0, 92.0, 92.0,
    ];
    let signals = vec![
        0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0,
    ];

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), ts),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap();

    let config = BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: "signal".to_string(),
        ..Default::default()
    };

    BacktestEngine::new(config)
        .run(df.lazy())
        .expect("gold-standard flip run")
}

#[test]
#[ignore = "helper to regenerate gold standard vectors"]
fn dump_gold_metrics() {
    let result = make_gold_standard_flip_result();
    let metrics = PerformanceMetrics::from_result(&result);
    println!("{metrics:#?}");
}

#[test]
fn test_metrics_gold_standard_basic() {
    let result = make_gold_standard_flip_result();
    let metrics = PerformanceMetrics::from_result(&result);

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/gold_standard/metrics_basic.json");
    let raw = std::fs::read_to_string(&path).expect("gold standard file");
    let spec: serde_json::Value = serde_json::from_str(&raw).expect("valid json");

    let expected = &spec["expected"];

    assert_relative_eq!(
        metrics.num_trades,
        expected["num_trades"].as_f64().unwrap(),
        epsilon = 1e-9
    );
    assert_relative_eq!(
        metrics.win_rate,
        expected["win_rate"].as_f64().unwrap(),
        epsilon = 1e-9
    );
    assert_relative_eq!(
        metrics.profit_factor,
        expected["profit_factor"].as_f64().unwrap(),
        epsilon = 1e-6
    );
    assert_relative_eq!(
        metrics.max_drawdown_pct,
        expected["max_drawdown_pct"].as_f64().unwrap(),
        epsilon = 1e-6
    );
    assert_relative_eq!(
        metrics.total_return,
        expected["total_return"].as_f64().unwrap(),
        epsilon = 1e-6
    );
    assert_relative_eq!(
        metrics.final_equity,
        expected["final_equity"].as_f64().unwrap(),
        epsilon = 1e-2
    );
    assert_relative_eq!(
        metrics.avg_trade_pnl,
        expected["avg_trade_pnl"].as_f64().unwrap(),
        epsilon = 1e-4
    );
}