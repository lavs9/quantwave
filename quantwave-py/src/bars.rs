//! PyO3 bindings for alternative bar construction (Renko first slice).
//!
//! Bar transforms change row count (price series → fewer bricks), so they do not
//! fit the 1:1 `.ta` expression namespace. They are exposed as free functions
//! that return a fresh Polars DataFrame of bricks.

use polars::prelude::*;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use quantwave_core::{
    RangeBar, RenkoBrick, range_bars_atr_batch, range_bars_batch, renko_atr_batch, renko_batch,
};

fn bricks_to_frame(bricks: &[RenkoBrick]) -> PolarsResult<DataFrame> {
    let open: Vec<f64> = bricks.iter().map(|b| b.open).collect();
    let close: Vec<f64> = bricks.iter().map(|b| b.close).collect();
    let direction: Vec<i8> = bricks.iter().map(|b| b.direction).collect();
    df![
        "open" => open,
        "close" => close,
        "direction" => direction,
    ]
}

fn range_bars_to_frame(bars: &[RangeBar]) -> PolarsResult<DataFrame> {
    df![
        "open" => bars.iter().map(|b| b.open).collect::<Vec<f64>>(),
        "high" => bars.iter().map(|b| b.high).collect::<Vec<f64>>(),
        "low" => bars.iter().map(|b| b.low).collect::<Vec<f64>>(),
        "close" => bars.iter().map(|b| b.close).collect::<Vec<f64>>(),
    ]
}

/// Fixed-box Renko: `prices` → DataFrame with columns open/close/direction (one
/// row per brick).
#[pyfunction]
#[pyo3(signature = (prices, box_size))]
fn renko(prices: Vec<f64>, box_size: f64) -> PyResult<PyDataFrame> {
    if !(box_size > 0.0) {
        return Err(PyValueError::new_err("box_size must be > 0"));
    }
    let df = bricks_to_frame(&renko_batch(&prices, box_size))
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(PyDataFrame(df))
}

/// ATR-box Renko: box size = `multiplier * atr` (a single representative ATR).
#[pyfunction]
#[pyo3(signature = (prices, atr, multiplier=1.0))]
fn renko_atr(prices: Vec<f64>, atr: f64, multiplier: f64) -> PyResult<PyDataFrame> {
    if !(atr > 0.0) || !(multiplier > 0.0) {
        return Err(PyValueError::new_err("atr and multiplier must be > 0"));
    }
    let df = bricks_to_frame(&renko_atr_batch(&prices, atr, multiplier))
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(PyDataFrame(df))
}

/// Constant-range bars: `prices` → DataFrame with columns open/high/low/close.
#[pyfunction]
#[pyo3(signature = (prices, range_size))]
fn range_bars(prices: Vec<f64>, range_size: f64) -> PyResult<PyDataFrame> {
    if !(range_size > 0.0) {
        return Err(PyValueError::new_err("range_size must be > 0"));
    }
    let df = range_bars_to_frame(&range_bars_batch(&prices, range_size))
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(PyDataFrame(df))
}

/// ATR-range bars: range size = `multiplier * atr`.
#[pyfunction]
#[pyo3(signature = (prices, atr, multiplier=1.0))]
fn range_bars_atr(prices: Vec<f64>, atr: f64, multiplier: f64) -> PyResult<PyDataFrame> {
    if !(atr > 0.0) || !(multiplier > 0.0) {
        return Err(PyValueError::new_err("atr and multiplier must be > 0"));
    }
    let df = range_bars_to_frame(&range_bars_atr_batch(&prices, atr, multiplier))
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
    Ok(PyDataFrame(df))
}

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(renko, m)?)?;
    m.add_function(wrap_pyfunction!(renko_atr, m)?)?;
    m.add_function(wrap_pyfunction!(range_bars, m)?)?;
    m.add_function(wrap_pyfunction!(range_bars_atr, m)?)?;
    Ok(())
}
