//! PyO3 bindings for `quantwave-backtest`.
//!
//! Exposes `BacktestConfig`, `BacktestEngine`, `BacktestResult`, `PerformanceMetrics`,
//! and `BacktestReport` to Python with zero-copy Polars DataFrame interop via `pyo3-polars`.

use chrono::{DateTime, Utc};
use polars::io::ipc::IpcReader;
use polars::prelude::*;
use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyType};
use pyo3_polars::PyDataFrame;
use quantwave_backtest::risk::{
    InverseVolConfig, PositionLimitConfig, PreTradeConfig, RiskModel, VolTargetConfig,
};
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, BacktestError, BacktestReport, BacktestResult,
    BenchmarkMetrics, CostModel, ExecutionDelay, ExecutionModel, InFoldOptimizer, MonteCarloConfig,
    MonteCarloPathSummary, MonteCarloReturnConfig, MonteCarloSummary, Order, OrderType,
    PerformanceMetrics, PortfolioAllocator, PortfolioMode, RebalancePolicy, Side, StopConfig,
    StopEvaluationMode, SweepVariant, TearsheetOptions, TpeConfig, WalkForwardConfig,
    monte_carlo_return_paths, monte_carlo_trade_bootstrap, render_tearsheet_html,
    run_order_simulation, run_walk_forward, run_walk_forward_optimize_with,
};
use std::io::Cursor;

fn parse_stop_evaluation(touched_exit: bool) -> StopEvaluationMode {
    if touched_exit {
        StopEvaluationMode::OhlcTouched
    } else {
        StopEvaluationMode::CloseOnly
    }
}

fn parse_portfolio_mode(s: &str) -> PyResult<PortfolioMode> {
    match s.to_ascii_lowercase().as_str() {
        "independent" | "independent_books" | "independentbooks" => {
            Ok(PortfolioMode::IndependentBooks)
        }
        "shared_capital" | "shared" | "sharedcapital" => Ok(PortfolioMode::SharedCapital),
        other => Err(PyValueError::new_err(format!(
            "portfolio_mode must be 'independent_books' or 'shared_capital', got '{other}'"
        ))),
    }
}

fn parse_portfolio_allocator(s: &str) -> PyResult<PortfolioAllocator> {
    match s.to_ascii_lowercase().as_str() {
        "equal_weight" | "equalweight" | "equal" => Ok(PortfolioAllocator::EqualWeight),
        "signal_weighted" | "signalweighted" | "signal" => Ok(PortfolioAllocator::SignalWeighted),
        other => Err(PyValueError::new_err(format!(
            "portfolio_allocator must be 'equal_weight' or 'signal_weighted', got '{other}'"
        ))),
    }
}

/// Parse the in-fold optimizer selection for `walk_forward_optimize`. `"grid"`
/// (default, exhaustive — existing behavior) or `"tpe"` (Bayesian, adaptive subset —
/// quantwave-lzzq). `n_trials` is required for `"tpe"`.
fn parse_in_fold_optimizer(
    optimizer: &str,
    n_trials: Option<usize>,
    seed: u64,
) -> PyResult<InFoldOptimizer> {
    match optimizer.to_ascii_lowercase().as_str() {
        "grid" => Ok(InFoldOptimizer::Grid),
        "tpe" => {
            let n_trials = n_trials.ok_or_else(|| {
                PyValueError::new_err("n_trials is required when optimizer='tpe'")
            })?;
            if n_trials == 0 {
                return Err(PyValueError::new_err("n_trials must be > 0"));
            }
            Ok(InFoldOptimizer::Tpe(TpeConfig::new(n_trials, seed)))
        }
        other => Err(PyValueError::new_err(format!(
            "optimizer must be 'grid' or 'tpe', got '{other}'"
        ))),
    }
}

fn parse_execution_delay(s: &str) -> PyResult<ExecutionDelay> {
    match s.to_ascii_lowercase().as_str() {
        "same_bar" | "samebar" | "t0" => Ok(ExecutionDelay::SameBar),
        "next_bar" | "nextbar" | "t1" => Ok(ExecutionDelay::NextBar),
        other => Err(PyValueError::new_err(format!(
            "execution_delay must be 'same_bar' or 'next_bar', got '{other}'"
        ))),
    }
}

fn get_subdict<'py>(dict: &Bound<'py, PyDict>, key: &str) -> PyResult<Option<Bound<'py, PyDict>>> {
    match dict.get_item(key)? {
        Some(v) if !v.is_none() => {
            Ok(Some(v.downcast_into::<PyDict>().map_err(|e| {
                PyTypeError::new_err(format!("'{key}' must be a dict, got {e}"))
            })?))
        }
        _ => Ok(None),
    }
}

fn get_f64(dict: &Bound<'_, PyDict>, key: &str, default: f64) -> PyResult<f64> {
    match dict.get_item(key)? {
        Some(v) if !v.is_none() => v.extract(),
        _ => Ok(default),
    }
}

fn get_opt_f64(dict: &Bound<'_, PyDict>, key: &str) -> PyResult<Option<f64>> {
    match dict.get_item(key)? {
        Some(v) if !v.is_none() => Ok(Some(v.extract()?)),
        _ => Ok(None),
    }
}

fn get_usize(dict: &Bound<'_, PyDict>, key: &str, default: usize) -> PyResult<usize> {
    match dict.get_item(key)? {
        Some(v) if !v.is_none() => v.extract(),
        _ => Ok(default),
    }
}

fn get_bool(dict: &Bound<'_, PyDict>, key: &str, default: bool) -> PyResult<bool> {
    match dict.get_item(key)? {
        Some(v) if !v.is_none() => v.extract(),
        _ => Ok(default),
    }
}

