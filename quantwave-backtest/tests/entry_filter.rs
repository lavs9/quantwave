//! entry_filter_col + size_multiplier_col batch wiring (quantwave-cr6v.3).
//!
//! `cargo nextest run -p quantwave-backtest entry_filter`

use approx::assert_relative_eq;
use polars::prelude::*;
use quantwave_backtest::{
    apply_signal_modifiers, backtest_simple_bool_signal, run_streaming_simulation, BacktestConfig,
    BacktestEngine, Bar, CostModel, ExecutionModel,
};

fn zero_cost_config() -> BacktestConfig {
    BacktestConfig {
        execution_model: ExecutionModel::Simple(CostModel {
            commission_bps: 0.0,
            slippage_bps: 0.0,
            initial_cash: 100_000.0,
        }),
        signal_col: "signal".to_string(),
        ..Default::default()
    }
}

fn run_with_optional_cols(
    timestamps: Vec<i64>,
    closes: Vec<f64>,
    signals: Vec<f64>,
    entry_filter: Option<Vec<bool>>,
    size_multiplier: Option<Vec<f64>>,
) -> quantwave_backtest::BacktestResult {
    let mut cols: Vec<Column> = vec![
        Column::new("timestamp".into(), timestamps),
        Column::new("close".into(), closes),
        Column::new("signal".into(), signals),
    ];
    if let Some(f) = entry_filter {
        cols.push(Column::new("regime_ok".into(), f));
    }
    if let Some(m) = size_multiplier {
        cols.push(Column::new("size_mult".into(), m));
    }

    let df = DataFrame::new(cols).unwrap();
    let mut config = zero_cost_config();
    if df.column("regime_ok").is_ok() {
        config.entry_filter_col = Some("regime_ok".to_string());
    }
    if df.column("size_mult").is_ok() {
        config.size_multiplier_col = Some("size_mult".to_string());
    }

    BacktestEngine::new(config)
        .run(df.lazy())
        .expect("backtest run")
}

#[test]
fn test_entry_filter_blocks_exposure() {
    // Signal wants long on bars 1-2, but filter blocks → no trades.
    let result = run_with_optional_cols(
        (0..4).map(|i| 1_700_020_000 + i).collect(),
        vec![100.0, 101.0, 102.0, 103.0],
        vec![0.0, 1.0, 1.0, 0.0],
        Some(vec![true, false, false, true]),
        None,
    );

    assert_eq!(result.trades.height(), 0);

    // Control: same signal with filter always true → one trade.
    let control = run_with_optional_cols(
        (0..4).map(|i| 1_700_020_100 + i).collect(),
        vec![100.0, 101.0, 102.0, 103.0],
        vec![0.0, 1.0, 1.0, 0.0],
        Some(vec![true; 4]),
        None,
    );
    assert_eq!(control.trades.height(), 1);
}

#[test]
fn test_entry_filter_size_multiplier_scales_quantity() {
    let ts: Vec<i64> = (0..4).map(|i| 1_700_020_200 + i).collect();
    let closes = vec![100.0, 100.0, 110.0, 110.0];
    let signals = vec![0.0, 1.0, 1.0, 0.0];

    let base = run_with_optional_cols(
        ts.clone(),
        closes.clone(),
        signals.clone(),
        None,
        Some(vec![1.0, 1.0, 1.0, 1.0]),
    );
    let scaled = run_with_optional_cols(
        ts,
        closes,
        signals,
        None,
        Some(vec![1.0, 2.0, 2.0, 1.0]),
    );

    assert_eq!(base.trades.height(), 1);
    assert_eq!(scaled.trades.height(), 1);

    let base_pnl = base.trades.column("pnl_net").unwrap().f64().unwrap().get(0).unwrap();
    let scaled_pnl = scaled
        .trades
        .column("pnl_net")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap();
    assert_relative_eq!(base_pnl, 10.0, epsilon = 1e-6);
    assert_relative_eq!(scaled_pnl, 20.0, epsilon = 1e-6);

    let base_pos = base
        .equity_curve
        .column("position")
        .unwrap()
        .f64()
        .unwrap()
        .get(2)
        .unwrap();
    let scaled_pos = scaled
        .equity_curve
        .column("position")
        .unwrap()
        .f64()
        .unwrap()
        .get(2)
        .unwrap();
    assert_relative_eq!(base_pos, 1.0, epsilon = 1e-9);
    assert_relative_eq!(scaled_pos, 2.0, epsilon = 1e-9);
}

