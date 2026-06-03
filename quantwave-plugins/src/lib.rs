#![allow(clippy::unused_unit)]
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use pyo3::prelude::*;
use serde::Deserialize;
use quantwave_core::indicators::smoothing::SMA;
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct SmaKwargs {
    period: usize,
}

#[polars_expr(output_type=Float64)]
fn sma(inputs: &[Series], kwargs: SmaKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let s_f64 = s.f64()?;
    
    let mut indicator = SMA::new(kwargs.period);
    
    let out: Float64Chunked = s_f64.into_iter().map(|opt_v| {
        match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        }
    }).collect();
    
    Ok(out.into_series())
}

#[pymodule]
#[pyo3(name = "quantwave_plugins")]
fn quantwave_plugins(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
