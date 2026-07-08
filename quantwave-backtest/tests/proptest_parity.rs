#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::borrow_deref_ref,
    clippy::field_reassign_with_default
)]
//! Proptest batch↔streaming parity for core engine (quantwave-qzpi.13).
//!
//! `cargo nextest run -p quantwave-backtest proptest_parity`

use approx::assert_relative_eq;
use polars::prelude::IntoLazy;
use proptest::prelude::*;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, Bar, CostModel, ExecutionModel, StrategySignal,
    run_streaming_simulation,
};
use quantwave_core::traits::Next;

struct BoolReplay {
    values: Vec<f64>,
    idx: usize,
}

impl Next<&Bar> for BoolReplay {
    type Output = StrategySignal;

    fn next(&mut self, _bar: &Bar) -> Self::Output {
        let v = self.values[self.idx.min(self.values.len() - 1)];
        self.idx += 1;
        StrategySignal {
            exposure: v,
            metadata: None,
        }
    }
}

fn make_df(closes: &[f64], signals: &[f64]) -> polars::prelude::DataFrame {
    let n = closes.len();
    let ts: Vec<i64> = (0..n as i64).map(|i| 1_700_000_000 + i * 60).collect();
    polars::prelude::DataFrame::new(vec![
        polars::prelude::Column::new("timestamp".into(), ts),
        polars::prelude::Column::new("close".into(), closes.to_vec()),
        polars::prelude::Column::new("signal".into(), signals.to_vec()),
    ])
    .unwrap()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn proptest_single_symbol_batch_streaming_equity_parity(
        closes in prop::collection::vec(50.0f64..200.0, 4..40),
        signal_bits in prop::collection::vec(0u8..2, 4..40),
    ) {
        let n = closes.len().min(signal_bits.len());
        let closes: Vec<f64> = closes.into_iter().take(n).collect();
        let signals: Vec<f64> = signal_bits.into_iter().take(n).map(|b| b as f64).collect();

        let df = make_df(&closes, &signals);
        let config = BacktestConfig {
            execution_model: ExecutionModel::Simple(CostModel {
                commission_bps: 0.0,
                slippage_bps: 0.0,
                initial_cash: 100_000.0,
            }),
            ..Default::default()
        };
        let batch = BacktestEngine::new(config.clone())
            .run(df.lazy())
            .expect("batch");

        let bars: Vec<Bar> = (0..n)
            .map(|i| Bar {
                ts: chrono::DateTime::<chrono::Utc>::from_timestamp(1_700_000_000 + i as i64 * 60, 0).unwrap(),
                close: closes[i],
                high: None,
                low: None,
            })
            .collect();

        let stream = run_streaming_simulation(
            &bars,
            BoolReplay { values: signals.clone(), idx: 0 },
            config,
        )
        .expect("stream");

        let batch_eq: Vec<f64> = batch
            .equity_curve
            .column("equity")
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|v| v.unwrap())
            .collect();
        let stream_eq: Vec<f64> = stream
            .equity_curve
            .column("equity")
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|v| v.unwrap())
            .collect();

        prop_assert_eq!(batch_eq.len(), stream_eq.len());
        for (b, s) in batch_eq.iter().zip(stream_eq.iter()) {
            prop_assert!((b - s).abs() < 1e-6, "equity mismatch: {b} vs {s}");
        }
        prop_assert_eq!(batch.trades.height(), stream.trades.height());
        assert_relative_eq!(
            *batch.stats.get("final_equity").unwrap(),
            *stream.stats.get("final_equity").unwrap(),
            epsilon = 1e-6
        );
    }
}
