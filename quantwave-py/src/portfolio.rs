//! PyO3 bindings for `quantwave_core::portfolio`.
//!
//! Rust/nalgebra port (quantwave-a465) of the numpy v1 shipped at
//! `quantwave/portfolio.py` (p2k0.5, PR #21). Exposed as `quantwave._portfolio`,
//! following the same "native submodule alongside the pure-Python surface"
//! pattern used by `quantwave._backtest` / `quantwave._patterns`. The
//! existing pure-Python `quantwave.portfolio` module is left untouched and
//! remains the default import path; this native module is an additional,
//! opt-in fast path with an identical numerical contract (see the parity
//! tests in `quantwave-core/src/portfolio/mod.rs`).
//!
//! Matrices cross the FFI boundary as `Vec<Vec<f64>>` (row-major, one inner
//! `Vec` per row) rather than via a `numpy`-crate zero-copy buffer, matching
//! the plain `Vec<f64>` convention already used by `quantwave-py`'s
//! indicator bindings (PyO3 extracts `Vec<f64>` from any Python sequence,
//! including numpy arrays, via iteration).

use nalgebra::{DMatrix, DVector};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use quantwave_core::portfolio::{self, PortfolioError};

fn map_err(e: PortfolioError) -> PyErr {
    PyValueError::new_err(e.to_string())
}

fn matrix_from_rows(rows: &[Vec<f64>]) -> PyResult<DMatrix<f64>> {
    if rows.is_empty() {
        return Err(PyValueError::new_err("matrix is empty"));
    }
    let ncols = rows[0].len();
    if ncols == 0 || rows.iter().any(|r| r.len() != ncols) {
        return Err(PyValueError::new_err(
            "matrix rows must be non-empty and equal length",
        ));
    }
    let nrows = rows.len();
    let mut m = DMatrix::<f64>::zeros(nrows, ncols);
    for (i, row) in rows.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            m[(i, j)] = v;
        }
    }
    Ok(m)
}

fn matrix_to_rows(m: &DMatrix<f64>) -> Vec<Vec<f64>> {
    (0..m.nrows())
        .map(|i| (0..m.ncols()).map(|j| m[(i, j)]).collect())
        .collect()
}

fn vector_to_vec(v: &DVector<f64>) -> Vec<f64> {
    v.iter().copied().collect()
}

/// Sample covariance matrix (`np.cov(returns, rowvar=False, ddof=1)` equivalent).
#[pyfunction]
fn sample_cov(returns: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
    let r = matrix_from_rows(&returns)?;
    let cov = portfolio::sample_cov(&r).map_err(map_err)?;
    Ok(matrix_to_rows(&cov))
}

/// Exponentially-weighted covariance matrix (RiskMetrics-style).
#[pyfunction]
fn ewma_cov(returns: Vec<Vec<f64>>, halflife: f64) -> PyResult<Vec<Vec<f64>>> {
    let r = matrix_from_rows(&returns)?;
    let cov = portfolio::ewma_cov(&r, halflife).map_err(map_err)?;
    Ok(matrix_to_rows(&cov))
}

/// Ledoit-Wolf shrinkage covariance estimator (identity-scaled target).
#[pyfunction]
fn ledoit_wolf_cov(returns: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
    let r = matrix_from_rows(&returns)?;
    let cov = portfolio::ledoit_wolf_cov(&r).map_err(map_err)?;
    Ok(matrix_to_rows(&cov))
}

/// Minimum-variance portfolio weights: `w ∝ Σ⁻¹·1`, projected onto bounds+budget.
#[pyfunction]
#[pyo3(signature = (cov, bounds=None))]
fn min_variance(cov: Vec<Vec<f64>>, bounds: Option<Vec<(f64, f64)>>) -> PyResult<Vec<f64>> {
    let c = matrix_from_rows(&cov)?;
    let w = portfolio::min_variance(&c, bounds.as_deref()).map_err(map_err)?;
    Ok(vector_to_vec(&w))
}

/// Maximum-Sharpe (tangency) portfolio weights: `w ∝ Σ⁻¹·μ`, projected onto bounds+budget.
#[pyfunction]
#[pyo3(signature = (mean, cov, bounds=None))]
fn max_sharpe(
    mean: Vec<f64>,
    cov: Vec<Vec<f64>>,
    bounds: Option<Vec<(f64, f64)>>,
) -> PyResult<Vec<f64>> {
    let c = matrix_from_rows(&cov)?;
    let m = DVector::from_vec(mean);
    let w = portfolio::max_sharpe(&m, &c, bounds.as_deref()).map_err(map_err)?;
    Ok(vector_to_vec(&w))
}

/// Equal risk-contribution ("risk parity") portfolio weights (Spinu fixed-point).
#[pyfunction]
#[pyo3(signature = (cov, bounds=None, max_iter=200, tol=1e-10))]
fn risk_parity(
    cov: Vec<Vec<f64>>,
    bounds: Option<Vec<(f64, f64)>>,
    max_iter: usize,
    tol: f64,
) -> PyResult<Vec<f64>> {
    let c = matrix_from_rows(&cov)?;
    let w = portfolio::risk_parity(&c, bounds.as_deref(), max_iter, tol).map_err(map_err)?;
    Ok(vector_to_vec(&w))
}

/// Hierarchical Risk Parity (Lopez de Prado) portfolio weights.
///
/// `returns_or_cov` is either a `(T, N)` returns matrix (`is_cov=False`) or
/// a precomputed `(N, N)` covariance matrix (`is_cov=True`).
#[pyfunction]
#[pyo3(signature = (returns_or_cov, bounds=None, is_cov=false))]
fn hrp(
    returns_or_cov: Vec<Vec<f64>>,
    bounds: Option<Vec<(f64, f64)>>,
    is_cov: bool,
) -> PyResult<Vec<f64>> {
    let m = matrix_from_rows(&returns_or_cov)?;
    let w = portfolio::hrp(&m, bounds.as_deref(), is_cov).map_err(map_err)?;
    Ok(vector_to_vec(&w))
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sample_cov, m)?)?;
    m.add_function(wrap_pyfunction!(ewma_cov, m)?)?;
    m.add_function(wrap_pyfunction!(ledoit_wolf_cov, m)?)?;
    m.add_function(wrap_pyfunction!(min_variance, m)?)?;
    m.add_function(wrap_pyfunction!(max_sharpe, m)?)?;
    m.add_function(wrap_pyfunction!(risk_parity, m)?)?;
    m.add_function(wrap_pyfunction!(hrp, m)?)?;
    Ok(())
}
