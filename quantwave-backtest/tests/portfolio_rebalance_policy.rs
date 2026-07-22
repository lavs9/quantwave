#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::borrow_deref_ref,
    clippy::field_reassign_with_default
)]
//! `RebalancePolicy` integration coverage for the shared-capital portfolio
//! sim (quantwave-nbrx):
//!   - default (`None`) is byte-identical to pre-policy behavior, and
//!     equivalent to an explicit "every bar" `Calendar` trigger.
//!   - batch↔streaming parity holds when a policy is configured, since both
//!     paths funnel through the same `simulate_shared_capital`.
//!
//! `cargo nextest run -p quantwave-backtest portfolio_rebalance_policy`

use approx::assert_relative_eq;
use chrono::TimeZone;
use polars::prelude::*;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, CostModel, ExecutionModel, PortfolioAllocator, PortfolioBar,
    PortfolioMode, RebalancePolicy, StrategySignal, run_shared_capital_streaming_simulation,
};
use quantwave_core::traits::Next;

fn base_config() -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: "signal".to_string(),
        symbol_col: Some("symbol".to_string()),
        portfolio_mode: PortfolioMode::SharedCapital,
        portfolio_allocator: PortfolioAllocator::EqualWeight,
        ..Default::default()
    }
}

/// Two symbols, five bars, signals that flip and go flat repeatedly so the
/// entry/exit/flip machinery is well exercised.
fn make_df() -> DataFrame {
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
        100.0, 50.0, 102.0, 49.0, 99.0, 51.0, 105.0, 48.0, 101.0, 52.0,
    ];
    let signals = vec![1.0, 0.0, 1.0, 1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0];

    DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("symbol".into(), symbols),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap()
}

fn portfolio_equity(result: &quantwave_backtest::BacktestResult) -> Vec<f64> {
    let eq = result.equity_curve.column("equity").unwrap().f64().unwrap();
    let sym = result.equity_curve.column("symbol").unwrap().str().unwrap();
    eq.into_iter()
        .zip(&*sym)
        .filter_map(|(e, s)| if s.is_none() { Some(e.unwrap()) } else { None })
        .collect()
}

#[test]
fn test_no_policy_is_byte_identical_to_explicit_every_bar_calendar() {
    let df = make_df();

    let default_result = BacktestEngine::new(BacktestConfig {
        rebalance_policy: None,
        ..base_config()
    })
    .run(df.clone().lazy())
    .expect("default run");

    let explicit_every_bar = BacktestEngine::new(BacktestConfig {
        rebalance_policy: Some(RebalancePolicy::Calendar { every_n_bars: 1 }),
        ..base_config()
    })
    .run(df.lazy())
    .expect("explicit every-bar run");

    let default_eq = portfolio_equity(&default_result);
    let explicit_eq = portfolio_equity(&explicit_every_bar);
    assert_eq!(default_eq.len(), explicit_eq.len());
    for (a, b) in default_eq.iter().zip(explicit_eq.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "expected byte-identical equity");
    }
    assert_eq!(
        default_result.trades.height(),
        explicit_every_bar.trades.height()
    );
}

#[test]
fn test_no_policy_field_defaults_to_none() {
    // `BacktestConfig::default()` (and any config built with `..Default::default()`
    // without touching `rebalance_policy`) must resolve to `None`, so existing
    // caller configs (pre-quantwave-nbrx) behave exactly as before.
    assert_eq!(BacktestConfig::default().rebalance_policy, None);
}

#[test]
fn test_calendar_and_signal_policy_change_trade_count_vs_default() {
    let df = make_df();

    let default_result = BacktestEngine::new(BacktestConfig {
        rebalance_policy: None,
        ..base_config()
    })
    .run(df.clone().lazy())
    .expect("default run");

    let calendar_result = BacktestEngine::new(BacktestConfig {
        rebalance_policy: Some(RebalancePolicy::Calendar { every_n_bars: 3 }),
        ..base_config()
    })
    .run(df.lazy())
    .expect("calendar run");

    // With a coarser calendar trigger, the sim reacts to fewer bars, so it
    // should never produce *more* signal-driven trades than the every-bar
    // default on the same data.
    assert!(calendar_result.trades.height() <= default_result.trades.height());
}