fn parse_vol_target_config(d: &Bound<'_, PyDict>) -> PyResult<VolTargetConfig> {
    let default = VolTargetConfig::default();
    Ok(VolTargetConfig {
        target_annual_vol: get_f64(d, "target_annual_vol", default.target_annual_vol)?,
        lookback: get_usize(d, "lookback", default.lookback)?,
        bars_per_year: get_f64(d, "bars_per_year", default.bars_per_year)?,
        min_scale: get_f64(d, "min_scale", default.min_scale)?,
        max_scale: get_f64(d, "max_scale", default.max_scale)?,
    })
}

fn parse_inverse_vol_config(d: &Bound<'_, PyDict>) -> PyResult<InverseVolConfig> {
    let default = InverseVolConfig::default();
    Ok(InverseVolConfig {
        target_annual_vol: get_f64(d, "target_annual_vol", default.target_annual_vol)?,
        lookback: get_usize(d, "lookback", default.lookback)?,
        bars_per_year: get_f64(d, "bars_per_year", default.bars_per_year)?,
        min_scale: get_f64(d, "min_scale", default.min_scale)?,
        max_scale: get_f64(d, "max_scale", default.max_scale)?,
    })
}

fn parse_position_limit_config(d: &Bound<'_, PyDict>) -> PyResult<PositionLimitConfig> {
    Ok(PositionLimitConfig {
        max_abs_exposure: get_opt_f64(d, "max_abs_exposure")?,
        max_leverage: get_opt_f64(d, "max_leverage")?,
    })
}

fn parse_pre_trade_config(d: &Bound<'_, PyDict>) -> PyResult<PreTradeConfig> {
    Ok(PreTradeConfig {
        max_notional: get_opt_f64(d, "max_notional")?,
        max_leverage: get_opt_f64(d, "max_leverage")?,
        veto_on_breach: get_bool(d, "veto_on_breach", false)?,
    })
}

/// Parse a Python dict like
/// `{"vol_target": {"target_annual_vol": 0.15, "lookback": 20}, "position_limit": {"max_abs_exposure": 50.0}}`
/// into a `RiskModel`. Only the overlays present as keys are set; every
/// sub-config field not supplied falls back to its Rust-side `Default`.
/// Passing `None` at the call site (not this function) is what keeps
/// default backtests byte-identical to pre-risk-model behavior.
fn parse_risk_model(dict: &Bound<'_, PyDict>) -> PyResult<RiskModel> {
    let mut model = RiskModel::default();
    if let Some(d) = get_subdict(dict, "vol_target")? {
        model.vol_target = Some(parse_vol_target_config(&d)?);
    }
    if let Some(d) = get_subdict(dict, "inverse_vol")? {
        model.inverse_vol = Some(parse_inverse_vol_config(&d)?);
    }
    if let Some(d) = get_subdict(dict, "position_limit")? {
        model.position_limit = Some(parse_position_limit_config(&d)?);
    }
    if let Some(d) = get_subdict(dict, "pre_trade")? {
        model.pre_trade = Some(parse_pre_trade_config(&d)?);
    }

    const KNOWN: [&str; 4] = ["vol_target", "inverse_vol", "position_limit", "pre_trade"];
    for key in dict.keys().iter() {
        let key_str: String = key.extract()?;
        if !KNOWN.contains(&key_str.as_str()) {
            return Err(PyValueError::new_err(format!(
                "risk_model: unknown key '{key_str}', expected one of {KNOWN:?}"
            )));
        }
    }
    Ok(model)
}

/// Parse a Python dict like `{"calendar": {"every_n_bars": 5}}`,
/// `{"drift": {"threshold": 0.05}}`, `{"signal": {}}`, or
/// `{"turnover": {"min_turnover": 0.02}}` into a `RebalancePolicy`. Exactly
/// one top-level key is required.
fn parse_rebalance_policy(dict: &Bound<'_, PyDict>) -> PyResult<RebalancePolicy> {
    if dict.len() != 1 {
        return Err(PyValueError::new_err(
            "rebalance_policy dict must have exactly one key: 'calendar', 'drift', 'signal', or 'turnover'",
        ));
    }
    let (key, value) = dict
        .iter()
        .next()
        .ok_or_else(|| PyValueError::new_err("rebalance_policy dict is empty"))?;
    let key: String = key.extract()?;
    match key.as_str() {
        "calendar" => {
            let sub = value
                .downcast::<PyDict>()
                .map_err(|e| PyTypeError::new_err(format!("'calendar' must be a dict: {e}")))?;
            let every_n_bars = get_usize(sub, "every_n_bars", 1)?;
            Ok(RebalancePolicy::Calendar { every_n_bars })
        }
        "drift" => {
            let sub = value
                .downcast::<PyDict>()
                .map_err(|e| PyTypeError::new_err(format!("'drift' must be a dict: {e}")))?;
            let threshold = get_f64(sub, "threshold", 0.0)?;
            Ok(RebalancePolicy::Drift { threshold })
        }
        "signal" => Ok(RebalancePolicy::Signal),
        "turnover" => {
            let sub = value
                .downcast::<PyDict>()
                .map_err(|e| PyTypeError::new_err(format!("'turnover' must be a dict: {e}")))?;
            let min_turnover = get_f64(sub, "min_turnover", 0.0)?;
            Ok(RebalancePolicy::Turnover { min_turnover })
        }
        other => Err(PyValueError::new_err(format!(
            "rebalance_policy: unknown key '{other}', expected one of 'calendar', 'drift', 'signal', 'turnover'"
        ))),
    }
}

fn map_err(e: BacktestError) -> PyErr {
    match e {
        BacktestError::MissingColumn { name } => PyKeyError::new_err(name),
        BacktestError::InvalidDtype { col, expected, got } => PyTypeError::new_err(format!(
            "invalid dtype for column '{col}': expected {expected}, got {got}"
        )),
        BacktestError::InternalInvariant { context } => PyRuntimeError::new_err(context),
        BacktestError::UnsortedData | BacktestError::InvalidInput(_) => {
            PyValueError::new_err(e.to_string())
        }
        BacktestError::Polars(pe) => PyValueError::new_err(pe.to_string()),
    }
}

