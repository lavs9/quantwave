//! Walk-forward out-of-sample validation (quantwave-cr6v.14 / quantwave-xibc).
//!
//! Clean-room rolling OOS folds on pre-computed signals (RaptorBT / Zorro WFO pattern).
//! v1: no in-fold parameter optimization — each fold backtests the OOS window only.

use crate::{BacktestConfig, BacktestEngine, BacktestError, PerformanceMetrics};
use polars::prelude::*;
use std::collections::HashMap;

/// Rolling walk-forward configuration (bar counts on the unique timestamp index).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalkForwardConfig {
    /// In-sample warmup bars (skipped for OOS metrics; advances the window).
    pub train_bars: usize,
    /// Out-of-sample bars backtested per fold.
    pub test_bars: usize,
    /// Step between folds (defaults to `test_bars`).
    pub step_bars: Option<usize>,
}

impl WalkForwardConfig {
    pub fn new(train_bars: usize, test_bars: usize) -> Self {
        Self {
            train_bars,
            test_bars,
            step_bars: None,
        }
    }

    fn step(&self) -> usize {
        self.step_bars.unwrap_or(self.test_bars).max(1)
    }
}

/// Run walk-forward OOS backtests; returns fold × metrics DataFrame.
pub fn run_walk_forward(
    lf: LazyFrame,
    base_config: &BacktestConfig,
    wf: &WalkForwardConfig,
) -> Result<DataFrame, BacktestError> {
    if wf.train_bars == 0 || wf.test_bars == 0 {
        return Err(BacktestError::InvalidInput(
            "train_bars and test_bars must be > 0".into(),
        ));
    }

    let df = lf.collect()?;
    if df.height() == 0 {
        return Err(BacktestError::InvalidInput("empty dataframe".into()));
    }

    let ts_col = &base_config.timestamp_col;
    let timestamps = unique_sorted_timestamps(&df, ts_col)?;
    let step = wf.step();
    let mut fold_id = 0usize;
    let mut fold_ids = Vec::new();
    let mut oos_start = Vec::new();
    let mut oos_end = Vec::new();
    let mut train_lens = Vec::new();
    let mut test_lens = Vec::new();
    let mut metric_cols: HashMap<&'static str, Vec<f64>> = PerformanceMetrics::column_names()
        .iter()
        .map(|&n| (n, Vec::new()))
        .collect();

    let mut start = 0usize;
    while start + wf.train_bars + wf.test_bars <= timestamps.len() {
        let test_start_idx = start + wf.train_bars;
        let test_end_idx = test_start_idx + wf.test_bars;
        let ts_min = timestamps[test_start_idx];
        let ts_max = timestamps[test_end_idx - 1];

        let oos_lf = df
            .clone()
            .lazy()
            .filter(col(ts_col).gt_eq(lit(ts_min)).and(col(ts_col).lt_eq(lit(ts_max))));

        let report = BacktestEngine::new(base_config.clone()).backtest_with_report(oos_lf)?;

        fold_ids.push(fold_id as f64);
        oos_start.push(ts_min as f64);
        oos_end.push(ts_max as f64);
        train_lens.push(wf.train_bars as f64);
        test_lens.push(wf.test_bars as f64);
        for (name, value) in report.metrics.row_iter() {
            metric_cols.get_mut(name).unwrap().push(value);
        }

        fold_id += 1;
        start += step;
    }

    if fold_ids.is_empty() {
        return Err(BacktestError::InvalidInput(format!(
            "insufficient bars for walk-forward: need >= {} unique timestamps, got {}",
            wf.train_bars + wf.test_bars,
            timestamps.len()
        )));
    }

    let mut columns = vec![
        Column::new("fold_id".into(), fold_ids),
        Column::new("oos_start_ts".into(), oos_start),
        Column::new("oos_end_ts".into(), oos_end),
        Column::new("train_bars".into(), train_lens),
        Column::new("test_bars".into(), test_lens),
    ];
    for name in PerformanceMetrics::column_names() {
        columns.push(Column::new(
            PlSmallStr::from_str(name),
            metric_cols.remove(name).unwrap(),
        ));
    }

    DataFrame::new(columns).map_err(BacktestError::from)
}

fn unique_sorted_timestamps(df: &DataFrame, ts_col: &str) -> Result<Vec<i64>, BacktestError> {
    let ts = df
        .column(ts_col)
        .map_err(|e| BacktestError::InvalidInput(e.to_string()))?;
    let mut values: Vec<i64> = match ts.dtype() {
        DataType::Int64 => ts.i64().unwrap().into_iter().flatten().collect(),
        DataType::Int32 => ts
            .i32()
            .unwrap()
            .into_iter()
            .flatten()
            .map(|v| v as i64)
            .collect(),
        other => {
            return Err(BacktestError::InvalidInput(format!(
                "timestamp column must be Int64/Int32, got {other:?}"
            )));
        }
    };
    values.sort_unstable();
    values.dedup();
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn wf_base_df(n: usize) -> DataFrame {
        DataFrame::new(vec![
            Column::new(
                "timestamp".into(),
                (0..n as i64).map(|i| 1_700_000_000 + i * 3600).collect::<Vec<_>>(),
            ),
            Column::new(
                "close".into(),
                (0..n).map(|i| 100.0 + i as f64 * 0.1).collect::<Vec<_>>(),
            ),
            Column::new(
                "signal".into(),
                (0..n)
                    .map(|i| if (i / 20) % 2 == 0 { 1.0 } else { 0.0 })
                    .collect::<Vec<_>>(),
            ),
        ])
        .unwrap()
    }

    fn zero_cost_config() -> BacktestConfig {
        BacktestConfig {
            cost_model: crate::CostModel {
                commission_bps: 0.0,
                slippage_bps: 0.0,
                initial_cash: 100_000.0,
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_walk_forward_produces_two_folds() {
        let wf = WalkForwardConfig::new(30, 20);
        let df = run_walk_forward(
            wf_base_df(100).lazy(),
            &zero_cost_config(),
            &wf,
        )
        .unwrap();

        // 100 unique bars, train=30, test=20, step=20 → folds at 0, 20, 40
        assert_eq!(df.height(), 3);
        assert!(df.column("fold_id").is_ok());
        assert!(df.column("num_trades").is_ok());
        assert_relative_eq!(
            df.column("fold_id").unwrap().f64().unwrap().get(2).unwrap(),
            2.0,
            epsilon = 1e-9
        );
    }

    #[test]
    fn test_walk_forward_insufficient_bars_errors() {
        let wf = WalkForwardConfig::new(50, 50);
        let err = run_walk_forward(wf_base_df(60).lazy(), &zero_cost_config(), &wf)
            .unwrap_err()
            .to_string();
        assert!(err.contains("insufficient bars"));
    }

    #[test]
    fn test_walk_forward_oos_windows_do_not_overlap_when_step_equals_test() {
        let wf = WalkForwardConfig::new(20, 15);
        let df = run_walk_forward(wf_base_df(80).lazy(), &zero_cost_config(), &wf).unwrap();
        let starts = df.column("oos_start_ts").unwrap().f64().unwrap();
        let ends = df.column("oos_end_ts").unwrap().f64().unwrap();
        for i in 0..df.height() - 1 {
            assert!(ends.get(i).unwrap() < starts.get(i + 1).unwrap());
        }
    }
}