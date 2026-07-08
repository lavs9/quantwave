#![allow(
    clippy::panic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::borrow_deref_ref,
    clippy::field_reassign_with_default
)]
//! Proptest batch↔streaming parity for shared-capital portfolio (quantwave-z3k1 / qzpi.11).
//!
//! `cargo nextest run -p quantwave-backtest portfolio_proptest_parity`

use approx::assert_relative_eq;
use chrono::TimeZone;
use polars::prelude::*;
use proptest::prelude::*;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, CostModel, ExecutionModel, PortfolioAllocator, PortfolioBar,
    PortfolioMode, StrategySignal, run_shared_capital_streaming_simulation,
};
use quantwave_core::traits::Next;

const SYMBOL_POOL: [&str; 5] = ["S0", "S1", "S2", "S3", "S4"];

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

fn portfolio_config(allocator: PortfolioAllocator) -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: "signal".to_string(),
        symbol_col: Some("symbol".to_string()),
        portfolio_mode: PortfolioMode::SharedCapital,
        portfolio_allocator: allocator,
        ..Default::default()
    }
}

fn make_long_format_df(
    symbols: &[&str],
    n_ts: usize,
    closes: &[f64],
    signals: &[f64],
) -> DataFrame {
    let n = symbols.len();
    assert_eq!(closes.len(), n_ts * n);
    assert_eq!(signals.len(), n_ts * n);

    let mut timestamps = Vec::with_capacity(n_ts * n);
    let mut sym_col = Vec::with_capacity(n_ts * n);
    let mut close_col = Vec::with_capacity(n_ts * n);
    let mut signal_col = Vec::with_capacity(n_ts * n);

    for t in 0..n_ts {
        let ts = 1_700_000_000i64 + t as i64;
        for (s_idx, sym) in symbols.iter().enumerate() {
            let i = t * n + s_idx;
            timestamps.push(ts);
            sym_col.push(*sym);
            close_col.push(closes[i]);
            signal_col.push(signals[i]);
        }
    }

    DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("symbol".into(), sym_col),
        Column::new("close".into(), close_col),
        Column::new("signal".into(), signal_col),
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

fn assert_batch_streaming_parity(
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
        epsilon = 1e-6,
        max_relative = 1e-6,
    );
}

fn run_streaming_pair(
    df: &DataFrame,
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

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    #[test]
    fn proptest_shared_capital_equal_weight_parity(
        n_syms in 3usize..=5,
        n_ts in 4usize..=12,
        close_seed in 1.0f64..5.0,
        signal_bits in prop::collection::vec(0u8..2, 16..120),
    ) {
        let symbols: Vec<&str> = SYMBOL_POOL.iter().take(n_syms).copied().collect();
        let cells = n_ts * n_syms;
        let mut bits: Vec<u8> = signal_bits.into_iter().take(cells).collect();
        bits.resize(cells, 0);
        let closes: Vec<f64> = (0..cells)
            .map(|i| 50.0 + close_seed * (i as f64 % 7.0))
            .collect();
        let signals: Vec<f64> = bits.iter().map(|&b| b as f64).collect();

        let df = make_long_format_df(&symbols, n_ts, &closes, &signals);
        let config = portfolio_config(PortfolioAllocator::EqualWeight);
        let (batch, stream) = run_streaming_pair(&df, config);
        assert_batch_streaming_parity(&batch, &stream);
    }

    #[test]
    fn proptest_shared_capital_signal_weighted_parity(
        n_syms in 3usize..=5,
        n_ts in 4usize..=12,
        weights in prop::collection::vec(0.0f64..1.0, 16..120),
    ) {
        let symbols: Vec<&str> = SYMBOL_POOL.iter().take(n_syms).copied().collect();
        let cells = n_ts * n_syms;
        let mut w: Vec<f64> = weights.into_iter().take(cells).collect();
        w.resize(cells, 0.0);
        let closes: Vec<f64> = (0..cells).map(|i| 80.0 + (i % 11) as f64).collect();
        // Quantize to {0, 0.25, 0.5, 0.75, 1.0} for stable signal-weighted peers.
        let signals: Vec<f64> = w
            .iter()
            .map(|x| ((x * 4.0).round() / 4.0).clamp(0.0, 1.0))
            .collect();

        let df = make_long_format_df(&symbols, n_ts, &closes, &signals);
        let config = portfolio_config(PortfolioAllocator::SignalWeighted);
        let (batch, stream) = run_streaming_pair(&df, config);
        assert_batch_streaming_parity(&batch, &stream);
    }
}
