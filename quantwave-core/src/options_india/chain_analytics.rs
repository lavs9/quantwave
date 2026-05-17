use serde::{Serialize, Deserialize};

/// Max Pain strike calculation.
/// Returns the strike price where the total loss to option buyers is minimized.
pub fn max_pain(
    strikes: &[f64],
    ce_oi: &[u64],
    pe_oi: &[u64],
    lot_size: u32,
) -> f64 {
    if strikes.is_empty() {
        return 0.0;
    }

    let mut min_pain = f64::MAX;
    let mut max_pain_strike = strikes[0];

    for &expiry_price in strikes {
        let mut total_pain = 0.0;
        for i in 0..strikes.len() {
            let strike = strikes[i];
            // CE pain: buyer loses if expiry_price > strike
            if expiry_price > strike {
                total_pain += (expiry_price - strike) * ce_oi[i] as f64 * lot_size as f64;
            }
            // PE pain: buyer loses if expiry_price < strike
            if expiry_price < strike {
                total_pain += (strike - expiry_price) * pe_oi[i] as f64 * lot_size as f64;
            }
        }

        if total_pain < min_pain {
            min_pain = total_pain;
            max_pain_strike = expiry_price;
        }
    }

    max_pain_strike
}

/// Per-strike Put-Call Ratio (PCR).
/// Returns Vec of PE_OI / CE_OI for each strike.
pub fn strike_pcr(ce_oi: &[u64], pe_oi: &[u64]) -> Vec<f64> {
    ce_oi.iter().zip(pe_oi.iter()).map(|(&ce, &pe)| {
        if ce == 0 {
            0.0
        } else {
            pe as f64 / ce as f64
        }
    }).collect()
}

/// Chain-level Put-Call Ratio (PCR).
/// Returns total PE_OI / total CE_OI.
pub fn chain_pcr(ce_oi: &[u64], pe_oi: &[u64]) -> f64 {
    let total_ce: u64 = ce_oi.iter().sum();
    let total_pe: u64 = pe_oi.iter().sum();
    if total_ce == 0 {
        0.0
    } else {
        total_pe as f64 / total_ce as f64
    }
}

/// OI concentration zones (Support and Resistance).
#[derive(Debug, Serialize, Deserialize)]
pub struct OIZones {
    pub resistance_strikes: Vec<f64>,  // Top N strikes by CE OI
    pub support_strikes: Vec<f64>,     // Top N strikes by PE OI
}

pub fn oi_zones(strikes: &[f64], ce_oi: &[u64], pe_oi: &[u64], n: usize) -> OIZones {
    let mut ce_pairs: Vec<(f64, u64)> = strikes.iter().cloned().zip(ce_oi.iter().cloned()).collect();
    let mut pe_pairs: Vec<(f64, u64)> = strikes.iter().cloned().zip(pe_oi.iter().cloned()).collect();

    ce_pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pe_pairs.sort_by(|a, b| b.1.cmp(&a.1));

    OIZones {
        resistance_strikes: ce_pairs.iter().take(n).map(|p| p.0).collect(),
        support_strikes: pe_pairs.iter().take(n).map(|p| p.0).collect(),
    }
}

/// Gamma Exposure (GEX) per strike.
/// Returns a Vec of (ce_gex, pe_gex, net_gex) per strike.
/// CE GEX = OI * Gamma * Spot * LotSize * 0.01
/// PE GEX = OI * Gamma * Spot * LotSize * -0.01
pub fn gex_per_strike(
    spot: f64,
    strikes: &[f64],
    ce_gamma: &[f64],
    pe_gamma: &[f64],
    ce_oi: &[u64],
    pe_oi: &[u64],
    lot_size: u32,
) -> Vec<(f64, f64, f64)> {
    let mut result = Vec::with_capacity(strikes.len());
    for i in 0..strikes.len() {
        let ce_g = ce_oi[i] as f64 * ce_gamma[i] * spot * lot_size as f64 * 0.01;
        let pe_g = pe_oi[i] as f64 * pe_gamma[i] * spot * lot_size as f64 * -0.01;
        result.push((ce_g, pe_g, ce_g + pe_g));
    }
    result
}

/// GEX Flip Strike.
/// Returns the strike price where the cumulative Net GEX changes sign.
pub fn gex_flip_strike(strikes: &[f64], net_gex: &[f64]) -> Option<f64> {
    if strikes.len() < 2 {
        return None;
    }
    
    for i in 0..net_gex.len() - 1 {
        if (net_gex[i] < 0.0 && net_gex[i+1] > 0.0) || (net_gex[i] > 0.0 && net_gex[i+1] < 0.0) {
            // Linear interpolation for more precision if needed, but returning nearest strike for now
            return Some(strikes[i]);
        }
    }
    None
}

/// ATM Straddle analytics.
/// Returns (atm_strike, straddle_premium, implied_move_pct).
pub fn atm_straddle(
    spot: f64,
    strikes: &[f64],
    ce_ltp: &[f64],
    pe_ltp: &[f64],
) -> (f64, f64, f64) {
    if strikes.is_empty() {
        return (0.0, 0.0, 0.0);
    }

    let mut closest_idx = 0;
    let mut min_diff = f64::MAX;

    for i in 0..strikes.len() {
        let diff = (strikes[i] - spot).abs();
        if diff < min_diff {
            min_diff = diff;
            closest_idx = i;
        }
    }

    let atm_strike = strikes[closest_idx];
    let straddle_premium = ce_ltp[closest_idx] + pe_ltp[closest_idx];
    let implied_move_pct = (straddle_premium / spot) * 100.0;

    (atm_strike, straddle_premium, implied_move_pct)
}

/// Synthetic Futures calculation per strike.
/// Future Price = CE_LTP - PE_LTP + Strike
pub fn synthetic_futures(
    strikes: &[f64],
    ce_ltp: &[f64],
    pe_ltp: &[f64],
) -> Vec<f64> {
    strikes.iter().enumerate().map(|(i, &k)| {
        ce_ltp[i] - pe_ltp[i] + k
    }).collect()
}
