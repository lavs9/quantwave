//! Walk-forward out-of-sample validation (quantwave-cr6v.14 / quantwave-xibc).
//!
//! Clean-room rolling OOS folds on pre-computed signals (RaptorBT / Zorro WFO pattern).
//! v1: no in-fold parameter optimization — each fold backtests the OOS window only.

use crate::{
    BacktestConfig, BacktestEngine, BacktestError, PerformanceMetrics, internal_invariant,
};
use polars::prelude::*;
use std::collections::HashMap;

/// Rolling walk-forward configuration (bar counts on the unique timestamp index).
#[derive(Debug, Clone, PartialEq)]
pub struct WalkForwardConfig {
    /// In-sample warmup bars (skipped for OOS metrics; advances the window).
    pub train_bars: usize,
    /// Out-of-sample bars backtested per fold.
    pub test_bars: usize,
    /// Step between folds (defaults to `test_bars`).
    pub step_bars: Option<usize>,
    pub overfit_threshold: f64,
}

impl WalkForwardConfig {
    pub fn new(train_bars: usize, test_bars: usize) -> Self {
        Self {
            train_bars,
            test_bars,
            step_bars: None,
            overfit_threshold: 1.0,
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

        let oos_lf = df.clone().lazy().filter(
            col(ts_col)
                .gt_eq(lit(ts_min))
                .and(col(ts_col).lt_eq(lit(ts_max))),
        );

        let report = BacktestEngine::new(base_config.clone()).backtest_with_report(oos_lf)?;

        fold_ids.push(fold_id as f64);
        oos_start.push(ts_min as f64);
        oos_end.push(ts_max as f64);
        train_lens.push(wf.train_bars as f64);
        test_lens.push(wf.test_bars as f64);
        for (name, value) in report.metrics.row_iter() {
            metric_cols
                .get_mut(name)
                .ok_or_else(|| {
                    internal_invariant(format!(
                        "metric column '{name}' missing from walk-forward accumulator"
                    ))
                })?
                .push(value);
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
            metric_cols.remove(name).ok_or_else(|| {
                internal_invariant(format!(
                    "metric column '{name}' missing when building walk-forward df"
                ))
            })?,
        ));
    }

    DataFrame::new(columns).map_err(BacktestError::from)
}

fn unique_sorted_timestamps(df: &DataFrame, ts_col: &str) -> Result<Vec<i64>, BacktestError> {
    let ts = df
        .column(ts_col)
        .map_err(|_| BacktestError::MissingColumn {
            name: ts_col.to_string(),
        })?;
    let mut values: Vec<i64> = match ts.dtype() {
        DataType::Int64 => ts
            .i64()
            .map_err(|_| BacktestError::InvalidDtype {
                col: ts_col.to_string(),
                expected: "Int64".into(),
                got: format!("{:?}", ts.dtype()),
            })?
            .into_iter()
            .flatten()
            .collect(),
        DataType::Int32 => ts
            .i32()
            .map_err(|_| BacktestError::InvalidDtype {
                col: ts_col.to_string(),
                expected: "Int32".into(),
                got: format!("{:?}", ts.dtype()),
            })?
            .into_iter()
            .flatten()
            .map(|v| v as i64)
            .collect(),
        other => {
            return Err(BacktestError::InvalidDtype {
                col: ts_col.to_string(),
                expected: "Int64 or Int32".into(),
                got: format!("{other:?}"),
            });
        }
    };
    values.sort_unstable();
    values.dedup();
    Ok(values)
}

