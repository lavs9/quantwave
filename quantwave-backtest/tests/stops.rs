#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::borrow_deref_ref,
    clippy::field_reassign_with_default
)]
//! Stop-loss / take-profit / trailing stops (quantwave-cr6v.9).
//!
//! `cargo nextest run -p quantwave-backtest stops`

use approx::assert_relative_eq;
use chrono::{TimeZone, Utc};
use polars::prelude::*;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, Bar, CostModel, ExecutionDelay, ExecutionModel, StopConfig,
    StopEvaluationMode, StrategySignal, run_streaming_simulation,
};

fn zero_cost_config(stops: StopConfig) -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        // quantwave-zmjw: these tests assert exact T+0 fill prices; the engine
        // default is now NextBar, so pin SameBar to preserve their original intent.
        execution_delay: ExecutionDelay::SameBar,
        stop_config: stops,
        ..Default::default()
    }
}

fn ohlc_config(stops: StopConfig) -> BacktestConfig {
    BacktestConfig {
        high_col: Some("high".to_string()),
        low_col: Some("low".to_string()),
        ..zero_cost_config(stops)
    }
}

fn exit_price(result: &quantwave_backtest::BacktestResult) -> f64 {
    result
        .trades
        .column("exit_price")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap()
}

fn exit_ts_unix(result: &quantwave_backtest::BacktestResult) -> i64 {
    result
        .trades
        .column("exit_ts")
        .unwrap()
        .i64()
        .unwrap()
        .get(0)
        .unwrap()
}

#[test]
fn test_fixed_stop_loss_exits() {
    // Enter bar 1 @100; 2% SL → 98; bar 3 close 97 triggers stop (signal still 1).
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..5)
                .map(|i| 1_700_100_000i64 + (i as i64) * 3600)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 99.0, 97.0, 98.0]),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, 1.0, 0.0]),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config(StopConfig {
        stop_loss_pct: Some(0.02),
        ..Default::default()
    }))
    .run(df.lazy())
    .expect("stop loss run");

    assert_eq!(result.trades.height(), 1);
    assert_relative_eq!(exit_price(&result), 97.0, epsilon = 1e-9);
    assert_eq!(exit_ts_unix(&result), 1_700_100_000 + 3 * 3600);
}

#[test]
fn test_take_profit_exits() {
    // Enter @100; 3% TP → 103; bar 3 close hits target while signal stays long.
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..5)
                .map(|i| 1_700_200_000i64 + (i as i64) * 3600)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 101.0, 103.0, 104.0]),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, 1.0, 1.0]),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config(StopConfig {
        take_profit_pct: Some(0.03),
        ..Default::default()
    }))
    .run(df.lazy())
    .expect("take profit run");

    assert_eq!(result.trades.height(), 1);
    assert_relative_eq!(exit_price(&result), 103.0, epsilon = 1e-9);
    assert_eq!(exit_ts_unix(&result), 1_700_200_000 + 3 * 3600);
}

#[test]
fn test_trailing_stop_ratchets() {
    // RaptorBT semantics: 5% trail ratchets with highs; exit when close breaches stop.
    // Bar1 enter@100 (stop 95); bar2 close 110 → stop 104.5; bar3 close 104 → exit.
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..4)
                .map(|i| 1_700_300_000i64 + (i as i64) * 3600)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 110.0, 104.0, 100.0]),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, 0.0]),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config(StopConfig {
        trailing_stop_pct: Some(0.05),
        ..Default::default()
    }))
    .run(df.lazy())
    .expect("trailing stop run");

    assert_eq!(result.trades.height(), 1);
    assert_relative_eq!(exit_price(&result), 104.0, epsilon = 1e-9);
    assert_eq!(exit_ts_unix(&result), 1_700_300_000 + 2 * 3600);
}

struct SignalReplay {
    signals: Vec<f64>,
    idx: usize,
}

impl quantwave_core::traits::Next<&Bar> for SignalReplay {
    type Output = StrategySignal;

    fn next(&mut self, _bar: &Bar) -> Self::Output {
        let i = self.idx.min(self.signals.len().saturating_sub(1));
        let exposure = self.signals[i];
        self.idx += 1;
        StrategySignal {
            exposure,
            metadata: None,
        }
    }
}

#[test]
fn test_ohlc_stop_loss_wick_exits_before_close_breach() {
    // Enter bar 1 @100; 2% SL = 98. Bar 3: close=99 but low=96 wicks through stop.
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..5)
                .map(|i| 1_700_500_000i64 + (i as i64) * 3600)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 99.0, 99.0, 98.0]),
        Column::new("high".into(), vec![100.0, 100.0, 99.0, 100.0, 98.0]),
        Column::new("low".into(), vec![100.0, 100.0, 99.0, 96.0, 98.0]),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, 1.0, 0.0]),
    ])
    .unwrap();

    let close_only = BacktestEngine::new(ohlc_config(StopConfig {
        stop_loss_pct: Some(0.02),
        stop_evaluation: StopEvaluationMode::CloseOnly,
        ..Default::default()
    }))
    .run(df.clone().lazy())
    .expect("close-only");

    let ohlc = BacktestEngine::new(ohlc_config(StopConfig {
        stop_loss_pct: Some(0.02),
        stop_evaluation: StopEvaluationMode::OhlcTouched,
        ..Default::default()
    }))
    .run(df.lazy())
    .expect("ohlc");

    assert_relative_eq!(exit_price(&close_only), 98.0, epsilon = 1e-9);
    assert_eq!(exit_ts_unix(&close_only), 1_700_500_000 + 4 * 3600);

    assert_relative_eq!(exit_price(&ohlc), 98.0, epsilon = 1e-9);
    assert_eq!(exit_ts_unix(&ohlc), 1_700_500_000 + 3 * 3600);
}

