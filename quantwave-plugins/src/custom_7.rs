use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use quantwave_core::traits::Next;
use quantwave_core::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AdaptiveEmaKwargs {
    pub period: usize,
    pub pds: usize,
}

#[derive(Deserialize)]
pub struct RegimesTransitionMatrixKwargs {
    pub num_states: usize,
}

#[derive(Deserialize)]
pub struct VpnKwargs {
    pub period: usize,
    pub smooth_period: usize,
}

fn ms_garch_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "ms_garch_data".into(),
        DataType::Struct(vec![
            Field::new("regime".into(), DataType::UInt32),
            Field::new("estimated_vol".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=ms_garch_output)]
fn regimes_ms_garch(inputs: &[Series]) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.f64()?;
    let mut model = quantwave_core::regimes::ms_garch::MSGarch::low_high_vol();
    let mut regimes = Vec::with_capacity(s.len());
    let mut vols = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let ret = ca.get(i).unwrap_or(0.0);
        let (regime, vol) = model.next(ret);

        let r_val = match regime {
            quantwave_core::regimes::MarketRegime::Steady => 0u32,
            quantwave_core::regimes::MarketRegime::Crisis => 1,
            quantwave_core::regimes::MarketRegime::Bull => 2,
            quantwave_core::regimes::MarketRegime::Bear => 3,
            quantwave_core::regimes::MarketRegime::Cluster(c) => 4 + (c as u32),
        };
        regimes.push(r_val);
        vols.push(vol);
    }

    let s_regime = Series::new("regime".into(), regimes);
    let s_vol = Series::new("estimated_vol".into(), vols);
    let struct_series =
        StructChunked::from_series("ms_garch_data".into(), s.len(), [s_regime, s_vol].iter())?;
    Ok(struct_series.into_series())
}

fn adaptive_ema_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new("adaptive_ema".into(), DataType::Float64))
}

#[polars_expr(output_type_func=adaptive_ema_output)]
fn adaptive_ema(inputs: &[Series], kwargs: AdaptiveEmaKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut indicator = quantwave_core::AdaptiveEMA::new(kwargs.period, kwargs.pds);
    let mut values = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next((h, l, c)));
    }

    Ok(Series::new("adaptive_ema".into(), values))
}

fn regimes_transition_matrix_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "regime_transition_matrix".into(),
        DataType::List(Box::new(DataType::Float64)),
    ))
}

#[polars_expr(output_type_func=regimes_transition_matrix_output)]
fn regimes_transition_matrix(
    inputs: &[Series],
    kwargs: RegimesTransitionMatrixKwargs,
) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.u32()?;
    let states: Vec<u32> = ca.into_iter().map(|v| v.unwrap_or(0)).collect();
    let matrix = quantwave_core::RegimeAnalytics::transition_matrix(&states, kwargs.num_states);

    let mut builders = ListPrimitiveChunkedBuilder::<Float64Type>::new(
        "transition_matrix".into(),
        matrix.len(),
        matrix.len() * kwargs.num_states,
        DataType::Float64,
    );
    for row in matrix {
        builders.append_slice(&row);
    }

    let list_ca = builders.finish();
    Ok(list_ca.into_series())
}

fn vpn_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new("vpn".into(), DataType::Float64))
}

#[polars_expr(output_type_func=vpn_output)]
fn vpn(inputs: &[Series], kwargs: VpnKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;
    let volume = inputs[3].f64()?;

    let mut indicator = quantwave_core::VPNIndicator::new(kwargs.period, kwargs.smooth_period);
    let mut values = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h = high.get(i).unwrap_or(f64::NAN);
        let l = low.get(i).unwrap_or(f64::NAN);
        let c = close.get(i).unwrap_or(f64::NAN);
        let v = volume.get(i).unwrap_or(f64::NAN);
        values.push(indicator.next((h, l, c, v)));
    }

    Ok(Series::new("vpn".into(), values))
}

fn regimes_hsmm_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new("hsmm_regime".into(), DataType::UInt32))
}

#[polars_expr(output_type_func=regimes_hsmm_output)]
fn regimes_hsmm(inputs: &[Series]) -> PolarsResult<Series> {
    let s = &inputs[0];
    let ca = s.f64()?;
    // Default 2-state HSMM: Poisson durations (5 days Bull, 2 days Bear)
    let mut model = quantwave_core::regimes::hsmm::HSMM::new(
        vec![vec![0.0, 1.0], vec![1.0, 0.0]], // Always switch
        vec![0.001, -0.002],
        vec![0.01, 0.02],
        vec![
            quantwave_core::regimes::hsmm::DurationDistribution::Poisson { lambda: 5.0 },
            quantwave_core::regimes::hsmm::DurationDistribution::Poisson { lambda: 2.0 },
        ],
    );
    let mut values = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let val = ca.get(i).unwrap_or(f64::NAN);
        let regime = model.next(val);
        let out = match regime {
            quantwave_core::regimes::MarketRegime::Steady => 0u32,
            quantwave_core::regimes::MarketRegime::Crisis => 1,
            _ => 2, // Map others
        };
        values.push(out);
    }

    Ok(Series::new("hsmm_regime".into(), values))
}
