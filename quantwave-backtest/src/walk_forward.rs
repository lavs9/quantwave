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

/// In-fold parameter search strategy used by [`run_walk_forward_optimize_with`] to
/// pick the winning variant on each training window.
///
/// `Grid` (the default everywhere) exhaustively backtests every variant on the train
/// fold, exactly as `run_walk_forward_optimize` has always done — existing behavior
/// and existing tests are unaffected.
///
/// `Tpe` is an optional Bayesian alternative: Tree-structured Parzen Estimator
/// (Bergstra et al. 2011, "Algorithms for Hyper-Parameter Optimization"). Instead of
/// backtesting every variant, it adaptively selects `n_trials` variants to backtest
/// per fold, useful when the grid is large. See `crate::tpe` for the implementation.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum InFoldOptimizer {
    #[default]
    Grid,
    Tpe(crate::TpeConfig),
}

/// Map each variant's parameter values to a point normalized to `[0, 1]` per
/// dimension (min-max over the variant pool), in `param_keys` order. TPE's KDE
/// scoring assumes roughly comparable dimension scales, so this keeps e.g. a
/// `period` parameter spanning 1..200 from swamping a `threshold` spanning 0..1.
/// A constant dimension (min == max across the pool) maps to `0.5` for every variant.
fn normalized_param_points(
    variants: &[crate::SweepVariant],
    param_keys: &[String],
) -> Vec<Vec<f64>> {
    let mut bounds: Vec<(f64, f64)> = param_keys
        .iter()
        .map(|k| {
            let mut lo = f64::INFINITY;
            let mut hi = f64::NEG_INFINITY;
            for v in variants {
                if let Some(&val) = v.params.get(k) {
                    lo = lo.min(val);
                    hi = hi.max(val);
                }
            }
            (lo, hi)
        })
        .collect();
    // Guard against no variant carrying a given key (shouldn't happen since
    // param_keys is derived from variants, but keeps this total).
    for b in &mut bounds {
        if !b.0.is_finite() || !b.1.is_finite() {
            *b = (0.0, 0.0);
        }
    }

    variants
        .iter()
        .map(|v| {
            param_keys
                .iter()
                .zip(bounds.iter())
                .map(|(k, &(lo, hi))| {
                    let val = v.params.get(k).copied().unwrap_or(lo);
                    if hi > lo { (val - lo) / (hi - lo) } else { 0.5 }
                })
                .collect()
        })
        .collect()
}

/// Pick the argmax of an in-fold objective column, skipping nulls and any value
/// that is not a usable measurement.
///
/// The selection is deliberately **NaN-safe** (quantwave-s3iu / quantwave-gz7d).
/// Ratio metrics report `f64::NAN` when their denominator is empty — no losing
/// trades, no downside deviation, no drawdown. `NaN > best_val` is always false
/// and `NaN.is_finite()` is false, so an undefined variant can never be selected;
/// it is skipped exactly like a null. This is the whole reason the module returns
/// `NaN` instead of `inf`: `inf > best_val` is *true*, so a degenerate variant
/// that simply never lost would beat every real one and be carried into the OOS
/// fold.
///
/// When every candidate is null/NaN there is no defensible winner: index 0 is
/// returned with `f64::NEG_INFINITY` as the objective value, which flows into the
/// fold's `train_metric` column and makes the degenerate fold visible downstream.
fn select_best_objective(values: impl Iterator<Item = Option<f64>>) -> (usize, f64) {
    let mut best_idx = 0;
    let mut best_val = f64::NEG_INFINITY;
    for (i, val) in values.enumerate() {
        if let Some(v) = val
            && (v > best_val || (best_val == f64::NEG_INFINITY && v.is_finite()))
        {
            best_val = v;
            best_idx = i;
        }
    }
    (best_idx, best_val)
}

/// Run walk-forward optimization: sweep on train fold, pick best by objective, backtest OOS.
///
/// Always uses the grid (exhaustive) in-fold optimizer — thin wrapper around
/// [`run_walk_forward_optimize_with`] kept for backward compatibility.
pub fn run_walk_forward_optimize(
    lf: LazyFrame,
    base_config: &BacktestConfig,
    wf: &WalkForwardConfig,
    variants: &[crate::SweepVariant],
    objective_metric: &str,
) -> Result<DataFrame, BacktestError> {
    run_walk_forward_optimize_with(
        lf,
        base_config,
        wf,
        variants,
        objective_metric,
        &InFoldOptimizer::Grid,
    )
}

