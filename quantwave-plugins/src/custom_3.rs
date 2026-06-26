use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use quantwave_core::*;
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct HmaKwargs {
    period: usize,
}

pub fn hma_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new("hma".into(), DataType::Float64))
}

#[polars_expr(output_type_func=hma_output)]
fn hma(inputs: &[Series], kwargs: HmaKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut hma = HMA::new(kwargs.period);
    let mut values = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(0.0);
        values.push(hma.next(val));
    }

    Ok(Series::new("hma".into(), values))
}

#[derive(Deserialize)]
struct ObvmKwargs {
    obvm_period: usize,
    signal_period: usize,
}

pub fn obvm_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "obvm_data".into(),
        DataType::Struct(vec![
            Field::new("obvm".into(), DataType::Float64),
            Field::new("signal".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=obvm_output)]
fn obvm(inputs: &[Series], kwargs: ObvmKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let volume = inputs[3].f64()?;

    let mut indicator = Obvm::new(kwargs.obvm_period, kwargs.signal_period);
    let mut obvm_vals = Vec::with_capacity(high.len());
    let mut signal_vals = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        let v = volume.get(i).unwrap_or(f64::NAN);
        let (o, sig) = indicator.next((h, l, c, v));
        obvm_vals.push(o);
        signal_vals.push(sig);
    }

    let s_obvm = Series::new("obvm".into(), obvm_vals);
    let s_signal = Series::new("signal".into(), signal_vals);
    let out = StructChunked::from_series(
        "obvm_data".into(),
        high.len(),
        [s_obvm, s_signal].iter(),
    )?;
    Ok(out.into_series())
}

pub fn fractals_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "fractals_data".into(),
        DataType::Struct(vec![
            Field::new("bearish".into(), DataType::Boolean),
            Field::new("bullish".into(), DataType::Boolean),
        ]),
    ))
}

#[polars_expr(output_type_func=fractals_output)]
fn bill_williams_fractals(inputs: &[Series]) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;

    let mut fractals = BillWilliamsFractals::new();
    let mut bearish_vals = Vec::with_capacity(high.len());
    let mut bullish_vals = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(0.0);
        let l = low.get(i).unwrap_or(0.0);
        let (bear, bull) = fractals.next((h, l));
        bearish_vals.push(bear);
        bullish_vals.push(bull);
    }

    let bearish_series = Series::new("bearish".into(), bearish_vals);
    let bullish_series = Series::new("bullish".into(), bullish_vals);

    let out = StructChunked::from_series(
        "fractals_data".into(),
        high.len(),
        [bearish_series, bullish_series].iter(),
    )?;
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct PeriodKwargs {
    period: usize,
}

pub fn donchian_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "donchian_data".into(),
        DataType::Struct(vec![
            Field::new("upper".into(), DataType::Float64),
            Field::new("middle".into(), DataType::Float64),
            Field::new("lower".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=donchian_output)]
fn donchian_channels(inputs: &[Series], kwargs: PeriodKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;

    let mut dc = DonchianChannels::new(kwargs.period);
    let mut uppers = Vec::with_capacity(high.len());
    let mut middles = Vec::with_capacity(high.len());
    let mut lowers = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(0.0);
        let l = low.get(i).unwrap_or(0.0);
        let (upper, middle, lower) = dc.next((h, l));
        uppers.push(upper);
        middles.push(middle);
        lowers.push(lower);
    }

    let upper_series = Series::new("upper".into(), uppers);
    let middle_series = Series::new("middle".into(), middles);
    let lower_series = Series::new("lower".into(), lowers);

    let out = StructChunked::from_series(
        "donchian_data".into(),
        high.len(),
        [upper_series, middle_series, lower_series].iter(),
    )?;
    Ok(out.into_series())
}

#[derive(Deserialize)]
struct GmmKwargs {
    k: usize,
}

pub fn gmm_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new("gmm_regime".into(), DataType::UInt32))
}

#[polars_expr(output_type_func=gmm_output)]
fn gmm(inputs: &[Series], kwargs: GmmKwargs) -> PolarsResult<Series> {
    let n_rows = inputs[0].len();

    let mut values = Vec::with_capacity(n_rows);
    for _ in 0..n_rows {
        values.push(0u32);
    }

    Ok(Series::new("gmm_regime".into(), values))
}
