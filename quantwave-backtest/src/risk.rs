//! Pluggable risk overlays applied to the per-bar *target exposure* before it
//! reaches the fill logic (quantwave-pvmr).
//!
//! Design constraint (the parity moat): every overlay here is a **pure
//! function of price history up to and including the current completed
//! bar** — no lookahead, no RNG, no wall-clock. That means the same
//! `RiskModel::apply` call, fed the same `(closes, bar_idx, price, equity)`,
//! produces bit-identical output whether it is invoked from the batch
//! DataFrame path (`simulate_dataframe`) or the streaming path
//! (`run_streaming_simulation`). Both paths funnel through the single shared
//! `run_simulation` core in `lib.rs`, and the overlay is applied at exactly
//! one point inside that core (right after the position sizer, right before
//! the desired exposure is used to open/close/flip a position) — so there is
//! only one call site to keep batch and streaming in sync.
//!
//! ## Entry-time semantics
//!
//! The engine model is **no intra-trade resizing**: a position is sized when it
//! *opens* and held (unchanged in magnitude) until it exits or flips. These
//! overlays therefore size a position **at entry**. Continuous magnitude
//! scalers (`vol_target`, `inverse_vol`) take effect on the bar a position is
//! opened; they do **not** re-scale an already-open position as trailing vol
//! drifts. `position_limit` / `pre_trade` likewise clamp at entry. (True
//! continuous rebalancing would require an intra-trade resize in the engine
//! core — deliberately out of scope; see `planning/ORDER_MODE_EXECUTION_ADR.md`
//! for the parity-preserving philosophy this follows.)
//!
//! ## Formulas / sources
//!
//! - **Trailing realized volatility**: sample stdev of close-to-close log
//!   returns over a trailing lookback window, annualized by `sqrt(bars_per_year)`.
//!   Standard realized-vol estimator, e.g. Andersen & Bollerslev (1998),
//!   "Answering the Skeptics: Yes, Standard Volatility Models Do Provide
//!   Accurate Forecasts".
//! - **Vol targeting**: scale an existing exposure so trailing realized vol
//!   matches a target annualized vol: `scale = target_vol / realized_vol`,
//!   clamped to `[min_scale, max_scale]`. See Moreira & Muir (2017),
//!   "Volatility-Managed Portfolios", J. Finance; also standard CTA/risk-parity
//!   desk practice.
//! - **Inverse-vol sizing**: size a position inversely proportional to its
//!   own trailing vol so each entry contributes roughly the same risk,
//!   `units = sign(signal) * (target_vol / realized_vol) * equity / price`.
//!   This is the single-asset special case of naive risk parity /
//!   inverse-volatility weighting (Lopez de Prado, *Advances in Financial
//!   Machine Learning*, ch. 16, HRP). For a proper multi-asset risk-parity
//!   book (correlation-aware, not just inverse-vol), reuse
//!   `quantwave_core::portfolio::risk_parity` (Spinu 2013) — that requires a
//!   cross-asset covariance matrix and a portfolio-level exposure vector,
//!   which the current single-symbol `run_simulation` core does not carry.
//!   Wiring true cross-sectional risk parity into the sim loop is left as a
//!   follow-up (needs a portfolio-level overlay hook, analogous to the
//!   existing `SharedCapital` / `PortfolioMode` machinery) rather than forced
//!   in here.
//! - **Position limit / pre-trade check**: hard caps on absolute exposure
//!   (units) and/or leverage (notional / equity). `position_limit` always
//!   clamps toward the cap; `pre_trade` can additionally veto (zero out) a
//!   position outright when configured to do so — modeling a pre-trade
//!   compliance check rather than a soft risk-budget scaler.

use serde::{Deserialize, Serialize};

