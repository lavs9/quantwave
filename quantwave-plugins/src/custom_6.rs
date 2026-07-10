use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::Bias;
use quantwave_core::KeltnerChannels;
use quantwave_core::MarketStructure;
use quantwave_core::RegimeAnalytics;
use quantwave_core::SAREXT;
use quantwave_core::regimes::MarketRegime;
use quantwave_core::regimes::hmm_gas::HMMGAS;
use quantwave_core::traits::Next;

#[derive(Deserialize)]
pub struct SarExtKwargs {
    startvalue: f64,
    offsetonreverse: f64,
    accelerationinitlong: f64,
    accelerationlong: f64,
    accelerationmaxlong: f64,
    accelerationinitshort: f64,
    accelerationshort: f64,
    accelerationmaxshort: f64,
}

#[polars_expr(output_type=Float64)]
fn sarext(inputs: &[Series], kwargs: SarExtKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;

    let mut indicator = SAREXT::new(
        kwargs.startvalue,
        kwargs.offsetonreverse,
        kwargs.accelerationinitlong,
        kwargs.accelerationlong,
        kwargs.accelerationmaxlong,
        kwargs.accelerationinitshort,
        kwargs.accelerationshort,
        kwargs.accelerationmaxshort,
    );

    let mut out_vec = Vec::with_capacity(high.len());

    for (h, l) in high.into_iter().zip(low.into_iter()) {
        match (h, l) {
            (Some(hv), Some(lv)) if !hv.is_nan() && !lv.is_nan() => {
                out_vec.push(Some(indicator.next((hv, lv))));
            }
            (Some(_), Some(_)) => out_vec.push(Some(f64::NAN)),
            _ => out_vec.push(None),
        }
    }

    let out = Float64Chunked::new("sarext".into(), out_vec);
    Ok(out.into_series())
}

#[derive(Deserialize)]
pub struct KeltnerChannelsKwargs {
    ema_period: usize,
    atr_period: usize,
    multiplier: f64,
}

pub fn keltner_channels_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "keltner_channels".into(),
        DataType::Struct(vec![
            Field::new("upper".into(), DataType::Float64),
            Field::new("middle".into(), DataType::Float64),
            Field::new("lower".into(), DataType::Float64),
        ]),
    ))
}

#[polars_expr(output_type_func=keltner_channels_output)]
fn keltner_channels(inputs: &[Series], kwargs: KeltnerChannelsKwargs) -> PolarsResult<Series> {
    let high = inputs[0].f64()?;
    let low = inputs[1].f64()?;
    let close = inputs[2].f64()?;

    let mut kc = KeltnerChannels::new(kwargs.ema_period, kwargs.atr_period, kwargs.multiplier);

    let mut uppers = Vec::with_capacity(high.len());
    let mut middles = Vec::with_capacity(high.len());
    let mut lowers = Vec::with_capacity(high.len());

    for i in 0..high.len() {
        let h_opt = high.get(i);
        let l_opt = low.get(i);
        let c_opt = close.get(i);

        match (h_opt, l_opt, c_opt) {
            (Some(hv), Some(lv), Some(cv)) => {
                let (upper, middle, lower) = kc.next((hv, lv, cv));
                uppers.push(Some(upper));
                middles.push(Some(middle));
                lowers.push(Some(lower));
            }
            _ => {
                uppers.push(None);
                middles.push(None);
                lowers.push(None);
            }
        }
    }

    let upper_series = Float64Chunked::new("upper".into(), uppers).into_series();
    let middle_series = Float64Chunked::new("middle".into(), middles).into_series();
    let lower_series = Float64Chunked::new("lower".into(), lowers).into_series();

    let out = StructChunked::from_series(
        "keltner_channels".into(),
        high.len(),
        [upper_series, middle_series, lower_series].iter(),
    )?;

    Ok(out.into_series())
}

#[derive(Deserialize)]
pub struct RegimesDurationStatsKwargs {
    num_states: usize,
}

