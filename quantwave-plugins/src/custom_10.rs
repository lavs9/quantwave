use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use quantwave_core::traits::Next;
use quantwave_core::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct HarringtonAdxKwargs {
    pub adx_length: usize,
    pub adx_smooth_length: usize,
}

#[polars_expr(output_type=Float64)]
fn harrington_adx(inputs: &[Series], kwargs: HarringtonAdxKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut indicator =
        quantwave_core::HarringtonADXOscillator::new(kwargs.adx_length, kwargs.adx_smooth_length);
    let mut values = Vec::with_capacity(close.len());

    for i in 0..close.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next((h, l, c)));
    }

    Ok(Series::new(inputs[0].name().clone(), values))
}

#[derive(Deserialize)]
pub struct KalmanKwargs {
    pub q: f64,
    pub r: f64,
}

#[polars_expr(output_type=Float64)]
fn kalman(inputs: &[Series], kwargs: KalmanKwargs) -> PolarsResult<Series> {
    let ca = inputs[0].f64()?;
    let mut indicator = quantwave_core::indicators::kalman::KalmanFilter::new(kwargs.q, kwargs.r);
    let mut values = Vec::with_capacity(ca.len());

    for i in 0..ca.len() {
        let val = ca.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next(val));
    }

    Ok(Series::new(inputs[0].name().clone(), values))
}
