use chrono::{Datelike, NaiveDate};

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

/// NSE standard lot sizes, optionally date-aware.
/// Supports historical revisions (NIFTY: 75 -> 50 -> 25 -> 75 -> 65 effective Nov 2025).
/// When `as_of` is None, returns the current active contract lot size (65 for NIFTY).
pub fn nse_lot_size_dated(symbol: &str, as_of: Option<NaiveDate>) -> Option<u32> {
    match symbol.to_uppercase().as_str() {
        "NIFTY" => {
            if let Some(date) = as_of {
                let d = (date.year(), date.month(), date.day());
                if d < (2021, 7, 1) {
                    Some(75)
                } else if d < (2024, 4, 26) {
                    Some(50)
                } else if d < (2024, 11, 21) {
                    Some(25)
                } else if d < (2025, 11, 26) {
                    Some(75)
                } else {
                    Some(65)
                }
            } else {
                Some(65)
            }
        }
        "BANKNIFTY" => {
            if let Some(date) = as_of {
                let d = (date.year(), date.month(), date.day());
                if d < (2023, 7, 1) {
                    Some(25)
                } else {
                    Some(15)
                }
            } else {
                Some(15)
            }
        }
        "FINNIFTY" => Some(40),
        "MIDCPNIFTY" => Some(75),
        "SENSEX" => Some(10),
        _ => None,
    }
}

/// NSE standard lot sizes for current contract specifications.
pub fn nse_lot_size(symbol: &str) -> Option<u32> {
    nse_lot_size_dated(symbol, None)
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
