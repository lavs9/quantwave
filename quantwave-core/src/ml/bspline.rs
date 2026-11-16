//! B-spline basis evaluation using the Cox-de Boor recursion formula.
//!
//! Provides numerically stable, uniform knot B-spline basis functions for
//! Kolmogorov-Arnold Networks (KANs).
//!
//! # Mathematical Foundation
//! - de Boor, C. (1972). *On calculating with B-splines*. Journal of Approximation Theory, 6(1), 50-62.
//! - Liu, Z. et al. (2024). *KAN: Kolmogorov-Arnold Networks*. arXiv:2404.19756.

use crate::ml::error::MlError;
use serde::{Deserialize, Serialize};

/// Uniform B-spline basis generator on a compact 1D interval `[x_min, x_max]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BSplineBasis {
    /// Number of grid intervals $G \ge 1$.
    num_intervals: usize,
    /// Polynomial degree $k \ge 0$ (default cubic: $k = 3$).
    degree: usize,
    /// Minimum domain bound.
    x_min: f64,
    /// Maximum domain bound.
    x_max: f64,
    /// Knot vector $t_0, \dots, t_{m}$ where $m = G + 2k$.
    knots: Vec<f64>,
}

impl BSplineBasis {
    /// Creates a new uniform B-spline basis on `[x_min, x_max]`.
    ///
    /// # Errors
    /// Returns `MlError::InvalidGridIntervals` if `num_intervals == 0`.
    /// Returns `MlError::InvalidSplineDegree` if `degree > 10`.
    /// Returns `MlError::InvalidGridRange` if `x_min >= x_max` or values are non-finite.
    pub fn new(
        num_intervals: usize,
        degree: usize,
        x_min: f64,
        x_max: f64,
    ) -> Result<Self, MlError> {
        if num_intervals == 0 {
            return Err(MlError::InvalidGridIntervals(0));
        }
        if degree > 10 {
            return Err(MlError::InvalidSplineDegree(degree));
        }
        if !x_min.is_finite() || !x_max.is_finite() || x_min >= x_max {
            return Err(MlError::InvalidGridRange(x_min, x_max));
        }

        let num_knots = num_intervals + 2 * degree + 1;
        let step = (x_max - x_min) / (num_intervals as f64);

        let mut knots = Vec::with_capacity(num_knots);
        for i in 0..num_knots {
            let knot = x_min + ((i as isize - degree as isize) as f64) * step;
            knots.push(knot);
        }

        Ok(Self {
            num_intervals,
            degree,
            x_min,
            x_max,
            knots,
        })
    }

    /// Number of basis functions $N = G + k$.
    #[inline]
    #[must_use]
    pub fn num_basis(&self) -> usize {
        self.num_intervals + self.degree
    }

    /// Polynomial degree $k$.
    #[inline]
    #[must_use]
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Lower domain bound.
    #[inline]
    #[must_use]
    pub fn x_min(&self) -> f64 {
        self.x_min
    }

    /// Upper domain bound.
    #[inline]
    #[must_use]
    pub fn x_max(&self) -> f64 {
        self.x_max
    }

    /// Evaluates all $N$ basis functions at point `x` without heap allocation.
    ///
    /// `out` must have length at least `self.num_basis()`.
    ///
    /// Values outside `[x_min, x_max]` are clamped to the boundary to ensure
    /// graceful extrapolation without NaN injection.
    pub fn eval_into(&self, x: f64, out: &mut [f64]) {
        let num_basis = self.num_basis();
        assert!(
            out.len() >= num_basis,
            "output buffer too small: expected {}, got {}",
            num_basis,
            out.len()
        );

        // Clamp x to domain bounds to handle out-of-distribution values safely
        let clamped_x = x.clamp(self.x_min, self.x_max);

        // Total number of intervals at degree 0 is num_basis + degree = knots.len() - 1
        let total_deg0 = num_basis + self.degree;
        let mut stack_table = [0.0; 64];
        let mut heap_table = if total_deg0 > 64 {
            vec![0.0; total_deg0]
        } else {
            Vec::new()
        };
        let table: &mut [f64] = if total_deg0 <= 64 {
            &mut stack_table[..total_deg0]
        } else {
            &mut heap_table[..total_deg0]
        };

        // Degree 0 initialization (step functions on knot intervals)
        for i in 0..total_deg0 {
            let t_i = self.knots[i];
            let t_next = self.knots[i + 1];
            // Last interval includes right boundary
            let in_interval = if i == total_deg0 - 1 {
                clamped_x >= t_i && clamped_x <= t_next
            } else {
                clamped_x >= t_i && clamped_x < t_next
            };
            table[i] = if in_interval { 1.0 } else { 0.0 };
        }

        // Higher-degree Cox-de Boor recursion
        // At degree p, table size shrinks from (total_deg0 - p + 1) to (total_deg0 - p)
        for p in 1..=self.degree {
            for i in 0..(total_deg0 - p) {
                let denom1 = self.knots[i + p] - self.knots[i];
                let term1 = if denom1 > 0.0 {
                    (clamped_x - self.knots[i]) / denom1 * table[i]
                } else {
                    0.0
                };

                let denom2 = self.knots[i + p + 1] - self.knots[i + 1];
                let term2 = if denom2 > 0.0 {
                    (self.knots[i + p + 1] - clamped_x) / denom2 * table[i + 1]
                } else {
                    0.0
                };

                table[i] = term1 + term2;
            }
        }

        // After completing degree k, exactly num_basis elements remain at table[0..num_basis]
        out[..num_basis].copy_from_slice(&table[..num_basis]);
    }

    /// Evaluates all basis functions at `x`, allocating a `Vec<f64>`.
    #[must_use]
    pub fn eval(&self, x: f64) -> Vec<f64> {
        let mut out = vec![0.0; self.num_basis()];
        self.eval_into(x, &mut out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_parameters() {
        assert!(BSplineBasis::new(0, 3, -1.0, 1.0).is_err());
        assert!(BSplineBasis::new(5, 11, -1.0, 1.0).is_err());
        assert!(BSplineBasis::new(5, 3, 1.0, -1.0).is_err());
        assert!(BSplineBasis::new(5, 3, f64::NAN, 1.0).is_err());
    }

    #[test]
    fn test_partition_of_unity() {
        let basis = BSplineBasis::new(5, 3, -2.0, 2.0).expect("valid basis");
        assert_eq!(basis.num_basis(), 8);

        // Test at various interior and boundary points
        let test_points = [-2.0, -1.5, -0.7, 0.0, 0.45, 1.2, 1.99, 2.0];
        let mut out = vec![0.0; basis.num_basis()];

        for &x in &test_points {
            basis.eval_into(x, &mut out);
            let sum: f64 = out.iter().sum();
            assert!(
                (sum - 1.0).abs() < 1e-10,
                "Partition of unity violated at x={x}: sum={sum}"
            );
            for &val in &out {
                assert!(
                    val >= -1e-12,
                    "Negative basis function value at x={x}: {val}"
                );
            }
        }
    }

    #[test]
    fn test_clamping_behavior() {
        let basis = BSplineBasis::new(5, 3, -1.0, 1.0).expect("valid basis");
        let eval_min = basis.eval(-1.0);
        let eval_below = basis.eval(-50.0);
        assert_eq!(eval_min, eval_below);

        let eval_max = basis.eval(1.0);
        let eval_above = basis.eval(100.0);
        assert_eq!(eval_max, eval_above);
    }
}