/// Convert a Python Polars DataFrame via IPC bytes (no pyarrow / compat_level coupling).
fn dataframe_from_py(ob: &Bound<'_, PyAny>) -> PyResult<DataFrame> {
    let py = ob.py();
    let io = PyModule::import(py, "io")?;
    let buf = io.getattr("BytesIO")?.call0()?;
    ob.call_method1("write_ipc", (buf.clone(),))?;
    let bytes = buf
        .call_method0("getvalue")?
        .extract::<Bound<'_, PyBytes>>()?;
    let cursor = Cursor::new(bytes.as_bytes());
    IpcReader::new(cursor)
        .finish()
        .map_err(|e| PyValueError::new_err(e.to_string()))
}

fn stats_to_dict<'py>(
    py: Python<'py>,
    stats: &std::collections::HashMap<String, f64>,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (k, v) in stats {
        dict.set_item(k, v)?;
    }
    Ok(dict)
}

fn metrics_to_dict<'py>(py: Python<'py>, m: &PerformanceMetrics) -> PyResult<Bound<'py, PyDict>> {
    // NOTE: keys here are a stable, tested contract (see
    // tests/python/test_backtest_output_contract.py, test_backtest.py,
    // test_pa_flag_backtest.py — all assert an *exact* key set). Do not add keys
    // here; new additive metrics (calmar/VaR/CVaR/benchmark) live in
    // `extended_metrics_to_dict` / `.extended_metrics()` / `.metrics_with_benchmark()`
    // instead (quantwave-b5gr).
    let dict = PyDict::new(py);
    dict.set_item("num_trades", m.num_trades)?;
    dict.set_item("win_rate", m.win_rate)?;
    dict.set_item("profit_factor", m.profit_factor)?;
    dict.set_item("max_drawdown_pct", m.max_drawdown_pct)?;
    dict.set_item("cagr", m.cagr)?;
    dict.set_item("sharpe_ratio", m.sharpe_ratio)?;
    dict.set_item("sortino_ratio", m.sortino_ratio)?;
    dict.set_item("total_return", m.total_return)?;
    dict.set_item("final_equity", m.final_equity)?;
    dict.set_item("avg_trade_pnl", m.avg_trade_pnl)?;
    Ok(dict)
}

fn benchmark_metrics_to_dict<'py>(
    py: Python<'py>,
    b: &BenchmarkMetrics,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("alpha", b.alpha)?;
    dict.set_item("beta", b.beta)?;
    dict.set_item("cumulative_return", b.cumulative_return)?;
    dict.set_item("benchmark_cumulative_return", b.benchmark_cumulative_return)?;
    dict.set_item("excess_cumulative_return", b.excess_cumulative_return)?;
    Ok(dict)
}

/// Statistical-trustworthiness diagnostics for a metrics bundle (quantwave-s3iu).
///
/// Additive surface only — never merged into `metrics_to_dict`, whose 10 keys are
/// a frozen, test-enforced contract.
fn diagnostics_to_dict<'py>(
    py: Python<'py>,
    m: &PerformanceMetrics,
) -> PyResult<Bound<'py, PyDict>> {
    let d = m.diagnostics();
    let dict = PyDict::new(py);
    dict.set_item("low_sample_size", d.low_sample_size)?;
    dict.set_item("num_trades", d.num_trades)?;
    dict.set_item(
        "min_trades_for_reliable_ratios",
        d.min_trades_for_reliable_ratios,
    )?;
    dict.set_item("undefined_metrics", d.undefined_metrics)?;
    dict.set_item("warnings", d.warnings)?;
    Ok(dict)
}

/// Extended (additive, quantwave-b5gr) metrics: base 10 keys plus
/// `calmar_ratio` / `var_95` / `cvar_95`, a nested `benchmark` dict when
/// benchmark-relative analytics are attached, and a `diagnostics` dict
/// (quantwave-s3iu). Opt-in surface — `.metrics()` keeps its original 10-key
/// contract untouched.
fn extended_metrics_to_dict<'py>(
    py: Python<'py>,
    m: &PerformanceMetrics,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = metrics_to_dict(py, m)?;
    dict.set_item("calmar_ratio", m.calmar_ratio)?;
    dict.set_item("var_95", m.var_95)?;
    dict.set_item("cvar_95", m.cvar_95)?;
    dict.set_item("diagnostics", diagnostics_to_dict(py, m)?)?;
    match &m.benchmark {
        Some(b) => dict.set_item("benchmark", benchmark_metrics_to_dict(py, b)?)?,
        None => dict.set_item("benchmark", py.None())?,
    }
    Ok(dict)
}

#[pyclass(name = "BacktestConfig")]
#[derive(Clone)]
pub struct PyBacktestConfig {
    inner: BacktestConfig,
}

