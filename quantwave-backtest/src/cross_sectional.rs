//! Cross-sectional factor panel backtest (quantwave-cr6v.15 / quantwave-6ypp).
//!
//! sigc-inspired rank → long/short exposure assignment on universe panels.
//! Clean-room: no sigc runtime dependency.

use crate::{BacktestConfig, BacktestEngine, BacktestError, BacktestReport};
use polars::prelude::{RankMethod, RankOptions, *};

/// Factor panel long/short configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct CrossSectionalConfig {
    pub factor_col: String,
    /// Fraction of names to long (top rank by factor). e.g. 0.2 = top 20%.
    pub top_frac: f64,
    /// Fraction of names to short (bottom rank). e.g. 0.2 = bottom 20%.
    pub bottom_frac: f64,
}

impl CrossSectionalConfig {
    pub fn long_short(factor_col: impl Into<String>, top_frac: f64, bottom_frac: f64) -> Self {
        Self {
            factor_col: factor_col.into(),
            top_frac,
            bottom_frac,
        }
    }
}

/// Demean a factor within groups (e.g. sectors or timestamps).
pub fn neutralize_factor(lf: LazyFrame, factor_col: &str, group_col: &str) -> LazyFrame {
    lf.with_column(
        (col(factor_col) - col(factor_col).mean().over([col(group_col)])).alias(factor_col),
    )
}

/// Cross-sectional z-score (demean and divide by std deviation per timestamp).
pub fn zscore_factor(lf: LazyFrame, factor_col: &str, timestamp_col: &str) -> LazyFrame {
    let mean = col(factor_col).mean().over([col(timestamp_col)]);
    let std = col(factor_col).std(1).over([col(timestamp_col)]);
    lf.with_column(((col(factor_col) - mean) / std).alias(factor_col))
}

/// Clip outliers in a factor per timestamp beyond the given percentiles (0.0 to 1.0).
pub fn winsorize_factor(
    lf: LazyFrame,
    factor_col: &str,
    timestamp_col: &str,
    lower_pct: f64,
    upper_pct: f64,
) -> LazyFrame {
    let lower = col(factor_col)
        .quantile(lit(lower_pct), QuantileMethod::Nearest)
        .over([col(timestamp_col)]);
    let upper = col(factor_col)
        .quantile(lit(upper_pct), QuantileMethod::Nearest)
        .over([col(timestamp_col)]);

    lf.with_column(
        when(col(factor_col).lt(lower.clone()))
            .then(lower)
            .when(col(factor_col).gt(upper.clone()))
            .then(upper)
            .otherwise(col(factor_col))
            .alias(factor_col),
    )
}

/// Assign signed exposure from cross-sectional factor ranks per timestamp.
///
/// Returns a LazyFrame with `exposure_col` added (equal-weight within long/short legs).
pub fn assign_long_short_exposure(
    lf: LazyFrame,
    timestamp_col: &str,
    _symbol_col: &str,
    cs: &CrossSectionalConfig,
    exposure_col: &str,
) -> Result<LazyFrame, BacktestError> {
    if cs.top_frac <= 0.0 || cs.bottom_frac <= 0.0 {
        return Err(BacktestError::InvalidInput(
            "top_frac and bottom_frac must be > 0".into(),
        ));
    }
    if cs.top_frac + cs.bottom_frac > 1.0 {
        return Err(BacktestError::InvalidInput(
            "top_frac + bottom_frac must be <= 1".into(),
        ));
    }

    let n_per_ts = col(timestamp_col)
        .count()
        .over([col(timestamp_col)])
        .cast(DataType::Float64);

    // Rank 1 = highest factor (sigc-style long top names).
    let rank_best = col(&cs.factor_col)
        .rank(
            RankOptions {
                method: RankMethod::Min,
                descending: true,
            },
            None,
        )
        .over([col(timestamp_col)])
        .cast(DataType::Float64);

    let top_slots = when(
        (n_per_ts.clone() * lit(cs.top_frac))
            .cast(DataType::Int64)
            .cast(DataType::Float64)
            .lt(lit(1.0)),
    )
    .then(lit(1.0))
    .otherwise(
        (n_per_ts.clone() * lit(cs.top_frac))
            .cast(DataType::Int64)
            .cast(DataType::Float64),
    );
    let bottom_slots = when(
        (n_per_ts.clone() * lit(cs.bottom_frac))
            .cast(DataType::Int64)
            .cast(DataType::Float64)
            .lt(lit(1.0)),
    )
    .then(lit(1.0))
    .otherwise(
        (n_per_ts.clone() * lit(cs.bottom_frac))
            .cast(DataType::Int64)
            .cast(DataType::Float64),
    );
    let short_cut = n_per_ts - bottom_slots.clone() + lit(1.0);

    let exposure = when(rank_best.clone().lt_eq(top_slots.clone()))
        .then(lit(1.0) / top_slots.clone())
        .when(rank_best.gt_eq(short_cut))
        .then(lit(-1.0) / bottom_slots)
        .otherwise(lit(0.0))
        .alias(exposure_col);

    Ok(lf.with_column(exposure))
}

