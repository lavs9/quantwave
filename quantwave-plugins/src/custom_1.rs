use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use quantwave_core::*;
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct GapMomentumKwargs {
    period: usize,
    signal_period: usize,
}

pub fn gap_momentum_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "gap_momentum_result".into(),
        DataType::Struct(vec![
            Field::new("gap_ratio".into(), DataType::Float64),
            Field::new("gap_signal".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=gap_momentum_output)]
fn gap_momentum(inputs: &[Series], kwargs: GapMomentumKwargs) -> PolarsResult<Series> {
    let s_o = &inputs[0];
    let s_c = &inputs[1];
    
    let open = s_o.f64()?;
    let close = s_c.f64()?;
    
    let mut indicator = quantwave_core::GapMomentum::new(kwargs.period, kwargs.signal_period);
    
    let mut ratio_vals = Vec::with_capacity(s_o.len());
    let mut signal_vals = Vec::with_capacity(s_o.len());
    
    for i in 0..s_o.len() {
        let o = open.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        let (ratio, signal) = indicator.next((o, c));
        ratio_vals.push(ratio);
        signal_vals.push(signal);
    }
    
    let s_ratio = Series::new("gap_ratio".into(), ratio_vals);
    let s_signal = Series::new("gap_signal".into(), signal_vals);
    
    let series_vec = vec![s_ratio, s_signal];
    let struct_series = StructChunked::from_series(
        "gap_momentum_result".into(),
        s_o.len(),
        series_vec.iter(),
    )?;
    Ok(struct_series.into_series())
}

#[derive(Deserialize)]
struct PeltKwargs {
    penalty: f64,
    min_dist: usize,
}

#[polars_expr(output_type=UInt32)]
fn pelt(inputs: &[Series], kwargs: PeltKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.f64()?;
    
    let data: Vec<f64> = ca.into_iter().map(|v| v.unwrap_or(f64::NAN)).collect();
    let pelt = quantwave_core::regimes::pelt::PELT::new(kwargs.penalty, kwargs.min_dist);
    let cps = pelt.detect(&data);
    
    let mut values = vec![0u32; s.len()];
    for cp in cps {
        if cp < values.len() {
            values[cp] = 1;
        }
    }
    
    Ok(Series::new("changepoints".into(), values))
}

#[derive(Deserialize)]
struct ZlemaKwargs {
    period: usize,
}

#[polars_expr(output_type=Float64)]
fn zlema(inputs: &[Series], kwargs: ZlemaKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.f64()?;
    
    let mut zlema = quantwave_core::ZLEMA::new(kwargs.period);
    let mut values = Vec::with_capacity(s.len());
    
    for i in 0..s.len() {
        let val = ca.get(i).unwrap_or(0.0);
        values.push(zlema.next(val));
    }
    
    Ok(Series::new("zlema".into(), values))
}

#[derive(Deserialize)]
struct RodcKwargs {
    window_size: usize,
    threshold: f64,
    smooth_period: usize,
}

#[polars_expr(output_type=Float64)]
fn rodc(inputs: &[Series], kwargs: RodcKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.f64()?;
    
    let mut indicator = quantwave_core::RODC::new(kwargs.window_size, kwargs.threshold, kwargs.smooth_period);
    let mut values = Vec::with_capacity(s.len());
    
    for i in 0..s.len() {
        let val = ca.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next(val));
    }
    
    Ok(Series::new("rodc".into(), values))
}

pub fn heikin_ashi_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "heikin_ashi_output".into(),
        DataType::Struct(vec![
            Field::new("ha_open".into(), DataType::Float64),
            Field::new("ha_high".into(), DataType::Float64),
            Field::new("ha_low".into(), DataType::Float64),
            Field::new("ha_close".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=heikin_ashi_output)]
fn heikin_ashi(inputs: &[Series]) -> PolarsResult<Series> {
    let s_o = &inputs[0];
    let s_h = &inputs[1];
    let s_l = &inputs[2];
    let s_c = &inputs[3];
    
    let open = s_o.f64()?;
    let high = s_h.f64()?;
    let low = s_l.f64()?;
    let close = s_c.f64()?;
    
    let mut ha = quantwave_core::HeikinAshi::new();
    let mut ha_opens = Vec::with_capacity(s_o.len());
    let mut ha_highs = Vec::with_capacity(s_o.len());
    let mut ha_lows = Vec::with_capacity(s_o.len());
    let mut ha_closes = Vec::with_capacity(s_o.len());
    
    for i in 0..s_o.len() {
        let o = open.get(i).unwrap_or(0.0);
        let h = high.get(i).unwrap_or(0.0);
        let l = low.get(i).unwrap_or(0.0);
        let c = close.get(i).unwrap_or(0.0);
        let (ha_o, ha_h, ha_l, ha_c) = ha.next((o, h, l, c));
        ha_opens.push(ha_o);
        ha_highs.push(ha_h);
        ha_lows.push(ha_l);
        ha_closes.push(ha_c);
    }
    
    let o_series = Series::new("ha_open".into(), ha_opens);
    let h_series = Series::new("ha_high".into(), ha_highs);
    let l_series = Series::new("ha_low".into(), ha_lows);
    let c_series = Series::new("ha_close".into(), ha_closes);
    
    let series_vec = vec![o_series, h_series, l_series, c_series];
    let out = StructChunked::from_series(
        "heikin_ashi_output".into(),
        s_o.len(),
        series_vec.iter(),
    )?;
    Ok(out.into_series())
}