/// Trailing annualized realized volatility of close-to-close log returns,
/// computed over the full `lookback`-bar window `closes[upto - lookback ..= upto]`.
///
/// Uses only `closes[..=upto]` — never any index beyond `upto` — which is
/// what keeps this a deterministic, lookahead-free function of "data
/// available up to and including the current completed bar".
///
/// Returns `None` when there isn't yet a *full* lookback window of history
/// (warm-up period), or the window is degenerate (non-positive prices, zero
/// variance). Requiring a full window (rather than an expanding partial one)
/// keeps the estimator's window size — and therefore its statistical
/// behavior — constant once warm-up completes, in both batch and streaming.
pub fn trailing_annualized_vol(
    closes: &[f64],
    upto: usize,
    lookback: usize,
    bars_per_year: f64,
) -> Option<f64> {
    if lookback < 2 || upto >= closes.len() || upto < lookback {
        return None;
    }
    let start = upto - lookback;
    let window = &closes[start..=upto];

    let rets: Vec<f64> = window
        .windows(2)
        .filter_map(|w| {
            let (p0, p1) = (w[0], w[1]);
            if p0.is_finite() && p1.is_finite() && p0 > 0.0 && p1 > 0.0 {
                Some((p1 / p0).ln())
            } else {
                None
            }
        })
        .collect();
    if rets.len() < 2 {
        return None;
    }

    let mean = rets.iter().sum::<f64>() / rets.len() as f64;
    let var = rets.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (rets.len() as f64 - 1.0);
    let daily_vol = var.sqrt();
    if !daily_vol.is_finite() || daily_vol <= 0.0 {
        return None;
    }
    Some(daily_vol * bars_per_year.sqrt())
}

/// Scale an existing target exposure toward a target annualized volatility.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolTargetConfig {
    /// Desired annualized volatility of the traded instrument's returns (e.g. 0.15 = 15%/yr).
    pub target_annual_vol: f64,
    /// Trailing window (in bars) used to estimate realized volatility.
    pub lookback: usize,
    /// Bars per year for annualization (252 daily, 252*~6.5*60 for 1m equities, etc).
    pub bars_per_year: f64,
    /// Floor on the scale factor applied to the raw exposure (e.g. 0.0).
    pub min_scale: f64,
    /// Ceiling on the scale factor applied to the raw exposure (avoid runaway leverage
    /// when realized vol is near zero).
    pub max_scale: f64,
}

impl Default for VolTargetConfig {
    fn default() -> Self {
        Self {
            target_annual_vol: 0.15,
            lookback: 20,
            bars_per_year: 252.0,
            min_scale: 0.0,
            max_scale: 5.0,
        }
    }
}

/// Size a position purely from trailing volatility (single-asset inverse-vol /
/// naive risk-parity sizing). Direction comes from the sign of the incoming
/// desired exposure; magnitude is fully re-derived from vol + equity + price.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InverseVolConfig {
    /// Target annualized volatility contribution of this position (the "risk unit").
    pub target_annual_vol: f64,
    /// Trailing window (in bars) used to estimate realized volatility.
    pub lookback: usize,
    /// Bars per year for annualization.
    pub bars_per_year: f64,
    /// Floor on implied leverage (units * price / equity).
    pub min_scale: f64,
    /// Ceiling on implied leverage (units * price / equity).
    pub max_scale: f64,
}

impl Default for InverseVolConfig {
    fn default() -> Self {
        Self {
            target_annual_vol: 0.15,
            lookback: 20,
            bars_per_year: 252.0,
            min_scale: 0.0,
            max_scale: 5.0,
        }
    }
}

/// Hard cap on absolute exposure (units) and/or leverage (notional / equity).
/// Always clamps (never vetoes) — a softer control than `PreTradeConfig`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PositionLimitConfig {
    /// Maximum absolute position size in units (e.g. shares/contracts).
    pub max_abs_exposure: Option<f64>,
    /// Maximum absolute leverage, i.e. `|exposure * price| / equity`.
    pub max_leverage: Option<f64>,
}

/// Pre-trade compliance-style check: can clamp (default) or veto outright
/// (zero the position) when a cap would be breached.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PreTradeConfig {
    /// Maximum notional (|exposure * price|) for this single name.
    pub max_notional: Option<f64>,
    /// Maximum leverage (|exposure * price| / equity) for this single name.
    pub max_leverage: Option<f64>,
    /// If true, a breach zeroes the position instead of clamping to the cap.
    pub veto_on_breach: bool,
}

/// Composite set of risk overlays applied, in order, to the per-bar target
/// exposure: `vol_target -> inverse_vol -> position_limit -> pre_trade`.
///
/// All fields are optional and independently composable. Leaving every field
/// `None` (the `BacktestConfig` default) makes `apply` an identity function,
/// so default backtests are byte-identical to pre-overlay behavior.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RiskModel {
    pub vol_target: Option<VolTargetConfig>,
    pub inverse_vol: Option<InverseVolConfig>,
    pub position_limit: Option<PositionLimitConfig>,
    pub pre_trade: Option<PreTradeConfig>,
}

