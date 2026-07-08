//! Polars `.bt` namespace integration tests (quantwave-cr6v.5).
//!
//! `cargo nextest run -p quantwave-polars bt`
#![allow(clippy::unwrap_used, clippy::expect_used)]

use approx::assert_relative_eq;
use polars::prelude::*;
use quantwave_backtest::{MonteCarloConfig, SweepVariant, WalkForwardConfig};
use quantwave_polars::prelude::*;

fn single_trade_df() -> DataFrame {
    DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..6)
                .map(|i| 1_700_000_000i64 + (i as i64) * 3600)
                .collect::<Vec<_>>(),
        ),
        Column::new(
            "close".into(),
            vec![100.0, 101.0, 102.5, 103.0, 102.0, 101.0],
        ),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0]),
    ])
    .unwrap()
}

fn zero_cost_options(signal_col: &str) -> BtOptions {
    BtOptions {
        signal_col: signal_col.to_string(),
        commission_bps: 0.0,
        slippage_bps: 0.0,
        ..Default::default()
    }
}

#[test]
fn test_bt_namespace_exists() {
    let lf = single_trade_df().lazy();
    let _ns = lf.bt();
}

#[test]
fn test_bt_backtest_lazyframe() {
    let result = single_trade_df()
        .lazy()
        .bt()
        .backtest(zero_cost_options("signal"))
        .expect("backtest should succeed");

    assert_eq!(result.trades.height(), 1);
    assert_relative_eq!(
        *result.stats.get("num_trades").unwrap(),
        1.0,
        epsilon = 1e-9
    );
    assert_eq!(result.equity_curve.height(), 6);
}

#[test]
fn test_bt_backtest_custom_columns() {
    let df = DataFrame::new(vec![
        Column::new("ts".into(), vec![1i64, 2, 3, 4, 5, 6]),
        Column::new("px".into(), vec![100.0, 101.0, 102.5, 103.0, 102.0, 101.0]),
        Column::new("exposure".into(), vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0]),
    ])
    .unwrap();

    let opts = BtOptions {
        signal_col: "exposure".to_string(),
        timestamp_col: "ts".to_string(),
        close_col: "px".to_string(),
        commission_bps: 0.0,
        slippage_bps: 0.0,
        ..Default::default()
    };

    let result = df.lazy().bt().backtest(opts).expect("custom columns run");
    assert_eq!(result.trades.height(), 1);
}

#[test]
fn test_bt_backtest_with_report() {
    let report = single_trade_df()
        .lazy()
        .bt()
        .backtest_with_report(zero_cost_options("signal"))
        .expect("report run");

    assert_eq!(report.result.trades.height(), 1);
    assert_relative_eq!(report.metrics.num_trades, 1.0, epsilon = 1e-9);
    assert!(report.metrics.final_equity > 100_000.0);
}

#[test]
fn test_bt_backtest_filter_and_multiplier() {
    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), vec![1i64, 2, 3, 4, 5, 6]),
        Column::new(
            "close".into(),
            vec![100.0, 100.0, 101.0, 102.0, 101.0, 100.0],
        ),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0]),
        Column::new(
            "regime_ok".into(),
            vec![true, false, true, true, true, true],
        ),
        Column::new("size_mult".into(), vec![1.0, 1.0, 0.5, 0.5, 1.0, 1.0]),
    ])
    .unwrap();

    let opts = BtOptions {
        signal_col: "signal".to_string(),
        entry_filter_col: Some("regime_ok".to_string()),
        size_multiplier_col: Some("size_mult".to_string()),
        commission_bps: 0.0,
        slippage_bps: 0.0,
        ..Default::default()
    };

    let result = df.lazy().bt().backtest(opts).expect("filter+mult run");
    // Filter blocks bar 1 entry; later entry at bar 2 with 0.5 size → still one trade.
    assert_eq!(result.trades.height(), 1);
    let qty = result
        .trades
        .column("pnl_net")
        .expect("pnl")
        .f64()
        .unwrap()
        .get(0)
        .unwrap();
    assert!(qty.is_finite());
}

#[test]
fn test_bt_backtest_multi_symbol_smoke() {
    let timestamps = vec![
        1_700_010_000i64,
        1_700_010_000,
        1_700_010_001,
        1_700_010_001,
        1_700_010_002,
        1_700_010_002,
    ];
    let symbols = vec!["AAA", "BBB", "AAA", "BBB", "AAA", "BBB"];
    let closes = vec![100.0, 50.0, 101.0, 51.0, 102.0, 52.0];
    let signals = vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0];

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("symbol".into(), symbols),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap();

    let opts = BtOptions {
        symbol_col: Some("symbol".to_string()),
        commission_bps: 0.0,
        slippage_bps: 0.0,
        ..Default::default()
    };

    let result = df.lazy().bt().backtest(opts).expect("multi-symbol smoke");
    assert_eq!(result.trades.height(), 2);
    assert!(result.equity_curve.column("symbol").is_ok());
    assert_relative_eq!(
        *result.stats.get("num_symbols").unwrap(),
        2.0,
        epsilon = 1e-9
    );
}

#[test]
fn test_bt_walk_forward_optimize_smoke() {
    let n = 60i64;
    let timestamps: Vec<i64> = (0..n).collect();
    let closes: Vec<f64> = (0..n)
        .map(|i| {
            100.0
                + if i < 30 {
                    i as f64
                } else {
                    30.0 - (i - 30) as f64
                }
        })
        .collect();
    let signals_a: Vec<f64> = (0..n)
        .map(|i| if i % 2 == 0 { 1.0 } else { -1.0 })
        .collect();
    let signals_b: Vec<f64> = vec![1.0; n as usize];

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("close".into(), closes),
        Column::new("signal_a".into(), signals_a),
        Column::new("signal_b".into(), signals_b),
    ])
    .unwrap();

    let variants = vec![
        SweepVariant {
            params: std::collections::HashMap::from([("thresh".into(), 1.0)]),
            signal_col: "signal_a".into(),
        },
        SweepVariant {
            params: std::collections::HashMap::from([("thresh".into(), 2.0)]),
            signal_col: "signal_b".into(),
        },
    ];

    let wf = WalkForwardConfig::new(20, 10);
    let out = df
        .lazy()
        .bt()
        .walk_forward_optimize(wf, &variants, "total_return", zero_cost_options("signal_a"))
        .expect("wfo optimize");

    assert!(out.height() >= 1);
    assert!(out.column("best_thresh").is_ok());
}

#[test]
fn test_bt_monte_carlo_trade_bootstrap_smoke() {
    let summary = single_trade_df()
        .lazy()
        .bt()
        .monte_carlo_trade_bootstrap(
            zero_cost_options("signal"),
            MonteCarloConfig {
                n_simulations: 50,
                seed: 7,
            },
        )
        .expect("mc bootstrap");

    assert_eq!(summary.n_simulations, 50);
    assert!(summary.n_trades_sampled >= 1);
    assert!(summary.p50_final_equity.is_finite());
}
