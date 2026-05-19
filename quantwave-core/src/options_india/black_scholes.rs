use super::iv_solver::{norm_cdf, norm_pdf};

pub fn bs_call_price(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return (s - k).max(0.0);
    }
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma * sigma) * t ) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();
    s * norm_cdf(d1) - k * (-r * t).exp() * norm_cdf(d2)
}

pub fn bs_put_price(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 {
        return (k - s).max(0.0);
    }
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma * sigma) * t ) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();
    k * (-r * t).exp() * norm_cdf(-d2) - s * norm_cdf(-d1)
}

pub fn bs_delta(s: f64, k: f64, r: f64, t: f64, sigma: f64, is_call: bool) -> f64 {
    if t <= 0.0 {
        return if is_call {
            if s > k { 1.0 } else { 0.0 }
        } else {
            if s < k { -1.0 } else { 0.0 }
        };
    }
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma * sigma) * t ) / (sigma * t.sqrt());
    if is_call {
        norm_cdf(d1)
    } else {
        norm_cdf(d1) - 1.0
    }
}

pub fn bs_gamma(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    if t <= 0.0 || s <= 0.0 {
        return 0.0;
    }
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma * sigma) * t ) / (sigma * t.sqrt());
    norm_pdf(d1) / (s * sigma * t.sqrt())
}

/// Theta per calendar day. Signed negative for long options (theta decay).
pub fn bs_theta(s: f64, k: f64, r: f64, t: f64, sigma: f64, is_call: bool) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma * sigma) * t ) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    let term1 = -(s * norm_pdf(d1) * sigma) / (2.0 * t.sqrt());
    let theta_annual = if is_call {
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
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma * sigma) * t ) / (sigma * t.sqrt());
    s * norm_pdf(d1) * t.sqrt()
}

pub fn bs_rho(s: f64, k: f64, r: f64, t: f64, sigma: f64, is_call: bool) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let d1 = ( (s / k).ln() + (r + 0.5 * sigma * sigma) * t ) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    if is_call {
        k * t * (-r * t).exp() * norm_cdf(d2)
    } else {
        -k * t * (-r * t).exp() * norm_cdf(-d2)
    }
}
