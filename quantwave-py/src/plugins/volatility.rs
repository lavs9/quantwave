use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

use quantwave_core::indicators::incremental::ta_atr::{AtrSeed, TaATR};
use quantwave_core::indicators::incremental::trange::{TaNATR, TaTRANGE};
use quantwave_core::indicators::smoothing::WarmupPolicy;
use quantwave_core::traits::Next;

#[derive(Deserialize)]
struct SinglePeriodKwargs {
    timeperiod: usize,
}

#[derive(Deserialize)]
struct AtrKwargs {
    timeperiod: usize,
    smoothing: Option<String>,
    seed: Option<String>,
    warmup: Option<String>,
}

#[polars_expr(output_type=Float64)]
fn atr(inputs: &[Series], kwargs: AtrKwargs) -> PolarsResult<Series> {
    let close = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let is_ema = kwargs
        .smoothing
        .as_deref()
        .map(|s| s.eq_ignore_ascii_case("ema"))
        .unwrap_or(false);

    let seed_str = kwargs.seed.as_deref().or_else(|| {
        kwargs.warmup.as_deref().filter(|&w| {
            matches!(
                w.to_ascii_lowercase().as_str(),
                "talib"
                    | "standard"
                    | "ta_lib"
                    | "ta-lib"
                    | "classic"
                    | "wilder1978"
                    | "wilder_1978"
                    | "wilder-1978"
                    | "wilder"
                    | "book"
            )
        })
    });

    let atr_seed = match seed_str {
        None | Some("talib") | Some("standard") | Some("ta_lib") | Some("ta-lib") => AtrSeed::TaLib,
        Some("wilder1978") | Some("wilder_1978") | Some("wilder-1978") | Some("classic")
        | Some("wilder") | Some("book") => AtrSeed::Wilder1978,
        _ => AtrSeed::TaLib,
    };

    let warmup_policy = match kwargs.warmup.as_deref() {
        Some("nan") | Some("NAN") => Some(WarmupPolicy::Nan),
        Some("partial") | Some("PARTIAL") => Some(WarmupPolicy::Partial),
        _ => None,
    };

    let out: Float64Chunked = if is_ema {
        let mut indicator = match warmup_policy {
            Some(w) => {
                quantwave_core::indicators::volatility::ATR::with_warmup(kwargs.timeperiod, w)
            }
            None => quantwave_core::indicators::volatility::ATR::new(kwargs.timeperiod),
        };
        close
            .into_iter()
            .zip(high.into_iter())
            .zip(low.into_iter())
            .map(|((c, h), l)| match (c, h, l) {
                (Some(cv), Some(hv), Some(lv)) => Some(indicator.next((hv, lv, cv))),
                _ => {
                    indicator.next((f64::NAN, f64::NAN, f64::NAN));
                    None
                }
            })
            .collect()
    } else {
        let mut indicator = TaATR::with_seed(kwargs.timeperiod, atr_seed);
        close
            .into_iter()
            .zip(high.into_iter())
            .zip(low.into_iter())
            .map(|((c, h), l)| match (c, h, l) {
                (Some(cv), Some(hv), Some(lv)) => Some(indicator.next((hv, lv, cv))),
                _ => {
                    indicator.next((f64::NAN, f64::NAN, f64::NAN));
                    None
                }
            })
            .collect()
    };

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn natr(inputs: &[Series], kwargs: SinglePeriodKwargs) -> PolarsResult<Series> {
    let close = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let mut indicator = TaNATR::new(kwargs.timeperiod);

    let out: Float64Chunked = close
        .into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .map(|((c, h), l)| match (c, h, l) {
            (Some(cv), Some(hv), Some(lv)) => Some(indicator.next((hv, lv, cv))),
            _ => {
                indicator.next((f64::NAN, f64::NAN, f64::NAN));
                None
            }
        })
        .collect();

    Ok(out.into_series())
}

#[polars_expr(output_type=Float64)]
fn trange(inputs: &[Series]) -> PolarsResult<Series> {
    let close = inputs[0].f64()?;
    let high = inputs[1].f64()?;
    let low = inputs[2].f64()?;
    let mut indicator = TaTRANGE::new();

    let out: Float64Chunked = close
        .into_iter()
        .zip(high.into_iter())
        .zip(low.into_iter())
        .map(|((c, h), l)| match (c, h, l) {
            (Some(cv), Some(hv), Some(lv)) => Some(indicator.next((hv, lv, cv))),
            _ => {
                indicator.next((f64::NAN, f64::NAN, f64::NAN));
                None
            }
        })
        .collect();

    Ok(out.into_series())
}