/// Run walk-forward optimization with a selectable in-fold optimizer (grid or TPE).
/// See [`InFoldOptimizer`] for the strategies available. `run_walk_forward_optimize`
/// is the grid-only, backward-compatible entry point that delegates here.
pub fn run_walk_forward_optimize_with(
    lf: LazyFrame,
    base_config: &BacktestConfig,
    wf: &WalkForwardConfig,
    variants: &[crate::SweepVariant],
    objective_metric: &str,
    optimizer: &InFoldOptimizer,
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
    if !PerformanceMetrics::column_names().contains(&objective_metric) {
        return Err(BacktestError::InvalidInput(format!(
            "objective_metric '{objective_metric}' is not a known metric column"
        )));
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

        // 1. Train Sweep (in-fold optimization: grid = exhaustive, tpe = adaptive subset)
        let train_lf = df.clone().lazy().filter(
            col(ts_col)
                .gt_eq(lit(ts_train_start))
                .and(col(ts_col).lt_eq(lit(ts_train_end))),
        );

        let (best_idx, best_val) = match optimizer {
            InFoldOptimizer::Grid => {
                let sweep_df =
                    crate::sweep::run_param_sweep(train_lf.clone(), variants, base_config)?;

                let obj_col = sweep_df.column(objective_metric).map_err(|e| {
                    BacktestError::InvalidInput(format!("objective_metric not found: {e}"))
                })?;
                let obj_series = obj_col
                    .f64()
                    .map_err(|e| BacktestError::InvalidInput(e.to_string()))?;

                select_best_objective(obj_series.into_iter())
            }
            InFoldOptimizer::Tpe(tpe_config) => {
                let points = normalized_param_points(variants, &param_keys);
                let mut eval_err: Option<BacktestError> = None;
                let (idx, val, _history) = crate::tpe_select_from_pool(&points, tpe_config, |i| {
                    if eval_err.is_some() {
                        return f64::NEG_INFINITY;
                    }
                    let mut cfg = base_config.clone();
                    cfg.signal_col = variants[i].signal_col.clone();
                    match BacktestEngine::new(cfg).backtest_with_report(train_lf.clone()) {
                        Ok(report) => report
                            .metrics
                            .row_iter()
                            .find(|(n, _)| *n == objective_metric)
                            .map(|(_, v)| v)
                            .unwrap_or(f64::NEG_INFINITY),
                        Err(e) => {
                            eval_err = Some(e);
                            f64::NEG_INFINITY
                        }
                    }
                });
                if let Some(e) = eval_err {
                    return Err(e);
                }
                (idx, val)
            }
        };

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

    #[test]
    fn test_wfo_tpe_picks_higher_return_param_on_train() {
        // Same scenario as test_wfo_opt_picks_higher_sharpe_param_on_train, but using
        // the TPE in-fold optimizer instead of the default grid.
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

        let optimizer = InFoldOptimizer::Tpe(crate::TpeConfig::new(2, 7));
        let out = run_walk_forward_optimize_with(
            df.lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
            &optimizer,
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
        assert_eq!(best_param, 1.0);
    }

    #[test]
    fn test_wfo_tpe_matches_grid_fold_count() {
        let wf = WalkForwardConfig::new(20, 10);
        let df = wfo_base_df(60);
        let variants = vec![
            crate::SweepVariant {
                params: std::collections::HashMap::from([("p".into(), 1.0)]),
                signal_col: "signal_A".into(),
            },
            crate::SweepVariant {
                params: std::collections::HashMap::from([("p".into(), 2.0)]),
                signal_col: "signal_B".into(),
            },
        ];
        let optimizer = InFoldOptimizer::Tpe(crate::TpeConfig::new(2, 3));
        let out_grid = run_walk_forward_optimize(
            df.clone().lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
        )
        .unwrap();
        let out_tpe = run_walk_forward_optimize_with(
            df.lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
            &optimizer,
        )
        .unwrap();

        assert_eq!(out_grid.height(), out_tpe.height());
    }

    #[test]
    fn test_wfo_tpe_deterministic_given_seed() {
        let wf = WalkForwardConfig::new(20, 10);
        let df = wfo_base_df(60);
        let variants = vec![
            crate::SweepVariant {
                params: std::collections::HashMap::from([("p".into(), 1.0)]),
                signal_col: "signal_A".into(),
            },
            crate::SweepVariant {
                params: std::collections::HashMap::from([("p".into(), 2.0)]),
                signal_col: "signal_B".into(),
            },
        ];
        let optimizer = InFoldOptimizer::Tpe(crate::TpeConfig::new(2, 99));
        let out1 = run_walk_forward_optimize_with(
            df.clone().lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
            &optimizer,
        )
        .unwrap();
        let out2 = run_walk_forward_optimize_with(
            df.lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "total_return",
            &optimizer,
        )
        .unwrap();

        assert_eq!(
            out1.column("best_p").unwrap().f64().unwrap().get(0),
            out2.column("best_p").unwrap().f64().unwrap().get(0)
        );
        assert_eq!(
            out1.column("train_metric").unwrap().f64().unwrap().get(0),
            out2.column("train_metric").unwrap().f64().unwrap().get(0)
        );
    }

    #[test]
    fn test_wfo_unknown_objective_metric_errors() {
        let wf = WalkForwardConfig::new(20, 20);
        let df = wfo_base_df(40);
        let variants = vec![crate::SweepVariant {
            params: std::collections::HashMap::from([("param".into(), 1.0)]),
            signal_col: "signal_A".into(),
        }];
        let err = run_walk_forward_optimize(
            df.lazy(),
            &zero_cost_config(),
            &wf,
            &variants,
            "not_a_real_metric",
        )
        .unwrap_err()
        .to_string();
        assert!(err.contains("not_a_real_metric"));
    }

    // --- In-fold objective selection is NaN-safe (quantwave-s3iu / quantwave-gz7d) ---

    #[test]
    fn objective_selection_picks_the_finite_maximum() {
        let (idx, val) = select_best_objective([Some(0.1), Some(0.9), Some(0.4)].into_iter());
        assert_eq!(idx, 1);
        assert!((val - 0.9).abs() < 1e-12);
    }

    /// An undefined (NaN) objective must never win — it is skipped like a null.
    /// This is the property that makes the NaN convention safe for optimizers:
    /// with `inf` the degenerate variant at index 1 would be selected instead.
    #[test]
    fn objective_selection_skips_nan_variants() {
        let (idx, val) = select_best_objective([Some(0.3), Some(f64::NAN), Some(0.5)].into_iter());
        assert_eq!(
            idx, 2,
            "the real maximum must win, not the undefined variant"
        );
        assert!((val - 0.5).abs() < 1e-12);
    }

    /// The contrast case, asserted so the regression is explicit: `inf` *does*
    /// win a `>` comparison. This is why calmar_ratio no longer returns it.
    #[test]
    fn objective_selection_would_have_been_hijacked_by_inf() {
        let (idx, _) =
            select_best_objective([Some(0.3), Some(f64::INFINITY), Some(0.5)].into_iter());
        assert_eq!(idx, 1, "documents the hazard the NaN convention removes");
    }

    #[test]
    fn objective_selection_skips_nulls_and_nan_alike() {
        let (idx, val) =
            select_best_objective([None, Some(f64::NAN), Some(-2.0), None].into_iter());
        assert_eq!(idx, 2);
        assert!((val + 2.0).abs() < 1e-12);
    }

    /// Every candidate undefined: no defensible winner, so index 0 with a
    /// `-inf` objective, which surfaces as the fold's `train_metric`.
    #[test]
    fn objective_selection_all_nan_falls_back_to_first_with_neg_infinity() {
        let (idx, val) = select_best_objective([Some(f64::NAN), Some(f64::NAN)].into_iter());
        assert_eq!(idx, 0);
        assert_eq!(val, f64::NEG_INFINITY);
    }

    #[test]
    fn objective_selection_accepts_negative_maxima() {
        let (idx, val) = select_best_objective([Some(-5.0), Some(-1.0)].into_iter());
        assert_eq!(idx, 1);
        assert!((val + 1.0).abs() < 1e-12);
    }
}
