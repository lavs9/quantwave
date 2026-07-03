//! Multi-symbol long-format backtests (quantwave-cr6v.2).
//!
//! `cargo nextest run -p quantwave-backtest multi_symbol`

use approx::assert_relative_eq;
use polars::prelude::*;
use quantwave_backtest::{
    backtest_simple_bool_signal, run_streaming_simulation, BacktestConfig, BacktestEngine,
    Bar, CostModel, ExecutionModel,
};

fn zero_cost_config(signal_col: &str, symbol_col: Option<String>) -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: signal_col.to_string(),
        symbol_col,
        ..Default::default()
    }
}

/// Long-format rows sorted by (timestamp, symbol); full grid (5 ts × 2 syms).
fn make_two_symbol_df() -> DataFrame {
    // AAA: one round-trip (t1→t3). BBB: two round-trips (t1→t2, t3→t4).
    let timestamps = vec![
        1_700_010_000i64, 1_700_010_000, 1_700_010_001, 1_700_010_001, 1_700_010_002,
        1_700_010_002, 1_700_010_003, 1_700_010_003, 1_700_010_004, 1_700_010_004,
    ];
    let symbols = vec![
        "AAA", "BBB", "AAA", "BBB", "AAA", "BBB", "AAA", "BBB", "AAA", "BBB",
    ];
    let closes = vec![
        100.0, 50.0, 101.0, 51.0, 102.0, 52.0, 103.0, 53.0, 104.0, 54.0,
    ];
    let signals = vec![
        0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    ];

    DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("symbol".into(), symbols),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap()
}

fn trade_count_for_symbol(result: &quantwave_backtest::BacktestResult, sym: &str) -> usize {
    let sym_col = result.trades.column("symbol").expect("symbol column");
    let strs = sym_col.str().unwrap();
    strs.into_iter()
        .filter(|s| *s == Some(sym))
        .count()
}

fn portfolio_equity_series(result: &quantwave_backtest::BacktestResult) -> Vec<f64> {
    let eq = result.equity_curve.column("equity").unwrap().f64().unwrap();
    let sym = result.equity_curve.column("symbol").unwrap().str().unwrap();
    eq.into_iter()
        .zip(sym.into_iter())
        .filter_map(|(e, s)| if s.is_none() { e } else { None })
        .collect()
}

#[test]
fn test_multi_symbol_independent_trades() {
    let df = make_two_symbol_df();
    let engine = BacktestEngine::new(zero_cost_config("signal", Some("symbol".to_string())));
    let result = engine.run(df.lazy()).expect("multi-symbol run");

    assert_eq!(result.trades.height(), 3, "AAA:1 + BBB:2 trades");
    assert_eq!(trade_count_for_symbol(&result, "AAA"), 1);
    assert_eq!(trade_count_for_symbol(&result, "BBB"), 2);
    assert_relative_eq!(*result.stats.get("num_trades").unwrap(), 3.0, epsilon = 1e-9);
}

#[test]
fn test_multi_symbol_portfolio_equity_sum() {
    let df = make_two_symbol_df();
    let engine = BacktestEngine::new(zero_cost_config("signal", Some("symbol".to_string())));
    let multi = engine.run(df.clone().lazy()).expect("multi-symbol run");

    // Independent single-symbol runs for reference sums.
    let df_a = df
        .clone()
        .lazy()
        .filter(col("symbol").eq(lit("AAA")))
        .collect()
        .unwrap();
    let df_b = df
        .lazy()
        .filter(col("symbol").eq(lit("BBB")))
        .collect()
        .unwrap();

    let single_cfg = zero_cost_config("signal", None);
    let res_a = BacktestEngine::new(single_cfg.clone())
        .run(df_a.lazy())
        .unwrap();
    let res_b = BacktestEngine::new(single_cfg)
        .run(df_b.lazy())
        .unwrap();

    let eq_a: Vec<f64> = res_a
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();
    let eq_b: Vec<f64> = res_b
        .equity_curve
        .column("equity")
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .map(|v| v.unwrap())
        .collect();

    let portfolio = portfolio_equity_series(&multi);
    assert_eq!(portfolio.len(), eq_a.len());
    for (i, (a, b)) in eq_a.iter().zip(eq_b.iter()).enumerate() {
        assert_relative_eq!(portfolio[i], a + b, epsilon = 1e-6);
    }

    assert_relative_eq!(
        *multi.stats.get("initial_cash").unwrap(),
        200_000.0,
        epsilon = 1e-6
    );
}