#[pymethods]
impl PyBacktestConfig {
    /// Configuration for a backtest run.
    ///
    /// Parameters with `pct` suffixes (like stop_loss_pct) expect fractions (e.g., 0.05 for 5%).
    /// Default position sizing is 1 unit unless size_multiplier_col is provided.
    /// Execution delay can be 'next_bar' (fill on the next bar's close — the default,
    /// no same-bar look-ahead) or 'same_bar' (fill on the signal bar's own close).
    /// Only use 'same_bar' for close-auction execution or signals built purely from
    /// bar t-1 data; otherwise it is systematically optimistic (quantwave-zmjw).
    #[new]
    #[pyo3(signature = (
        signal_col = "signal",
        timestamp_col = "timestamp",
        close_col = "close",
        high_col = None,
        low_col = None,
        symbol_col = None,
        entry_filter_col = None,
        size_multiplier_col = None,
        initial_cash = 100_000.0,
        commission_bps = 5.0,
        slippage_bps = 2.0,
        execution_delay = "next_bar",
        stop_loss_pct = None,
        take_profit_pct = None,
        trailing_stop_pct = None,
        touched_exit = false,
        portfolio_mode = "independent_books",
        portfolio_allocator = "equal_weight",
        risk_model = None,
        rebalance_policy = None,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        signal_col: &str,
        timestamp_col: &str,
        close_col: &str,
        high_col: Option<String>,
        low_col: Option<String>,
        symbol_col: Option<String>,
        entry_filter_col: Option<String>,
        size_multiplier_col: Option<String>,
        initial_cash: f64,
        commission_bps: f64,
        slippage_bps: f64,
        execution_delay: &str,
        stop_loss_pct: Option<f64>,
        take_profit_pct: Option<f64>,
        trailing_stop_pct: Option<f64>,
        touched_exit: bool,
        portfolio_mode: &str,
        portfolio_allocator: &str,
        risk_model: Option<Bound<'_, PyDict>>,
        rebalance_policy: Option<Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let costs = CostModel {
            commission_bps,
            slippage_bps,
            initial_cash,
        };
        let risk_model = risk_model.as_ref().map(parse_risk_model).transpose()?;
        let rebalance_policy = rebalance_policy
            .as_ref()
            .map(parse_rebalance_policy)
            .transpose()?;
        Ok(Self {
            inner: BacktestConfig {
                cost_model: costs.clone(),
                execution_model: ExecutionModel::Simple(costs),
                timestamp_col: timestamp_col.to_string(),
                symbol_col,
                close_col: close_col.to_string(),
                high_col,
                low_col,
                signal_col: signal_col.to_string(),
                entry_filter_col,
                size_multiplier_col,
                execution_delay: parse_execution_delay(execution_delay)?,
                stop_config: StopConfig {
                    stop_loss_pct,
                    take_profit_pct,
                    trailing_stop_pct,
                    stop_evaluation: parse_stop_evaluation(touched_exit),
                },
                portfolio_mode: parse_portfolio_mode(portfolio_mode)?,
                portfolio_allocator: parse_portfolio_allocator(portfolio_allocator)?,
                risk_model,
                rebalance_policy,
                ..Default::default()
            },
        })
    }
}

#[pyclass(name = "BacktestEngine")]
pub struct PyBacktestEngine {
    inner: BacktestEngine,
}

#[pymethods]
impl PyBacktestEngine {
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<PyBacktestConfig>) -> Self {
        let config = config.map(|c| c.inner).unwrap_or_default();
        Self {
            inner: BacktestEngine::new(config),
        }
    }

    #[classmethod]
    fn with_default_costs(_cls: &Bound<'_, PyType>) -> Self {
        Self {
            inner: BacktestEngine::with_default_costs(),
        }
    }

    fn run(&self, df: &Bound<'_, PyAny>) -> PyResult<PyBacktestResult> {
        let result = self
            .inner
            .run(dataframe_from_py(df)?.lazy())
            .map_err(map_err)?;
        Ok(PyBacktestResult { inner: result })
    }

    fn backtest_with_report(&self, df: &Bound<'_, PyAny>) -> PyResult<PyBacktestReport> {
        let report = self
            .inner
            .backtest_with_report(dataframe_from_py(df)?.lazy())
            .map_err(map_err)?;
        Ok(PyBacktestReport { inner: report })
    }

    fn run_metrics_only<'py>(
        &self,
        py: Python<'py>,
        df: &Bound<'_, PyAny>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let metrics = self
            .inner
            .run_metrics_only(dataframe_from_py(df)?.lazy())
            .map_err(map_err)?;
        metrics_to_dict(py, &metrics)
    }
}

#[pyclass(name = "BacktestResult")]
pub struct PyBacktestResult {
    inner: BacktestResult,
}

#[pymethods]
impl PyBacktestResult {
    #[getter]
    fn trades(&self) -> PyDataFrame {
        PyDataFrame(self.inner.trades.clone())
    }

    #[getter]
    fn equity_curve(&self) -> PyDataFrame {
        PyDataFrame(self.inner.equity_curve.clone())
    }

    fn stats<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        stats_to_dict(py, &self.inner.stats)
    }

    fn metrics<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metrics_to_dict(py, &self.inner.metrics())
    }

    /// Extended metrics (calmar_ratio, var_95, cvar_95, diagnostics, benchmark=None)
    /// — additive surface alongside `.metrics()` (quantwave-b5gr).
    fn extended_metrics<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        extended_metrics_to_dict(py, &self.inner.metrics())
    }

    /// Statistical-trustworthiness diagnostics for this run (quantwave-s3iu):
    /// thin-sample warnings and undefined (NaN) metrics.
    fn diagnostics<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        diagnostics_to_dict(py, &self.inner.metrics())
    }

    /// Extended metrics with benchmark-relative analytics (alpha/beta/cumulative
    /// return) computed against `benchmark_returns` (per-bar simple returns,
    /// aligned by index to the strategy's per-bar returns).
    fn metrics_with_benchmark<'py>(
        &self,
        py: Python<'py>,
        benchmark_returns: Vec<f64>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let metrics =
            PerformanceMetrics::from_result_with_benchmark(&self.inner, &benchmark_returns);
        extended_metrics_to_dict(py, &metrics)
    }
}

#[pyclass(name = "BacktestReport")]
pub struct PyBacktestReport {
    inner: BacktestReport,
}

#[pymethods]
impl PyBacktestReport {
    #[getter]
    fn result(&self) -> PyBacktestResult {
        // `BacktestReport` owns the result; clone for Python getter semantics.
        let trades = self.inner.result.trades.clone();
        let equity_curve = self.inner.result.equity_curve.clone();
        let stats = self.inner.result.stats.clone();
        PyBacktestResult {
            inner: BacktestResult {
                trades,
                equity_curve,
                stats,
            },
        }
    }

