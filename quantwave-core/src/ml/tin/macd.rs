//! Differentiable Moving Average Convergence Divergence (MACD).
//!
//! Provides an end-to-end differentiable Moving Average Convergence Divergence indicator
//! parameterized by continuous fast, slow, and signal spans, with analytical parameter gradients.
//!
//! # Mathematical Foundation
//! In Technical Indicator Networks (TINs, Lu 2025/2026), the standard MACD oscillator is parameterized
//! by continuous positive spans $\tau_{\text{fast}}, \tau_{\text{slow}}, \tau_{\text{signal}}$:
//!
//! 1. Fast and slow moving averages of price $x_t$:
//!    $$y_t^{\text{fast}} = \text{EMA}(x_t; \tau_{\text{fast}}), \quad y_t^{\text{slow}} = \text{EMA}(x_t; \tau_{\text{slow}})$$
//! 2. Continuous MACD line:
//!    $$M_t = y_t^{\text{fast}} - y_t^{\text{slow}}$$
//! 3. Signal line (EMA of MACD line):
//!    $$S_t = \text{EMA}(M_t; \tau_{\text{signal}})$$
//! 4. Divergence histogram:
//!    $$H_t = M_t - S_t$$
//! 5. Closed-form analytical gradients:
//!    $$\frac{\partial M_t}{\partial \tau_{\text{fast}}} = \frac{\partial y_t^{\text{fast}}}{\partial \tau_{\text{fast}}}$$
//!    $$\frac{\partial M_t}{\partial \tau_{\text{slow}}} = -\frac{\partial y_t^{\text{slow}}}{\partial \tau_{\text{slow}}}$$
//!    $$\frac{\partial S_t}{\partial \tau_{\text{signal}}} = \frac{\partial S_t}{\partial \alpha_{\text{signal}}} \cdot \left(-\frac{2}{(\tau_{\text{signal}} + 1)^2}\right)$$

use crate::ml::error::MlError;
use crate::ml::tin::smoothing::DifferentiableEma;
use crate::traits::Next;
use serde::{Deserialize, Serialize};

/// Output produced by [`DifferentiableMacd`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DifferentiableMacdOutput {
    /// MACD Line: $y^{\text{fast}} - y^{\text{slow}}$.
    pub macd_line: f64,
    /// Signal Line: $\text{EMA}(M_t; \tau_{\text{signal}})$.
    pub signal_line: f64,
    /// Histogram: $M_t - S_t$.
    pub histogram: f64,
}

/// Continuous differentiable Moving Average Convergence Divergence with analytical parameter gradients.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentiableMacd {
    /// Continuous fast EMA smoother.
    fast_ema: DifferentiableEma,
    /// Continuous slow EMA smoother.
    slow_ema: DifferentiableEma,
    /// Continuous signal EMA smoother applied to MACD line.
    signal_ema: DifferentiableEma,
    /// Number of bars consumed so far.
    bars_consumed: usize,
    /// Most recent calculation output, if any.
    last_output: Option<DifferentiableMacdOutput>,
}

impl DifferentiableMacd {
    /// Creates a new [`DifferentiableMacd`] with continuous spans.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if any span is $\le 0.0$ or non-finite.
    /// Returns [`MlError::InvalidGridRange`] if `fast_span >= slow_span`.
    pub fn new(fast_span: f64, slow_span: f64, signal_span: f64) -> Result<Self, MlError> {
        if !fast_span.is_finite() || fast_span <= 0.0 {
            return Err(MlError::InvalidParameter(fast_span));
        }
        if !slow_span.is_finite() || slow_span <= 0.0 {
            return Err(MlError::InvalidParameter(slow_span));
        }
        if !signal_span.is_finite() || signal_span <= 0.0 {
            return Err(MlError::InvalidParameter(signal_span));
        }
        if fast_span >= slow_span {
            return Err(MlError::InvalidGridRange(fast_span, slow_span));
        }

        Ok(Self {
            fast_ema: DifferentiableEma::new(fast_span)?,
            slow_ema: DifferentiableEma::new(slow_span)?,
            signal_ema: DifferentiableEma::new(signal_span)?,
            bars_consumed: 0,
            last_output: None,
        })
    }

