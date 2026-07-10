use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use quantwave_core::traits::Next;
use quantwave_core::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct AutoTuneFilterKwargs {
    window: usize,
    bandwidth: f64,
}

#[polars_expr(output_type=Float64)]
fn autotune_filter(inputs: &[Series], kwargs: AutoTuneFilterKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = AutoTuneFilter::new(kwargs.window, kwargs.bandwidth);
    let mut values = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next(val));
    }
    Ok(Series::new("autotune".into(), values))
}

#[derive(Deserialize)]
struct TRAdjEMAKwargs {
    period: usize,
    pds: usize,
    mltp: f64,
}

#[polars_expr(output_type=Float64)]
fn tradj_ema(inputs: &[Series], kwargs: TRAdjEMAKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut indicator = TRAdjEMA::new(kwargs.period, kwargs.pds, kwargs.mltp);
    let mut values = Vec::with_capacity(high.len());
    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next((h, l, c)));
    }
    Ok(Series::new("tradj_ema".into(), values))
}

#[derive(Deserialize)]
struct TEMAKwargs {
    period: usize,
}

#[polars_expr(output_type=Float64)]
fn tema(inputs: &[Series], kwargs: TEMAKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = TEMA::new(kwargs.period);
    let mut values = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(0.0);
        values.push(indicator.next(val));
    }
    Ok(Series::new("tema".into(), values))
}

#[derive(Deserialize)]
struct RSMKKwargs {
    length: usize,
    ema_length: usize,
}

#[polars_expr(output_type=Float64)]
fn rsmk(inputs: &[Series], kwargs: RSMKKwargs) -> PolarsResult<Series> {
    let price = inputs[0].f64()?;
    let benchmark = inputs[1].f64()?;

    let mut indicator = RSMK::new(kwargs.length, kwargs.ema_length);
    let mut values = Vec::with_capacity(price.len());
    for i in 0..price.len() {
        let p = price.get(i).unwrap_or(f64::NAN);
        let b = benchmark.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next((p, b)));
    }
    Ok(Series::new("rsmk".into(), values))
}

#[derive(Deserialize)]
struct ExpDevBandsKwargs {
    period: usize,
    multiplier: f64,
    use_sma: bool,
}

pub fn exp_dev_bands_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "exp_dev_bands".into(),
        DataType::Struct(vec![
            Field::new("upper".into(), DataType::Float64),
            Field::new("middle".into(), DataType::Float64),
            Field::new("lower".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=exp_dev_bands_output)]
fn exp_dev_bands(inputs: &[Series], kwargs: ExpDevBandsKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = ExpDevBands::new(kwargs.period, kwargs.multiplier, kwargs.use_sma);
    let mut upper_vals = Vec::with_capacity(s.len());
    let mut mid_vals = Vec::with_capacity(s.len());
    let mut lower_vals = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(f64::NAN);
        let (upper, mid, lower) = indicator.next(val);
        upper_vals.push(upper);
        mid_vals.push(mid);
        lower_vals.push(lower);
    }

    let s_upper = Series::new("upper".into(), upper_vals);
    let s_mid = Series::new("middle".into(), mid_vals);
    let s_lower = Series::new("lower".into(), lower_vals);

    let out = StructChunked::from_series(
        "exp_dev_bands".into(),
        s.len(),
        [s_upper, s_mid, s_lower].iter(),
    )?;
    Ok(out.into_series())
}
