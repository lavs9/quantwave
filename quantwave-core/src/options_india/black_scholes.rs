use super::iv_solver::{norm_cdf, norm_pdf};
use serde::{Deserialize, Serialize};

/// Type of vanilla option contract (Call or Put).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum OptionType {
    #[default]
    Call,
    Put,
}

impl OptionType {
    pub const fn is_call(self) -> bool {
        matches!(self, Self::Call)
    }

    pub const fn is_put(self) -> bool {
        matches!(self, Self::Put)
    }
}

impl From<bool> for OptionType {
    fn from(is_call: bool) -> Self {
        if is_call {
            Self::Call
        } else {
            Self::Put
        }
    }
}

impl From<OptionType> for bool {
    fn from(opt: OptionType) -> Self {
        opt.is_call()
    }
}

pub fn bs_call_price(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return (s - k).max(0.0);
    }
    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();
    s * norm_cdf(d1) - k * (-r * t).exp() * norm_cdf(d2)
}

pub fn bs_put_price(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return (k - s).max(0.0);
    }
    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();
    k * (-r * t).exp() * norm_cdf(-d2) - s * norm_cdf(-d1)
}

pub fn bs_delta(s: f64, k: f64, r: f64, t: f64, sigma: f64, option_type: OptionType) -> f64 {
    if t <= 0.0 {
        return if option_type.is_call() {
            if s > k {
                1.0
            } else {
                0.0
            }
        } else {
            if s < k {
                -1.0
            } else {
                0.0
            }
        };
    }
    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    if option_type.is_call() {
        norm_cdf(d1)
    } else {
        norm_cdf(d1) - 1.0
    }
}

pub fn bs_gamma(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 || s <= 0.0 {
        return 0.0;
    }
    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    norm_pdf(d1) / (s * sigma * t.sqrt())
}

/// Theta per calendar day. Signed negative for long options (theta decay).
pub fn bs_theta(s: f64, k: f64, r: f64, t: f64, sigma: f64, option_type: OptionType) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let term1 = -(s * norm_pdf(d1) * sigma) / (2.0 * t.sqrt());
    let theta_annual = if option_type.is_call() {
        term1 - r * k * (-r * t).exp() * norm_cdf(d2)
    } else {
        term1 + r * k * (-r * t).exp() * norm_cdf(-d2)
    };

    theta_annual / 365.0
}

pub fn bs_vega(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    s * norm_pdf(d1) * t.sqrt()
}

pub fn bs_rho(s: f64, k: f64, r: f64, t: f64, sigma: f64, option_type: OptionType) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let d1 = ((s / k).ln() + (r + 0.5 * sigma * sigma) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    if option_type.is_call() {
        k * t * (-r * t).exp() * norm_cdf(d2)
    } else {
        -k * t * (-r * t).exp() * norm_cdf(-d2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_type_conversion() {
        assert_eq!(OptionType::from(true), OptionType::Call);
        assert_eq!(OptionType::from(false), OptionType::Put);
        assert!(OptionType::Call.is_call());
        assert!(!OptionType::Call.is_put());
        assert!(OptionType::Put.is_put());
        assert!(!OptionType::Put.is_call());
        assert!(bool::from(OptionType::Call));
        assert!(!bool::from(OptionType::Put));
    }

    #[test]
    fn test_bs_greeks_with_option_type() {
        let s = 100.0;
        let k = 100.0;
        let r = 0.05;
        let t = 1.0;
        let sigma = 0.2;

        let delta_call = bs_delta(s, k, r, t, sigma, OptionType::Call);
        let delta_put = bs_delta(s, k, r, t, sigma, OptionType::Put);
        assert!(delta_call > 0.0 && delta_call < 1.0);
        assert!(delta_put < 0.0 && delta_put > -1.0);
        approx::assert_relative_eq!(delta_call - delta_put, 1.0, epsilon = 1e-10);

        let theta_call = bs_theta(s, k, r, t, sigma, OptionType::Call);
        let theta_put = bs_theta(s, k, r, t, sigma, OptionType::Put);
        assert!(theta_call < 0.0);
        assert!(theta_put < 0.0);

        let rho_call = bs_rho(s, k, r, t, sigma, OptionType::Call);
        let rho_put = bs_rho(s, k, r, t, sigma, OptionType::Put);
        assert!(rho_call > 0.0);
        assert!(rho_put < 0.0);
    }
}
