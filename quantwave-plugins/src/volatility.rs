use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::indicators::incremental::ta_atr::TaATR;
use quantwave_core::indicators::incremental::trange::{TaTRANGE, TaNATR};
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct SinglePeriodKwargs {
    timeperiod: usize,
}

#[polars_expr(output_type=Float64)]
fn atr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = TaATR::new(kwargs.timeperiod);
    
    let out: Float64Chunked = high.into_iter().zip(low.into_iter()).zip(close.into_iter()).map(|((h, l), c)| {
        match (h, l, c) {
            (Some(hv), Some(lv), Some(cv)) if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() => Some(indicator.next((hv, lv, cv))),
            (Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }
    }).collect();
    
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn natr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = TaNATR::new(kwargs.timeperiod);
    
    let out: Float64Chunked = high.into_iter().zip(low.into_iter()).zip(close.into_iter()).map(|((h, l), c)| {
        match (h, l, c) {
            (Some(hv), Some(lv), Some(cv)) if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() => Some(indicator.next((hv, lv, cv))),
            (Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }
    }).collect();
    
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn trange(inputs: &[Series]) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = TaTRANGE::new();
    
    let out: Float64Chunked = high.into_iter().zip(low.into_iter()).zip(close.into_iter()).map(|((h, l), c)| {
        match (h, l, c) {
            (Some(hv), Some(lv), Some(cv)) if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() => Some(indicator.next((hv, lv, cv))),
            (Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }
    }).collect();
    
    Ok(out.into_series())
}