    fn metrics<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        metrics_to_dict(py, &self.inner.metrics)
    }

    /// Extended metrics (calmar_ratio, var_95, cvar_95, diagnostics, benchmark=None)
    /// — additive surface alongside `.metrics()` (quantwave-b5gr).
    fn extended_metrics<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        extended_metrics_to_dict(py, &self.inner.metrics)
    }

    /// Statistical-trustworthiness diagnostics for this run (quantwave-s3iu):
    /// thin-sample warnings and undefined (NaN) metrics.
    fn diagnostics<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        diagnostics_to_dict(py, &self.inner.metrics)
    }

    /// Extended metrics with benchmark-relative analytics (alpha/beta/cumulative
    /// return) computed against `benchmark_returns` (per-bar simple returns,
    /// aligned by index to the strategy's per-bar returns).
    fn metrics_with_benchmark<'py>(
        &self,
        py: Python<'py>,
        benchmark_returns: Vec<f64>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let metrics =
            PerformanceMetrics::from_result_with_benchmark(&self.inner.result, &benchmark_returns);
        extended_metrics_to_dict(py, &metrics)
    }

    /// Self-contained HTML tear sheet: equity, drawdown, metrics, trades, monthly
    /// returns heatmap, rolling Sharpe/vol, trade blotter, and run metadata
    /// (quantwave-b5gr). All new sections are additive; `benchmark_returns` is
    /// optional and adds a Benchmark-Relative section when supplied.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (title=None, seed=None, run_metadata=None, benchmark_returns=None, rolling_window=None))]
    fn to_html(
        &self,
        title: Option<String>,
        seed: Option<u64>,
        run_metadata: Option<Vec<(String, String)>>,
        benchmark_returns: Option<Vec<f64>>,
        rolling_window: Option<usize>,
    ) -> String {
        let opts = TearsheetOptions {
            title: title.unwrap_or_else(|| "QuantWave Backtest Report".to_string()),
            seed,
            run_metadata: run_metadata.unwrap_or_default(),
            rolling_window: rolling_window.unwrap_or(20),
            ..Default::default()
        };

        let metrics = match &benchmark_returns {
            Some(bench) => {
                PerformanceMetrics::from_result_with_benchmark(&self.inner.result, bench)
            }
            None => self.inner.metrics.clone(),
        };
        let report_view = BacktestReport {
            result: BacktestResult {
                trades: self.inner.result.trades.clone(),
                equity_curve: self.inner.result.equity_curve.clone(),
                stats: self.inner.result.stats.clone(),
            },
            metrics,
        };
        render_tearsheet_html(&report_view, &opts)
    }

    /// Write tear sheet HTML to `path`.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (path, title=None, seed=None, run_metadata=None, benchmark_returns=None, rolling_window=None))]
    fn save_html(
        &self,
        path: &str,
        title: Option<String>,
        seed: Option<u64>,
        run_metadata: Option<Vec<(String, String)>>,
        benchmark_returns: Option<Vec<f64>>,
        rolling_window: Option<usize>,
    ) -> PyResult<()> {
        let html = self.to_html(title, seed, run_metadata, benchmark_returns, rolling_window);
        std::fs::write(path, html)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
    }
}

fn mc_summary_to_dict<'py>(py: Python<'py>, s: &MonteCarloSummary) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("mean_final_equity", s.mean_final_equity)?;
    dict.set_item("p5_final_equity", s.p5_final_equity)?;
    dict.set_item("p50_final_equity", s.p50_final_equity)?;
    dict.set_item("p95_final_equity", s.p95_final_equity)?;
    dict.set_item("probability_of_loss", s.probability_of_loss)?;
    dict.set_item("n_simulations", s.n_simulations)?;
    dict.set_item("n_trades_sampled", s.n_trades_sampled)?;
    Ok(dict)
}

fn mc_path_summary_to_dict<'py>(
    py: Python<'py>,
    s: &MonteCarloPathSummary,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    dict.set_item("var_95", s.var_95)?;
    dict.set_item("cvar_95", s.cvar_95)?;
    dict.set_item("p5_terminal_equity", s.p5_terminal_equity)?;
    dict.set_item("p50_terminal_equity", s.p50_terminal_equity)?;
    dict.set_item("p95_terminal_equity", s.p95_terminal_equity)?;
    dict.set_item("probability_of_loss", s.probability_of_loss)?;
    Ok(dict)
}

#[pyfunction]
#[pyo3(signature = (result, initial_cash=100_000.0, n_simulations=1000, seed=42))]
fn monte_carlo_trade_bootstrap_py<'py>(
    py: Python<'py>,
    result: &PyBacktestResult,
    initial_cash: f64,
    n_simulations: usize,
    seed: u64,
) -> PyResult<Bound<'py, PyDict>> {
    let cfg = MonteCarloConfig {
        n_simulations,
        seed,
    };
    let summary =
        monte_carlo_trade_bootstrap(&result.inner, initial_cash, &cfg).map_err(map_err)?;
    mc_summary_to_dict(py, &summary)
}

#[pyfunction]
#[pyo3(signature = (result, n_simulations=1000, seed=42, n_bars_forward=252))]
fn monte_carlo_return_paths_py<'py>(
    py: Python<'py>,
    result: &PyBacktestResult,
    n_simulations: usize,
    seed: u64,
    n_bars_forward: usize,
) -> PyResult<Bound<'py, PyDict>> {
    let cfg = MonteCarloReturnConfig {
        n_simulations,
        seed,
        n_bars_forward,
    };
    let summary = monte_carlo_return_paths(&result.inner, &cfg).map_err(map_err)?;
    mc_path_summary_to_dict(py, &summary)
}