pub fn regimes_duration_stats_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "duration_stats".into(),
        DataType::Struct(vec![
            Field::new("regime_id".into(), DataType::UInt32),
            Field::new("mean_duration".into(), DataType::Float64),
            Field::new("median_duration".into(), DataType::Float64),
            Field::new("std_duration".into(), DataType::Float64),
            Field::new("max_duration".into(), DataType::UInt32),
            Field::new("total_observations".into(), DataType::UInt32),
        ]),
    ))
}

#[polars_expr(output_type_func=regimes_duration_stats_output)]
fn regimes_duration_stats(
    inputs: &[Series],
    kwargs: RegimesDurationStatsKwargs,
) -> PolarsResult<Series> {
    let s = inputs[0].u32()?;
    let states: Vec<u32> = s.into_iter().map(|v| v.unwrap_or(0)).collect();
    let stats = RegimeAnalytics::duration_stats(&states, kwargs.num_states);

    let mut regime_ids = Vec::with_capacity(stats.len());
    let mut means = Vec::with_capacity(stats.len());
    let mut medians = Vec::with_capacity(stats.len());
    let mut stds = Vec::with_capacity(stats.len());
    let mut maxes = Vec::with_capacity(stats.len());
    let mut totals = Vec::with_capacity(stats.len());

    for stat in stats {
        regime_ids.push(Some(stat.regime_id));
        means.push(Some(stat.mean_duration));
        medians.push(Some(stat.median_duration));
        stds.push(Some(stat.std_duration));
        maxes.push(Some(stat.max_duration as u32));
        totals.push(Some(stat.total_observations as u32));
    }

    let s_id = UInt32Chunked::new("regime_id".into(), regime_ids).into_series();
    let s_mean = Float64Chunked::new("mean_duration".into(), means).into_series();
    let s_median = Float64Chunked::new("median_duration".into(), medians).into_series();
    let s_std = Float64Chunked::new("std_duration".into(), stds).into_series();
    let s_max = UInt32Chunked::new("max_duration".into(), maxes).into_series();
    let s_total = UInt32Chunked::new("total_observations".into(), totals).into_series();

    let out = StructChunked::from_series(
        "duration_stats".into(),
        s_id.len(),
        [s_id, s_mean, s_median, s_std, s_max, s_total].iter(),
    )?;

    Ok(out.into_series())
}

#[derive(Deserialize)]
pub struct MarketStructureKwargs {
    swing_strength: usize,
}

pub fn market_structure_output(_: &[Field]) -> PolarsResult<Field> {
    Ok(Field::new(
        "market_structure_result".into(),
        DataType::Struct(vec![
            Field::new("bias".into(), DataType::UInt32),
            Field::new("last_high_price".into(), DataType::Float64),
            Field::new("last_high_bar".into(), DataType::UInt64),
            Field::new("last_low_price".into(), DataType::Float64),
            Field::new("last_low_bar".into(), DataType::UInt64),
            Field::new("has_flip".into(), DataType::Boolean),
            Field::new("flip_bearish".into(), DataType::Boolean),
            Field::new("flip_price".into(), DataType::Float64),
            Field::new("flip_bar".into(), DataType::UInt64),
            Field::new("flip_strength".into(), DataType::UInt32),
            Field::new("swing_depth".into(), DataType::UInt32),
            Field::new("bar_index".into(), DataType::UInt64),
        ]),
    ))
}