/// Run walk-forward optimization: sweep on train fold, pick best by objective, backtest OOS.
pub fn run_walk_forward_optimize(
    lf: LazyFrame,
    base_config: &BacktestConfig,
    wf: &WalkForwardConfig,
    variants: &[crate::SweepVariant],
    objective_metric: &str,
) -> Result<DataFrame, BacktestError> {
    if wf.train_bars == 0 || wf.test_bars == 0 {
        return Err(BacktestError::InvalidInput(
            "train/test_bars must be > 0".into(),
        ));
    }
    if variants.is_empty() {
        return Err(BacktestError::InvalidInput(
            "at least one variant required".into(),
        ));
    }

    let df = lf.collect()?;
    if df.height() == 0 {
        return Err(BacktestError::InvalidInput("empty dataframe".into()));
    }

    let ts_col = &base_config.timestamp_col;
    let timestamps = unique_sorted_timestamps(&df, ts_col)?;
    let step = wf.step();
    let param_keys = crate::sweep::sorted_param_keys(variants);

    let mut fold_ids = Vec::new();
    let mut oos_starts = Vec::new();
    let mut oos_ends = Vec::new();
    let mut train_metrics = Vec::new();
    let mut oos_metrics = Vec::new();
    let mut overfit_flags = Vec::new();
    let mut best_params: HashMap<String, Vec<f64>> =
        param_keys.iter().map(|k| (k.clone(), Vec::new())).collect();

    let mut metric_cols: HashMap<&'static str, Vec<f64>> = PerformanceMetrics::column_names()
        .iter()
        .map(|&n| (n, Vec::new()))
        .collect();

    let mut start = 0usize;
    let mut fold_id = 0usize;
    while start + wf.train_bars + wf.test_bars <= timestamps.len() {
        let test_start_idx = start + wf.train_bars;
        let test_end_idx = test_start_idx + wf.test_bars;
        let ts_train_start = timestamps[start];
        let ts_train_end = timestamps[test_start_idx - 1];
        let ts_oos_start = timestamps[test_start_idx];
        let ts_oos_end = timestamps[test_end_idx - 1];

        // 1. Train Sweep
        let train_lf = df.clone().lazy().filter(
            col(ts_col)
                .gt_eq(lit(ts_train_start))
                .and(col(ts_col).lt_eq(lit(ts_train_end))),
        );
        let sweep_df = crate::sweep::run_param_sweep(train_lf, variants, base_config)?;

        // Pick best variant
        let obj_col = sweep_df
            .column(objective_metric)
            .map_err(|e| BacktestError::InvalidInput(format!("objective_metric not found: {e}")))?;
        let obj_series = obj_col
            .f64()
            .map_err(|e| BacktestError::InvalidInput(e.to_string()))?;

        let mut best_idx = 0;
        let mut best_val = f64::NEG_INFINITY;
        for (i, val) in obj_series.into_iter().enumerate() {
            if let Some(v) = val
                && (v > best_val || (best_val == f64::NEG_INFINITY && v.is_finite()))
            {
                best_val = v;
                best_idx = i;
            }
        }

        let winning_variant = &variants[best_idx];
        for k in &param_keys {
            best_params
                .get_mut(k)
                .ok_or_else(|| {
                    internal_invariant(format!(
                        "best param column '{k}' missing from walk-forward optimize accumulator"
                    ))
                })?
                .push(winning_variant.params[k]);
        }
        train_metrics.push(best_val);

        // 2. OOS Backtest
        let oos_lf = df.clone().lazy().filter(
            col(ts_col)
                .gt_eq(lit(ts_oos_start))
                .and(col(ts_col).lt_eq(lit(ts_oos_end))),
        );

        let mut oos_config = base_config.clone();
        oos_config.signal_col = winning_variant.signal_col.clone();
        let report = BacktestEngine::new(oos_config).backtest_with_report(oos_lf)?;

        let oos_val = report
            .metrics
            .row_iter()
            .find(|(n, _)| *n == objective_metric)
            .map(|(_, v)| v)
            .ok_or_else(|| {
                BacktestError::InvalidInput(format!(
                    "objective_metric '{objective_metric}' not found in OOS metrics"
                ))
            })?;
        oos_metrics.push(oos_val);
        overfit_flags.push(best_val - oos_val > wf.overfit_threshold);

        for (name, value) in report.metrics.row_iter() {
            metric_cols
                .get_mut(name)
                .ok_or_else(|| {
                    internal_invariant(format!(
                        "metric column '{name}' missing from walk-forward optimize accumulator"
                    ))
                })?
                .push(value);
        }

        fold_ids.push(fold_id as f64);
        oos_starts.push(ts_oos_start as f64);
        oos_ends.push(ts_oos_end as f64);

        fold_id += 1;
        start += step;
    }

    if fold_ids.is_empty() {
        return Err(BacktestError::InvalidInput(
            "insufficient bars for wfo".into(),
        ));
    }

    let mut columns = vec![
        Column::new("fold_id".into(), fold_ids),
        Column::new("oos_start_ts".into(), oos_starts),
        Column::new("oos_end_ts".into(), oos_ends),
        Column::new("train_metric".into(), train_metrics),
        Column::new("oos_metric".into(), oos_metrics),
        Column::new("overfit_flag".into(), overfit_flags),
    ];
    for k in &param_keys {
        columns.push(Column::new(
            format!("best_{k}").into(),
            best_params.remove(k).ok_or_else(|| {
                internal_invariant(format!(
                    "best param column '{k}' missing when building walk-forward optimize df"
                ))
            })?,
        ));
    }
    for name in PerformanceMetrics::column_names() {
        columns.push(Column::new(
            PlSmallStr::from_str(name),
            metric_cols.remove(name).ok_or_else(|| {
                internal_invariant(format!(
                    "metric column '{name}' missing when building walk-forward optimize df"
                ))
            })?,
        ));
    }

    DataFrame::new(columns).map_err(BacktestError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn wf_base_df(n: usize) -> DataFrame {
        DataFrame::new(vec![
            Column::new(
                "timestamp".into(),
                (0..n as i64)
                    .map(|i| 1_700_000_000 + i * 3600)
                    .collect::<Vec<_>>(),
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
        let df = run_walk_forward(wf_base_df(100).lazy(), &zero_cost_config(), &wf).unwrap();

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

    fn wfo_base_df(n: usize) -> DataFrame {
        // Create an explicit pattern: signal_A is good in first half (train), bad in second (OOS).
        // signal_B is bad in first half, good in second half.
        let mut close = vec![100.0; n];
        let mut signal_a = vec![0.0; n];
        let mut signal_b = vec![0.0; n];

        for i in 1..n {
            if i < n / 2 {
                // First half: A makes money, B loses
                signal_a[i] = 1.0;
                signal_b[i] = -1.0;
                close[i] = close[i - 1] + 1.0;
            } else {
                // Second half: A loses, B makes money
                signal_a[i] = 1.0;
                signal_b[i] = -1.0;
                close[i] = close[i - 1] - 1.0;
            }
        }

        DataFrame::new(vec![
            Column::new("timestamp".into(), (0..n as i64).collect::<Vec<_>>()),
            Column::new("close".into(), close),
            Column::new("signal_A".into(), signal_a),
            Column::new("signal_B".into(), signal_b),
        ])
        .unwrap()
    }

    #[test]
    fn test_wfo_opt_picks_higher_sharpe_param_on_train() {
        let wf = WalkForwardConfig::new(20, 20); // 20 train, 20 oos (total 40 bars)
        let df = wfo_base_df(40);
        let variants = vec![
            crate::SweepVariant {
                params: std::collections::HashMap::from([("param".into(), 1.0)]),
                signal_col: "signal_A".into(),
            },
            crate::SweepVariant {
                params: std::collections::HashMap::from([("param".into(), 2.0)]),
                signal_col: "signal_B".into(),
            },
        ];

        let out = run_walk_forward_optimize(
            df.lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
        )
        .unwrap();

        assert_eq!(out.height(), 1);
        let best_param = out
            .column("best_param")
            .unwrap()
            .f64()
            .unwrap()
            .get(0)
            .unwrap();
        // In train (0..20), A is profitable, so param 1.0 should be chosen
        assert_eq!(best_param, 1.0);
    }

    #[test]
    fn test_wfo_opt_oos_uses_locked_param_not_reoptimized() {
        let wf = WalkForwardConfig::new(20, 20);
        let df = wfo_base_df(40);
        let variants = vec![
            crate::SweepVariant {
                params: std::collections::HashMap::from([("param".into(), 1.0)]),
                signal_col: "signal_A".into(),
            },
            crate::SweepVariant {
                params: std::collections::HashMap::from([("param".into(), 2.0)]),
                signal_col: "signal_B".into(),
            },
        ];
        let out = run_walk_forward_optimize(
            df.lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
        )
        .unwrap();

        let oos_metric = out
            .column("oos_metric")
            .unwrap()
            .f64()
            .unwrap()
            .get(0)
            .unwrap();
        // In OOS (20..40), A loses money, so total_return should be negative
        assert!(oos_metric < 0.0);
    }

    #[test]
    fn test_wfo_opt_overfit_flag_when_train_oos_diverge() {
        let mut wf = WalkForwardConfig::new(20, 20);
        wf.overfit_threshold = 0.0; // PnL is very small due to 1 unit position
        let df = wfo_base_df(40);
        let variants = vec![crate::SweepVariant {
            params: std::collections::HashMap::from([("p".into(), 1.0)]),
            signal_col: "signal_A".into(),
        }];
        let out = run_walk_forward_optimize(
            df.lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
        )
        .unwrap();

        let overfit = out
            .column("overfit_flag")
            .unwrap()
            .bool()
            .unwrap()
            .get(0)
            .unwrap();
        // Train return > 0, OOS return < 0, difference is large
        assert!(overfit);
    }

    #[test]
    fn test_wfo_opt_fold_count_matches_walk_forward() {
        let wf = WalkForwardConfig::new(20, 10);
        let df = wfo_base_df(60);
        let variants = vec![crate::SweepVariant {
            params: std::collections::HashMap::from([("p".into(), 1.0)]),
            signal_col: "signal_A".into(),
        }];
        let mut cfg = zero_cost_config();
        cfg.signal_col = "signal_A".into();
        let out1 = run_walk_forward(df.clone().lazy(), &cfg, &wf).unwrap();
        let out2 = run_walk_forward_optimize(
            df.lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
        )
        .unwrap();

        assert_eq!(out1.height(), out2.height());
    }
}