#[pyfunction]
#[pyo3(signature = (df, config, train_bars, test_bars, step_bars=None))]
fn run_walk_forward_py(
    df: &Bound<'_, PyAny>,
    config: PyBacktestConfig,
    train_bars: usize,
    test_bars: usize,
    step_bars: Option<usize>,
) -> PyResult<PyDataFrame> {
    let mut wf = WalkForwardConfig::new(train_bars, test_bars);
    wf.step_bars = step_bars;
    let out =
        run_walk_forward(dataframe_from_py(df)?.lazy(), &config.inner, &wf).map_err(map_err)?;
    Ok(PyDataFrame(out))
}

fn sweep_variants_from_py_list(variants: &Bound<'_, PyList>) -> PyResult<Vec<SweepVariant>> {
    let mut out = Vec::with_capacity(variants.len());
    for item in variants.iter() {
        let (params, signal_col): (std::collections::HashMap<String, f64>, String) =
            item.extract()?;
        out.push(SweepVariant { params, signal_col });
    }
    Ok(out)
}

#[pyfunction]
#[pyo3(signature = (
    df, config, train_bars, test_bars, variants, objective_metric="sharpe_ratio",
    step_bars=None, overfit_threshold=1.0, optimizer="grid", n_trials=None, seed=42
))]
#[allow(clippy::too_many_arguments)]
fn run_walk_forward_optimize_py(
    df: &Bound<'_, PyAny>,
    config: PyBacktestConfig,
    train_bars: usize,
    test_bars: usize,
    variants: &Bound<'_, PyList>,
    objective_metric: &str,
    step_bars: Option<usize>,
    overfit_threshold: f64,
    optimizer: &str,
    n_trials: Option<usize>,
    seed: u64,
) -> PyResult<PyDataFrame> {
    let mut wf = WalkForwardConfig::new(train_bars, test_bars);
    wf.step_bars = step_bars;
    wf.overfit_threshold = overfit_threshold;
    let sweep_variants = sweep_variants_from_py_list(variants)?;
    let in_fold_optimizer = parse_in_fold_optimizer(optimizer, n_trials, seed)?;
    let out = run_walk_forward_optimize_with(
        dataframe_from_py(df)?.lazy(),
        &config.inner,
        &wf,
        &sweep_variants,
        objective_metric,
        &in_fold_optimizer,
    )
    .map_err(map_err)?;
    Ok(PyDataFrame(out))
}

// ---------------------------------------------------------------------------
// Order-driven backtest (`.bt.order_backtest`, quantwave-bbhb).
//
// Additive surface: wraps `quantwave_backtest::run_order_simulation` (the
// order-execution core in `order_exec.rs`/`orders.rs`, which this file does
// not modify) for an explicit, long-format per-bar order spec instead of a
// signal column. Does not touch `PyBacktestConfig`/`PyBacktestEngine` or any
// existing pyfunction above.
// ---------------------------------------------------------------------------

/// Cast a column to `Float64` and pull it out as a plain `Vec<f64>` (nulls -> NaN).
fn extract_f64_col(df: &DataFrame, name: &str) -> PyResult<Vec<f64>> {
    let col = df
        .column(name)
        .map_err(|_| PyKeyError::new_err(name.to_string()))?;
    let casted = col
        .cast(&DataType::Float64)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let ca = casted
        .f64()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(ca.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect())
}

/// Parse a timestamp column: `Datetime` (ms since epoch) or `Int64`-castable
/// (treated as unix seconds, falling back to row index when null). Mirrors the
/// convention used by the existing `BacktestEngine` column parsing.
fn extract_timestamps_col(df: &DataFrame, name: &str) -> PyResult<Vec<DateTime<Utc>>> {
    let col = df
        .column(name)
        .map_err(|_| PyKeyError::new_err(name.to_string()))?;
    if let Ok(ca) = col.datetime() {
        return Ok(ca
            .into_iter()
            .map(|opt| {
                opt.map(|v| {
                    let secs = v / 1000;
                    let nanos = ((v % 1000) * 1_000_000) as u32;
                    DateTime::<Utc>::from_timestamp(secs, nanos).unwrap_or_else(Utc::now)
                })
                .unwrap_or_else(Utc::now)
            })
            .collect());
    }
    let casted = col
        .cast(&DataType::Int64)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let ca = casted
        .i64()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    Ok(ca
        .into_iter()
        .enumerate()
        .map(|(i, opt)| {
            let v = opt.unwrap_or(i as i64);
            DateTime::<Utc>::from_timestamp(v, 0).unwrap_or_else(Utc::now)
        })
        .collect())
}

fn parse_order_side(s: &str) -> PyResult<Side> {
    match s.to_ascii_lowercase().as_str() {
        "buy" | "long" => Ok(Side::Buy),
        "sell" | "short" => Ok(Side::Sell),
        other => Err(PyValueError::new_err(format!(
            "order side must be 'buy' or 'sell', got '{other}'"
        ))),
    }
}

/// Build one [`Order`] from a long-format orders-DataFrame row.
///
/// `price` is the limit level (for `limit`/`stop_limit`); `trigger` is the
/// breakout/stop level (for `stop`/`stop_limit`). `market` orders ignore both.
/// `take_profit`/`stop_loss`, when both present, attach a protective bracket to
/// the resulting position (an OCO exit pair); supplying only one is an error.
fn parse_order_row(
    side: &str,
    kind: &str,
    qty: f64,
    price: Option<f64>,
    trigger: Option<f64>,
    take_profit: Option<f64>,
    stop_loss: Option<f64>,
) -> PyResult<Order> {
    let side = parse_order_side(side)?;
    let order_type = match kind.to_ascii_lowercase().as_str() {
        "market" => OrderType::Market,
        "limit" => {
            let price =
                price.ok_or_else(|| PyValueError::new_err("limit order requires 'price'"))?;
            OrderType::Limit { price }
        }
        "stop" => {
            let trigger =
                trigger.ok_or_else(|| PyValueError::new_err("stop order requires 'trigger'"))?;
            OrderType::Stop { trigger }
        }
        "stop_limit" | "stoplimit" => {
            let trigger = trigger
                .ok_or_else(|| PyValueError::new_err("stop_limit order requires 'trigger'"))?;
            let limit = price.ok_or_else(|| {
                PyValueError::new_err("stop_limit order requires 'price' (the limit level)")
            })?;
            OrderType::StopLimit { trigger, limit }
        }
        other => {
            return Err(PyValueError::new_err(format!(
                "order type must be one of 'market'/'limit'/'stop'/'stop_limit', got '{other}'"
            )));
        }
    };
    match (take_profit, stop_loss) {
        (Some(tp), Some(sl)) => Ok(Order::with_bracket(side, order_type, qty, tp, sl)),
        (None, None) => Ok(Order::new(side, order_type, qty)),
        _ => Err(PyValueError::new_err(
            "a bracket requires both 'take_profit' and 'stop_loss' (got only one)",
        )),
    }
}

