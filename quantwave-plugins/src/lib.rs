use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use quantwave_core::indicators::smoothing::SMA;
use quantwave_core::traits::Next;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SmaKwargs {
    pub period: usize,
}

#[polars_expr(output_type=Float64)]
pub fn sma_plugin(inputs: &[Series], kwargs: SmaKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.f64()?;
    let mut indicator = SMA::new(kwargs.period);

    let out: Float64Chunked = ca
        .into_iter()
        .map(|opt_v| opt_v.map(|v| indicator.next(v)))
        .collect();

    Ok(out.into_series())
}
