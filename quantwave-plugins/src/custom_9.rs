use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use quantwave_core::*;
use quantwave_core::traits::Next;

// 1. sdo
#[derive(Deserialize)]
struct SdoKwargs {
    lookback_period: usize,
    period: usize,
    ema_pds: usize,
}

fn sdo_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new("sdo".into(), DataType::Float64))
}

#[polars_expr(output_type_func=sdo_output)]
fn sdo(inputs: &[Series], kwargs: SdoKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = quantwave_core::SDO::new(kwargs.lookback_period, kwargs.period, kwargs.ema_pds);
    let mut values = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(f64::NAN);
        if val.is_nan() {
            values.push(f64::NAN);
        } else {
            values.push(indicator.next(val));
        }
    }

    Ok(Float64Chunked::from_slice("sdo".into(), &values).into_series())
}

// 2. regimes_tar
#[derive(Deserialize)]
struct RegimesTarKwargs {
    thresholds: Vec<f64>,
}

fn regimes_tar_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new("tar_regime".into(), DataType::UInt32))
}

#[polars_expr(output_type_func=regimes_tar_output)]
fn regimes_tar(inputs: &[Series], kwargs: RegimesTarKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut model = quantwave_core::regimes::tar::TAR::multi(kwargs.thresholds.clone());
    let mut results = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(f64::NAN);
        // Even for NaN, we pass it to the model.next to keep behavior identical
        // to the user's closure if they didn't check for NaN explicitly.
        let regime = model.next(val);
        let out = match regime {
            quantwave_core::regimes::MarketRegime::Steady => 0u32,
            quantwave_core::regimes::MarketRegime::Crisis => 1,
            quantwave_core::regimes::MarketRegime::Bull => 2,
            quantwave_core::regimes::MarketRegime::Bear => 3,
            quantwave_core::regimes::MarketRegime::Cluster(c) => 4 + (c as u32),
        };
        results.push(out);
    }

    Ok(UInt32Chunked::from_slice("tar_regime".into(), &results).into_series())
}

// 3. ichimoku_cloud
#[derive(Deserialize)]
struct IchimokuCloudKwargs {
    p1: usize,
    p2: usize,
    p3: usize,
}

fn ichimoku_cloud_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "ichimoku_output".into(),
        DataType::Struct(vec![
            Field::new("tenkan".into(), DataType::Float64),
            Field::new("kijun".into(), DataType::Float64),
            Field::new("senkou_a".into(), DataType::Float64),
            Field::new("senkou_b".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=ichimoku_cloud_output)]
fn ichimoku_cloud(inputs: &[Series], kwargs: IchimokuCloudKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;

    let mut ic = quantwave_core::IchimokuCloud::new(kwargs.p1, kwargs.p2, kwargs.p3);
    let mut t_vals = Vec::with_capacity(high.len());
    let mut k_vals = Vec::with_capacity(high.len());
    let mut sa_vals = Vec::with_capacity(high.len());
    let mut sb_vals = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        
        if h.is_nan() || l.is_nan() {
            t_vals.push(Some(f64::NAN));
            k_vals.push(Some(f64::NAN));
            sa_vals.push(Some(f64::NAN));
            sb_vals.push(Some(f64::NAN));
        } else {
            let (t, k, sa, sb) = ic.next((h, l));
            t_vals.push(Some(t));
            k_vals.push(Some(k));
            sa_vals.push(Some(sa));
            sb_vals.push(Some(sb));
        }
    }

    let t_series = Float64Chunked::new("tenkan".into(), t_vals).into_series();
    let k_series = Float64Chunked::new("kijun".into(), k_vals).into_series();
    let sa_series = Float64Chunked::new("senkou_a".into(), sa_vals).into_series();
    let sb_series = Float64Chunked::new("senkou_b".into(), sb_vals).into_series();

    let out = StructChunked::from_series(
        "ichimoku_output".into(),
        high.len(),
        [t_series, k_series, sa_series, sb_series].iter(),
    )?;

    Ok(out.into_series())
}

// 4. mama
#[derive(Deserialize)]
struct MamaKwargs {
    fastlimit: f64,
    slowlimit: f64,
}

fn mama_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "mama_result".into(),
        DataType::Struct(vec![
            Field::new("mama".into(), DataType::Float64),
            Field::new("fama".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=mama_output)]
fn mama(inputs: &[Series], kwargs: MamaKwargs) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;
    let mut indicator = MAMA::new(kwargs.fastlimit, kwargs.slowlimit);
    let mut mama_vals = Vec::with_capacity(s.len());
    let mut fama_vals = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(f64::NAN);
        if val.is_nan() {
            mama_vals.push(Some(f64::NAN));
            fama_vals.push(Some(f64::NAN));
        } else {
            let (m, f) = indicator.next(val);
            mama_vals.push(Some(m));
            fama_vals.push(Some(f));
        }
    }

    let s_mama = Float64Chunked::new("mama".into(), mama_vals).into_series();
    let s_fama = Float64Chunked::new("fama".into(), fama_vals).into_series();

    let out = StructChunked::from_series(
        "mama_result".into(),
        s.len(),
        [s_mama, s_fama].iter(),
    )?;

    Ok(out.into_series())
}

// 5. atr_trailing_stop
#[derive(Deserialize)]
struct AtrTrailingStopKwargs {
    period: usize,
    multiplier: f64,
}

fn atr_trailing_stop_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "atr_ts_output".into(),
        DataType::Struct(vec![
            Field::new("stop".into(), DataType::Float64),
            Field::new("direction".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=atr_trailing_stop_output)]
fn atr_trailing_stop(inputs: &[Series], kwargs: AtrTrailingStopKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut atr_ts = quantwave_core::ATRTrailingStop::new(kwargs.period, kwargs.multiplier);
    let mut stops = Vec::with_capacity(high.len());
    let mut directions = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);

        if h.is_nan() || l.is_nan() || c.is_nan() {
            stops.push(Some(f64::NAN));
            directions.push(Some(f64::NAN));
        } else {
            let (stop, dir) = atr_ts.next((h, l, c));
            stops.push(Some(stop));
            directions.push(Some(dir as f64));
        }
    }

    let stop_series = Float64Chunked::new("stop".into(), stops).into_series();
    let dir_series = Float64Chunked::new("direction".into(), directions).into_series();

    let out = StructChunked::from_series(
        "atr_ts_output".into(),
        high.len(),
        [stop_series, dir_series].iter(),
    )?;

    Ok(out.into_series())
}
