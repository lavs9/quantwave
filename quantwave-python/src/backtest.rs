//! PyO3 bindings for `quantwave-backtest` (quantwave-cr6v.4).
//!
//! Exposes `BacktestConfig`, `BacktestEngine`, `BacktestResult`, `PerformanceMetrics`,
//! and `BacktestReport` to Python with zero-copy Polars DataFrame interop via `pyo3-polars`.

use polars::io::ipc::IpcReader;
use polars::prelude::*;
use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyType};
use pyo3_polars::PyDataFrame;
use std::io::Cursor;
use quantwave_backtest::{
    monte_carlo_return_paths, monte_carlo_trade_bootstrap, render_tearsheet_html,
    run_walk_forward, run_walk_forward_optimize, BacktestConfig, BacktestEngine, BacktestError,
    BacktestReport, BacktestResult, CostModel, ExecutionDelay, ExecutionModel, MonteCarloConfig,
    MonteCarloPathSummary, MonteCarloReturnConfig, MonteCarloSummary, PerformanceMetrics,
    PortfolioAllocator, PortfolioMode, StopConfig, StopEvaluationMode, SweepVariant,
    TearsheetOptions, WalkForwardConfig,
};

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

fn parse_execution_delay(s: &str) -> PyResult<ExecutionDelay> {
    match s.to_ascii_lowercase().as_str() {
        "same_bar" | "samebar" | "t0" => Ok(ExecutionDelay::SameBar),
        "next_bar" | "nextbar" | "t1" => Ok(ExecutionDelay::NextBar),
        other => Err(PyValueError::new_err(format!(
            "execution_delay must be 'same_bar' or 'next_bar', got '{other}'"
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
    let bytes = buf.call_method0("getvalue")?.extract::<Bound<'_, PyBytes>>()?;
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
    /// Execution delay can be 'same_bar' (fill on signal bar) or 'next_bar' (fill on next bar).
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
        execution_delay = "same_bar",
        stop_loss_pct = None,
        take_profit_pct = None,
        trailing_stop_pct = None,
        touched_exit = false,
        portfolio_mode = "independent_books",
        portfolio_allocator = "equal_weight",
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
    ) -> PyResult<Self> {
        let costs = CostModel {
            commission_bps,
            slippage_bps,
            initial_cash,
        };
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

    fn run_metrics_only<'py>(&self, py: Python<'py>, df: &Bound<'_, PyAny>) -> PyResult<Bound<'py, PyDict>> {
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

    /// Self-contained HTML tear sheet (equity, drawdown, metrics, trades).
    #[pyo3(signature = (title=None))]
    fn to_html(&self, title: Option<String>) -> String {
        let opts = TearsheetOptions {
            title: title.unwrap_or_else(|| "QuantWave Backtest Report".to_string()),
            ..Default::default()
        };
        render_tearsheet_html(&self.inner, &opts)
    }

    /// Write tear sheet HTML to `path`.
    #[pyo3(signature = (path, title=None))]
    fn save_html(&self, path: &str, title: Option<String>) -> PyResult<()> {
        std::fs::write(path, self.to_html(title))
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
    let summary = monte_carlo_trade_bootstrap(&result.inner, initial_cash, &cfg).map_err(map_err)?;
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
    let out = run_walk_forward(
        dataframe_from_py(df)?.lazy(),
        &config.inner,
        &wf,
    )
    .map_err(map_err)?;
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
#[pyo3(signature = (df, config, train_bars, test_bars, variants, objective_metric="sharpe_ratio", step_bars=None, overfit_threshold=1.0))]
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
) -> PyResult<PyDataFrame> {
    let mut wf = WalkForwardConfig::new(train_bars, test_bars);
    wf.step_bars = step_bars;
    wf.overfit_threshold = overfit_threshold;
    let sweep_variants = sweep_variants_from_py_list(variants)?;
    let out = run_walk_forward_optimize(
        dataframe_from_py(df)?.lazy(),
        &config.inner,
        &wf,
        &sweep_variants,
        objective_metric,
    )
    .map_err(map_err)?;
    Ok(PyDataFrame(out))
}

/// Native backtest engine (PyO3 + pyo3-polars).
#[pymodule]
fn _backtest(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBacktestConfig>()?;
    m.add_class::<PyBacktestEngine>()?;
    m.add_class::<PyBacktestResult>()?;
    m.add_class::<PyBacktestReport>()?;
    m.add_function(wrap_pyfunction!(monte_carlo_trade_bootstrap_py, m)?)?;
    m.add_function(wrap_pyfunction!(monte_carlo_return_paths_py, m)?)?;
    m.add_function(wrap_pyfunction!(run_walk_forward_py, m)?)?;
    m.add_function(wrap_pyfunction!(run_walk_forward_optimize_py, m)?)?;
    Ok(())
}