use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::indicators::incremental::ta_atr::TaATR;
use quantwave_core::indicators::incremental::trange::{TaNATR, TaTRANGE};
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct SinglePeriodKwargs {
    timeperiod: usize,
}

#[derive(Deserialize)]
struct AtrKwargs {
    timeperiod: usize,
    smoothing: Option<String>,
}

#[polars_expr(output_type=Float64)]
fn atr(inputs: &[Series], kwargs: AtrKwargs) -> PolarsResult<Series> {
    let close = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let is_ema = kwargs
        .smoothing
        .as_deref()
        .map(|s| s.eq_ignore_ascii_case("ema"))
        .unwrap_or(false);

    let out: Float64Chunked = if is_ema {
        let mut indicator = quantwave_core::indicators::volatility::ATR::new(kwargs.timeperiod);
        close
            .into_iter()
            .zip(high.into_iter())
            .zip(low.into_iter())
            .map(|((c, h), l)| match (c, h, l) {
                (Some(cv), Some(hv), Some(lv)) => Some(indicator.next((hv, lv, cv))),
                _ => {
                    indicator.next((f64::NAN, f64::NAN, f64::NAN));
                    None
                }
            })
            .collect()
    } else {
        let mut indicator = TaATR::new(kwargs.timeperiod);
        close
            .into_iter()
            .zip(high.into_iter())
            .zip(low.into_iter())
            .map(|((c, h), l)| match (c, h, l) {
                (Some(cv), Some(hv), Some(lv)) => Some(indicator.next((hv, lv, cv))),
                _ => {
                    indicator.next((f64::NAN, f64::NAN, f64::NAN));
                    None
                }
            })
            .collect()
    };

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn natr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let close = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let mut indicator = TaNATR::new(kwargs.timeperiod);

    let out: Float64Chunked = close
        .into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .map(|((c, h), l)| match (c, h, l) {
            (Some(cv), Some(hv), Some(lv)) => Some(indicator.next((hv, lv, cv))),
            _ => {
                indicator.next((f64::NAN, f64::NAN, f64::NAN));
                None
            }
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn trange(inputs: &[Series]) -> PolarsResult<Series> {
    let close = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let mut indicator = TaTRANGE::new();

    let out: Float64Chunked = close
        .into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .map(|((c, h), l)| match (c, h, l) {
            (Some(cv), Some(hv), Some(lv)) => Some(indicator.next((hv, lv, cv))),
            _ => {
                indicator.next((f64::NAN, f64::NAN, f64::NAN));
                None
            }
        })
        .collect();

    Ok(out.into_series())
}
