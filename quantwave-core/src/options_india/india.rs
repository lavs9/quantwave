use chrono::NaiveDate;

/// NSE risk-free rate (91-day T-bill, updated quarterly).
/// Current value as of May 2026.
pub const NSE_RISK_FREE_RATE: f64 = 0.065;

/// Days to expiry from today to expiry date (calendar days).
/// NSE options expire at 15:30 IST on expiry day.
pub fn dte_calendar(expiry_date: NaiveDate, as_of: NaiveDate) -> i32 {
    let duration = expiry_date.signed_duration_since(as_of);
    duration.num_days() as i32
}

/// Time to expiry in years for Black-Scholes.
/// Uses calendar days / 365.0 as per NSE convention.
pub fn t_years(expiry_date: NaiveDate, as_of: NaiveDate) -> f64 {
    let days = dte_calendar(expiry_date, as_of);
    if days <= 0 {
        return 0.0;
    }
    days as f64 / 365.0
}

/// NSE standard lot sizes.
/// Hardcoded current values (Revised: April 2026).
pub fn nse_lot_size(symbol: &str) -> Option<u32> {
    match symbol.to_uppercase().as_str() {
        "NIFTY" => Some(50),
        "BANKNIFTY" => Some(15),
        "FINNIFTY" => Some(40),
        "MIDCPNIFTY" => Some(75),
        "SENSEX" => Some(10),
        _ => None,
    }
}

/// Moneyness classification (from Call option perspective).
/// Returns "ITM", "ATM", or "OTM".
pub fn moneyness(spot: f64, strike: f64) -> &'static str {
    let lower_bound = spot * 0.998;
    let upper_bound = spot * 1.002;

    if strike < lower_bound {
        "ITM"
    } else if strike > upper_bound {
        "OTM"
    } else {
        "ATM"
    }
}
