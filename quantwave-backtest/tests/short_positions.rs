//! Short positions (quantwave-cr6v.10).
//!
//! `cargo nextest run -p quantwave-backtest --test short_positions`

use approx::assert_relative_eq;
use chrono::{TimeZone, Utc};
use polars::prelude::*;
use quantwave_backtest::{
    run_streaming_simulation, BacktestConfig, BacktestEngine, Bar, CostModel, ExecutionModel,
    StrategySignal,
};

fn zero_cost_config() -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        ..Default::default()
    }
}

fn trade_side(result: &quantwave_backtest::BacktestResult) -> i8 {
    result
        .trades
        .column("side")
        .unwrap()
        .i8()
        .unwrap()
        .get(0)
        .unwrap()
}

fn trade_pnl(result: &quantwave_backtest::BacktestResult) -> f64 {
    result
        .trades
        .column("pnl_net")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap()
}

#[test]
fn test_short_entry_exit_pnl_positive_on_decline() {
    // Short 1 unit @100, cover @95 → +5 gross.
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..5)
                .map(|i| 1_800_100_000i64 + i as i64)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 98.0, 95.0, 96.0]),
        Column::new("signal".into(), vec![0.0, -1.0, -1.0, 0.0, 0.0]),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config())
        .run(df.lazy())
        .expect("short run");

    assert_eq!(result.trades.height(), 1);
    assert_eq!(trade_side(&result), -1);
    // Enter bar1 @100, cover bar3 @95 → +5.
    assert_relative_eq!(trade_pnl(&result), 5.0, epsilon = 1e-9);
}

#[test]
fn test_short_loss_on_price_rise() {
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..4)
                .map(|i| 1_800_200_000i64 + i as i64)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 103.0, 105.0]),
        Column::new("signal".into(), vec![0.0, -1.0, -1.0, 0.0]),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config())
        .run(df.lazy())
        .expect("short loss run");

    assert_eq!(result.trades.height(), 1);
    assert_eq!(trade_side(&result), -1);
    assert_relative_eq!(trade_pnl(&result), -5.0, epsilon = 1e-9);
}

#[test]
fn test_long_short_flip_same_bar() {
    // Long bar1, flip to short bar3 (close long + open short same bar).
    let df = DataFrame::new(vec![
        Column::new(
            "timestamp".into(),
            (0..5)
                .map(|i| 1_800_300_000i64 + i as i64)
                .collect::<Vec<_>>(),
        ),
        Column::new("close".into(), vec![100.0, 100.0, 102.0, 101.0, 99.0]),
        Column::new("signal".into(), vec![0.0, 1.0, 1.0, -1.0, 0.0]),
    ])
    .unwrap();

    let result = BacktestEngine::new(zero_cost_config())
        .run(df.lazy())
        .expect("flip run");

    assert_eq!(result.trades.height(), 2);
    let sides: Vec<i8> = result
        .trades
        .column("side")
        .unwrap()
        .i8()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    assert_eq!(sides, vec![1, -1]);

    // Long: 100→101 = +1; Short: 101→99 = +2; net +3
    let pnls: Vec<f64> = result
        .trades
        .column("pnl_net")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    assert_relative_eq!(pnls[0], 1.0, epsilon = 1e-9);
    assert_relative_eq!(pnls[1], 2.0, epsilon = 1e-9);
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
fn test_short_batch_streaming_parity() {
    let ts: Vec<i64> = (0..5).map(|i| 1_800_400_000 + i).collect();
    let closes = vec![100.0, 100.0, 98.0, 95.0, 96.0];
    let signals = vec![0.0, -1.0, -1.0, 0.0, 0.0];

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), ts.clone()),
        Column::new("close".into(), closes.clone()),
        Column::new("signal".into(), signals.clone()),
    ])
    .unwrap();

    let batch = BacktestEngine::new(zero_cost_config())
        .run(df.lazy())
        .expect("batch short");

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
        SignalReplay {
            signals,
            idx: 0,
        },
        zero_cost_config(),
    )
    .expect("streaming short");

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
    assert_relative_eq!(trade_pnl(&batch), trade_pnl(&stream), epsilon = 1e-9);
    for k in ["final_equity", "net_pnl", "num_trades"] {
        let bv = *batch.stats.get(k).unwrap();
        let sv = *stream.stats.get(k).unwrap();
        assert_relative_eq!(bv, sv, epsilon = 1e-6, max_relative = 1e-6);
    }
}