#[polars_expr(output_type_func=market_structure_output)]
fn market_structure(inputs: &[Series], kwargs: MarketStructureKwargs) -> PolarsResult<Series> {
    let highs = inputs[0].f64()?;
    let lows = inputs[1].f64()?;

    let mut ms = MarketStructure::new(kwargs.swing_strength);
    let n = highs.len();

    let mut bias_vals = Vec::with_capacity(n);
    let mut lh_p = Vec::with_capacity(n);
    let mut lh_b = Vec::with_capacity(n);
    let mut ll_p = Vec::with_capacity(n);
    let mut ll_b = Vec::with_capacity(n);
    let mut has_f = Vec::with_capacity(n);
    let mut f_bear = Vec::with_capacity(n);
    let mut f_p = Vec::with_capacity(n);
    let mut f_ba = Vec::with_capacity(n);
    let mut f_str = Vec::with_capacity(n);
    let mut depths = Vec::with_capacity(n);
    let mut bars = Vec::with_capacity(n);

    for i in 0..n {
        let h = highs.get(i).unwrap_or(f64::NAN);
        let l = lows.get(i).unwrap_or(f64::NAN);

        let hh = if h.is_nan() || l.is_nan() {
            f64::NAN
        } else {
            h.max(l)
        };
        let ll = if h.is_nan() || l.is_nan() {
            f64::NAN
        } else {
            l.min(h)
        };

        let state = ms.next((hh, ll));

        let b = match state.bias {
            Bias::Neutral => 0u32,
            Bias::Bullish => 1,
            Bias::Bearish => 2,
        };
        bias_vals.push(Some(b));

        match &state.last_swing_high {
            Some(sh) => {
                lh_p.push(Some(sh.price));
                lh_b.push(Some(sh.bar as u64));
            }
            None => {
                lh_p.push(Some(f64::NAN));
                lh_b.push(Some(0));
            }
        }
        match &state.last_swing_low {
            Some(sl) => {
                ll_p.push(Some(sl.price));
                ll_b.push(Some(sl.bar as u64));
            }
            None => {
                ll_p.push(Some(f64::NAN));
                ll_b.push(Some(0));
            }
        }

        if let Some(f) = &state.current_flip {
            has_f.push(Some(true));
            f_bear.push(Some(f.is_bearish));
            f_p.push(Some(f.price));
            f_ba.push(Some(f.bar as u64));
            f_str.push(Some(f.structure_strength));
        } else {
            has_f.push(Some(false));
            f_bear.push(Some(false));
            f_p.push(Some(f64::NAN));
            f_ba.push(Some(0));
            f_str.push(Some(0));
        }

        depths.push(Some(state.swing_depth_used as u32));
        bars.push(Some(state.bar_index as u64));
    }

    let s_bias = UInt32Chunked::new("bias".into(), bias_vals).into_series();
    let s_lhp = Float64Chunked::new("last_high_price".into(), lh_p).into_series();
    let s_lhb = UInt64Chunked::new("last_high_bar".into(), lh_b).into_series();
    let s_llp = Float64Chunked::new("last_low_price".into(), ll_p).into_series();
    let s_llb = UInt64Chunked::new("last_low_bar".into(), ll_b).into_series();
    let s_hasf = BooleanChunked::new("has_flip".into(), has_f).into_series();
    let s_fb = BooleanChunked::new("flip_bearish".into(), f_bear).into_series();
    let s_fp = Float64Chunked::new("flip_price".into(), f_p).into_series();
    let s_fba = UInt64Chunked::new("flip_bar".into(), f_ba).into_series();
    let s_fstr = UInt32Chunked::new("flip_strength".into(), f_str).into_series();
    let s_dep = UInt32Chunked::new("swing_depth".into(), depths).into_series();
    let s_bar = UInt64Chunked::new("bar_index".into(), bars).into_series();

    let out = StructChunked::from_series(
        "market_structure_result".into(),
        n,
        [
            s_bias, s_lhp, s_lhb, s_llp, s_llb, s_hasf, s_fb, s_fp, s_fba, s_fstr, s_dep, s_bar,
        ]
        .iter(),
    )?;

    Ok(out.into_series())
}

#[polars_expr(output_type=UInt32)]
fn regimes_hmm_gas(inputs: &[Series]) -> PolarsResult<Series> {
    let s = inputs[0].f64()?;

    let mut model = HMMGAS::new(
        [0.1, 0.05, 0.9], // p11 params
        [0.1, 0.05, 0.9], // p22 params
        [0.001, -0.002],
        [0.01, 0.02],
    );

    let mut values = Vec::with_capacity(s.len());

    for i in 0..s.len() {
        let val = s.get(i).unwrap_or(f64::NAN);
        let regime = model.next(val);
        let out = match regime {
            MarketRegime::Steady => 0u32,
            MarketRegime::Crisis => 1,
            _ => 2,
        };
        values.push(Some(out));
    }

    let out = UInt32Chunked::new("hmm_gas_regime".into(), values);
    Ok(out.into_series())
}
