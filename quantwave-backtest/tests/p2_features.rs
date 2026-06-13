//! Integration tests for P2 backtest features (cr6v.14–16).

use polars::prelude::*;
use quantwave_backtest::{
    monte_carlo_trade_bootstrap, run_walk_forward, CrossSectionalConfig,
    MonteCarloConfig, RecordingLiveBridge, WalkForwardConfig, BacktestConfig, BacktestEngine,
    CostModel, LiveBridge, LiveSignalEvent, StrategySignal, Bar,
};

fn zero_cost_config() -> BacktestConfig {
    BacktestConfig {
        cost_model: CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        },
        ..Default::default()
    }
}

#[test]
fn test_walk_forward_integration_fold_metrics() {
    let n = 80usize;
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..n as i64).map(|i| 1_700_000_000 + i * 3600).collect::<Vec<_>>(),
        ),
        Column::new("close".into(), (0..n).map(|i| 100.0 + i as f64).collect::<Vec<_>>()),
        Column::new(
            "signal".into(),
            (0..n)
                .map(|i| if (i / 15) % 2 == 0 { 1.0 } else { 0.0 })
                .collect::<Vec<_>>(),
        ),
    ])
    .unwrap();

    let out = run_walk_forward(
        df.lazy(),
        &zero_cost_config(),
        &WalkForwardConfig::new(20, 15),
    )
    .unwrap();
    assert!(out.height() >= 2);
    assert!(out.column("sharpe_ratio").is_ok());
}

#[test]
fn test_monte_carlo_integration_after_backtest() {
    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), (0..6i64).map(|i| i + 1).collect::<Vec<_>>()),
        Column::new("close".into(), vec![100.0, 101.0, 102.5, 103.0, 102.0, 101.0]),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0]),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config())
        .run(df.lazy())
        .unwrap();
    let summary = monte_carlo_trade_bootstrap(
        &result,
        100_000.0,
        &MonteCarloConfig {
            n_simulations: 100,
            seed: 7,
        },
    )
    .unwrap();
    assert_eq!(summary.n_trades_sampled, 1);
    assert!(summary.p50_final_equity.is_finite());
}

#[test]
fn test_cross_sectional_panel_integration() {
    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), vec![1i64, 1, 1, 2, 2, 2]),
        Column::new("symbol".into(), vec!["A", "B", "C", "A", "B", "C"]),
        Column::new("close".into(), vec![10.0, 10.0, 10.0, 11.0, 11.0, 11.0]),
        Column::new("factor".into(), vec![3.0, 2.0, 1.0, 3.0, 2.0, 1.0]),
    ])
    .unwrap();

    let mut cfg = zero_cost_config();
    cfg.symbol_col = Some("symbol".into());

    let report = quantwave_backtest::run_cross_sectional_backtest(
        df.lazy(),
        &CrossSectionalConfig::long_short("factor", 0.34, 0.34),
        cfg,
    )
    .unwrap();
    assert!(report.metrics.final_equity.is_finite());
}

#[test]
fn test_live_bridge_stub_integration() {
    let mut bridge = RecordingLiveBridge::new();
    bridge.connect().unwrap();
    let event = LiveSignalEvent::from_bar_signal(
        &Bar {
            ts: chrono::Utc::now(),
            close: 50.0,
        },
        Some("QQQ".into()),
        &StrategySignal {
            exposure: -0.5,
            metadata: None,
        },
    );
    bridge.publish(&event).unwrap();
    assert_eq!(bridge.events[0].symbol.as_deref(), Some("QQQ"));
}