/// Group a long-format orders DataFrame (`bar_index, side, type, qty, price,
/// trigger`) into `Vec<Order>` per bar (length `n_bars`).
fn orders_by_bar_from_df(df: &DataFrame, n_bars: usize) -> PyResult<Vec<Vec<Order>>> {
    let bar_idx = df
        .column("bar_index")
        .map_err(|_| PyKeyError::new_err("bar_index"))?
        .cast(&DataType::Int64)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let bar_idx = bar_idx
        .i64()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let side_col = df
        .column("side")
        .map_err(|_| PyKeyError::new_err("side"))?
        .cast(&DataType::String)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let side_col = side_col
        .str()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let type_col = df
        .column("type")
        .map_err(|_| PyKeyError::new_err("type"))?
        .cast(&DataType::String)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let type_col = type_col
        .str()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let qty_col = df
        .column("qty")
        .map_err(|_| PyKeyError::new_err("qty"))?
        .cast(&DataType::Float64)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let qty_col = qty_col
        .f64()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let price_col = df
        .column("price")
        .map_err(|_| PyKeyError::new_err("price"))?
        .cast(&DataType::Float64)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let price_col = price_col
        .f64()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    let trigger_col = df
        .column("trigger")
        .map_err(|_| PyKeyError::new_err("trigger"))?
        .cast(&DataType::Float64)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
    let trigger_col = trigger_col
        .f64()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    // `take_profit`/`stop_loss` are optional bracket columns — a DataFrame
    // without them behaves exactly as before (every order is plain).
    let opt_f64_col = |name: &str| -> PyResult<Option<Column>> {
        match df.column(name) {
            Ok(c) => Ok(Some(
                c.cast(&DataType::Float64)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
            )),
            Err(_) => Ok(None),
        }
    };
    let tp_col = opt_f64_col("take_profit")?;
    let sl_col = opt_f64_col("stop_loss")?;

    let mut out: Vec<Vec<Order>> = vec![Vec::new(); n_bars];
    for i in 0..df.height() {
        let bar_i = bar_idx
            .get(i)
            .ok_or_else(|| PyValueError::new_err(format!("orders row {i}: bar_index is null")))?;
        if bar_i < 0 || bar_i as usize >= n_bars {
            return Err(PyValueError::new_err(format!(
                "orders row {i}: bar_index {bar_i} out of range [0, {n_bars})"
            )));
        }
        let side = side_col
            .get(i)
            .ok_or_else(|| PyValueError::new_err(format!("orders row {i}: side is null")))?;
        let kind = type_col
            .get(i)
            .ok_or_else(|| PyValueError::new_err(format!("orders row {i}: type is null")))?;
        let qty = qty_col
            .get(i)
            .ok_or_else(|| PyValueError::new_err(format!("orders row {i}: qty is null")))?;
        let price = price_col.get(i);
        let trigger = trigger_col.get(i);
        let take_profit = tp_col
            .as_ref()
            .and_then(|s| s.f64().ok().and_then(|ca| ca.get(i)));
        let stop_loss = sl_col
            .as_ref()
            .and_then(|s| s.f64().ok().and_then(|ca| ca.get(i)));
        let order = parse_order_row(side, kind, qty, price, trigger, take_profit, stop_loss)?;
        out[bar_i as usize].push(order);
    }
    Ok(out)
}

