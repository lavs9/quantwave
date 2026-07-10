use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::indicators::incremental::statistics_ta::{
    TaBETA, TaCORREL, TaLINEARREG, TaLINEARREG_ANGLE, TaLINEARREG_INTERCEPT, TaLINEARREG_SLOPE,
    TaSTDDEV, TaTSF, TaVAR,
};
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct SinglePeriodKwargs {
    timeperiod: usize,
}

#[derive(Deserialize)]
struct DevKwargs {
    timeperiod: usize,
    nbdev: f64,
}

#[polars_expr(output_type=Float64)]
fn stddev(inputs: &[Series], kwargs: DevKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TaSTDDEV::new(kwargs.timeperiod, kwargs.nbdev);

    let out: Float64Chunked = s
        .into_iter()
        .map(|opt_v| match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn var(inputs: &[Series], kwargs: DevKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TaVAR::new(kwargs.timeperiod, kwargs.nbdev);

    let out: Float64Chunked = s
        .into_iter()
        .map(|opt_v| match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn linearreg(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TaLINEARREG::new(kwargs.timeperiod);

    let out: Float64Chunked = s
        .into_iter()
        .map(|opt_v| match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn linearreg_slope(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TaLINEARREG_SLOPE::new(kwargs.timeperiod);

    let out: Float64Chunked = s
        .into_iter()
        .map(|opt_v| match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn linearreg_intercept(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TaLINEARREG_INTERCEPT::new(kwargs.timeperiod);

    let out: Float64Chunked = s
        .into_iter()
        .map(|opt_v| match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn linearreg_angle(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TaLINEARREG_ANGLE::new(kwargs.timeperiod);

    let out: Float64Chunked = s
        .into_iter()
        .map(|opt_v| match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn tsf(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TaTSF::new(kwargs.timeperiod);

    let out: Float64Chunked = s
        .into_iter()
        .map(|opt_v| match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn correl(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s0 = inputs[0].f64()?;
    let s1 = inputs[1].f64()?;
    let mut indicator = TaCORREL::new(kwargs.timeperiod);

    let out: Float64Chunked = s0
        .into_iter()
        .zip(s1.into_iter())
        .map(|(v0, v1)| match (v0, v1) {
            (Some(x), Some(y)) if !x.is_nan() && !y.is_nan() => Some(indicator.next((x, y))),
            (Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn beta(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s0 = inputs[0].f64()?;
    let s1 = inputs[1].f64()?;
    let mut indicator = TaBETA::new(kwargs.timeperiod);

    let out: Float64Chunked = s0
        .into_iter()
        .zip(s1.into_iter())
        .map(|(v0, v1)| match (v0, v1) {
            (Some(x), Some(y)) if !x.is_nan() && !y.is_nan() => Some(indicator.next((x, y))),
            (Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();

    Ok(out.into_series())
}
