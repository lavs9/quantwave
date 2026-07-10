use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use quantwave_core::traits::Next;
use quantwave_core::*;
use serde::Deserialize;

#[polars_expr(output_type=Float64)]
fn anchored_vwap(inputs: &[Series]) -> PolarsResult<Series> {
    let price_s = &inputs[0];
    let volume_s = &inputs[1];
    let anchor_s = &inputs[2];

    let price = price_s.f64()?;
    let volume = volume_s.f64()?;
    let anchor = anchor_s.bool()?;

    let mut avwap = quantwave_core::AnchoredVWAP::new();
    let mut values = Vec::with_capacity(price_s.len());

    for i in 0..price_s.len() {
        let p = price.get(i).unwrap_or(0.0);
        let v = volume.get(i).unwrap_or(0.0);
        let a = anchor.get(i).unwrap_or(false);
        values.push(avwap.next((p, v, a)));
    }

    Ok(Series::new("anchored_vwap".into(), values))
}

#[derive(Deserialize)]
pub struct KinematicKalmanKwargs {
    pub q_pos: f64,
    pub q_vel: f64,
    pub r: f64,
}

#[polars_expr(output_type=Float64)]
fn kinematic_kalman(inputs: &[Series], kwargs: KinematicKalmanKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::indicators::kinematic_kalman::KinematicKalmanFilter::new(
        kwargs.q_pos,
        kwargs.q_vel,
        kwargs.r,
    );
    let mut values = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next(val));
    }

    Ok(Series::new("kinematic_kalman".into(), values))
}

#[derive(Deserialize)]
pub struct VortexKwargs {
    pub period: usize,
}

pub fn vortex_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "vortex".into(),
        DataType::Struct(vec![
            Field::new("vi_plus".into(), DataType::Float64),
            Field::new("vi_minus".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=vortex_output)]
fn vortex_indicator(inputs: &[Series], kwargs: VortexKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut vi = quantwave_core::VortexIndicator::new(kwargs.period);
    let mut plus_vals = Vec::with_capacity(high.len());
    let mut minus_vals = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(0.0);
        let l = low.get(i).unwrap_or(0.0);
        let c = close.get(i).unwrap_or(0.0);
        let (plus, minus) = vi.next((h, l, c));
        plus_vals.push(plus);
        minus_vals.push(minus);
    }

    let plus_series = Series::new("vi_plus".into(), plus_vals);
    let minus_series = Series::new("vi_minus".into(), minus_vals);

    let out = StructChunked::from_series(
        "vortex_output".into(),
        high.len(),
        [plus_series, minus_series].iter(),
    )?;

    Ok(out.into_series())
}

#[derive(Deserialize)]
pub struct SveKwargs {
    pub bands_period: usize,
    pub bands_deviation: f64,
    pub low_band_adjust: f64,
    pub mid_line_length: usize,
}

pub fn sve_bands_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "sve_bands".into(),
        DataType::Struct(vec![
            Field::new("upper".into(), DataType::Float64),
            Field::new("middle".into(), DataType::Float64),
            Field::new("lower".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=sve_bands_output)]
fn sve_volatility_bands(inputs: &[Series], kwargs: SveKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut indicator = quantwave_core::SVEVolatilityBands::new(
        kwargs.bands_period,
        kwargs.bands_deviation,
        kwargs.low_band_adjust,
        kwargs.mid_line_length,
    );
    let mut upper_vals = Vec::with_capacity(high.len());
    let mut mid_vals = Vec::with_capacity(high.len());
    let mut lower_vals = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        let (upper, mid, lower) = indicator.next((h, l, c));
        upper_vals.push(upper);
        mid_vals.push(mid);
        lower_vals.push(lower);
    }

    let s_upper = Series::new("upper".into(), upper_vals);
    let s_mid = Series::new("middle".into(), mid_vals);
    let s_lower = Series::new("lower".into(), lower_vals);

    let out = StructChunked::from_series(
        "sve_bands_data".into(),
        high.len(),
        [s_upper, s_mid, s_lower].iter(),
    )?;

    Ok(out.into_series())
}

#[derive(Deserialize)]
pub struct TtmKwargs {
    pub period: usize,
    pub multiplier_bb: f64,
    pub multiplier_kc: f64,
}

pub fn ttm_squeeze_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "ttm_squeeze".into(),
        DataType::Struct(vec![
            Field::new("histogram".into(), DataType::Float64),
            Field::new("is_squeezed".into(), DataType::Boolean),
        ]),
    ))
}

#[polars_expr(output_type_func=ttm_squeeze_output)]
fn ttm_squeeze(inputs: &[Series], kwargs: TtmKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut ttm =
        quantwave_core::TTMSqueeze::new(kwargs.period, kwargs.multiplier_bb, kwargs.multiplier_kc);
    let mut histograms = Vec::with_capacity(high.len());
    let mut squeezed = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(0.0);
        let l = low.get(i).unwrap_or(0.0);
        let c = close.get(i).unwrap_or(0.0);
        let (hist, is_sq) = ttm.next((h, l, c));
        histograms.push(hist);
        squeezed.push(is_sq);
    }

    let hist_series = Series::new("histogram".into(), histograms);
    let squeezed_series = Series::new("is_squeezed".into(), squeezed);

    let out = StructChunked::from_series(
        "ttm_squeeze_output".into(),
        high.len(),
        [hist_series, squeezed_series].iter(),
    )?;

    Ok(out.into_series())
}