/// Rank factor panel → exposure column → multi-symbol backtest report.
pub fn run_cross_sectional_backtest(
    lf: LazyFrame,
    cs: &CrossSectionalConfig,
    mut base_config: BacktestConfig,
) -> Result<BacktestReport, BacktestError> {
    let symbol_col = base_config.symbol_col.clone().ok_or_else(|| {
        BacktestError::InvalidInput("symbol_col required for cross-sectional".into())
    })?;

    const EXPOSURE: &str = "cs_exposure";
    let with_exp =
        assign_long_short_exposure(lf, &base_config.timestamp_col, &symbol_col, cs, EXPOSURE)?;
    base_config.signal_col = EXPOSURE.to_string();
    BacktestEngine::new(base_config).backtest_with_report(with_exp)
}

#[cfg(test)]
#[allow(clippy::panic)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn panel_df() -> DataFrame {
        // 2 timestamps × 4 symbols; factor increases with symbol id at each ts.
        let timestamps = vec![1i64, 1, 1, 1, 2, 2, 2, 2];
        let symbols = vec!["A", "B", "C", "D", "A", "B", "C", "D"];
        let closes = vec![10.0, 10.0, 10.0, 10.0, 11.0, 11.0, 11.0, 11.0];
        let factor = vec![4.0, 3.0, 2.0, 1.0, 4.0, 3.0, 2.0, 1.0];
        DataFrame::new(vec![
            Column::new("timestamp".into(), timestamps),
            Column::new("symbol".into(), symbols),
            Column::new("close".into(), closes),
            Column::new("score".into(), factor),
        ])
        .unwrap()
    }

    #[test]
    fn test_factor_neutralize_demean_within_sector() {
        let df = DataFrame::new(vec![
            Column::new("sector".into(), vec!["Tech", "Tech", "Fin", "Fin"]),
            Column::new("score".into(), vec![10.0, 20.0, 100.0, 200.0]),
        ])
        .unwrap();

        let out = neutralize_factor(df.lazy(), "score", "sector")
            .collect()
            .unwrap();
        let scores = out.column("score").unwrap().f64().unwrap();

        let ts: Vec<f64> = scores.into_iter().map(|v| v.unwrap()).collect();
        assert_relative_eq!(ts[0], -5.0, epsilon = 1e-9);
        assert_relative_eq!(ts[1], 5.0, epsilon = 1e-9);
        assert_relative_eq!(ts[2], -50.0, epsilon = 1e-9);
        assert_relative_eq!(ts[3], 50.0, epsilon = 1e-9);
    }

    #[test]
    fn test_factor_zscore_zero_mean_smoke() {
        let df = panel_df();
        let out = zscore_factor(df.lazy(), "score", "timestamp")
            .collect()
            .unwrap();
        let scores = out.column("score").unwrap().f64().unwrap();
        let ts: Vec<f64> = scores.into_iter().map(|v| v.unwrap()).collect();

        // timestamp 1 (idx 0..4): values were 4, 3, 2, 1
        // mean = 2.5. std ≈ 1.2909944
        let mean_1 = (ts[0] + ts[1] + ts[2] + ts[3]) / 4.0;
        assert_relative_eq!(mean_1, 0.0, epsilon = 1e-9);
    }

    #[test]
    fn test_factor_winsorize_clips_extremes() {
        let df = DataFrame::new(vec![
            Column::new("timestamp".into(), vec![1i64, 1, 1, 1, 1]),
            Column::new("score".into(), vec![0.0, 10.0, 20.0, 30.0, 100.0]),
        ])
        .unwrap();

        let out = winsorize_factor(df.lazy(), "score", "timestamp", 0.2, 0.8)
            .collect()
            .unwrap();
        let scores = out.column("score").unwrap().f64().unwrap();
        let ts: Vec<f64> = scores.into_iter().map(|v| v.unwrap()).collect();

        // 5 elements. 0.2 quantile is rank 1 (idx 1 if sorted, but nearest).
        // 0.2 quantile of [0, 10, 20, 30, 100] ≈ 10.0
        // 0.8 quantile of [0, 10, 20, 30, 100] ≈ 30.0
        assert_relative_eq!(ts[0], 10.0, epsilon = 1e-9); // clipped
        assert_relative_eq!(ts[1], 10.0, epsilon = 1e-9);
        assert_relative_eq!(ts[4], 30.0, epsilon = 1e-9); // clipped
    }

    #[test]
    fn test_assign_long_short_exposure_top_bottom() {
        let cs = CrossSectionalConfig::long_short("score", 0.25, 0.25);
        let out =
            assign_long_short_exposure(panel_df().lazy(), "timestamp", "symbol", &cs, "exposure")
                .unwrap()
                .collect()
                .unwrap();

        let exposure = out.column("exposure").unwrap().f64().unwrap();
        // Top 25% of 4 = 1 name long (+1.0), bottom 25% = 1 short (-1.0)
        let ts1: Vec<f64> = exposure.into_iter().take(4).map(|v| v.unwrap()).collect();
        assert_eq!(ts1.iter().filter(|&&x| x > 0.0).count(), 1);
        assert_eq!(ts1.iter().filter(|&&x| x < 0.0).count(), 1);
        assert_relative_eq!(
            ts1.iter().map(|x| x.abs()).sum::<f64>(),
            2.0,
            epsilon = 1e-9
        );
    }

    #[test]
    fn test_cross_sectional_backtest_smoke() {
        let cs = CrossSectionalConfig::long_short("score", 0.25, 0.25);
        let cfg = BacktestConfig {
            cost_model: crate::CostModel {
                commission_bps: 0.0,
                slippage_bps: 0.0,
                initial_cash: 100_000.0,
            },
            symbol_col: Some("symbol".into()),
            ..Default::default()
        };
        // Hold top/bottom through both bars
        let mut df = panel_df();
        df = df
            .lazy()
            .with_column(lit(1.0).alias("score"))
            .collect()
            .unwrap();

        let report = run_cross_sectional_backtest(df.lazy(), &cs, cfg.clone()).unwrap();
        assert!(report.metrics.final_equity.is_finite());
    }

    #[test]
    fn test_cross_sectional_invalid_fracs_error() {
        let cs = CrossSectionalConfig::long_short("score", 0.6, 0.6);
        let result =
            assign_long_short_exposure(panel_df().lazy(), "timestamp", "symbol", &cs, "exposure");
        let Err(err) = result else {
            panic!("expected invalid frac error");
        };
        assert!(
            err.to_string().contains("top_frac"),
            "unexpected error message"
        );
    }
}