#[test]
fn test_entry_filter_and_multiplier_combined() {
    // Filter blocks bar 1; bar 2 passes filter with 3× size.
    let result = run_with_optional_cols(
        (0..4).map(|i| 1_700_020_300 + i).collect(),
        vec![100.0, 100.0, 100.0, 110.0],
        vec![0.0, 1.0, 1.0, 0.0],
        Some(vec![true, false, true, true]),
        Some(vec![1.0, 2.0, 3.0, 1.0]),
    );

    assert_eq!(result.trades.height(), 1);
    let pnl = result
        .trades
        .column("pnl_net")
        .unwrap()
        .f64()
        .unwrap()
        .get(0)
        .unwrap();
    // Enters on bar 2 @100 with 3 units, exits bar 3 @110 → pnl 30.
    assert_relative_eq!(pnl, 30.0, epsilon = 1e-6);
    assert_relative_eq!(
        apply_signal_modifiers(1.0, Some(false), Some(2.0)),
        0.0,
        epsilon = 1e-9
    );
    assert_relative_eq!(
        apply_signal_modifiers(1.0, Some(true), Some(3.0)),
        3.0,
        epsilon = 1e-9
    );
}

#[test]
fn test_entry_filter_optional_cols_none_regression() {
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

    let result = backtest_simple_bool_signal(df, "signal").expect("regression run");
    assert_eq!(result.trades.height(), 1);
    assert!(result.stats.get("num_trades").is_some());
}

struct FilterMultReplay {
    signals: Vec<f64>,
    filters: Option<Vec<bool>>,
    mults: Option<Vec<f64>>,
    idx: usize,
}

impl quantwave_core::traits::Next<&Bar> for FilterMultReplay {
    type Output = quantwave_backtest::StrategySignal;

    fn next(&mut self, _bar: &Bar) -> Self::Output {
        let i = self.idx.min(self.signals.len().saturating_sub(1));
        let exposure = apply_signal_modifiers(
            self.signals[i],
            self.filters.as_ref().map(|f| f[i]),
            self.mults.as_ref().map(|m| m[i]),
        );
        self.idx += 1;
        quantwave_backtest::StrategySignal {
            exposure,
            metadata: None,
        }
    }
}

#[test]
fn test_entry_filter_batch_streaming_parity() {
    let ts: Vec<i64> = (0..6).map(|i| 1_700_020_400 + i).collect();
    let closes = vec![100.0, 100.0, 101.0, 103.0, 104.0, 103.0];
    let signals = vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0];
    let filters = vec![true, false, true, true, true, true];
    let mults = vec![1.0, 1.0, 2.0, 2.0, 1.0, 1.0];

    let batch = run_with_optional_cols(
        ts.clone(),
        closes.clone(),
        signals.clone(),
        Some(filters.clone()),
        Some(mults.clone()),
    );

    let bars: Vec<Bar> = ts
        .iter()
        .zip(closes.iter())
        .map(|(&t, &close)| Bar {
            ts: chrono::DateTime::<chrono::Utc>::from_timestamp(t, 0).unwrap(),
            close,
        })
        .collect();

    let mut config = zero_cost_config();
    config.entry_filter_col = Some("regime_ok".to_string());
    config.size_multiplier_col = Some("size_mult".to_string());

    let stream = run_streaming_simulation(
        &bars,
        FilterMultReplay {
            signals,
            filters: Some(filters),
            mults: Some(mults),
            idx: 0,
        },
        config,
    )
    .expect("streaming run");

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