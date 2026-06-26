use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use quantwave_core::*;
use quantwave_core::traits::Next;

fn u8_to_matype(matype: u8) -> talib::MaType {
    match matype {
        0 => talib::MaType::Sma,
        1 => talib::MaType::Ema,
        2 => talib::MaType::Wma,
        3 => talib::MaType::Dema,
        4 => talib::MaType::Tema,
        5 => talib::MaType::Trima,
        6 => talib::MaType::Kama,
        7 => talib::MaType::Mama,
        8 => talib::MaType::T3,
        _ => talib::MaType::Sma,
    }
}

// 1. MAVP
#[derive(Deserialize)]
struct MavpKwargs {
    minperiod: usize,
    maxperiod: usize,
    matype: u8,
}

#[polars_expr(output_type=Float64)]
fn mavp(inputs: &[Series], kwargs: MavpKwargs) -> PolarsResult<Series> {
    let in1_ca = inputs[0].f64()?;
    let in2_ca = inputs[1].f64()?;

    let mut indicator = MAVP::new(kwargs.minperiod, kwargs.maxperiod, u8_to_matype(kwargs.matype));
    let mut values = Vec::with_capacity(in1_ca.len());

    for i in 0..in1_ca.len() {
        let i1 = in1_ca.get(i).unwrap_or(f64::NAN);
        let i2 = in2_ca.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next((i1, i2)));
    }

    Ok(Series::new("mavp".into(), values))
}

// 2. MFI
#[derive(Deserialize)]
struct MfiKwargs {
    period: usize,
}

#[polars_expr(output_type=Float64)]
fn mfi(inputs: &[Series], kwargs: MfiKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let volume = inputs[3].f64()?;

    let mut indicator = MFI::new(kwargs.period);
    let mut values = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        let v = volume.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next((h, l, c, v)));
    }

    Ok(Series::new("mfi".into(), values))
}

// 3. REVERSE_EMA
#[derive(Deserialize)]
struct ReverseEmaKwargs {
    alpha: f64,
}

#[polars_expr(output_type=Float64)]
fn reverse_ema(inputs: &[Series], kwargs: ReverseEmaKwargs) -> PolarsResult<Series> {
    let ca = inputs[0].f64()?;

    let mut indicator = quantwave_core::ReverseEMA::new(kwargs.alpha);
    let mut values = Vec::with_capacity(ca.len());

    for i in 0..ca.len() {
        let val = ca.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next(val));
    }

    Ok(Series::new("reverse_ema".into(), values))
}

// 4. VOLATILITY_CLUSTERER
#[derive(Deserialize)]
struct VolatilityClustererKwargs {
    atr_period: usize,
    window_size: usize,
    k: usize,
}

#[polars_expr(output_type=UInt32)]
fn volatility_clusterer(inputs: &[Series], kwargs: VolatilityClustererKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut clusterer = quantwave_core::regimes::volatility_clustering::VolatilityClusterer::new(
        kwargs.atr_period,
        kwargs.window_size,
        kwargs.k,
    );
    let mut values = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        let regime = clusterer.next((h, l, c));
        let val = match regime {
            quantwave_core::regimes::MarketRegime::Steady => 0u32,
            quantwave_core::regimes::MarketRegime::Bull => 1,
            quantwave_core::regimes::MarketRegime::Bear => 2,
            quantwave_core::regimes::MarketRegime::Crisis => 3,
            quantwave_core::regimes::MarketRegime::Cluster(c) => 4 + (c as u32),
        };
        values.push(val);
    }

    Ok(Series::new("volatility_regime".into(), values))
}

// 5. PIVOT_POINTS
pub fn pivot_points_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "pivot_points".into(),
        DataType::Struct(vec![
            Field::new("p".into(), DataType::Float64),
            Field::new("r1".into(), DataType::Float64),
            Field::new("s1".into(), DataType::Float64),
            Field::new("r2".into(), DataType::Float64),
            Field::new("s2".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=pivot_points_output)]
fn pivot_points(inputs: &[Series]) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut pivot = quantwave_core::PivotPoints::new();
    let mut p_vals = Vec::with_capacity(high.len());
    let mut r1_vals = Vec::with_capacity(high.len());
    let mut s1_vals = Vec::with_capacity(high.len());
    let mut r2_vals = Vec::with_capacity(high.len());
    let mut s2_vals = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(0.0);
        let l = low.get(i).unwrap_or(0.0);
        let c = close.get(i).unwrap_or(0.0);
        let (p, r1, s1, r2, s2) = pivot.next((h, l, c));
        p_vals.push(p);
        r1_vals.push(r1);
        s1_vals.push(s1);
        r2_vals.push(r2);
        s2_vals.push(s2);
    }

    let p_series = Series::new("p".into(), p_vals);
    let r1_series = Series::new("r1".into(), r1_vals);
    let s1_series = Series::new("s1".into(), s1_vals);
    let r2_series = Series::new("r2".into(), r2_vals);
    let s2_series = Series::new("s2".into(), s2_vals);

    let out = StructChunked::from_series(
        "pivot_output".into(),
        high.len(),
        [p_series, r1_series, s1_series, r2_series, s2_series].iter(),
    )?;
    Ok(out.into_series())
}