impl RiskModel {
    /// Apply the configured overlays to `desired_exposure` (signed units).
    ///
    /// - `closes` must contain price history through `bar_idx` inclusive
    ///   (i.e. `closes[..=bar_idx]`); nothing beyond `bar_idx` is read.
    /// - `price` is the fill/reference price for `bar_idx` (typically `closes[bar_idx]`).
    /// - `equity` is mark-to-market equity as of `bar_idx` (cash + current position * price).
    ///
    /// This function has no side effects and no hidden state — same inputs,
    /// same output, in batch or streaming.
    pub fn apply(
        &self,
        desired_exposure: f64,
        closes: &[f64],
        bar_idx: usize,
        price: f64,
        equity: f64,
    ) -> f64 {
        let mut exposure = desired_exposure;

        if let Some(cfg) = &self.vol_target {
            exposure = apply_vol_target(exposure, cfg, closes, bar_idx);
        }
        if let Some(cfg) = &self.inverse_vol {
            exposure = apply_inverse_vol(exposure, cfg, closes, bar_idx, price, equity);
        }
        if let Some(cfg) = &self.position_limit {
            exposure = apply_position_limit(exposure, cfg, price, equity);
        }
        if let Some(cfg) = &self.pre_trade {
            exposure = apply_pre_trade(exposure, cfg, price, equity);
        }

        if exposure.is_finite() { exposure } else { 0.0 }
    }
}

fn apply_vol_target(
    desired_exposure: f64,
    cfg: &VolTargetConfig,
    closes: &[f64],
    bar_idx: usize,
) -> f64 {
    if desired_exposure == 0.0 {
        return desired_exposure;
    }
    let Some(realized_vol) =
        trailing_annualized_vol(closes, bar_idx, cfg.lookback, cfg.bars_per_year)
    else {
        // Not enough history yet: pass through unscaled rather than fabricate a scale.
        return desired_exposure;
    };
    let raw_scale = cfg.target_annual_vol / realized_vol;
    let scale = raw_scale.clamp(cfg.min_scale, cfg.max_scale);
    desired_exposure * scale
}

fn apply_inverse_vol(
    desired_exposure: f64,
    cfg: &InverseVolConfig,
    closes: &[f64],
    bar_idx: usize,
    price: f64,
    equity: f64,
) -> f64 {
    if desired_exposure == 0.0 || price <= 0.0 || !price.is_finite() {
        return desired_exposure;
    }
    let sign = desired_exposure.signum();
    let Some(realized_vol) =
        trailing_annualized_vol(closes, bar_idx, cfg.lookback, cfg.bars_per_year)
    else {
        // Not enough history yet: fall back to the incoming (unsized) exposure.
        return desired_exposure;
    };
    let raw_leverage = cfg.target_annual_vol / realized_vol;
    let leverage = raw_leverage.clamp(cfg.min_scale, cfg.max_scale);
    sign * leverage * equity / price
}

fn apply_position_limit(
    desired_exposure: f64,
    cfg: &PositionLimitConfig,
    price: f64,
    equity: f64,
) -> f64 {
    if desired_exposure == 0.0 {
        return desired_exposure;
    }
    let mut exposure = desired_exposure;

    if let Some(max_units) = cfg.max_abs_exposure
        && exposure.abs() > max_units
    {
        exposure = max_units * exposure.signum();
    }

    if let Some(max_lev) = cfg.max_leverage
        && price > 0.0
        && price.is_finite()
        && equity > 0.0
    {
        let notional = exposure.abs() * price;
        let leverage = notional / equity;
        if leverage > max_lev {
            let max_units = (max_lev * equity) / price;
            exposure = max_units * exposure.signum();
        }
    }

    exposure
}

