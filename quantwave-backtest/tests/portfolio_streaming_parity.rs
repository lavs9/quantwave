//! Batch↔streaming parity for shared-capital portfolio (quantwave-qzpi.10).
//!
//! `cargo nextest run -p quantwave-backtest portfolio_streaming_parity`

use approx::assert_relative_eq;
use chrono::TimeZone;
use polars::prelude::*;
use quantwave_backtest::{
    run_shared_capital_streaming_simulation, BacktestConfig, BacktestEngine, CostModel,
    ExecutionModel, PortfolioAllocator, PortfolioBar, PortfolioMode, StrategySignal,
};
use quantwave_core::traits::Next;

fn shared_config() -> BacktestConfig {
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

fn make_two_symbol_df() -> DataFrame {
    let timestamps = vec![
        1_700_010_000i64, 1_700_010_000, 1_700_010_001, 1_700_010_001, 1_700_010_002,
        1_700_010_002,
    ];
    let symbols = vec!["AAA", "BBB", "AAA", "BBB", "AAA", "BBB"];
    let closes = vec![100.0, 50.0, 101.0, 51.0, 102.0, 52.0];
    let signals = vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0];

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
        .zip(sym.into_iter())
        .filter_map(|(e, s)| if s.is_none() { Some(e.unwrap()) } else { None })
        .collect()
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

#[test]
fn test_shared_capital_batch_streaming_parity() {
    let df = make_two_symbol_df();
    let config = shared_config();
    let batch = BacktestEngine::new(config.clone())
        .run(df.clone().lazy())
        .expect("batch");

    let ts: Vec<i64> = df.column("timestamp").unwrap().i64().unwrap().into_iter().map(|v| v.unwrap()).collect();
    let symbols: Vec<String> = df
        .column("symbol")
        .unwrap()
        .str()
        .unwrap()
        .into_iter()
        .map(|s| s.unwrap().to_string())
        .collect();
    let closes: Vec<f64> = df.column("close").unwrap().f64().unwrap().into_iter().map(|v| v.unwrap()).collect();
    let exposures: Vec<f64> = df.column("signal").unwrap().f64().unwrap().into_iter().map(|v| v.unwrap()).collect();

    let bars: Vec<PortfolioBar> = ts
        .iter()
        .zip(symbols.iter())
        .zip(closes.iter())
        .map(|((&t, sym), &close)| PortfolioBar {
            ts: chrono::Utc.timestamp_opt(t, 0).single().unwrap(),
            symbol: sym.clone(),
            close,
        })
        .collect();

    let stream = run_shared_capital_streaming_simulation(
        &bars,
        ExposureReplay { exposures, idx: 0 },
        config,
    )
    .expect("streaming");

    let batch_eq = portfolio_equity(&batch);
    let stream_eq = portfolio_equity(&stream);
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