    /// Returns the fast EMA span $\tau_{\text{fast}}$.
    #[inline]
    pub fn fast_span(&self) -> f64 {
        self.fast_ema.span()
    }

    /// Returns the slow EMA span $\tau_{\text{slow}}$.
    #[inline]
    pub fn slow_span(&self) -> f64 {
        self.slow_ema.span()
    }

    /// Returns the signal EMA span $\tau_{\text{signal}}$.
    #[inline]
    pub fn signal_span(&self) -> f64 {
        self.signal_ema.span()
    }

    /// Returns the number of bars consumed since creation or last reset.
    #[inline]
    pub fn bars_consumed(&self) -> usize {
        self.bars_consumed
    }

    /// Returns the most recent calculation output, if any.
    #[inline]
    pub fn last_output(&self) -> Option<DifferentiableMacdOutput> {
        self.last_output
    }

    /// Returns the analytical gradient $\frac{\partial M_t}{\partial \tau_{\text{fast}}}$.
    #[inline]
    pub fn grad_macd_fast_span(&self) -> f64 {
        self.fast_ema.grad_span()
    }

    /// Returns the analytical gradient $\frac{\partial M_t}{\partial \tau_{\text{slow}}}$.
    #[inline]
    pub fn grad_macd_slow_span(&self) -> f64 {
        -self.slow_ema.grad_span()
    }

    /// Returns the direct analytical gradient $\frac{\partial S_t}{\partial \tau_{\text{signal}}}$.
    #[inline]
    pub fn grad_signal_span(&self) -> f64 {
        self.signal_ema.grad_span()
    }

    /// Updates all three spans simultaneously, validating parameters.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if any span is $\le 0.0$ or non-finite.
    /// Returns [`MlError::InvalidGridRange`] if `fast_span >= slow_span`.
    pub fn set_parameters(
        &mut self,
        fast_span: f64,
        slow_span: f64,
        signal_span: f64,
    ) -> Result<(), MlError> {
        if !fast_span.is_finite() || fast_span <= 0.0 {
            return Err(MlError::InvalidParameter(fast_span));
        }
        if !slow_span.is_finite() || slow_span <= 0.0 {
            return Err(MlError::InvalidParameter(slow_span));
        }
        if !signal_span.is_finite() || signal_span <= 0.0 {
            return Err(MlError::InvalidParameter(signal_span));
        }
        if fast_span >= slow_span {
            return Err(MlError::InvalidGridRange(fast_span, slow_span));
        }

        self.fast_ema.set_span(fast_span)?;
        self.slow_ema.set_span(slow_span)?;
        self.signal_ema.set_span(signal_span)?;
        Ok(())
    }

    /// Updates spans along loss gradients with learning rate and bounds:
    ///
    /// # Arguments
    /// * `learning_rate` - Gradient step size $\eta$.
    /// * `grad_fast` - Loss gradient w.r.t fast span.
    /// * `grad_slow` - Loss gradient w.r.t slow span.
    /// * `grad_signal` - Loss gradient w.r.t signal span.
    /// * `min_span` - Minimum allowed span.
    /// * `max_span` - Maximum allowed span.
    pub fn update_spans(
        &mut self,
        learning_rate: f64,
        grad_fast: f64,
        grad_slow: f64,
        grad_signal: f64,
        min_span: f64,
        max_span: f64,
    ) {
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

        let mut new_fast = (self.fast_span() - learning_rate * grad_fast).clamp(lo, hi);
        let mut new_slow = (self.slow_span() - learning_rate * grad_slow).clamp(lo, hi);
        let new_signal = (self.signal_span() - learning_rate * grad_signal).clamp(lo, hi);

        // Ensure fast < slow remains invariant
        if new_fast >= new_slow {
            if new_fast + 0.1 <= hi {
                new_slow = new_fast + 0.1;
            } else if new_slow >= lo + 0.1 {
                new_fast = new_slow - 0.1;
            } else {
                new_fast = lo;
                new_slow = lo + 0.1;
            }
        }

        let _ = self.fast_ema.set_span(new_fast);
        let _ = self.slow_ema.set_span(new_slow);
        let _ = self.signal_ema.set_span(new_signal);
    }

    /// Resets all three internal smoothers, clearing history.
    pub fn reset(&mut self) {
        self.fast_ema.reset();
        self.slow_ema.reset();
        self.signal_ema.reset();
        self.bars_consumed = 0;
        self.last_output = None;
    }
}

