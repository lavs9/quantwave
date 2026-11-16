//! Differentiable Exponential Moving Average (EMA).
//!
//! Provides a streaming Exponential Moving Average with analytical parameter gradients
//! with respect to both smoothing factor $\alpha$ and continuous span $\tau$.
//!
//! # Mathematical Foundation
//! In Technical Indicator Networks (TINs, Lu 2025/2026), moving averages are parameterized
//! via continuous time spans $\tau \in \mathbb{R}^+$:
//!
//! $$y_t = \alpha x_t + (1 - \alpha) y_{t-1}, \quad \alpha = \frac{2}{\tau + 1}$$
//!
//! The analytical gradient with respect to the smoothing factor $\alpha$ follows the recurrence:
//!
//! $$\frac{\partial y_t}{\partial \alpha} = (x_t - y_{t-1}) + (1 - \alpha) \frac{\partial y_{t-1}}{\partial \alpha}$$
//!
//! with base condition $y_0 = x_0, \frac{\partial y_0}{\partial \alpha} = 0$.
//!
//! Using the chain rule $\frac{\partial \alpha}{\partial \tau} = -\frac{2}{(\tau + 1)^2}$, the derivative with
//! respect to the continuous span $\tau$ is:
//!
//! $$\frac{\partial y_t}{\partial \tau} = \frac{\partial y_t}{\partial \alpha} \cdot \left(-\frac{2}{(\tau + 1)^2}\right)$$

use crate::ml::error::MlError;
use crate::traits::Next;
use serde::{Deserialize, Serialize};

/// Continuous differentiable Exponential Moving Average with analytical parameter gradients.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentiableEma {
    /// Continuous time span $\tau > 0$.
    span: f64,
    /// Previous filtered output $y_{t-1}$.
    prev_output: Option<f64>,
    /// Analytical gradient $\frac{\partial y_t}{\partial \alpha}$.
    grad_alpha: f64,
    /// Total count of bars consumed so far.
    bars_consumed: usize,
}

impl DifferentiableEma {
    /// Creates a new [`DifferentiableEma`] with continuous span $\tau > 0$.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if `span <= 0.0` or `span` is not finite.
    pub fn new(span: f64) -> Result<Self, MlError> {
        if !span.is_finite() || span <= 0.0 {
            return Err(MlError::InvalidParameter(span));
        }

        Ok(Self {
            span,
            prev_output: None,
            grad_alpha: 0.0,
            bars_consumed: 0,
        })
    }

    /// Returns the current continuous span $\tau$.
    #[inline]
    pub fn span(&self) -> f64 {
        self.span
    }

    /// Returns the smoothing factor $\alpha = \frac{2}{\tau + 1}$.
    #[inline]
    pub fn alpha(&self) -> f64 {
        2.0 / (self.span + 1.0)
    }

    /// Returns the current analytical gradient $\frac{\partial y_t}{\partial \alpha}$.
    #[inline]
    pub fn grad_alpha(&self) -> f64 {
        self.grad_alpha
    }

    /// Returns the current analytical gradient with respect to continuous span $\tau$:
    /// $$\frac{\partial y_t}{\partial \tau} = \frac{\partial y_t}{\partial \alpha} \cdot \left(-\frac{2}{(\tau + 1)^2}\right)$$
    #[inline]
    pub fn grad_span(&self) -> f64 {
        let denom = self.span + 1.0;
        self.grad_alpha * (-2.0 / (denom * denom))
    }

    /// Returns the previous output $y_{t-1}$, if any bar has been processed.
    #[inline]
    pub fn prev_output(&self) -> Option<f64> {
        self.prev_output
    }

    /// Returns the number of bars consumed since creation or last reset.
    #[inline]
    pub fn bars_consumed(&self) -> usize {
        self.bars_consumed
    }

    /// Sets the continuous span $\tau$, maintaining existing internal state.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if `span <= 0.0` or `span` is not finite.
    pub fn set_span(&mut self, span: f64) -> Result<(), MlError> {
        if !span.is_finite() || span <= 0.0 {
            return Err(MlError::InvalidParameter(span));
        }
        self.span = span;
        Ok(())
    }

