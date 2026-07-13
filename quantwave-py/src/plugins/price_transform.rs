use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

use quantwave_core::indicators::incremental::price_transform::{
    AVGPRICE, MEDPRICE, TYPPRICE, WCLPRICE,
};
use quantwave_core::traits::Next;

#[polars_expr(output_type=Float64)]
fn avgprice(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;

    let mut indicator = AVGPRICE::new();

    let out: Float64Chunked = open
        .into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(ov), Some(hv), Some(lv), Some(cv))
                if !ov.is_nan() && !hv.is_nan() && !lv.is_nan() && !cv.is_nan() =>
            {
                Some(indicator.next((ov, hv, lv, cv)))
            }
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn medprice(inputs: &[Series]) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;

    let mut indicator = MEDPRICE::new();

    let out: Float64Chunked = high
        .into_iter()
        .zip(low.into_iter())
        .map(|(h, l)| match (h, l) {
            (Some(hv), Some(lv)) if !hv.is_nan() && !lv.is_nan() => Some(indicator.next((hv, lv))),
            (Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn typprice(inputs: &[Series]) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut indicator = TYPPRICE::new();

    let out: Float64Chunked = high
        .into_iter()
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|((h, l), c)| match (h, l, c) {
            (Some(hv), Some(lv), Some(cv)) if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() => {
                Some(indicator.next((hv, lv, cv)))
            }
            (Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn wclprice(inputs: &[Series]) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut indicator = WCLPRICE::new();

    let out: Float64Chunked = high
        .into_iter()
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|((h, l), c)| match (h, l, c) {
            (Some(hv), Some(lv), Some(cv)) if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() => {
                Some(indicator.next((hv, lv, cv)))
            }
            (Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();

    Ok(out.into_series())
}
