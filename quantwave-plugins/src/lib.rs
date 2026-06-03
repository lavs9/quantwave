#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use pyo3::prelude::*;

#[polars_expr(output_type=Float64)]
fn dummy_multiply(inputs: &[Series]) -> PolarsResult<Series> {
    let s = &inputs[0];
    let s_f64 = s.f64()?;
    
    // Simple Arrow element-wise multiply by 2
    let out: Float64Chunked = s_f64.apply_values(|v| v * 2.0);
    Ok(out.into_series())
}

#[pymodule]
#[pyo3(name = "quantwave_plugins")]
fn quantwave_plugins(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