    /// Updates the span $\tau$ along the negative gradient of a downstream loss:
    /// $$\tau \leftarrow \text{clamp}\left(\tau - \eta \cdot \frac{\partial L}{\partial y_t} \cdot \frac{\partial y_t}{\partial \tau}, \tau_{\min}, \tau_{\max}\right)$$
    ///
    /// # Arguments
    /// * `learning_rate` - Gradient step size $\eta$.
    /// * `loss_gradient` - Upstream derivative $\frac{\partial L}{\partial y_t}$.
    /// * `min_span` - Lower bound for span clamping (must be $> 0$).
    /// * `max_span` - Upper bound for span clamping.
    pub fn update_span(
        &mut self,
        learning_rate: f64,
        loss_gradient: f64,
        min_span: f64,
        max_span: f64,
    ) {
        let grad = loss_gradient * self.grad_span();
        let lo = if min_span.is_finite() && min_span > 0.0 {
            min_span
        } else {
            1e-4
        };
        let hi = if max_span.is_finite() && max_span >= lo {
            max_span
        } else {
            lo
        };
        self.span = (self.span - learning_rate * grad).clamp(lo, hi);
    }

    /// Resets internal accumulator states, clearing history while preserving the span parameter.
    pub fn reset(&mut self) {
        self.prev_output = None;
        self.grad_alpha = 0.0;
        self.bars_consumed = 0;
    }
}

impl Next<f64> for DifferentiableEma {
    type Output = f64;

