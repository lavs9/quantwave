use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::indicators::incremental::volume_ta::{AD, ADOSC};
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct AdoscKwargs {
    fastperiod: usize,
    slowperiod: usize,
}

#[polars_expr(output_type=Float64)]
fn ad(inputs: &[Series]) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let volume = inputs[3].f64()?;

    let mut indicator = AD::new();

    let out: Float64Chunked = high
        .into_iter()
        .zip(low.into_iter())
        .zip(close.into_iter())
        .zip(volume.into_iter())
        .map(|(((h, l), c), v)| match (h, l, c, v) {
            (Some(hv), Some(lv), Some(cv), Some(vv))
                if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() && !vv.is_nan() =>
            {
                Some(indicator.next((hv, lv, cv, vv)))
            }
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn adosc(inputs: &[Series], kwargs: AdoscKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let volume = inputs[3].f64()?;

    let mut indicator = ADOSC::new(kwargs.fastperiod, kwargs.slowperiod);

    let out: Float64Chunked = high
        .into_iter()
        .zip(low.into_iter())
        .zip(close.into_iter())
        .zip(volume.into_iter())
        .map(|(((h, l), c), v)| match (h, l, c, v) {
            (Some(hv), Some(lv), Some(cv), Some(vv))
                if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() && !vv.is_nan() =>
            {
                Some(indicator.next((hv, lv, cv, vv)))
            }
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();

    Ok(out.into_series())
}

// OBV is missing from volume_ta.rs in core or maybe it is there but I missed it in grep.
// Let's hold off on OBV until I double check if it's there.
