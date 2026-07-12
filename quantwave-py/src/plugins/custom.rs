use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use quantwave_core::SuperTrend;
use quantwave_core::traits::Next;
use serde::Deserialize;

#[derive(Deserialize)]
struct SupertrendKwargs {
    period: usize,
    multiplier: f64,
}

pub fn supertrend_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "supertrend".into(),
        DataType::Struct(vec![
            Field::new("supertrend".into(), DataType::Float64),
            Field::new("direction".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=supertrend_output)]
fn supertrend(inputs: &[Series], kwargs: SupertrendKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut indicator = SuperTrend::new(kwargs.period, kwargs.multiplier);

    let mut st_vec = Vec::with_capacity(high.len());
    let mut dir_vec = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);

        if h.is_nan() || l.is_nan() || c.is_nan() {
            st_vec.push(Some(f64::NAN));
            dir_vec.push(Some(f64::NAN));
        } else {
            let (val, dir) = indicator.next((h, l, c));
            st_vec.push(Some(val));
            dir_vec.push(Some(dir as f64));
        }
    }

    let ca_st = Float64Chunked::new("supertrend".into(), st_vec);
    let ca_dir = Float64Chunked::new("direction".into(), dir_vec);

    let series_vec = vec![ca_st.into_series(), ca_dir.into_series()];
    let out = StructChunked::from_series("supertrend".into(), high.len(), series_vec.iter())?;

    Ok(out.into_series())
}