/// Trade blotter -> DataFrame, matching the column set of the existing
/// (single-symbol) `BacktestEngine::trades_to_df` output.
fn order_trades_to_df(trades: &[quantwave_backtest::Trade]) -> PyResult<DataFrame> {
    if trades.is_empty() {
        let cols = vec![
            Column::new("trade_id".into(), Vec::<u32>::new()),
            Column::new("side".into(), Vec::<i8>::new()),
            Column::new("entry_ts".into(), Vec::<i64>::new()),
            Column::new("entry_price".into(), Vec::<f64>::new()),
            Column::new("entry_fill_price".into(), Vec::<f64>::new()),
            Column::new("exit_ts".into(), Vec::<Option<i64>>::new()),
            Column::new("exit_price".into(), Vec::<Option<f64>>::new()),
            Column::new("exit_fill_price".into(), Vec::<Option<f64>>::new()),
            Column::new("quantity".into(), Vec::<f64>::new()),
            Column::new("pnl_net".into(), Vec::<f64>::new()),
        ];
        return DataFrame::new(cols).map_err(|e| PyValueError::new_err(e.to_string()));
    }

    let ids: Vec<u32> = trades.iter().map(|t| t.trade_id).collect();
    let sides: Vec<i8> = trades.iter().map(|t| t.side).collect();
    let entry_ts: Vec<i64> = trades.iter().map(|t| t.entry_ts.timestamp()).collect();
    let entry_px: Vec<f64> = trades.iter().map(|t| t.entry_price).collect();
    let entry_fill_px: Vec<f64> = trades.iter().map(|t| t.entry_fill_price).collect();
    let exit_ts: Vec<Option<i64>> = trades
        .iter()
        .map(|t| t.exit_ts.map(|d| d.timestamp()))
        .collect();
    let exit_px: Vec<Option<f64>> = trades.iter().map(|t| t.exit_price).collect();
    let exit_fill_px: Vec<Option<f64>> = trades.iter().map(|t| t.exit_fill_price).collect();
    let qty: Vec<f64> = trades.iter().map(|t| t.quantity).collect();
    let pnl: Vec<f64> = trades.iter().map(|t| t.pnl_net).collect();

    let cols = vec![
        Column::new("trade_id".into(), ids),
        Column::new("side".into(), sides),
        Column::new("entry_ts".into(), entry_ts),
        Column::new("entry_price".into(), entry_px),
        Column::new("entry_fill_price".into(), entry_fill_px),
        Column::new("exit_ts".into(), exit_ts),
        Column::new("exit_price".into(), exit_px),
        Column::new("exit_fill_price".into(), exit_fill_px),
        Column::new("quantity".into(), qty),
        Column::new("pnl_net".into(), pnl),
    ];
    DataFrame::new(cols).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Equity curve -> DataFrame, matching the column set of the existing
/// (single-symbol) `BacktestEngine::equity_to_df` output.
fn order_equity_to_df(points: &[quantwave_backtest::EquityPoint]) -> PyResult<DataFrame> {
    if points.is_empty() {
        let cols = vec![
            Column::new("ts".into(), Vec::<i64>::new()),
            Column::new("equity".into(), Vec::<f64>::new()),
            Column::new("cash".into(), Vec::<f64>::new()),
            Column::new("position".into(), Vec::<f64>::new()),
            Column::new("close".into(), Vec::<f64>::new()),
        ];
        return DataFrame::new(cols).map_err(|e| PyValueError::new_err(e.to_string()));
    }

    let ts: Vec<i64> = points.iter().map(|p| p.ts.timestamp()).collect();
    let eq: Vec<f64> = points.iter().map(|p| p.equity).collect();
    let cash: Vec<f64> = points.iter().map(|p| p.cash).collect();
    let pos: Vec<f64> = points.iter().map(|p| p.position).collect();
    let close: Vec<f64> = points.iter().map(|p| p.close).collect();

    let cols = vec![
        Column::new("ts".into(), ts),
        Column::new("equity".into(), eq),
        Column::new("cash".into(), cash),
        Column::new("position".into(), pos),
        Column::new("close".into(), close),
    ];
    DataFrame::new(cols).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// Order-driven backtest (quantwave-bbhb): runs `run_order_simulation` over an
/// OHLC frame given an explicit, long-format per-bar order spec (columns
/// `bar_index, side, type, qty, price, trigger` — `price`/`trigger` nullable).
/// `side` is `"buy"`/`"sell"`; `type` is one of `"market"`/`"limit"`/`"stop"`/
/// `"stop_limit"`. Returns `(trades_df, equity_df)` with the same column shape
/// as the existing signal-driven `.bt.backtest()` output.
#[pyfunction]
#[pyo3(signature = (
    df, orders, timestamp_col="timestamp", open_col="open", high_col="high",
    low_col="low", close_col="close", initial_cash=100_000.0, commission_bps=5.0,
    slippage_bps=2.0
))]
#[allow(clippy::too_many_arguments)]
fn order_backtest_py(
    df: &Bound<'_, PyAny>,
    orders: &Bound<'_, PyAny>,
    timestamp_col: &str,
    open_col: &str,
    high_col: &str,
    low_col: &str,
    close_col: &str,
    initial_cash: f64,
    commission_bps: f64,
    slippage_bps: f64,
) -> PyResult<(PyDataFrame, PyDataFrame)> {
    let bars_df = dataframe_from_py(df)?;
    let orders_df = dataframe_from_py(orders)?;

    let timestamps = extract_timestamps_col(&bars_df, timestamp_col)?;
    let opens = extract_f64_col(&bars_df, open_col)?;
    let highs = extract_f64_col(&bars_df, high_col)?;
    let lows = extract_f64_col(&bars_df, low_col)?;
    let closes = extract_f64_col(&bars_df, close_col)?;

    let n = closes.len();
    if opens.len() != n || highs.len() != n || lows.len() != n || timestamps.len() != n {
        return Err(PyValueError::new_err(
            "open/high/low/close/timestamp columns must have equal length",
        ));
    }

    let orders_by_bar = orders_by_bar_from_df(&orders_df, n)?;

    let exec = ExecutionModel::Simple(CostModel {
        commission_bps,
        slippage_bps,
        initial_cash,
    });

    let (trades, equity) = run_order_simulation(
        &timestamps,
        &opens,
        &highs,
        &lows,
        &closes,
        |i| orders_by_bar[i].clone(),
        &exec,
    );

    let trades_df = order_trades_to_df(&trades)?;
    let equity_df = order_equity_to_df(&equity)?;
    Ok((PyDataFrame(trades_df), PyDataFrame(equity_df)))
}

/// Native backtest engine (PyO3 + pyo3-polars).
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBacktestConfig>()?;
    m.add_class::<PyBacktestEngine>()?;
    m.add_class::<PyBacktestResult>()?;
    m.add_class::<PyBacktestReport>()?;
    m.add_function(wrap_pyfunction!(monte_carlo_trade_bootstrap_py, m)?)?;
    m.add_function(wrap_pyfunction!(monte_carlo_return_paths_py, m)?)?;
    m.add_function(wrap_pyfunction!(run_walk_forward_py, m)?)?;
    m.add_function(wrap_pyfunction!(run_walk_forward_optimize_py, m)?)?;
    m.add_function(wrap_pyfunction!(order_backtest_py, m)?)?;
    Ok(())
}