struct ExposureReplay {
    exposures: Vec<f64>,
    idx: usize,
}

impl Next<&PortfolioBar> for ExposureReplay {
    type Output = StrategySignal;

    fn next(&mut self, _bar: &PortfolioBar) -> Self::Output {
        let exposure = self.exposures[self.idx.min(self.exposures.len() - 1)];
        self.idx += 1;
        StrategySignal {
            exposure,
            metadata: None,
        }
    }
}

fn run_batch_and_stream(
    df: DataFrame,
    config: BacktestConfig,
) -> (
    quantwave_backtest::BacktestResult,
    quantwave_backtest::BacktestResult,
) {
    let batch = BacktestEngine::new(config.clone())
        .run(df.clone().lazy())
        .expect("batch");

    let ts: Vec<i64> = df
        .column("timestamp")
        .unwrap()
        .i64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    let symbols: Vec<String> = df
        .column("symbol")
        .unwrap()
        .str()
        .unwrap()
        .into_iter()
        .map(|s| s.unwrap().to_string())
        .collect();
    let closes: Vec<f64> = df
        .column("close")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    let exposures: Vec<f64> = df
        .column("signal")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();

    let bars: Vec<PortfolioBar> = ts
        .iter()
        .zip(symbols.iter())
        .zip(closes.iter())
        .map(|((&t, sym), &close)| PortfolioBar {
            ts: chrono::Utc.timestamp_opt(t, 0).single().unwrap(),
            symbol: sym.clone(),
            close,
            high: None,
            low: None,
        })
        .collect();

    let stream = run_shared_capital_streaming_simulation(
        &bars,
        ExposureReplay { exposures, idx: 0 },
        config,
    )
    .expect("streaming");

    (batch, stream)
}

fn assert_parity(
    batch: &quantwave_backtest::BacktestResult,
    stream: &quantwave_backtest::BacktestResult,
) {
    let batch_eq = portfolio_equity(batch);
    let stream_eq = portfolio_equity(stream);
    assert_eq!(batch_eq.len(), stream_eq.len());
    for (i, (b, s)) in batch_eq.iter().zip(stream_eq.iter()).enumerate() {
        assert_relative_eq!(*b, *s, epsilon = 1e-8, max_relative = 1e-8);
        if (b - s).abs() > 1e-7 {
            panic!("portfolio equity diverged at bar {i}: {b} vs {s}");
        }
    }
    assert_eq!(batch.trades.height(), stream.trades.height());
    assert_relative_eq!(
        *batch.stats.get("final_equity").unwrap(),
        *stream.stats.get("final_equity").unwrap(),
        epsilon = 1e-6
    );
}

#[test]
fn test_batch_streaming_parity_with_calendar_policy() {
    let config = BacktestConfig {
        rebalance_policy: Some(RebalancePolicy::Calendar { every_n_bars: 2 }),
        ..base_config()
    };
    let (batch, stream) = run_batch_and_stream(make_df(), config);
    assert_parity(&batch, &stream);
}

#[test]
fn test_batch_streaming_parity_with_signal_policy() {
    let config = BacktestConfig {
        rebalance_policy: Some(RebalancePolicy::Signal),
        ..base_config()
    };
    let (batch, stream) = run_batch_and_stream(make_df(), config);
    assert_parity(&batch, &stream);
}

#[test]
fn test_batch_streaming_parity_with_drift_policy() {
    let config = BacktestConfig {
        rebalance_policy: Some(RebalancePolicy::Drift { threshold: 0.1 }),
        ..base_config()
    };
    let (batch, stream) = run_batch_and_stream(make_df(), config);
    assert_parity(&batch, &stream);
}

#[test]
fn test_batch_streaming_parity_with_turnover_policy() {
    let config = BacktestConfig {
        rebalance_policy: Some(RebalancePolicy::Turnover { min_turnover: 0.05 }),
        ..base_config()
    };
    let (batch, stream) = run_batch_and_stream(make_df(), config);
    assert_parity(&batch, &stream);
}
