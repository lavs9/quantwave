use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use quantwave_core::traits::Next;
use quantwave_core::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TaVarKwargs {
    pub period: usize,
    pub nbdev: f64,
}

#[polars_expr(output_type=Float64)]
fn ta_var(inputs: &[Series], kwargs: TaVarKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.f64()?;
    let mut indicator = TaVAR::new(kwargs.period, kwargs.nbdev);
    let mut values = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        let val = ca.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next(val));
    }
    Ok(Series::new("ta_var".into(), values))
}

#[derive(Deserialize)]
pub struct VfiKwargs {
    pub period: usize,
    pub coef: f64,
    pub vcoef: f64,
    pub smoothing_period: usize,
}

#[polars_expr(output_type=Float64)]
fn vfi(inputs: &[Series], kwargs: VfiKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let volume = inputs[3].f64()?;

    let mut indicator = quantwave_core::Vfi::new(
        kwargs.period,
        kwargs.coef,
        kwargs.vcoef,
        kwargs.smoothing_period,
    );
    let mut values = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        let v = volume.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next((h, l, c, v)));
    }

    Ok(Series::new("vfi".into(), values))
}

#[derive(Deserialize)]
pub struct WaveTrendKwargs {
    pub n1: usize,
    pub n2: usize,
    pub n3: usize,
}

fn wavetrend_output(_input_fields: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "wavetrend".into(),
        DataType::Struct(vec![
            Field::new("wt1".into(), DataType::Float64),
            Field::new("wt2".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=wavetrend_output)]
fn wavetrend(inputs: &[Series], kwargs: WaveTrendKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut wt = quantwave_core::WaveTrend::new(kwargs.n1, kwargs.n2, kwargs.n3);
    let mut wt1_vals = Vec::with_capacity(high.len());
    let mut wt2_vals = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(0.0);
        let l = low.get(i).unwrap_or(0.0);
        let c = close.get(i).unwrap_or(0.0);
        let (wt1, wt2) = wt.next((h, l, c));
        wt1_vals.push(wt1);
        wt2_vals.push(wt2);
    }

    let wt1_series = Series::new("wt1".into(), wt1_vals);
    let wt2_series = Series::new("wt2".into(), wt2_vals);

    let out = StructChunked::from_series(
        "wavetrend".into(),
        high.len(),
        [wt1_series, wt2_series].iter(),
    )?;
    Ok(out.into_series())
}

#[derive(Deserialize)]
pub struct RegimesEnsembleKwargs {
    pub weights: Vec<f64>,
}

#[polars_expr(output_type=UInt32)]
fn regimes_ensemble(inputs: &[Series], kwargs: RegimesEnsembleKwargs) -> PolarsResult<Series> {
    let n_rows = inputs[0].len();
    let n_dims = inputs.len();
    let ensemble = quantwave_core::regimes::ensemble::RegimeEnsemble::new(kwargs.weights);

    let mut cas = Vec::with_capacity(n_dims);
    for s in inputs {
        cas.push(s.u32()?);
    }

    let mut results = Vec::with_capacity(n_rows);
    for i in 0..n_rows {
        let mut row_regimes = Vec::with_capacity(n_dims);
        for ca in &cas {
            let val = ca.get(i).unwrap_or(0);
            let regime = match val {
                0 => quantwave_core::regimes::MarketRegime::Steady,
                1 => quantwave_core::regimes::MarketRegime::Crisis,
                2 => quantwave_core::regimes::MarketRegime::Bull,
                3 => quantwave_core::regimes::MarketRegime::Bear,
                v if v >= 4 => quantwave_core::regimes::MarketRegime::Cluster((v - 4) as u8),
                _ => quantwave_core::regimes::MarketRegime::Steady,
            };
            row_regimes.push(regime);
        }

        let consensus = ensemble.vote(&row_regimes);
        let out = match consensus {
            quantwave_core::regimes::MarketRegime::Steady => 0u32,
            quantwave_core::regimes::MarketRegime::Crisis => 1,
            quantwave_core::regimes::MarketRegime::Bull => 2,
            quantwave_core::regimes::MarketRegime::Bear => 3,
            quantwave_core::regimes::MarketRegime::Cluster(c) => 4 + (c as u32),
        };
        results.push(out);
    }

    Ok(Series::new("ensemble_regime".into(), results))
}

#[derive(Deserialize)]
pub struct TaStddevKwargs {
    pub period: usize,
    pub nbdev: f64,
}

#[polars_expr(output_type=Float64)]
fn ta_stddev(inputs: &[Series], kwargs: TaStddevKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.f64()?;
    let mut indicator = TaSTDDEV::new(kwargs.period, kwargs.nbdev);
    let mut values = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        let val = ca.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next(val));
    }
    Ok(Series::new("ta_stddev".into(), values))
}
