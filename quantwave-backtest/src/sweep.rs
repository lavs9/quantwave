//! Parameter sweep helper (quantwave-cr6v.12).
//!
//! Runs one backtest per variant and returns a Polars DataFrame with param columns
//! plus [`PerformanceMetrics`] columns (vectorbt / RaptorBT `batch_spread` pattern).

use crate::{BacktestConfig, BacktestEngine, BacktestError, PerformanceMetrics};
use polars::prelude::*;
use std::collections::HashMap;

/// One grid point: parameter values and the signal column to backtest.
#[derive(Debug, Clone, PartialEq)]
pub struct SweepVariant {
    pub params: HashMap<String, f64>,
    pub signal_col: String,
}

/// Build variants for a single-parameter grid (e.g. `hurst_period` → signal columns).
pub fn single_param_variants(
    param_name: impl Into<String>,
    param_values: &[f64],
    signal_cols: &[impl AsRef<str>],
) -> Result<Vec<SweepVariant>, BacktestError> {
    if param_values.len() != signal_cols.len() {
        return Err(BacktestError::InvalidInput(format!(
            "param_values len {} != signal_cols len {}",
            param_values.len(),
            signal_cols.len()
        )));
    }
    if param_values.is_empty() {
        return Err(BacktestError::InvalidInput(
            "sweep requires at least one variant".into(),
        ));
    }

    let name = param_name.into();
    Ok(param_values
        .iter()
        .zip(signal_cols.iter())
        .map(|(&value, col)| SweepVariant {
            params: HashMap::from([(name.clone(), value)]),
            signal_col: col.as_ref().to_string(),
        })
        .collect())
}

/// Run backtests for each variant and return a param × metrics DataFrame.
pub fn run_param_sweep(
    lf: LazyFrame,
    variants: &[SweepVariant],
    base_config: &BacktestConfig,
) -> Result<DataFrame, BacktestError> {
    if variants.is_empty() {
        return Err(BacktestError::InvalidInput(
            "sweep requires at least one variant".into(),
        ));
    }

    let param_keys = sorted_param_keys(variants);
    let mut param_cols: HashMap<String, Vec<f64>> =
        param_keys.iter().map(|k| (k.clone(), Vec::new())).collect();
    let mut metric_cols: HashMap<&'static str, Vec<f64>> = PerformanceMetrics::column_names()
        .iter()
        .map(|&name| (name, Vec::new()))
        .collect();

    for variant in variants {
        for key in &param_keys {
            let value = variant.params.get(key).copied().ok_or_else(|| {
                BacktestError::InvalidInput(format!(
                    "variant missing param key '{key}' (expected keys: {param_keys:?})"
                ))
            })?;
            param_cols.get_mut(key).unwrap().push(value);
        }

        let mut config = base_config.clone();
        config.signal_col = variant.signal_col.clone();
        let report = BacktestEngine::new(config).backtest_with_report(lf.clone())?;
        for (name, value) in report.metrics.row_iter() {
            metric_cols.get_mut(name).unwrap().push(value);
        }
    }

    let mut columns: Vec<Column> = Vec::new();
    for key in &param_keys {
        columns.push(Column::new(
            PlSmallStr::from_str(key),
            param_cols.remove(key).unwrap(),
        ));
    }
    for name in PerformanceMetrics::column_names() {
        columns.push(Column::new(
            PlSmallStr::from_str(name),
            metric_cols.remove(name).unwrap(),
        ));
    }

    DataFrame::new(columns).map_err(BacktestError::from)
}

fn sorted_param_keys(variants: &[SweepVariant]) -> Vec<String> {
    let mut keys: Vec<String> = variants[0].params.keys().cloned().collect();
    keys.sort();
    keys
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn sweep_base_df() -> DataFrame {
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
            Column::new("signal_early".into(), vec![0.0, 1.0, 1.0, 1.0, 0.0, 0.0]),
            Column::new("signal_late".into(), vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0]),
            Column::new("signal_flat".into(), vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
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
    fn test_sweep_single_param_returns_metrics_df() {
        let variants = single_param_variants(
            "threshold",
            &[0.5, 1.0, 2.0],
            &["signal_early", "signal_late", "signal_flat"],
        )
        .unwrap();

        let df = run_param_sweep(sweep_base_df().lazy(), &variants, &zero_cost_config()).unwrap();

        assert_eq!(df.height(), 3);
        assert!(df.column("threshold").is_ok());
        assert!(df.column("num_trades").is_ok());
        assert!(df.column("final_equity").is_ok());
        assert!(df.column("total_return").is_ok());

        let thresholds = df.column("threshold").unwrap().f64().unwrap();
        assert_relative_eq!(thresholds.get(0).unwrap(), 0.5, epsilon = 1e-9);
        assert_relative_eq!(thresholds.get(1).unwrap(), 1.0, epsilon = 1e-9);
        assert_relative_eq!(thresholds.get(2).unwrap(), 2.0, epsilon = 1e-9);

        let trades = df.column("num_trades").unwrap().f64().unwrap();
        assert_relative_eq!(trades.get(0).unwrap(), 1.0, epsilon = 1e-9);
        assert_relative_eq!(trades.get(1).unwrap(), 1.0, epsilon = 1e-9);
        assert_relative_eq!(trades.get(2).unwrap(), 0.0, epsilon = 1e-9);
    }

    #[test]
    fn test_sweep_variants_produce_different_final_equity() {
        let variants = single_param_variants(
            "entry_bar",
            &[1.0, 2.0],
            &["signal_early", "signal_late"],
        )
        .unwrap();

        let df = run_param_sweep(sweep_base_df().lazy(), &variants, &zero_cost_config()).unwrap();
        assert_eq!(df.height(), 2);

        let equity = df.column("final_equity").unwrap().f64().unwrap();
        let e0 = equity.get(0).unwrap();
        let e1 = equity.get(1).unwrap();
        assert!(
            (e0 - e1).abs() > 1.0,
            "early vs late entry should differ: {e0} vs {e1}"
        );
    }

    #[test]
    fn test_sweep_multi_param_explicit_variants() {
        let variants = vec![
            SweepVariant {
                params: HashMap::from([("stop_pct".to_string(), 0.05), ("mode".to_string(), 1.0)]),
                signal_col: "signal_early".into(),
            },
            SweepVariant {
                params: HashMap::from([("stop_pct".to_string(), 0.10), ("mode".to_string(), 1.0)]),
                signal_col: "signal_late".into(),
            },
            SweepVariant {
                params: HashMap::from([("stop_pct".to_string(), 0.05), ("mode".to_string(), 2.0)]),
                signal_col: "signal_flat".into(),
            },
        ];

        let df = run_param_sweep(sweep_base_df().lazy(), &variants, &zero_cost_config()).unwrap();
        assert_eq!(df.height(), 3);
        assert!(df.column("mode").is_ok());
        assert!(df.column("stop_pct").is_ok());
        assert_eq!(
            df.column("mode").unwrap().f64().unwrap().len(),
            3
        );
    }
}