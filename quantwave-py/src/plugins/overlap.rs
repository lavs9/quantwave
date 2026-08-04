use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::MaType;
use quantwave_core::indicators::incremental::dema::DEMA;
use quantwave_core::indicators::incremental::macd_ext::{MACDEXT, MACDFIX};
use quantwave_core::indicators::incremental::overlap_ta::{KAMA, MIDPOINT, MIDPRICE, T3, TRIMA};
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct SinglePeriodKwargs {
    timeperiod: usize,
}

#[polars_expr(output_type=Float64)]
fn trima(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TRIMA::new(kwargs.timeperiod);

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
fn midpoint(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = MIDPOINT::new(kwargs.timeperiod);

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
fn midprice(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let mut indicator = MIDPRICE::new(kwargs.timeperiod);

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
fn kama(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = KAMA::new(kwargs.timeperiod);

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

#[derive(Deserialize)]
struct T3Kwargs {
    timeperiod: usize,
    vfactor: f64,
}

#[polars_expr(output_type=Float64)]
fn t3(inputs: &[Series], kwargs: T3Kwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = T3::new(kwargs.timeperiod, kwargs.vfactor);

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
fn dema(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = DEMA::new(kwargs.timeperiod);

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

#[derive(Deserialize)]
struct MacdExtKwargs {
    fastperiod: usize,
    fastmatype: u8,
    slowperiod: usize,
    slowmatype: u8,
    signalperiod: usize,
    signalmatype: u8,
}

pub fn macd_ext_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "macdext".into(),
        DataType::Struct(vec![
            Field::new("macd".into(), DataType::Float64),
            Field::new("signal".into(), DataType::Float64),
            Field::new("hist".into(), DataType::Float64),
        ]),
    ))
}

fn u8_to_matype(matype: u8) -> MaType {
    MaType::from_u8_or_sma(matype)
}

#[polars_expr(output_type_func=macd_ext_output)]
fn macdext(inputs: &[Series], kwargs: MacdExtKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;

    let mut indicator = MACDEXT::new(
        kwargs.fastperiod,
        u8_to_matype(kwargs.fastmatype),
        kwargs.slowperiod,
        u8_to_matype(kwargs.slowmatype),
        kwargs.signalperiod,
        u8_to_matype(kwargs.signalmatype),
    );

    let mut macd_vec = Vec::with_capacity(s.len());
    let mut signal_vec = Vec::with_capacity(s.len());
    let mut hist_vec = Vec::with_capacity(s.len());

    for opt_v in s.into_iter() {
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

    let series_vec = vec![
        ca_macd.into_series(),
        ca_signal.into_series(),
        ca_hist.into_series(),
    ];
    let out = StructChunked::from_series("macdext".into(), s.len(), series_vec.iter())?;

    Ok(out.into_series())
}

#[derive(Deserialize)]
struct MacdFixKwargs {
    signalperiod: usize,
}

#[polars_expr(output_type_func=macd_ext_output)]
fn macdfix(inputs: &[Series], kwargs: MacdFixKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;

    let mut indicator = MACDFIX::new(kwargs.signalperiod);

    let mut macd_vec = Vec::with_capacity(s.len());
    let mut signal_vec = Vec::with_capacity(s.len());
    let mut hist_vec = Vec::with_capacity(s.len());

    for opt_v in s.into_iter() {
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

    let series_vec = vec![
        ca_macd.into_series(),
        ca_signal.into_series(),
        ca_hist.into_series(),
    ];
    let out = StructChunked::from_series("macdfix".into(), s.len(), series_vec.iter())?;

    Ok(out.into_series())
}
