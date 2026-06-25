use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::traits::Next;

#[derive(Deserialize)]
pub struct SinglePeriodKwargs {
    pub timeperiod: usize,
}

#[polars_expr(output_type=Float64)]
fn acos(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::ACOS::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn asin(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::ASIN::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn atan(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::ATAN::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ceil(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::CEIL::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cos(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::COS::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cosh(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::COSH::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn exp(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::EXP::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn floor(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::FLOOR::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ln(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::LN::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn log10(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::LOG10::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn sin(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::SIN::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn sinh(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::SINH::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn sqrt(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::SQRT::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn tan(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::TAN::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn tanh(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::TANH::new();
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn max(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::MAX::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn maxindex(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::MAXINDEX::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn min(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::MIN::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn minindex(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::MININDEX::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn sum(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::SUM::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_linearreg(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::TaLINEARREG::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_linearreg_angle(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::TaLINEARREG_ANGLE::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_linearreg_intercept(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::TaLINEARREG_INTERCEPT::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_linearreg_slope(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::TaLINEARREG_SLOPE::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_tsf(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::TaTSF::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn wma(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::WMA::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn add(inputs: &[Series]) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::ADD::new();
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn div(inputs: &[Series]) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::DIV::new();
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn mult(inputs: &[Series]) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::MULT::new();
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn obv(inputs: &[Series]) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::OBV::new();
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn sub(inputs: &[Series]) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::SUB::new();
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_beta(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::TaBETA::new(kwargs.timeperiod);
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_correl(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let mut indicator = quantwave_core::TaCORREL::new(kwargs.timeperiod);
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).map(|(v1, v2)| match (v1, v2) {
        (Some(a), Some(b)) if !a.is_nan() && !b.is_nan() => Some(indicator.next((a, b))),
        (Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_trange(inputs: &[Series]) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let in3 = inputs[2].f64()?;
    let mut indicator = quantwave_core::TaTRANGE::new();
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).zip(in3.into_iter()).map(|((v1, v2), v3)| match (v1, v2, v3) {
        (Some(a), Some(b), Some(c)) if !a.is_nan() && !b.is_nan() && !c.is_nan() => Some(indicator.next((a, b, c))),
        (Some(_), Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_atr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let in3 = inputs[2].f64()?;
    let mut indicator = quantwave_core::TaATR::new(kwargs.timeperiod);
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).zip(in3.into_iter()).map(|((v1, v2), v3)| match (v1, v2, v3) {
        (Some(a), Some(b), Some(c)) if !a.is_nan() && !b.is_nan() && !c.is_nan() => Some(indicator.next((a, b, c))),
        (Some(_), Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ta_natr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let in1 = inputs[0].f64()?;
    let in2 = inputs[1].f64()?;
    let in3 = inputs[2].f64()?;
    let mut indicator = quantwave_core::TaNATR::new(kwargs.timeperiod);
    let out: Float64Chunked = in1.into_iter().zip(in2.into_iter()).zip(in3.into_iter()).map(|((v1, v2), v3)| match (v1, v2, v3) {
        (Some(a), Some(b), Some(c)) if !a.is_nan() && !b.is_nan() && !c.is_nan() => Some(indicator.next((a, b, c))),
        (Some(_), Some(_), Some(_)) => Some(f64::NAN),
        _ => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn bop(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::BOP::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_2crows(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDL2CROWS::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_3blackcrows(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDL3BLACKCROWS::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_3inside(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDL3INSIDE::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_3linestrike(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDL3LINESTRIKE::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_3outside(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDL3OUTSIDE::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_3starsinsouth(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDL3STARSINSOUTH::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_3whitesoldiers(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDL3WHITESOLDIERS::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_abandonedbaby(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLABANDONEDBABY::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_advanceblock(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLADVANCEBLOCK::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_belthold(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLBELTHOLD::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_breakaway(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLBREAKAWAY::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_closingmarubozu(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLCLOSINGMARUBOZU::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_concealbabyswall(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLCONCEALBABYSWALL::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_counterattack(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLCOUNTERATTACK::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_darkcloudcover(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLDARKCLOUDCOVER::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_doji(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLDOJI::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_dojistar(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLDOJISTAR::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_dragonflydoji(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLDRAGONFLYDOJI::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_engulfing(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLENGULFING::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_eveningdojistar(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLEVENINGDOJISTAR::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_eveningstar(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLEVENINGSTAR::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_gapsidesidewhite(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLGAPSIDESIDEWHITE::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_gravestonedoji(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLGRAVESTONEDOJI::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_hammer(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLHAMMER::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_hangingman(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLHANGINGMAN::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_harami(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLHARAMI::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_haramicross(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLHARAMICROSS::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_highwave(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLHIGHWAVE::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_hikkake(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLHIKKAKE::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_hikkakemod(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLHIKKAKEMOD::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_homingpigeon(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLHOMINGPIGEON::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_identical3crows(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLIDENTICAL3CROWS::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_inneck(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLINNECK::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_invertedhammer(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLINVERTEDHAMMER::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_kicking(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLKICKING::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_kickingbylength(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLKICKINGBYLENGTH::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_ladderbottom(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLLADDERBOTTOM::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_longleggeddoji(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLLONGLEGGEDDOJI::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_longline(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLLONGLINE::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_marubozu(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLMARUBOZU::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_matchinglow(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLMATCHINGLOW::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_mathold(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLMATHOLD::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_morningdojistar(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLMORNINGDOJISTAR::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_morningstar(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLMORNINGSTAR::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_onneck(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLONNECK::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_piercing(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLPIERCING::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_rickshawman(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLRICKSHAWMAN::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_risefall3methods(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLRISEFALL3METHODS::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_separatinglines(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLSEPARATINGLINES::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_shootingstar(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLSHOOTINGSTAR::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_shortline(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLSHORTLINE::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_spinningtop(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLSPINNINGTOP::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_stalledpattern(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLSTALLEDPATTERN::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_sticksandwich(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLSTICKSANDWICH::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_takuri(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLTAKURI::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_tasukigap(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLTASUKIGAP::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_thrusting(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLTHRUSTING::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_tristar(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLTRISTAR::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_unique3river(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLUNIQUE3RIVER::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_upsidegap2crows(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLUPSIDEGAP2CROWS::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn cdl_xsidegap3methods(inputs: &[Series]) -> PolarsResult<Series> {
    let open = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let close = inputs[3].f64()?;
    let mut indicator = quantwave_core::CDLXSIDEGAP3METHODS::new();
    let out: Float64Chunked = open.into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .zip(close.into_iter())
        .map(|(((o, h), l), c)| match (o, h, l, c) {
            (Some(o_), Some(h_), Some(l_), Some(c_)) if !o_.is_nan() && !h_.is_nan() && !l_.is_nan() && !c_.is_nan() => 
                Some(indicator.next((o_, h_, l_, c_)) as f64),
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }).collect();
    Ok(out.into_series())
}
