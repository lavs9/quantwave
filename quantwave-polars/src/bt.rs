//! Polars `.bt` backtest namespace (quantwave-cr6v.5).
//!
//! Usage: `df.lazy().bt().backtest(BtOptions { signal_col: "exposure".into(), .. })`

use polars::prelude::*;
use quantwave_backtest::{
    run_cross_sectional_backtest, run_param_sweep, run_walk_forward, single_param_variants,
    BacktestConfig, BacktestEngine, BacktestError, BacktestReport, BacktestResult, CostModel,
    CrossSectionalConfig, ExecutionDelay, StopConfig, SweepVariant, WalkForwardConfig,
};

/// Extension trait: `LazyFrame::bt()`.
pub trait QuantWaveBtExt {
    fn bt(&self) -> BtNamespace<'_>;
}

/// Namespace handle returned by [`QuantWaveBtExt::bt`].
pub struct BtNamespace<'a>(pub(crate) &'a LazyFrame);

/// Column names + cost knobs for a vectorized backtest run.
#[derive(Debug, Clone)]
pub struct BtOptions {
    pub signal_col: String,
    pub timestamp_col: String,
    pub close_col: String,
    pub symbol_col: Option<String>,
    pub entry_filter_col: Option<String>,
    pub size_multiplier_col: Option<String>,
    pub commission_bps: f64,
    pub slippage_bps: f64,
    pub initial_cash: f64,
    pub execution_delay: ExecutionDelay,
    pub stop_loss_pct: Option<f64>,
    pub take_profit_pct: Option<f64>,
    pub trailing_stop_pct: Option<f64>,
}

impl Default for BtOptions {
    fn default() -> Self {
        Self {
            signal_col: "signal".to_string(),
            timestamp_col: "timestamp".to_string(),
            close_col: "close".to_string(),
            symbol_col: None,
            entry_filter_col: None,
            size_multiplier_col: None,
            commission_bps: 5.0,
            slippage_bps: 2.0,
            initial_cash: 100_000.0,
            execution_delay: ExecutionDelay::SameBar,
            stop_loss_pct: None,
            take_profit_pct: None,
            trailing_stop_pct: None,
        }
    }
}

impl BtOptions {
    pub fn signal(signal_col: impl Into<String>) -> Self {
        Self {
            signal_col: signal_col.into(),
            ..Default::default()
        }
    }

    pub fn into_config(self) -> BacktestConfig {
        let costs = CostModel {
            commission_bps: self.commission_bps,
            slippage_bps: self.slippage_bps,
            initial_cash: self.initial_cash,
        };
        BacktestConfig {
            cost_model: costs.clone(),
            execution_model: quantwave_backtest::ExecutionModel::Simple(costs),
            timestamp_col: self.timestamp_col,
            symbol_col: self.symbol_col,
            close_col: self.close_col,
            signal_col: self.signal_col,
            entry_filter_col: self.entry_filter_col,
            size_multiplier_col: self.size_multiplier_col,
            execution_delay: self.execution_delay,
            stop_config: StopConfig {
                stop_loss_pct: self.stop_loss_pct,
                take_profit_pct: self.take_profit_pct,
                trailing_stop_pct: self.trailing_stop_pct,
            },
            ..Default::default()
        }
    }
}

impl<'a> BtNamespace<'a> {
    /// Run vectorized backtest on this LazyFrame (collected internally).
    pub fn backtest(self, options: BtOptions) -> Result<BacktestResult, BacktestError> {
        BacktestEngine::new(options.into_config()).run(self.0.clone())
    }

    /// Run backtest and attach [`BacktestReport`] metrics.
    pub fn backtest_with_report(self, options: BtOptions) -> Result<BacktestReport, BacktestError> {
        BacktestEngine::new(options.into_config()).backtest_with_report(self.0.clone())
    }

    /// Parameter sweep: one backtest per variant → param × metrics DataFrame (cr6v.12).
    pub fn sweep(
        self,
        variants: &[SweepVariant],
        options: BtOptions,
    ) -> Result<DataFrame, BacktestError> {
        run_param_sweep(self.0.clone(), variants, &options.into_config())
    }

    /// Convenience: single-param grid via parallel `param_values` and `signal_cols` slices.
    pub fn sweep_single_param(
        self,
        param_name: &str,
        param_values: &[f64],
        signal_cols: &[&str],
        options: BtOptions,
    ) -> Result<DataFrame, BacktestError> {
        let variants = single_param_variants(param_name, param_values, signal_cols)?;
        self.sweep(&variants, options)
    }

    /// Walk-forward OOS validation → fold × metrics DataFrame (cr6v.14).
    pub fn walk_forward(
        self,
        wf: WalkForwardConfig,
        options: BtOptions,
    ) -> Result<DataFrame, BacktestError> {
        run_walk_forward(self.0.clone(), &options.into_config(), &wf)
    }

    /// Cross-sectional factor rank → long/short backtest report (cr6v.15).
    pub fn cross_sectional_backtest(
        self,
        cs: CrossSectionalConfig,
        options: BtOptions,
    ) -> Result<BacktestReport, BacktestError> {
        run_cross_sectional_backtest(self.0.clone(), &cs, options.into_config())
    }
}

impl QuantWaveBtExt for LazyFrame {
    fn bt(&self) -> BtNamespace<'_> {
        BtNamespace(self)
    }
}