#[test]
fn test_multi_symbol_requires_sorted() {
    let df = make_two_symbol_df();
    let n = df.height();
    // Move last row to front → timestamp decreases on row 1.
    let unsorted = df
        .slice((n - 1) as i64, 1)
        .vstack(&df.slice(0, n - 1))
        .unwrap();

    let engine = BacktestEngine::new(zero_cost_config("signal", Some("symbol".to_string())));
    let err = engine.run(unsorted.lazy()).unwrap_err();
    assert!(
        matches!(err, quantwave_backtest::BacktestError::UnsortedData),
        "expected UnsortedData, got {err:?}"
    );
}

#[test]
fn test_multi_symbol_single_regression() {
    // Existing single-symbol path must be unchanged when symbol_col is None.
    let n: usize = 6;
    let timestamps: Vec<i64> = (0..n)
        .map(|i| 1_700_000_000i64 + (i as i64) * 3600)
        .collect();
    let closes = vec![100.0, 101.0, 102.5, 103.0, 102.0, 101.0];
    let signals = vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0];

    let df = DataFrame::new(vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ])
    .unwrap();

    let result = backtest_simple_bool_signal(df, "signal").expect("single-symbol run");
    assert_eq!(result.trades.height(), 1);
    assert!(result.equity_curve.column("symbol").is_err());
    assert!(result.trades.column("symbol").is_err());
    assert_relative_eq!(*result.stats.get("initial_cash").unwrap(), 100_000.0, epsilon = 1e-6);
}

/// Replay precomputed exposures per symbol through streaming sim; portfolio equity
/// must match batch multi-symbol portfolio rows.
struct ExposureReplay {
    exposures: Vec<f64>,
    idx: usize,
}

impl quantwave_core::traits::Next<&Bar> for ExposureReplay {
    type Output = quantwave_backtest::StrategySignal;

    fn next(&mut self, _bar: &Bar) -> Self::Output {
        let exposure = self.exposures[self.idx.min(self.exposures.len() - 1)];
        self.idx += 1;
        quantwave_backtest::StrategySignal {
            exposure,
            metadata: None,
        }
    }
}

#[test]
fn test_multi_symbol_batch_streaming_parity() {
    let df = make_two_symbol_df();
    let config = zero_cost_config("signal", Some("symbol".to_string()));
    let batch = BacktestEngine::new(config.clone())
        .run(df.clone().lazy())
        .expect("batch multi-symbol");

    let mut streaming_portfolio: Vec<f64> = Vec::new();
    for sym in ["AAA", "BBB"] {
        let sub = df
            .clone()
            .lazy()
            .filter(col("symbol").eq(lit(sym)))
            .collect()
            .unwrap();

        let ts: Vec<i64> = sub
            .column("timestamp")
            .unwrap()
            .i64()
            .unwrap()
            .into_iter()
            .map(|v| v.unwrap())
            .collect();
        let closes: Vec<f64> = sub
            .column("close")
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|v| v.unwrap())
            .collect();
        let exposures: Vec<f64> = sub
            .column("signal")
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|v| v.unwrap())
            .collect();

        let bars: Vec<Bar> = ts
            .iter()
            .zip(closes.iter())
            .map(|(&t, &close)| Bar {
                ts: chrono::DateTime::<chrono::Utc>::from_timestamp(t, 0).unwrap(),
                close,
                high: None,
                low: None,
            })
            .collect();

        let stream_res = run_streaming_simulation(
            &bars,
            ExposureReplay {
                exposures,
                idx: 0,
            },
            zero_cost_config("signal", None),
        )
        .expect("streaming per-symbol");

        let eq: Vec<f64> = stream_res
            .equity_curve
            .column("equity")
            .unwrap()
            .f64()
            .unwrap()
            .into_iter()
            .map(|v| v.unwrap())
            .collect();

        if streaming_portfolio.is_empty() {
            streaming_portfolio = eq;
        } else {
            for (i, v) in eq.iter().enumerate() {
                streaming_portfolio[i] += v;
            }
        }
    }

    let batch_portfolio = portfolio_equity_series(&batch);
    assert_eq!(batch_portfolio.len(), streaming_portfolio.len());
    for (i, (b, s)) in batch_portfolio.iter().zip(streaming_portfolio.iter()).enumerate() {
        assert_relative_eq!(*b, *s, epsilon = 1e-8, max_relative = 1e-8);
        if (b - s).abs() > 1e-7 {
            panic!("portfolio equity diverged at bar {i}: {b} vs {s}");
        }
    }

    assert_relative_eq!(
        *batch.stats.get("final_equity").unwrap(),
        streaming_portfolio.last().copied().unwrap(),
        epsilon = 1e-6
    );
}