//! India-Specific Regime Detection Helpers
//! 
//! Provides tools to adjust regime detection for the specific dynamics of the Indian 
//! markets, such as F&O expiry days and NSE holidays.

use chrono::{Datelike, NaiveDate, Weekday};

/// Detects if a given date is a standard NSE Weekly Expiry.
/// 
/// Nifty/FinNifty/Midcap typically expire on Thursdays/Tuesdays/Mondays.
/// BankNifty shifted to Wednesdays for weekly expiries.
pub fn is_likely_nse_expiry(date: NaiveDate, symbol: &str) -> bool {
    let weekday = date.weekday();
    match symbol.to_uppercase().as_str() {
        "NIFTY" => weekday == Weekday::Thu,
        "BANKNIFTY" => weekday == Weekday::Wed,
        "FINNIFTY" => weekday == Weekday::Tue,
        "MIDCPNIFTY" => weekday == Weekday::Mon,
        _ => weekday == Weekday::Thu, // Default to Thursday
    }
}

/// A filter that suppresses volatility spikes on expiry days.
/// 
/// Use this to prevent expiry-day 'pinning' or volatility expansion from 
/// triggering false regime shifts.
pub struct ExpiryVolFilter {
    symbol: String,
}

impl ExpiryVolFilter {
    pub fn new(symbol: &str) -> Self {
        Self { symbol: symbol.to_string() }
    }

    pub fn should_suppress(&self, date: NaiveDate) -> bool {
        is_likely_nse_expiry(date, &self.symbol)
    }
}