#[test]
fn test_ohlc_trailing_ratchets_on_high() {
    // Enter bar1 @100; bar2 high=115 ratchets trail to 109.25; bar3 low=103 breaches trail.
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..4)
                .map(|i| 1_700_600_000i64 + (i as i64) * 3600)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 110.0, 104.0]),
        Column::new("high".into(), vec![100.0, 100.0, 115.0, 104.0]),
        Column::new("low".into(), vec![100.0, 100.0, 110.0, 103.0]),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, 0.0]),
    ])
    .unwrap();

    let result = BacktestEngine::new(ohlc_config(StopConfig {
        trailing_stop_pct: Some(0.05),
        stop_evaluation: StopEvaluationMode::OhlcTouched,
        ..Default::default()
    }))
    .run(df.lazy())
    .expect("ohlc trailing");

    assert_eq!(result.trades.height(), 1);
    assert_relative_eq!(exit_price(&result), 109.25, epsilon = 1e-9);
    assert_eq!(exit_ts_unix(&result), 1_700_600_000 + 3 * 3600);
}

#[test]
fn test_ohlc_stops_batch_streaming_parity() {
    let ts: Vec<i64> = (0..4).map(|i| 1_700_700_000 + i * 3600).collect();
    let closes = vec![100.0, 100.0, 99.0, 98.0];
    let highs = vec![100.0, 100.0, 99.0, 100.0];
    let lows = vec![100.0, 100.0, 99.0, 96.0];
    let signals = vec![0.0, 1.0, 1.0, 0.0];
    let stops = StopConfig {
        stop_loss_pct: Some(0.02),
        stop_evaluation: StopEvaluationMode::OhlcTouched,
        ..Default::default()
    };

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), ts.clone()),
        Column::new("close".into(), closes.clone()),
        Column::new("high".into(), highs.clone()),
        Column::new("low".into(), lows.clone()),
        Column::new("signal".into(), signals.clone()),
    ])
    .unwrap();

    let batch = BacktestEngine::new(ohlc_config(stops.clone()))
        .run(df.lazy())
        .expect("batch ohlc stops");

    let bars: Vec<Bar> = ts
        .iter()
        .zip(closes.iter())
        .zip(highs.iter())
        .zip(lows.iter())
        .map(|(((&t, &close), &high), &low)| Bar {
            ts: Utc.timestamp_opt(t, 0).unwrap(),
            close,
            high: Some(high),
            low: Some(low),
        })
        .collect();

    let stream =
        run_streaming_simulation(&bars, SignalReplay { signals, idx: 0 }, ohlc_config(stops))
            .expect("streaming ohlc stops");

    assert_eq!(batch.trades.height(), stream.trades.height());
    assert_relative_eq!(exit_price(&batch), exit_price(&stream), epsilon = 1e-9);
    assert_eq!(exit_ts_unix(&batch), exit_ts_unix(&stream));
}

#[test]
fn test_stops_batch_streaming_parity() {
    let ts: Vec<i64> = (0..4).map(|i| 1_700_400_000 + i).collect();
    let closes = vec![100.0, 110.0, 104.0, 100.0];
    let signals = vec![0.0, 1.0, 1.0, 0.0];
    let stops = StopConfig {
        trailing_stop_pct: Some(0.05),
        ..Default::default()
    };

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), ts.clone()),
        Column::new("close".into(), closes.clone()),
        Column::new("signal".into(), signals.clone()),
    ])
    .unwrap();

    let batch = BacktestEngine::new(zero_cost_config(stops.clone()))
        .run(df.lazy())
        .expect("batch stops");

    let bars: Vec<Bar> = ts
        .iter()
        .zip(closes.iter())
        .map(|(&t, &close)| Bar {
            ts: Utc.timestamp_opt(t, 0).unwrap(),
            close,
            high: None,
            low: None,
        })
        .collect();

    let stream = run_streaming_simulation(
        &bars,
        SignalReplay { signals, idx: 0 },
        zero_cost_config(stops),
    )
    .expect("streaming stops");

    let b_eq: Vec<f64> = batch
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    let s_eq: Vec<f64> = stream
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();

    assert_eq!(b_eq.len(), s_eq.len());
    for (i, (b, s)) in b_eq.iter().zip(s_eq.iter()).enumerate() {
        assert_relative_eq!(*b, *s, epsilon = 1e-8, max_relative = 1e-8);
        if (b - s).abs() > 1e-7 {
            panic!("equity diverged at bar {i}: {b} vs {s}");
        }
    }

    assert_eq!(batch.trades.height(), stream.trades.height());
    assert_relative_eq!(exit_price(&batch), exit_price(&stream), epsilon = 1e-9);
    for k in ["final_equity", "net_pnl", "num_trades"] {
        let bv = *batch.stats.get(k).unwrap();
        let sv = *stream.stats.get(k).unwrap();
        assert_relative_eq!(bv, sv, epsilon = 1e-6, max_relative = 1e-6);
    }
}
