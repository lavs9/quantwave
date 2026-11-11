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

/// Black-76 European call price on forward / futures price F.
pub fn black76_call_price(f: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return (f - k).max(0.0) * (-r * t).exp();
    }
    let d1 = ((f / k).ln() + 0.5 * sigma * sigma * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();
    (-r * t).exp() * (f * norm_cdf(d1) - k * norm_cdf(d2))
}

/// Black-76 European put price on forward / futures price F.
pub fn black76_put_price(f: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return (k - f).max(0.0) * (-r * t).exp();
    }
    let d1 = ((f / k).ln() + 0.5 * sigma * sigma * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();
    (-r * t).exp() * (k * norm_cdf(-d2) - f * norm_cdf(-d1))
}

/// Black-76 Delta w.r.t forward price F.
pub fn black76_delta(f: f64, k: f64, r: f64, t: f64, sigma: f64, option_type: OptionType) -> f64 {
    if t <= 0.0 {
        return if option_type.is_call() {
            if f > k { (-r * t).exp() } else { 0.0 }
        } else if f < k {
            -(-r * t).exp()
        } else {
            0.0
        };
    }
    let d1 = ((f / k).ln() + 0.5 * sigma * sigma * t) / (sigma * t.sqrt());
    let df = (-r * t).exp();
    if option_type.is_call() {
        df * norm_cdf(d1)
    } else {
        df * (norm_cdf(d1) - 1.0)
    }
}

/// Black-76 Gamma w.r.t forward price F.
pub fn black76_gamma(f: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 || f <= 0.0 {
        return 0.0;
    }
    let d1 = ((f / k).ln() + 0.5 * sigma * sigma * t) / (sigma * t.sqrt());
    (-r * t).exp() * norm_pdf(d1) / (f * sigma * t.sqrt())
}

/// Black-76 Vega w.r.t volatility sigma.
pub fn black76_vega(f: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let d1 = ((f / k).ln() + 0.5 * sigma * sigma * t) / (sigma * t.sqrt());
    (-r * t).exp() * f * norm_pdf(d1) * t.sqrt()
}

/// Black-76 Theta per calendar day. Signed negative for long options.
pub fn black76_theta(f: f64, k: f64, r: f64, t: f64, sigma: f64, option_type: OptionType) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let d1 = ((f / k).ln() + 0.5 * sigma * sigma * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();
    let df = (-r * t).exp();
    let term1 = -(df * f * norm_pdf(d1) * sigma) / (2.0 * t.sqrt());
    let theta_annual = if option_type.is_call() {
        term1 - r * df * (f * norm_cdf(d1) - k * norm_cdf(d2))
    } else {
        term1 - r * df * (k * norm_cdf(-d2) - f * norm_cdf(-d1))
    };
    theta_annual / 365.0
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

    #[test]
    fn test_black76_parity_with_spot_black_scholes() {
        let f: f64 = 8000.0;
        let k: f64 = 7500.0;
        let r: f64 = 0.065;
        let t: f64 = 1.2 / 365.0;
        let sigma: f64 = 0.15;

        // Spot s = F * exp(-r * t)
        let s = f * (-r * t).exp();

        let c_bs = bs_call_price(s, k, r, t, sigma);
        let c_76 = black76_call_price(f, k, r, t, sigma);
        approx::assert_relative_eq!(c_bs, c_76, epsilon = 1e-11);

        let p_bs = bs_put_price(s, k, r, t, sigma);
        let p_76 = black76_put_price(f, k, r, t, sigma);
        approx::assert_relative_eq!(p_bs, p_76, epsilon = 1e-11);

        let v_bs = bs_vega(s, k, r, t, sigma);
        let v_76 = black76_vega(f, k, r, t, sigma);
        approx::assert_relative_eq!(v_bs, v_76, epsilon = 1e-11);
    }
}
