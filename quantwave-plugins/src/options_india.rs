//! Vectorized Polars expression plugins for Options India (Black–Scholes / IV).

use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use quantwave_core::options_india;
use serde::Deserialize;

#[derive(Deserialize)]
struct SpotCallKwargs {
    spot: f64,
    is_call: bool,
}

#[derive(Deserialize)]
struct SpotKwargs {
    spot: f64,
}

fn map4(inputs: &[Series], f: impl Fn(f64, f64, f64, f64) -> f64) -> PolarsResult<Series> {
    let a = inputs[0].f64()?;
    let b = inputs[1].f64()?;
    let c = inputs[2].f64()?;
    let d = inputs[3].f64()?;
    let out: Float64Chunked = a
        .into_iter()
        .zip(b.into_iter())
        .zip(c.into_iter())
        .zip(d.into_iter())
        .map(|(((av, bv), cv), dv)| match (av, bv, cv, dv) {
            (Some(a), Some(b), Some(c), Some(d))
                if !a.is_nan() && !b.is_nan() && !c.is_nan() && !d.is_nan() =>
            {
                Some(f(a, b, c, d))
            }
            (Some(_), Some(_), Some(_), Some(_)) => Some(f64::NAN),
            _ => None,
        })
        .collect();
    Ok(out.into_series())
}

/// inputs: strike, sigma, r, t — kwargs: spot, is_call
#[polars_expr(output_type = Float64)]
fn options_bs_delta(inputs: &[Series], kwargs: SpotCallKwargs) -> PolarsResult<Series> {
    let spot = kwargs.spot;
    let is_call = kwargs.is_call;
    map4(inputs, |k, sigma, r, t| options_india::bs_delta(spot, k, r, t, sigma, is_call))
}

/// inputs: strike, sigma, r, t — kwargs: spot
#[polars_expr(output_type = Float64)]
fn options_bs_gamma(inputs: &[Series], kwargs: SpotKwargs) -> PolarsResult<Series> {
    let spot = kwargs.spot;
    map4(inputs, |k, sigma, r, t| options_india::bs_gamma(spot, k, r, t, sigma))
}

/// inputs: strike, sigma, r, t — kwargs: spot
#[polars_expr(output_type = Float64)]
fn options_bs_vega(inputs: &[Series], kwargs: SpotKwargs) -> PolarsResult<Series> {
    let spot = kwargs.spot;
    map4(inputs, |k, sigma, r, t| options_india::bs_vega(spot, k, r, t, sigma))
}

/// inputs: strike, sigma, r, t — kwargs: spot, is_call
#[polars_expr(output_type = Float64)]
fn options_bs_theta(inputs: &[Series], kwargs: SpotCallKwargs) -> PolarsResult<Series> {
    let spot = kwargs.spot;
    let is_call = kwargs.is_call;
    map4(inputs, |k, sigma, r, t| options_india::bs_theta(spot, k, r, t, sigma, is_call))
}

/// inputs: strike, sigma, r, t — kwargs: spot, is_call
#[polars_expr(output_type = Float64)]
fn options_bs_rho(inputs: &[Series], kwargs: SpotCallKwargs) -> PolarsResult<Series> {
    let spot = kwargs.spot;
    let is_call = kwargs.is_call;
    map4(inputs, |k, sigma, r, t| options_india::bs_rho(spot, k, r, t, sigma, is_call))
}

/// inputs: strike, r, t, sigma — kwargs: spot
#[polars_expr(output_type = Float64)]
fn options_bs_call_price(inputs: &[Series], kwargs: SpotKwargs) -> PolarsResult<Series> {
    let spot = kwargs.spot;
    map4(inputs, |k, r, t, sigma| options_india::bs_call_price(spot, k, r, t, sigma))
}

/// inputs: strike, r, t, sigma — kwargs: spot
#[polars_expr(output_type = Float64)]
fn options_bs_put_price(inputs: &[Series], kwargs: SpotKwargs) -> PolarsResult<Series> {
    let spot = kwargs.spot;
    map4(inputs, |k, r, t, sigma| options_india::bs_put_price(spot, k, r, t, sigma))
}

/// inputs: market_price, strike, r, t — kwargs: spot, is_call
#[polars_expr(output_type = Float64)]
fn options_implied_vol(inputs: &[Series], kwargs: SpotCallKwargs) -> PolarsResult<Series> {
    let spot = kwargs.spot;
    let is_call = kwargs.is_call;
    map4(inputs, |price, k, r, t| {
        if t <= 0.0 {
            return f64::NAN;
        }
        let theta = if is_call { 1.0 } else { -1.0 };
        let forward = spot * (r * t).exp();
        let undiscounted_price = price * (r * t).exp();
        let iv = options_india::implied_black_volatility(undiscounted_price, forward, k, t, theta);
        if iv >= f64::MAX || iv <= -f64::MAX {
            f64::NAN
        } else {
            iv
        }
    })
}