impl Next<f64> for DifferentiableMacd {
    type Output = DifferentiableMacdOutput;

    #[inline]
    fn next(&mut self, input: f64) -> Self::Output {
        self.bars_consumed += 1;
        let fast = self.fast_ema.next(input);
        let slow = self.slow_ema.next(input);
        let macd_line = fast - slow;
        let signal_line = self.signal_ema.next(macd_line);
        let histogram = macd_line - signal_line;

        let out = DifferentiableMacdOutput {
            macd_line,
            signal_line,
            histogram,
        };
        self.last_output = Some(out);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_parameters() {
        assert!(matches!(
            DifferentiableMacd::new(0.0, 26.0, 9.0),
            Err(MlError::InvalidParameter(_))
        ));
        assert!(matches!(
            DifferentiableMacd::new(26.0, 12.0, 9.0),
            Err(MlError::InvalidGridRange(_, _))
        ));
        assert!(matches!(
            DifferentiableMacd::new(12.0, 12.0, 9.0),
            Err(MlError::InvalidGridRange(_, _))
        ));
        assert!(matches!(
            DifferentiableMacd::new(12.0, 26.0, -1.0),
            Err(MlError::InvalidParameter(_))
        ));
    }

    #[test]
    fn test_basic_macd_flow() {
        let mut macd = DifferentiableMacd::new(12.0, 26.0, 9.0).expect("valid macd");
        assert_eq!(macd.fast_span(), 12.0);
        assert_eq!(macd.slow_span(), 26.0);
        assert_eq!(macd.signal_span(), 9.0);

        // First bar: fast = slow = input => macd_line = 0, signal_line = 0, histogram = 0
        let out0 = macd.next(100.0);
        assert_eq!(out0.macd_line, 0.0);
        assert_eq!(out0.signal_line, 0.0);
        assert_eq!(out0.histogram, 0.0);
        assert_eq!(macd.bars_consumed(), 1);

        // When prices jump up, fast EMA moves faster than slow EMA => MACD line > 0
        let out1 = macd.next(110.0);
        assert!(out1.macd_line > 0.0);
        assert_eq!(out1.histogram, out1.macd_line - out1.signal_line);
    }

    #[test]
    fn test_finite_difference_macd_fast_grad() {
        let prices = [
            50.0, 52.0, 51.5, 53.0, 55.0, 54.5, 57.0, 56.0, 58.5, 60.0, 61.0, 59.5, 62.0, 63.5,
            63.0, 65.0, 67.0, 66.5, 69.0, 70.0,
        ];
        let fast_span = 8.0;
        let slow_span = 20.0;
        let signal_span = 5.0;

        let mut macd =
            DifferentiableMacd::new(fast_span, slow_span, signal_span).expect("valid macd");
        for &p in &prices {
            macd.next(p);
        }
        let analytical_grad = macd.grad_macd_fast_span();

        let eps = 1e-6;
        let eval_macd = |f_span: f64| -> f64 {
            let mut m =
                DifferentiableMacd::new(f_span, slow_span, signal_span).expect("valid macd");
            let mut last_macd = 0.0;
            for &p in &prices {
                last_macd = m.next(p).macd_line;
            }
            last_macd
        };

        let m_plus = eval_macd(fast_span + eps);
        let m_minus = eval_macd(fast_span - eps);
        let numerical_grad = (m_plus - m_minus) / (2.0 * eps);

        let rel_diff = (analytical_grad - numerical_grad).abs() / analytical_grad.abs().max(1.0);
        assert!(
            rel_diff < 1e-5,
            "MACD fast grad mismatch: analytical={analytical_grad}, numerical={numerical_grad}, rel_diff={rel_diff}"
        );
    }

    #[test]
    fn test_finite_difference_macd_slow_grad() {
        let prices = [
            50.0, 52.0, 51.5, 53.0, 55.0, 54.5, 57.0, 56.0, 58.5, 60.0, 61.0, 59.5, 62.0, 63.5,
            63.0, 65.0, 67.0, 66.5, 69.0, 70.0,
        ];
        let fast_span = 8.0;
        let slow_span = 20.0;
        let signal_span = 5.0;

        let mut macd =
            DifferentiableMacd::new(fast_span, slow_span, signal_span).expect("valid macd");
        for &p in &prices {
            macd.next(p);
        }
        let analytical_grad = macd.grad_macd_slow_span();

        let eps = 1e-6;
        let eval_macd = |s_span: f64| -> f64 {
            let mut m =
                DifferentiableMacd::new(fast_span, s_span, signal_span).expect("valid macd");
            let mut last_macd = 0.0;
            for &p in &prices {
                last_macd = m.next(p).macd_line;
            }
            last_macd
        };

        let m_plus = eval_macd(slow_span + eps);
        let m_minus = eval_macd(slow_span - eps);
        let numerical_grad = (m_plus - m_minus) / (2.0 * eps);

        let rel_diff = (analytical_grad - numerical_grad).abs() / analytical_grad.abs().max(1.0);
        assert!(
            rel_diff < 1e-5,
            "MACD slow grad mismatch: analytical={analytical_grad}, numerical={numerical_grad}, rel_diff={rel_diff}"
        );
    }

    #[test]
    fn test_finite_difference_signal_span_grad() {
        let prices = [
            50.0, 52.0, 51.5, 53.0, 55.0, 54.5, 57.0, 56.0, 58.5, 60.0, 61.0, 59.5, 62.0, 63.5,
            63.0, 65.0, 67.0, 66.5, 69.0, 70.0,
        ];
        let fast_span = 8.0;
        let slow_span = 20.0;
        let signal_span = 5.0;

        let mut macd =
            DifferentiableMacd::new(fast_span, slow_span, signal_span).expect("valid macd");
        for &p in &prices {
            macd.next(p);
        }
        let analytical_grad = macd.grad_signal_span();

        let eps = 1e-6;
        // Holding fast and slow fixed, change signal span
        let eval_sig = |sig_span: f64| -> f64 {
            let mut m =
                DifferentiableMacd::new(fast_span, slow_span, sig_span).expect("valid macd");
            let mut last_sig = 0.0;
            for &p in &prices {
                last_sig = m.next(p).signal_line;
            }
            last_sig
        };

        let s_plus = eval_sig(signal_span + eps);
        let s_minus = eval_sig(signal_span - eps);
        let numerical_grad = (s_plus - s_minus) / (2.0 * eps);

        let rel_diff = (analytical_grad - numerical_grad).abs() / analytical_grad.abs().max(1.0);
        assert!(
            rel_diff < 1e-5,
            "MACD signal span grad mismatch: analytical={analytical_grad}, numerical={numerical_grad}, rel_diff={rel_diff}"
        );
    }

    #[test]
    fn test_reset_and_determinism() {
        let prices = [10.0, 12.0, 11.0, 13.0, 15.0, 14.0];
        let mut macd = DifferentiableMacd::new(4.0, 10.0, 3.0).expect("valid macd");

        let outputs1: Vec<_> = prices.iter().map(|&p| macd.next(p)).collect();
        macd.reset();
        assert_eq!(macd.bars_consumed(), 0);
        assert!(macd.last_output().is_none());

        let outputs2: Vec<_> = prices.iter().map(|&p| macd.next(p)).collect();
        assert_eq!(outputs1, outputs2);
    }

    #[test]
    fn test_serde() {
        let mut macd = DifferentiableMacd::new(12.0, 26.0, 9.0).expect("valid macd");
        macd.next(100.0);
        macd.next(105.0);

        let serialized = serde_json::to_string(&macd).expect("serialize");
        let deserialized: DifferentiableMacd =
            serde_json::from_str(&serialized).expect("deserialize");

        assert_eq!(macd.fast_span(), deserialized.fast_span());
        assert_eq!(macd.slow_span(), deserialized.slow_span());
        assert_eq!(macd.signal_span(), deserialized.signal_span());
        assert_eq!(macd.bars_consumed(), deserialized.bars_consumed());

        let out1 = macd.last_output().expect("out1");
        let out2 = deserialized.last_output().expect("out2");
        assert!((out1.macd_line - out2.macd_line).abs() < 1e-9);
        assert!((out1.signal_line - out2.signal_line).abs() < 1e-9);
        assert!((out1.histogram - out2.histogram).abs() < 1e-9);
    }
}
