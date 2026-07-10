#![allow(clippy::unused_unit)]
pub mod momentum;
pub mod options_india;
pub mod volatility;
pub mod volume;
pub mod price_transform;
pub mod overlap;
pub mod statistics;
pub mod hilbert;

use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use pyo3::prelude::*;
use serde::Deserialize;

use quantwave_core::indicators::smoothing::{SMA, EMA};
use quantwave_core::indicators::incremental::rsi::RSI;
use quantwave_core::indicators::incremental::macd::MACD;
use quantwave_core::indicators::incremental::bbands::BBANDS;
use quantwave_core::traits::Next;
use talib_rs::MaType;

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

#[derive(Deserialize)]
struct EmaKwargs {
    period: usize,
}

#[polars_expr(output_type=Float64)]
fn ema(inputs: &[Series], kwargs: EmaKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let s_f64 = s.f64()?;
    
    let mut indicator = EMA::new(kwargs.period);
    
    let out: Float64Chunked = s_f64.into_iter().map(|opt_v| {
        match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        }
    }).collect();
    
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct RsiKwargs {
    timeperiod: usize,
}

#[polars_expr(output_type=Float64)]
fn rsi(inputs: &[Series], kwargs: RsiKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let s_f64 = s.f64()?;
    
    let mut indicator = RSI::new(kwargs.timeperiod);
    
    let out: Float64Chunked = s_f64.into_iter().map(|opt_v| {
        match opt_v {
            Some(v) if !v.is_nan() => Some(indicator.next(v)),
            Some(_) => Some(f64::NAN),
            None => None,
        }
    }).collect();
    
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct MacdKwargs {
    fast: usize,
    slow: usize,
    signal: usize,
}

pub fn macd_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "macd".into(),
        DataType::Struct(vec![
            Field::new("macd".into(), DataType::Float64),
            Field::new("signal".into(), DataType::Float64),
            Field::new("hist".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=macd_output)]
fn macd(inputs: &[Series], kwargs: MacdKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let s_f64 = s.f64()?;
    
    let mut indicator = MACD::new(kwargs.fast, kwargs.slow, kwargs.signal);
    
    let mut macd_vec = Vec::with_capacity(s_f64.len());
    let mut signal_vec = Vec::with_capacity(s_f64.len());
    let mut hist_vec = Vec::with_capacity(s_f64.len());
    
    for opt_v in s_f64.into_iter() {
        match opt_v {
            Some(v) if !v.is_nan() => {
                let (m, s, h) = indicator.next(v);
                macd_vec.push(Some(m));
                signal_vec.push(Some(s));
                hist_vec.push(Some(h));
            }
            Some(_) => {
                macd_vec.push(Some(f64::NAN));
                signal_vec.push(Some(f64::NAN));
                hist_vec.push(Some(f64::NAN));
            }
            None => {
                macd_vec.push(None);
                signal_vec.push(None);
                hist_vec.push(None);
            }
        }
    }
    
    let ca_macd = Float64Chunked::new("macd".into(), macd_vec);
    let ca_signal = Float64Chunked::new("signal".into(), signal_vec);
    let ca_hist = Float64Chunked::new("hist".into(), hist_vec);
    
    let series_vec = vec![ca_macd.into_series(), ca_signal.into_series(), ca_hist.into_series()];
    let out = StructChunked::from_series("macd".into(), s_f64.len(), series_vec.iter())?;
    
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct BbandsKwargs {
    timeperiod: usize,
    nbdevup: f64,
    nbdevdn: f64,
    matype: u8,
}

pub fn bbands_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "bbands".into(),
        DataType::Struct(vec![
            Field::new("upper".into(), DataType::Float64),
            Field::new("middle".into(), DataType::Float64),
            Field::new("lower".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=bbands_output)]
fn bbands(inputs: &[Series], kwargs: BbandsKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let s_f64 = s.f64()?;
    
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
    
    let mut indicator = BBANDS::new(kwargs.timeperiod, kwargs.nbdevup, kwargs.nbdevdn, ma_type);
    
    let mut upper_vec = Vec::with_capacity(s_f64.len());
    let mut middle_vec = Vec::with_capacity(s_f64.len());
    let mut lower_vec = Vec::with_capacity(s_f64.len());
    
    for opt_v in s_f64.into_iter() {
        match opt_v {
            Some(v) if !v.is_nan() => {
                let (u, m, l) = indicator.next(v);
                upper_vec.push(Some(u));
                middle_vec.push(Some(m));
                lower_vec.push(Some(l));
            }
            Some(_) => {
                upper_vec.push(Some(f64::NAN));
                middle_vec.push(Some(f64::NAN));
                lower_vec.push(Some(f64::NAN));
            }
            None => {
                upper_vec.push(None);
                middle_vec.push(None);
                lower_vec.push(None);
            }
        }
    }
    
    let ca_upper = Float64Chunked::new("upper".into(), upper_vec);
    let ca_middle = Float64Chunked::new("middle".into(), middle_vec);
    let ca_lower = Float64Chunked::new("lower".into(), lower_vec);
    
    let series_vec = vec![ca_upper.into_series(), ca_middle.into_series(), ca_lower.into_series()];
    let out = StructChunked::from_series("bbands".into(), s_f64.len(), series_vec.iter())?;
    
    Ok(out.into_series())
}

#[pymodule]
#[pyo3(name = "quantwave_plugins")]
fn quantwave_plugins(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
pub mod generated;
pub mod custom;
pub mod custom_0;
pub mod custom_1;
pub mod custom_2;
pub mod custom_3;
pub mod custom_4;
pub mod custom_5;
pub mod custom_6;
pub mod custom_7;
pub mod custom_8;
pub mod custom_9;
pub mod custom_10;