fn apply_pre_trade(desired_exposure: f64, cfg: &PreTradeConfig, price: f64, equity: f64) -> f64 {
    if desired_exposure == 0.0 {
        return desired_exposure;
    }
    let mut breached = false;
    let mut exposure = desired_exposure;

    if let Some(max_notional) = cfg.max_notional
        && price > 0.0
        && price.is_finite()
    {
        let notional = exposure.abs() * price;
        if notional > max_notional {
            breached = true;
            exposure = (max_notional / price) * exposure.signum();
        }
    }

    if let Some(max_lev) = cfg.max_leverage
        && price > 0.0
        && price.is_finite()
        && equity > 0.0
    {
        let notional = exposure.abs() * price;
        let leverage = notional / equity;
        if leverage > max_lev {
            breached = true;
            let max_units = (max_lev * equity) / price;
            exposure = max_units * exposure.signum();
        }
    }

    if breached && cfg.veto_on_breach {
        0.0
    } else {
        exposure
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synth_closes(n: usize, start: f64, daily_ret: f64) -> Vec<f64> {
        let mut v = Vec::with_capacity(n);
        let mut p = start;
        for _ in 0..n {
            v.push(p);
            p *= 1.0 + daily_ret;
        }
        v
    }

    #[test]
    fn trailing_vol_needs_warmup() {
        let closes = vec![100.0, 101.0];
        assert_eq!(trailing_annualized_vol(&closes, 1, 20, 252.0), None);
        // bar_idx beyond slice must not panic / must be None
        assert_eq!(trailing_annualized_vol(&closes, 5, 20, 252.0), None);
    }

    #[test]
    fn trailing_vol_zero_variance_is_none() {
        let closes = vec![100.0; 30];
        assert_eq!(trailing_annualized_vol(&closes, 29, 20, 252.0), None);
    }

    #[test]
    fn trailing_vol_positive_for_moving_series() {
        let mut closes = synth_closes(30, 100.0, 0.001);
        // add a little noise so variance isn't degenerate
        for (i, c) in closes.iter_mut().enumerate() {
            *c *= 1.0 + 0.002 * if i % 2 == 0 { 1.0 } else { -1.0 };
        }
        let vol = trailing_annualized_vol(&closes, 29, 20, 252.0);
        assert!(vol.is_some());
        assert!(vol.unwrap() > 0.0);
    }

    #[test]
    fn vol_target_scales_toward_target() {
        // Build a series with a known, sizable daily vol via alternating shocks.
        let mut closes = Vec::with_capacity(40);
        let mut p = 100.0;
        for i in 0..40 {
            closes.push(p);
            let shock = if i % 2 == 0 { 0.02 } else { -0.02 };
            p *= 1.0 + shock;
        }
        let cfg = VolTargetConfig {
            target_annual_vol: 0.10,
            lookback: 20,
            bars_per_year: 252.0,
            min_scale: 0.0,
            max_scale: 10.0,
        };
        let realized =
            trailing_annualized_vol(&closes, 39, cfg.lookback, cfg.bars_per_year).unwrap();
        let out = apply_vol_target(1.0, &cfg, &closes, 39);
        let expected_scale = (cfg.target_annual_vol / realized).clamp(cfg.min_scale, cfg.max_scale);
        assert!((out - expected_scale).abs() < 1e-9);
        // Realized vol here is large (big daily shocks), so target should scale DOWN.
        assert!(out < 1.0);
    }

    #[test]
    fn vol_target_passthrough_during_warmup() {
        let closes = vec![100.0, 101.0, 99.0];
        let cfg = VolTargetConfig::default();
        let out = apply_vol_target(2.0, &cfg, &closes, 2);
        assert_eq!(out, 2.0);
    }

    #[test]
    fn vol_target_leaves_flat_exposure_flat() {
        let closes = synth_closes(30, 100.0, 0.001);
        let cfg = VolTargetConfig::default();
        assert_eq!(apply_vol_target(0.0, &cfg, &closes, 29), 0.0);
    }

    #[test]
    fn inverse_vol_sizes_by_target_over_realized() {
        let mut closes = Vec::with_capacity(40);
        let mut p = 100.0;
        for i in 0..40 {
            closes.push(p);
            let shock = if i % 2 == 0 { 0.01 } else { -0.01 };
            p *= 1.0 + shock;
        }
        let cfg = InverseVolConfig {
            target_annual_vol: 0.20,
            lookback: 20,
            bars_per_year: 252.0,
            min_scale: 0.0,
            max_scale: 10.0,
        };
        let price = closes[39];
        let equity = 100_000.0;
        let realized =
            trailing_annualized_vol(&closes, 39, cfg.lookback, cfg.bars_per_year).unwrap();
        let out = apply_inverse_vol(1.0, &cfg, &closes, 39, price, equity);
        let leverage = (cfg.target_annual_vol / realized).clamp(cfg.min_scale, cfg.max_scale);
        let expected = leverage * equity / price;
        assert!((out - expected).abs() < 1e-6);

        // Flip direction only (magnitude driven by cfg, not incoming magnitude).
        let out_small = apply_inverse_vol(0.001, &cfg, &closes, 39, price, equity);
        let out_large = apply_inverse_vol(999.0, &cfg, &closes, 39, price, equity);
        assert!((out_small - out_large).abs() < 1e-6);
    }

    #[test]
    fn position_limit_caps_abs_exposure() {
        let cfg = PositionLimitConfig {
            max_abs_exposure: Some(10.0),
            max_leverage: None,
        };
        assert_eq!(apply_position_limit(50.0, &cfg, 100.0, 100_000.0), 10.0);
        assert_eq!(apply_position_limit(-50.0, &cfg, 100.0, 100_000.0), -10.0);
        assert_eq!(apply_position_limit(5.0, &cfg, 100.0, 100_000.0), 5.0);
    }

    #[test]
    fn position_limit_caps_leverage() {
        let cfg = PositionLimitConfig {
            max_abs_exposure: None,
            max_leverage: Some(1.0),
        };
        // 2000 units * 100 price = 200,000 notional vs 100,000 equity => 2x leverage, capped to 1x.
        let out = apply_position_limit(2000.0, &cfg, 100.0, 100_000.0);
        assert!((out - 1000.0).abs() < 1e-9);
    }

    #[test]
    fn pre_trade_clamps_by_default() {
        let cfg = PreTradeConfig {
            max_notional: Some(10_000.0),
            max_leverage: None,
            veto_on_breach: false,
        };
        let out = apply_pre_trade(200.0, &cfg, 100.0, 100_000.0); // 20,000 notional
        assert!((out - 100.0).abs() < 1e-9); // clamped to 10,000 / 100
    }

    #[test]
    fn pre_trade_vetoes_when_configured() {
        let cfg = PreTradeConfig {
            max_notional: Some(10_000.0),
            max_leverage: None,
            veto_on_breach: true,
        };
        let out = apply_pre_trade(200.0, &cfg, 100.0, 100_000.0);
        assert_eq!(out, 0.0);
    }

    #[test]
    fn pre_trade_no_op_within_limits() {
        let cfg = PreTradeConfig {
            max_notional: Some(10_000.0),
            max_leverage: None,
            veto_on_breach: true,
        };
        let out = apply_pre_trade(50.0, &cfg, 100.0, 100_000.0); // 5,000 notional, within cap
        assert_eq!(out, 50.0);
    }

    #[test]
    fn default_risk_model_is_identity() {
        let model = RiskModel::default();
        let closes = synth_closes(30, 100.0, 0.001);
        for raw in [-5.0, 0.0, 3.5, 1234.0] {
            let out = model.apply(raw, &closes, 29, closes[29], 100_000.0);
            assert_eq!(out, raw, "default RiskModel must be an identity function");
        }
    }

    #[test]
    fn overlays_compose_in_documented_order() {
        // vol_target scales, then position_limit caps the scaled result.
        let mut closes = Vec::with_capacity(40);
        let mut p = 100.0;
        for i in 0..40 {
            closes.push(p);
            let shock = if i % 2 == 0 { 0.001 } else { -0.001 };
            p *= 1.0 + shock;
        }
        let model = RiskModel {
            vol_target: Some(VolTargetConfig {
                target_annual_vol: 5.0, // deliberately huge -> big scale-up
                lookback: 20,
                bars_per_year: 252.0,
                min_scale: 0.0,
                max_scale: 1000.0,
            }),
            inverse_vol: None,
            position_limit: Some(PositionLimitConfig {
                max_abs_exposure: Some(3.0),
                max_leverage: None,
            }),
            pre_trade: None,
        };
        let out = model.apply(1.0, &closes, 39, closes[39], 100_000.0);
        assert!(
            (out - 3.0).abs() < 1e-9,
            "position_limit must cap the vol_target-scaled exposure, got {out}"
        );
    }
}
