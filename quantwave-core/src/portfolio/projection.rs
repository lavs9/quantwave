//! Constraint projection: capped-simplex projection for box + budget=1.

use super::{Bounds, PortfolioError, PortfolioResult};
use nalgebra::DVector;

/// Project raw weights onto the box-constrained, budget=1 feasible set.
///
/// Computes the Euclidean projection of `w` onto
/// `{x : lo <= x <= hi, sum(x) = 1}` via bisection on a scalar shift
/// `lambda`, using the standard capped-simplex projection identity
/// `x_i = clip(w_i - lambda, lo_i, hi_i)`. `sum(clip(w - lambda, lo, hi))`
/// is monotonically non-increasing in `lambda`, so bisection converges to
/// the unique `lambda` with `sum(x) = 1` whenever
/// `sum(lo) <= 1 <= sum(hi)` (validated below). This guarantees bounds are
/// respected *exactly* (unlike clip-then-renormalize, which can push
/// values back out of bounds).
///
/// Direct port of `_apply_bounds_and_budget` in the Python v1
/// (`quantwave/portfolio.py`).
pub fn apply_bounds_and_budget(
    w: &DVector<f64>,
    bounds: Option<&[Bounds]>,
    long_only: bool,
) -> PortfolioResult<DVector<f64>> {
    let n = w.len();

    let (lo, hi): (DVector<f64>, DVector<f64>) = match bounds {
        None => {
            let lo = if long_only {
                DVector::zeros(n)
            } else {
                DVector::from_element(n, f64::NEG_INFINITY)
            };
            let hi = DVector::from_element(n, 1.0);
            (lo, hi)
        }
        Some(b) => {
            if b.len() != n {
                return Err(PortfolioError::BoundsLengthMismatch(b.len(), n));
            }
            let lo = DVector::from_iterator(n, b.iter().map(|x| x.0));
            let hi = DVector::from_iterator(n, b.iter().map(|x| x.1));
            if lo.iter().zip(hi.iter()).any(|(l, h)| l > h) {
                return Err(PortfolioError::BoundsLowGreaterThanHigh);
            }
            (lo, hi)
        }
    };

    if hi.sum() < 1.0 - 1e-9 {
        return Err(PortfolioError::BoundsUpperSumTooLow);
    }
    let finite_lo_sum: f64 = lo.iter().filter(|v| v.is_finite()).sum();
    if finite_lo_sum > 1.0 + 1e-9 {
        return Err(PortfolioError::BoundsLowerSumTooHigh);
    }

    let lo_b: DVector<f64> =
        DVector::from_iterator(n, lo.iter().map(|v| if v.is_finite() { *v } else { -1e9 }));
    let hi_b: DVector<f64> =
        DVector::from_iterator(n, hi.iter().map(|v| if v.is_finite() { *v } else { 1e9 }));

    let excess = |lam: f64| -> f64 {
        let sum: f64 = (0..n).map(|i| (w[i] - lam).clamp(lo_b[i], hi_b[i])).sum();
        sum - 1.0
    };

    let mut lam_lo = -1e9_f64;
    let mut lam_hi = 1e9_f64;
    for _ in 0..200 {
        let mid = (lam_lo + lam_hi) / 2.0;
        if excess(mid) > 0.0 {
            lam_lo = mid;
        } else {
            lam_hi = mid;
        }
        if lam_hi - lam_lo < 1e-14 {
            break;
        }
    }
    let lam = (lam_lo + lam_hi) / 2.0;

    let mut w_final =
        DVector::from_iterator(n, (0..n).map(|i| (w[i] - lam).clamp(lo_b[i], hi_b[i])));
    let total = w_final.sum();
    if !w_final.iter().all(|v| v.is_finite()) || (total - 1.0).abs() > 1e-6 {
        return Err(PortfolioError::ProjectionFailed);
    }
    // Nudge for the last bit of float error so sum is exactly 1.
    w_final /= total;
    Ok(w_final)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unconstrained_long_only_sums_to_one() {
        let w = DVector::from_vec(vec![0.5, 0.3, 0.2]);
        let out = apply_bounds_and_budget(&w, None, true).unwrap();
        assert!((out.sum() - 1.0).abs() < 1e-9);
        assert!(out.iter().all(|v| *v >= 0.0));
    }

    #[test]
    fn negative_weight_clipped_to_zero_and_redistributed() {
        let w = DVector::from_vec(vec![-0.1, 0.6, 0.5]);
        let out = apply_bounds_and_budget(&w, None, true).unwrap();
        assert!((out.sum() - 1.0).abs() < 1e-9);
        assert!(out.iter().all(|v| *v >= -1e-9));
    }

    #[test]
    fn infeasible_upper_bounds_error() {
        let w = DVector::from_vec(vec![0.5, 0.5]);
        let bounds = vec![(0.0, 0.3), (0.0, 0.3)];
        let err = apply_bounds_and_budget(&w, Some(&bounds), true).unwrap_err();
        assert_eq!(err, PortfolioError::BoundsUpperSumTooLow);
    }
}
