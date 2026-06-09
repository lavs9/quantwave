use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

use quantwave_core::indicators::incremental::hilbert_ta::{
    HT_DCPERIOD, HT_DCPHASE, HT_PHASOR, HT_SINE, HT_TRENDMODE, HT_TRENDLINE
};
use quantwave_core::traits::Next;

#[polars_expr(output_type=Float64)]
fn ht_dcperiod(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = HT_DCPERIOD::new();
    
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ht_dcphase(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = HT_DCPHASE::new();
    
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    
    Ok(out.into_series())
}

pub fn ht_phasor_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "ht_phasor".into(),
        DataType::Struct(vec![
            Field::new("inphase".into(), DataType::Float64),
            Field::new("quadrature".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=ht_phasor_output)]
fn ht_phasor(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = HT_PHASOR::new();
    
    let mut inphase_vec = Vec::with_capacity(s.len());
    let mut quadrature_vec = Vec::with_capacity(s.len());
    
    for opt_v in s.into_iter() {
        match opt_v {
            Some(v) if !v.is_nan() => {
                let (i, q) = indicator.next(v);
                inphase_vec.push(Some(i));
                quadrature_vec.push(Some(q));
            }
            Some(_) => {
                inphase_vec.push(Some(f64::NAN));
                quadrature_vec.push(Some(f64::NAN));
            }
            None => {
                inphase_vec.push(None);
                quadrature_vec.push(None);
            }
        }
    }
    
    let ca_inphase = Float64Chunked::new("inphase".into(), inphase_vec);
    let ca_quadrature = Float64Chunked::new("quadrature".into(), quadrature_vec);
    
    let series_vec = vec![ca_inphase.into_series(), ca_quadrature.into_series()];
    let out = StructChunked::from_series("ht_phasor".into(), s.len(), series_vec.iter())?;
    
    Ok(out.into_series())
}

pub fn ht_sine_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "ht_sine".into(),
        DataType::Struct(vec![
            Field::new("sine".into(), DataType::Float64),
            Field::new("leadsine".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=ht_sine_output)]
fn ht_sine(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = HT_SINE::new();
    
    let mut sine_vec = Vec::with_capacity(s.len());
    let mut leadsine_vec = Vec::with_capacity(s.len());
    
    for opt_v in s.into_iter() {
        match opt_v {
            Some(v) if !v.is_nan() => {
                let (si, ls) = indicator.next(v);
                sine_vec.push(Some(si));
                leadsine_vec.push(Some(ls));
            }
            Some(_) => {
                sine_vec.push(Some(f64::NAN));
                leadsine_vec.push(Some(f64::NAN));
            }
            None => {
                sine_vec.push(None);
                leadsine_vec.push(None);
            }
        }
    }
    
    let ca_sine = Float64Chunked::new("sine".into(), sine_vec);
    let ca_leadsine = Float64Chunked::new("leadsine".into(), leadsine_vec);
    
    let series_vec = vec![ca_sine.into_series(), ca_leadsine.into_series()];
    let out = StructChunked::from_series("ht_sine".into(), s.len(), series_vec.iter())?;
    
    Ok(out.into_series())
}

#[polars_expr(output_type=Int32)]
fn ht_trendmode(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = HT_TRENDMODE::new();
    
    let out: Int32Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v) as i32),
        _ => None,
    }).collect();
    
    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn ht_trendline(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = HT_TRENDLINE::new();
    
    let out: Float64Chunked = s.into_iter().map(|opt_v| match opt_v {
        Some(v) if !v.is_nan() => Some(indicator.next(v)),
        Some(_) => Some(f64::NAN),
        None => None,
    }).collect();
    
    Ok(out.into_series())
}
