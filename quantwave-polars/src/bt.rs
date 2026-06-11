//! Polars `.bt` backtest namespace (quantwave-cr6v.5).
//!
//! Usage: `df.lazy().bt().backtest(BtOptions { signal_col: "exposure".into(), .. })`

use polars::prelude::*;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, BacktestError, BacktestReport, BacktestResult, CostModel,
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
        BacktestConfig {
            cost_model: CostModel {
                commission_bps: self.commission_bps,
                slippage_bps: self.slippage_bps,
                initial_cash: self.initial_cash,
            },
            timestamp_col: self.timestamp_col,
            symbol_col: self.symbol_col,
            close_col: self.close_col,
            signal_col: self.signal_col,
            entry_filter_col: self.entry_filter_col,
            size_multiplier_col: self.size_multiplier_col,
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
}

impl QuantWaveBtExt for LazyFrame {
    fn bt(&self) -> BtNamespace<'_> {
        BtNamespace(self)
    }
}