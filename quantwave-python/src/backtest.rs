//! PyO3 bindings for `quantwave-backtest` (quantwave-cr6v.4).
//!
//! Exposes `BacktestConfig`, `BacktestEngine`, `BacktestResult`, `PerformanceMetrics`,
//! and `BacktestReport` to Python with zero-copy Polars DataFrame interop via `pyo3-polars`.

use polars::io::ipc::IpcReader;
use polars::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyType};
use pyo3_polars::PyDataFrame;
use std::io::Cursor;
use quantwave_backtest::{
    BacktestConfig, BacktestEngine, BacktestError, BacktestReport, BacktestResult, CostModel,
    ExecutionDelay, ExecutionModel, PerformanceMetrics, StopConfig,
};

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
    PyValueError::new_err(e.to_string())
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
    #[new]
    #[pyo3(signature = (
        signal_col = "signal",
        timestamp_col = "timestamp",
        close_col = "close",
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
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        signal_col: &str,
        timestamp_col: &str,
        close_col: &str,
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
                signal_col: signal_col.to_string(),
                entry_filter_col,
                size_multiplier_col,
                execution_delay: parse_execution_delay(execution_delay)?,
                stop_config: StopConfig {
                    stop_loss_pct,
                    take_profit_pct,
                    trailing_stop_pct,
                },
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
}

/// Native backtest engine (PyO3 + pyo3-polars).
#[pymodule]
fn _backtest(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyBacktestConfig>()?;
    m.add_class::<PyBacktestEngine>()?;
    m.add_class::<PyBacktestResult>()?;
    m.add_class::<PyBacktestReport>()?;
    Ok(())
}