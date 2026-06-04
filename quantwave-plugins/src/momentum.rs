use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use talib_rs::MaType;

use quantwave_core::indicators::incremental::cci::CCI;
use quantwave_core::indicators::incremental::cmo::CMO;
use quantwave_core::indicators::incremental::mom::{MOM, ROC, ROCP, ROCR, ROCR100};
use quantwave_core::indicators::incremental::ultosc::ULTOSC;
use quantwave_core::indicators::incremental::willr::WILLR;
use quantwave_core::indicators::incremental::trix::TRIX;
use quantwave_core::indicators::incremental::apo::{APO, PPO};
use quantwave_core::indicators::incremental::sar::SAR;
use quantwave_core::indicators::incremental::aroon::{AROON, AROONOSC};
use quantwave_core::indicators::incremental::stoch::{STOCH, STOCHF, STOCHRSI};
use quantwave_core::indicators::incremental::dmi::{DX, ADX, ADXR, MINUS_DI, PLUS_DI};
use quantwave_core::indicators::incremental::dm::{MINUS_DM, PLUS_DM};
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct SinglePeriodKwargs {
    timeperiod: usize,
}

#[polars_expr(output_type=Float64)]
fn cci(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = CCI::new(kwargs.timeperiod);
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
fn cmo(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = CMO::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| {
        match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        }
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn mom(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = MOM::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn roc(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = ROC::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn rocp(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = ROCP::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn rocr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = ROCR::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn rocr100(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = ROCR100::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn trix(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TRIX::new(kwargs.timeperiod);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn willr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = WILLR::new(kwargs.timeperiod);
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
fn adx(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = ADX::new(kwargs.timeperiod);
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
fn adxr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = ADXR::new(kwargs.timeperiod);
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
fn dx(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = DX::new(kwargs.timeperiod);
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
fn plus_di(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = PLUS_DI::new(kwargs.timeperiod);
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
fn minus_di(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = MINUS_DI::new(kwargs.timeperiod);
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
fn plus_dm(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let mut indicator = PLUS_DM::new(kwargs.timeperiod);
    let out: Float64Chunked = high.into_iter().zip(low.into_iter()).map(|(h, l)| {
        match (h, l) {
            (Some(hv), Some(lv)) if !hv.is_nan() && !lv.is_nan() => Some(indicator.next((hv, lv))),
            (Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn minus_dm(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let mut indicator = MINUS_DM::new(kwargs.timeperiod);
    let out: Float64Chunked = high.into_iter().zip(low.into_iter()).map(|(h, l)| {
        match (h, l) {
            (Some(hv), Some(lv)) if !hv.is_nan() && !lv.is_nan() => Some(indicator.next((hv, lv))),
            (Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn aroonosc(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let mut indicator = AROONOSC::new(kwargs.timeperiod);
    let out: Float64Chunked = high.into_iter().zip(low.into_iter()).map(|(h, l)| {
        match (h, l) {
            (Some(hv), Some(lv)) if !hv.is_nan() && !lv.is_nan() => Some(indicator.next((hv, lv))),
            (Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }
    }).collect();
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct UltoscKwargs {
    timeperiod1: usize,
    timeperiod2: usize,
    timeperiod3: usize,
}

#[polars_expr(output_type=Float64)]
fn ultosc(inputs: &[Series], kwargs: UltoscKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let mut indicator = ULTOSC::new(kwargs.timeperiod1, kwargs.timeperiod2, kwargs.timeperiod3);
    let out: Float64Chunked = high.into_iter().zip(low.into_iter()).zip(close.into_iter()).map(|((h, l), c)| {
        match (h, l, c) {
            (Some(hv), Some(lv), Some(cv)) if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() => Some(indicator.next((hv, lv, cv))),
            (Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }
    }).collect();
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct ApoKwargs {
    fastperiod: usize,
    slowperiod: usize,
    matype: u8,
}

#[polars_expr(output_type=Float64)]
fn apo(inputs: &[Series], kwargs: ApoKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let ma_type = match kwargs.matype {
        0 => MaType::Sma,
        1 => MaType::Ema,
        2 => MaType::Wma,
        3 => MaType::Dema,
        4 => MaType::Tema,
        5 => MaType::Trima,
        6 => MaType::Kama,
        7 => MaType::Mama,
        8 => MaType::T3,
        _ => MaType::Sma,
    };
    let mut indicator = APO::new(kwargs.fastperiod, kwargs.slowperiod, ma_type);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ppo(inputs: &[Series], kwargs: ApoKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let ma_type = match kwargs.matype {
        0 => MaType::Sma,
        1 => MaType::Ema,
        2 => MaType::Wma,
        3 => MaType::Dema,
        4 => MaType::Tema,
        5 => MaType::Trima,
        6 => MaType::Kama,
        7 => MaType::Mama,
        8 => MaType::T3,
        _ => MaType::Sma,
    };
    let mut indicator = PPO::new(kwargs.fastperiod, kwargs.slowperiod, ma_type);
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct SarKwargs {
    optinacceleration: f64,
    optinmaximum: f64,
}

#[polars_expr(output_type=Float64)]
fn sar(inputs: &[Series], kwargs: SarKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let mut indicator = SAR::new(kwargs.optinacceleration, kwargs.optinmaximum);
    let out: Float64Chunked = high.into_iter().zip(low.into_iter()).map(|(h, l)| {
        match (h, l) {
            (Some(hv), Some(lv)) if !hv.is_nan() && !lv.is_nan() => Some(indicator.next((hv, lv))),
            (Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        }
    }).collect();
    Ok(out.into_series())
}

pub fn aroon_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "aroon".into(),
        DataType::Struct(vec![
            Field::new("down".into(), DataType::Float64),
            Field::new("up".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=aroon_output)]
fn aroon(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let mut indicator = AROON::new(kwargs.timeperiod);
    
    let mut down_vec = Vec::with_capacity(high.len());
    let mut up_vec = Vec::with_capacity(high.len());
    
    for (h, l) in high.into_iter().zip(low.into_iter()) {
        match (h, l) {
            (Some(hv), Some(lv)) if !hv.is_nan() && !lv.is_nan() => {
                let (d, u) = indicator.next((hv, lv));
                down_vec.push(Some(d));
                up_vec.push(Some(u));
            }
            (Some(_), Some(_)) => {
                down_vec.push(Some(f64::NAN));
                up_vec.push(Some(f64::NAN));
            }
            _ => {
                down_vec.push(None);
                up_vec.push(None);
            }
        }
    }
    
    let ca_down = Float64Chunked::new("down".into(), down_vec);
    let ca_up = Float64Chunked::new("up".into(), up_vec);
    
    let series_vec = vec![ca_down.into_series(), ca_up.into_series()];
    let out = StructChunked::from_series("aroon".into(), high.len(), series_vec.iter())?;
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct StochKwargs {
    fastk_period: usize,
    slowk_period: usize,
    slowk_matype: u8,
    slowd_period: usize,
    slowd_matype: u8,
}

pub fn stoch_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "stoch".into(),
        DataType::Struct(vec![
            Field::new("slowk".into(), DataType::Float64),
            Field::new("slowd".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=stoch_output)]
fn stoch(inputs: &[Series], kwargs: StochKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    
    let slowk_matype = match kwargs.slowk_matype {
        0 => MaType::Sma, 1 => MaType::Ema, 2 => MaType::Wma, 3 => MaType::Dema, 4 => MaType::Tema, 5 => MaType::Trima, 6 => MaType::Kama, 7 => MaType::Mama, 8 => MaType::T3, _ => MaType::Sma,
    };
    let slowd_matype = match kwargs.slowd_matype {
        0 => MaType::Sma, 1 => MaType::Ema, 2 => MaType::Wma, 3 => MaType::Dema, 4 => MaType::Tema, 5 => MaType::Trima, 6 => MaType::Kama, 7 => MaType::Mama, 8 => MaType::T3, _ => MaType::Sma,
    };
    
    let mut indicator = STOCH::new(kwargs.fastk_period, kwargs.slowk_period, slowk_matype, kwargs.slowd_period, slowd_matype);
    
    let mut slowk_vec = Vec::with_capacity(high.len());
    let mut slowd_vec = Vec::with_capacity(high.len());
    
    for ((h, l), c) in high.into_iter().zip(low.into_iter()).zip(close.into_iter()) {
        match (h, l, c) {
            (Some(hv), Some(lv), Some(cv)) if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() => {
                let (k, d) = indicator.next((hv, lv, cv));
                slowk_vec.push(Some(k));
                slowd_vec.push(Some(d));
            }
            (Some(_), Some(_), Some(_)) => {
                slowk_vec.push(Some(f64::NAN));
                slowd_vec.push(Some(f64::NAN));
            }
            _ => {
                slowk_vec.push(None);
                slowd_vec.push(None);
            }
        }
    }
    
    let ca_k = Float64Chunked::new("slowk".into(), slowk_vec);
    let ca_d = Float64Chunked::new("slowd".into(), slowd_vec);
    
    let series_vec = vec![ca_k.into_series(), ca_d.into_series()];
    let out = StructChunked::from_series("stoch".into(), high.len(), series_vec.iter())?;
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct StochfKwargs {
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: u8,
}

pub fn stochf_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "stochf".into(),
        DataType::Struct(vec![
            Field::new("fastk".into(), DataType::Float64),
            Field::new("fastd".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=stochf_output)]
fn stochf(inputs: &[Series], kwargs: StochfKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    
    let fastd_matype = match kwargs.fastd_matype {
        0 => MaType::Sma, 1 => MaType::Ema, 2 => MaType::Wma, 3 => MaType::Dema, 4 => MaType::Tema, 5 => MaType::Trima, 6 => MaType::Kama, 7 => MaType::Mama, 8 => MaType::T3, _ => MaType::Sma,
    };
    
    let mut indicator = STOCHF::new(kwargs.fastk_period, kwargs.fastd_period, fastd_matype);
    
    let mut fastk_vec = Vec::with_capacity(high.len());
    let mut fastd_vec = Vec::with_capacity(high.len());
    
    for ((h, l), c) in high.into_iter().zip(low.into_iter()).zip(close.into_iter()) {
        match (h, l, c) {
            (Some(hv), Some(lv), Some(cv)) if !hv.is_nan() && !lv.is_nan() && !cv.is_nan() => {
                let (k, d) = indicator.next((hv, lv, cv));
                fastk_vec.push(Some(k));
                fastd_vec.push(Some(d));
            }
            (Some(_), Some(_), Some(_)) => {
                fastk_vec.push(Some(f64::NAN));
                fastd_vec.push(Some(f64::NAN));
            }
            _ => {
                fastk_vec.push(None);
                fastd_vec.push(None);
            }
        }
    }
    
    let ca_k = Float64Chunked::new("fastk".into(), fastk_vec);
    let ca_d = Float64Chunked::new("fastd".into(), fastd_vec);
    
    let series_vec = vec![ca_k.into_series(), ca_d.into_series()];
    let out = StructChunked::from_series("stochf".into(), high.len(), series_vec.iter())?;
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct StochrsiKwargs {
    timeperiod: usize,
    fastk_period: usize,
    fastd_period: usize,
    fastd_matype: u8,
}

pub fn stochrsi_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "stochrsi".into(),
        DataType::Struct(vec![
            Field::new("fastk".into(), DataType::Float64),
            Field::new("fastd".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=stochrsi_output)]
fn stochrsi(inputs: &[Series], kwargs: StochrsiKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    
    let fastd_matype = match kwargs.fastd_matype {
        0 => MaType::Sma, 1 => MaType::Ema, 2 => MaType::Wma, 3 => MaType::Dema, 4 => MaType::Tema, 5 => MaType::Trima, 6 => MaType::Kama, 7 => MaType::Mama, 8 => MaType::T3, _ => MaType::Sma,
    };
    
    let mut indicator = STOCHRSI::new(kwargs.timeperiod, kwargs.fastk_period, kwargs.fastd_period, fastd_matype);
    
    let mut fastk_vec = Vec::with_capacity(s.len());
    let mut fastd_vec = Vec::with_capacity(s.len());
    
    for opt_v in s.into_iter() {
        match opt_v {
            Some(v) if !v.is_nan() => {
                let (k, d) = indicator.next(v);
                fastk_vec.push(Some(k));
                fastd_vec.push(Some(d));
            }
            Some(_) => {
                fastk_vec.push(Some(f64::NAN));
                fastd_vec.push(Some(f64::NAN));
            }
            None => {
                fastk_vec.push(None);
                fastd_vec.push(None);
            }
        }
    }
    
    let ca_k = Float64Chunked::new("fastk".into(), fastk_vec);
    let ca_d = Float64Chunked::new("fastd".into(), fastd_vec);
    
    let series_vec = vec![ca_k.into_series(), ca_d.into_series()];
    let out = StructChunked::from_series("stochrsi".into(), s.len(), series_vec.iter())?;
    Ok(out.into_series())
}