    #[inline]
    fn next(&mut self, input: f64) -> Self::Output {
        self.bars_consumed += 1;
        match self.prev_output {
            None => {
                self.prev_output = Some(input);
                self.grad_alpha = 0.0;
                input
            }
            Some(prev) => {
                let alpha = self.alpha();
                let output = alpha * input + (1.0 - alpha) * prev;
                self.grad_alpha = (input - prev) + (1.0 - alpha) * self.grad_alpha;
                self.prev_output = Some(output);
                output
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::SmoothingAlgorithm;

    fn assert_is_smoothing_algorithm<T: SmoothingAlgorithm>() {}

    #[test]
    fn test_trait_implementation() {
        assert_is_smoothing_algorithm::<DifferentiableEma>();
    }

    #[test]
    fn test_invalid_span() {
        assert!(matches!(
            DifferentiableEma::new(0.0),
            Err(MlError::InvalidParameter(_))
        ));
        assert!(matches!(
            DifferentiableEma::new(-5.0),
            Err(MlError::InvalidParameter(_))
        ));
        assert!(matches!(
            DifferentiableEma::new(f64::NAN),
            Err(MlError::InvalidParameter(_))
        ));
        assert!(matches!(
            DifferentiableEma::new(f64::INFINITY),
            Err(MlError::InvalidParameter(_))
        ));
    }

    #[test]
    fn test_step_by_step_calculation() {
        let mut ema = DifferentiableEma::new(3.0).expect("valid span");
        // span = 3.0 => alpha = 2 / (3 + 1) = 0.5
        assert_eq!(ema.alpha(), 0.5);

        // Bar 0: input 10.0
        let y0 = ema.next(10.0);
        assert_eq!(y0, 10.0);
        assert_eq!(ema.grad_alpha(), 0.0);
        assert_eq!(ema.grad_span(), 0.0);
        assert_eq!(ema.bars_consumed(), 1);

        // Bar 1: input 14.0
        // y1 = 0.5 * 14.0 + 0.5 * 10.0 = 12.0
        // grad_alpha = (14 - 10) + 0.5 * 0 = 4.0
        let y1 = ema.next(14.0);
        assert_eq!(y1, 12.0);
        assert_eq!(ema.grad_alpha(), 4.0);
        // grad_span = grad_alpha * (-2 / (3 + 1)^2) = 4.0 * (-2 / 16) = -0.5
        assert_eq!(ema.grad_span(), -0.5);
        assert_eq!(ema.bars_consumed(), 2);

        // Bar 2: input 8.0
        // y2 = 0.5 * 8.0 + 0.5 * 12.0 = 10.0
        // grad_alpha = (8 - 12) + 0.5 * 4.0 = -4.0 + 2.0 = -2.0
        let y2 = ema.next(8.0);
        assert_eq!(y2, 10.0);
        assert_eq!(ema.grad_alpha(), -2.0);
        // grad_span = -2.0 * (-2 / 16) = 0.25
        assert_eq!(ema.grad_span(), 0.25);
    }

    #[test]
    fn test_finite_difference_grad_alpha() {
        let prices = [
            100.0, 102.5, 101.8, 104.2, 103.5, 106.0, 105.2, 108.0, 107.5, 109.1, 111.0, 108.9,
            112.5, 114.0, 113.2, 115.0, 116.8, 115.5, 118.0, 120.0,
        ];
        let span = 9.0;
        let mut ema = DifferentiableEma::new(span).expect("valid span");
        let alpha = ema.alpha(); // 0.2

        for &p in &prices {
            ema.next(p);
        }
        let analytical_grad = ema.grad_alpha();

        // Numerical finite difference: y(alpha + eps) vs y(alpha - eps)
        let eps = 1e-6;
        let eval_with_alpha = |target_alpha: f64| -> f64 {
            let mut y_prev = prices[0];
            for &p in &prices[1..] {
                y_prev = target_alpha * p + (1.0 - target_alpha) * y_prev;
            }
            y_prev
        };

        let y_plus = eval_with_alpha(alpha + eps);
        let y_minus = eval_with_alpha(alpha - eps);
        let numerical_grad = (y_plus - y_minus) / (2.0 * eps);

        let rel_diff = (analytical_grad - numerical_grad).abs() / analytical_grad.abs().max(1.0);
        assert!(
            rel_diff < 1e-5,
            "grad_alpha mismatch: analytical={analytical_grad}, numerical={numerical_grad}, rel_diff={rel_diff}"
        );
    }

    #[test]
    fn test_finite_difference_grad_span() {
        let prices = [
            50.0, 52.0, 51.0, 54.0, 53.0, 56.0, 55.0, 58.0, 57.0, 60.0, 62.0, 61.5, 63.0, 62.5,
            65.0, 67.0, 66.0, 68.5, 70.0, 69.0,
        ];
        let span = 12.0;
        let mut ema = DifferentiableEma::new(span).expect("valid span");

        for &p in &prices {
            ema.next(p);
        }
        let analytical_grad_span = ema.grad_span();

        let eps = 1e-6;
        let eval_with_span = |target_span: f64| -> f64 {
            let mut model = DifferentiableEma::new(target_span).expect("valid span");
            let mut last = 0.0;
            for &p in &prices {
                last = model.next(p);
            }
            last
        };

        let y_plus = eval_with_span(span + eps);
        let y_minus = eval_with_span(span - eps);
        let numerical_grad_span = (y_plus - y_minus) / (2.0 * eps);

        let rel_diff = (analytical_grad_span - numerical_grad_span).abs()
            / analytical_grad_span.abs().max(1.0);
        assert!(
            rel_diff < 1e-5,
            "grad_span mismatch: analytical={analytical_grad_span}, numerical={numerical_grad_span}, rel_diff={rel_diff}"
        );
    }

    #[test]
    fn test_parameter_update_convergence() {
        // Target span = 8.0. Train candidate initialized at span = 20.0 to converge towards 8.0.
        let target_span = 8.0;
        let prices: Vec<f64> = (0..80)
            .map(|i| 100.0 + 10.0 * (i as f64 * 0.1).sin() + i as f64 * 0.3)
            .collect();

        let mut target_ema = DifferentiableEma::new(target_span).expect("valid target");
        let target_outputs: Vec<f64> = prices.iter().map(|&p| target_ema.next(p)).collect();

        let mut candidate = DifferentiableEma::new(20.0).expect("valid candidate");
        let initial_error = (candidate.span() - target_span).abs();

        let lr = 8.0;
        for _epoch in 0..60 {
            candidate.reset();
            for (&p, &y_target) in prices.iter().zip(target_outputs.iter()) {
                let y = candidate.next(p);
                let loss_grad = y - y_target;
                candidate.update_span(lr, loss_grad, 2.0, 40.0);
            }
        }

        let final_error = (candidate.span() - target_span).abs();
        assert!(
            final_error < 0.25,
            "Candidate span did not converge: initial_err={initial_error}, final_err={final_error}, span={}",
            candidate.span()
        );
    }

    #[test]
    fn test_reset_and_serde() {
        let mut ema = DifferentiableEma::new(5.0).expect("valid span");
        ema.next(10.0);
        ema.next(20.0);
        ema.reset();

        assert_eq!(ema.bars_consumed(), 0);
        assert_eq!(ema.prev_output(), None);
        assert_eq!(ema.grad_alpha(), 0.0);
        assert_eq!(ema.grad_span(), 0.0);

        // Re-feeding should match a fresh instance
        let mut fresh = DifferentiableEma::new(5.0).expect("valid span");
        assert_eq!(ema.next(15.0), fresh.next(15.0));

        // Serde test
        let json = serde_json::to_string(&ema).expect("serialize");
        let deserialized: DifferentiableEma = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ema, deserialized);
    }
}
