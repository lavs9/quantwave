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
    let symbol_col = base_config
        .symbol_col
        .clone()
        .ok_or_else(|| BacktestError::InvalidInput("symbol_col required for cross-sectional".into()))?;

    const EXPOSURE: &str = "cs_exposure";
    let with_exp = assign_long_short_exposure(
        lf,
        &base_config.timestamp_col,
        &symbol_col,
        cs,
        EXPOSURE,
    )?;
    base_config.signal_col = EXPOSURE.to_string();
    BacktestEngine::new(base_config).backtest_with_report(with_exp)
}

#[cfg(test)]
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
    fn test_assign_long_short_exposure_top_bottom() {
        let cs = CrossSectionalConfig::long_short("score", 0.25, 0.25);
        let out = assign_long_short_exposure(
            panel_df().lazy(),
            "timestamp",
            "symbol",
            &cs,
            "exposure",
        )
        .unwrap()
        .collect()
        .unwrap();

        let exposure = out.column("exposure").unwrap().f64().unwrap();
        // Top 25% of 4 = 1 name long (+1.0), bottom 25% = 1 short (-1.0)
        let ts1: Vec<f64> = exposure.into_iter().take(4).map(|v| v.unwrap()).collect();
        assert_eq!(ts1.iter().filter(|&&x| x > 0.0).count(), 1);
        assert_eq!(ts1.iter().filter(|&&x| x < 0.0).count(), 1);
        assert_relative_eq!(ts1.iter().map(|x| x.abs()).sum::<f64>(), 2.0, epsilon = 1e-9);
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
        match assign_long_short_exposure(
            panel_df().lazy(),
            "timestamp",
            "symbol",
            &cs,
            "exposure",
        ) {
            Err(e) => assert!(e.to_string().contains("top_frac")),
            Ok(_) => panic!("expected invalid frac error"),
        }
    }
}