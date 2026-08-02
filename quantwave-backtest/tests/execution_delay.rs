#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::borrow_deref_ref,
    clippy::field_reassign_with_default
)]
//! T+1 execution delay (quantwave-cr6v.8).
//!
//! `cargo nextest run -p quantwave-backtest execution_delay`

use approx::assert_relative_eq;
use chrono::{TimeZone, Utc};
use polars::prelude::*;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, Bar, CostModel, ExecutionDelay, ExecutionModel, StrategySignal,
    run_streaming_simulation,
};

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

fn zero_cost_config(delay: ExecutionDelay) -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        execution_delay: delay,
        ..Default::default()
    }
}

fn entry_ts_unix(result: &quantwave_backtest::BacktestResult) -> i64 {
    result
        .trades
        .column("entry_ts")
        .unwrap()
        .i64()
        .unwrap()
        .get(0)
        .unwrap()
}

fn entry_price(result: &quantwave_backtest::BacktestResult) -> f64 {
    result
        .trades
        .column("entry_price")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap()
}

#[test]
fn test_t1_signal_delays_fill_one_bar() {
    let df = single_trade_df();
    let t0 = BacktestEngine::new(zero_cost_config(ExecutionDelay::SameBar))
        .run(df.clone().lazy())
        .expect("t0 run");
    let t1 = BacktestEngine::new(zero_cost_config(ExecutionDelay::NextBar))
        .run(df.lazy())
        .expect("t1 run");

    assert_eq!(t0.trades.height(), 1);
    assert_eq!(t1.trades.height(), 1);

    let bar1_ts = 1_700_000_000 + 3600;
    let bar2_ts = 1_700_000_000 + 2 * 3600;

    assert_eq!(entry_ts_unix(&t0), bar1_ts, "T+0 enters on signal bar");
    assert_eq!(
        entry_ts_unix(&t1),
        bar2_ts,
        "T+1 enters one bar after signal"
    );
}

#[test]
fn test_t1_execution_price_close() {
    let df = single_trade_df();
    let t1 = BacktestEngine::new(zero_cost_config(ExecutionDelay::NextBar))
        .run(df.lazy())
        .expect("t1 run");

    // Signal flips 0→1 on bar 1; T+1 fill at bar 2 close = 102.5
    assert_relative_eq!(entry_price(&t1), 102.5, epsilon = 1e-9);
}

/// quantwave-zmjw: the default must be T+1 (`NextBar`). `SameBar` fills at the close
/// of the very bar that produced the signal, which is systematically optimistic when
/// the signal is derived from that same close — so it is opt-in, never the default.
#[test]
fn test_default_delay_is_next_bar() {
    assert_eq!(
        BacktestConfig::default().execution_delay,
        ExecutionDelay::NextBar,
        "default must be T+1 (NextBar) — no same-bar look-ahead (quantwave-zmjw)"
    );

    let df = single_trade_df();
    let explicit_t1 = BacktestEngine::new(zero_cost_config(ExecutionDelay::NextBar))
        .run(df.clone().lazy())
        .expect("explicit t1");
    let default_delay = BacktestEngine::new(zero_cost_config(ExecutionDelay::default()))
        .run(df.clone().lazy())
        .expect("default delay");

    for k in ["num_trades", "final_equity", "net_pnl"] {
        let a = *explicit_t1.stats.get(k).unwrap();
        let b = *default_delay.stats.get(k).unwrap();
        assert_relative_eq!(a, b, epsilon = 1e-9, max_relative = 1e-9);
    }
    assert_eq!(entry_ts_unix(&explicit_t1), entry_ts_unix(&default_delay));

    // Signal flips 0→1 on bar 1 (close 101.0). The default must fill at bar 2's
    // close (102.5), not bar 1's own close.
    assert_relative_eq!(entry_price(&default_delay), 102.5, epsilon = 1e-9);
    assert_eq!(
        entry_ts_unix(&default_delay),
        1_700_000_000 + 2 * 3600,
        "default fill lands one bar after the signal bar"
    );

    // `SameBar` remains available for callers who ask for it explicitly.
    let explicit_t0 = BacktestEngine::new(zero_cost_config(ExecutionDelay::SameBar))
        .run(df.lazy())
        .expect("explicit t0");
    assert_relative_eq!(entry_price(&explicit_t0), 101.0, epsilon = 1e-9);
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
fn test_t1_batch_streaming_parity() {
    let ts: Vec<i64> = (0..6).map(|i| 1_700_030_000 + i).collect();
    let closes = vec![100.0, 101.0, 102.5, 103.0, 102.0, 101.0];
    let signals = vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0];

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), ts.clone()),
        Column::new("close".into(), closes.clone()),
        Column::new("signal".into(), signals.clone()),
    ])
    .unwrap();

    let batch = BacktestEngine::new(zero_cost_config(ExecutionDelay::NextBar))
        .run(df.lazy())
        .expect("batch t1");

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
        zero_cost_config(ExecutionDelay::NextBar),
    )
    .expect("streaming t1");

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
    for k in ["final_equity", "net_pnl", "num_trades"] {
        let bv = *batch.stats.get(k).unwrap();
        let sv = *stream.stats.get(k).unwrap();
        assert_relative_eq!(bv, sv, epsilon = 1e-6, max_relative = 1e-6);
    }
}
