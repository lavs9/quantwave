//! Differentiable Relative Strength Index (RSI).
//!
//! Provides a learnable Relative Strength Index parameterized by a continuous time span $\tau$,
//! equipped with closed-form analytical parameter gradients and smooth sigmoidal thresholding
//! for continuous end-to-end differentiable trading signals.
//!
//! # Mathematical Foundation
//! In Technical Indicator Networks (TINs, Lu 2025/2026), classic indicator thresholds are relaxed
//! into continuous, smooth functions amenable to gradient-based optimization:
//!
//! 1. Price changes: $\Delta P_t = P_t - P_{t-1}$
//! 2. Decomposed gains and losses:
//!    $$U_t = \max(\Delta P_t, 0), \quad D_t = \max(-\Delta P_t, 0)$$
//! 3. Smoothed average gain and loss using [`DifferentiableEma`] smoothers with continuous span $\tau$:
//!    $$\bar{U}_t = \text{EMA}(U_t; \tau), \quad \bar{D}_t = \text{EMA}(D_t; \tau)$$
//! 4. Continuous Relative Strength Index:
//!    $$\text{RSI}_t = 100 - \frac{100}{1 + \text{RS}_t} = 100 \cdot \frac{\bar{U}_t}{\bar{U}_t + \bar{D}_t}$$
//! 5. Closed-form gradient with respect to span $\tau$:
//!    $$\frac{\partial \text{RSI}_t}{\partial \tau} = 100 \cdot \frac{\bar{D}_t \frac{\partial \bar{U}_t}{\partial \tau} - \bar{U}_t \frac{\partial \bar{D}_t}{\partial \tau}}{(\bar{U}_t + \bar{D}_t)^2}$$
//! 6. Differentiable trading signal $\in [-1.0, 1.0]$ via soft sigmoidal bounds:
//!    $$\text{signal}_t = \sigma(\beta \cdot (\text{RSI}_t - \theta_{\text{ob}})) - \sigma(\beta \cdot (\theta_{\text{os}} - \text{RSI}_t))$$
//!    where $\sigma(z) = \frac{1}{1 + e^{-z}}$, $\theta_{\text{ob}}$ is overbought, $\theta_{\text{os}}$ is oversold, and $\beta$ is steepness.

use crate::ml::error::MlError;
use crate::ml::tin::smoothing::DifferentiableEma;
use crate::traits::Next;
use serde::{Deserialize, Serialize};

/// Output produced by [`DifferentiableRsi`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DifferentiableRsiOutput {
    /// Relative Strength Index value $\in [0.0, 100.0]$.
    pub rsi: f64,
    /// Smooth continuous signal $\in [-1.0, 1.0]$ computed via sigmoidal thresholding.
    pub signal: f64,
    /// Analytical partial derivative $\frac{\partial \text{RSI}}{\partial \tau}$ with respect to span $\tau$.
    pub grad_span: f64,
}

/// Continuous differentiable Relative Strength Index with analytical parameter gradients.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentiableRsi {
    /// Exponential smoother for upward gains $U_t$.
    gain_ema: DifferentiableEma,
    /// Exponential smoother for downward losses $D_t$.
    loss_ema: DifferentiableEma,
    /// Previous raw price $P_{t-1}$.
    prev_price: Option<f64>,
    /// Upper overbought threshold $\theta_{\text{ob}}$ (default: 70.0).
    overbought: f64,
    /// Lower oversold threshold $\theta_{\text{os}}$ (default: 30.0).
    oversold: f64,
    /// Sigmoid steepness parameter $\beta > 0$ (default: 0.1).
    steepness: f64,
    /// Number of bars consumed so far.
    bars_consumed: usize,
    /// Most recent calculation output, if any.
    last_output: Option<DifferentiableRsiOutput>,
}

impl DifferentiableRsi {
    /// Creates a new [`DifferentiableRsi`] with continuous span $\tau > 0$ and thresholds.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if `span <= 0.0` or parameters are non-finite.
    /// Returns [`MlError::InvalidGridRange`] if `oversold >= overbought`.
    pub fn new(span: f64, overbought: f64, oversold: f64) -> Result<Self, MlError> {
        if !span.is_finite() || span <= 0.0 {
            return Err(MlError::InvalidParameter(span));
        }
        if !overbought.is_finite() {
            return Err(MlError::InvalidParameter(overbought));
        }
        if !oversold.is_finite() {
            return Err(MlError::InvalidParameter(oversold));
        }
        if oversold >= overbought {
            return Err(MlError::InvalidGridRange(oversold, overbought));
        }

        Ok(Self {
            gain_ema: DifferentiableEma::new(span)?,
            loss_ema: DifferentiableEma::new(span)?,
            prev_price: None,
            overbought,
            oversold,
            steepness: 0.1,
            bars_consumed: 0,
            last_output: None,
        })
    }

    /// Sets the sigmoid steepness parameter $\beta > 0$.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if `steepness <= 0.0` or not finite.
    pub fn with_steepness(mut self, steepness: f64) -> Result<Self, MlError> {
        self.set_steepness(steepness)?;
        Ok(self)
    }

    /// Returns the current continuous span $\tau$.
    #[inline]
    pub fn span(&self) -> f64 {
        self.gain_ema.span()
    }

    /// Returns the overbought threshold $\theta_{\text{ob}}$.
    #[inline]
    pub fn overbought(&self) -> f64 {
        self.overbought
    }

    /// Returns the oversold threshold $\theta_{\text{os}}$.
    #[inline]
    pub fn oversold(&self) -> f64 {
        self.oversold
    }

    /// Returns the sigmoid steepness $\beta$.
    #[inline]
    pub fn steepness(&self) -> f64 {
        self.steepness
    }

    /// Returns the number of bars consumed since creation or last reset.
    #[inline]
    pub fn bars_consumed(&self) -> usize {
        self.bars_consumed
    }

    /// Returns the most recent calculation output, if any.
    #[inline]
    pub fn last_output(&self) -> Option<DifferentiableRsiOutput> {
        self.last_output
    }

    /// Returns the current analytical gradient $\frac{\partial \text{RSI}}{\partial \tau}$.
    #[inline]
    pub fn grad_span(&self) -> f64 {
        self.last_output.map(|o| o.grad_span).unwrap_or(0.0)
    }

    /// Mutates the sigmoid steepness parameter $\beta$.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if `steepness <= 0.0` or not finite.
    pub fn set_steepness(&mut self, steepness: f64) -> Result<(), MlError> {
        if !steepness.is_finite() || steepness <= 0.0 {
            return Err(MlError::InvalidParameter(steepness));
        }
        self.steepness = steepness;
        Ok(())
    }

    /// Mutates the continuous span $\tau$ for both internal EMA smoothers.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if `span <= 0.0` or not finite.
    pub fn set_span(&mut self, span: f64) -> Result<(), MlError> {
        self.gain_ema.set_span(span)?;
        self.loss_ema.set_span(span)?;
        Ok(())
    }

    /// Updates the overbought and oversold thresholds.
    ///
    /// # Errors
    /// Returns [`MlError::InvalidParameter`] if thresholds are non-finite.
    /// Returns [`MlError::InvalidGridRange`] if `oversold >= overbought`.
    pub fn set_thresholds(&mut self, overbought: f64, oversold: f64) -> Result<(), MlError> {
        if !overbought.is_finite() {
            return Err(MlError::InvalidParameter(overbought));
        }
        if !oversold.is_finite() {
            return Err(MlError::InvalidParameter(oversold));
        }
        if oversold >= overbought {
            return Err(MlError::InvalidGridRange(oversold, overbought));
        }
        self.overbought = overbought;
        self.oversold = oversold;
        Ok(())
    }

    /// Updates the span parameter along the gradient of a downstream loss:
    /// $$\tau \leftarrow \text{clamp}\left(\tau - \eta \cdot \frac{\partial L}{\partial \text{RSI}_t} \cdot \frac{\partial \text{RSI}_t}{\partial \tau}, \tau_{\min}, \tau_{\max}\right)$$
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
        let new_span = (self.span() - learning_rate * grad).clamp(lo, hi);
        let _ = self.set_span(new_span);
    }

    /// Resets all internal smoothers, prices, and bar counters.
    pub fn reset(&mut self) {
        self.gain_ema.reset();
        self.loss_ema.reset();
        self.prev_price = None;
        self.bars_consumed = 0;
        self.last_output = None;
    }
}

/// Numerically stable sigmoid activation function $\sigma(z) = \frac{1}{1 + e^{-z}}$.
#[inline]
fn sigmoid(z: f64) -> f64 {
    if z >= 40.0 {
        1.0
    } else if z <= -40.0 {
        0.0
    } else {
        1.0 / (1.0 + (-z).exp())
    }
}

impl Next<f64> for DifferentiableRsi {
    type Output = DifferentiableRsiOutput;

    #[inline]
    fn next(&mut self, price: f64) -> Self::Output {
        self.bars_consumed += 1;

        let (gain, loss) = match self.prev_price {
            None => (0.0, 0.0),
            Some(prev) => {
                let change = price - prev;
                if change > 0.0 {
                    (change, 0.0)
                } else if change < 0.0 {
                    (0.0, -change)
                } else {
                    (0.0, 0.0)
                }
            }
        };
        self.prev_price = Some(price);

        let avg_gain = self.gain_ema.next(gain);
        let avg_loss = self.loss_ema.next(loss);

        let denom = avg_gain + avg_loss;
        let (rsi, grad_span) = if denom <= 1e-12 {
            (50.0, 0.0)
        } else {
            let rsi = 100.0 * avg_gain / denom;
            let grad_gain = self.gain_ema.grad_span();
            let grad_loss = self.loss_ema.grad_span();
            let grad = 100.0 * (avg_loss * grad_gain - avg_gain * grad_loss) / (denom * denom);
            (rsi, grad)
        };

        let z_ob = self.steepness * (rsi - self.overbought);
        let z_os = self.steepness * (self.oversold - rsi);
        let signal = sigmoid(z_ob) - sigmoid(z_os);

        let out = DifferentiableRsiOutput {
            rsi,
            signal,
            grad_span,
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
            DifferentiableRsi::new(0.0, 70.0, 30.0),
            Err(MlError::InvalidParameter(_))
        ));
        assert!(matches!(
            DifferentiableRsi::new(14.0, 30.0, 70.0),
            Err(MlError::InvalidGridRange(_, _))
        ));
        assert!(matches!(
            DifferentiableRsi::new(14.0, 50.0, 50.0),
            Err(MlError::InvalidGridRange(_, _))
        ));
        assert!(matches!(
            DifferentiableRsi::new(14.0, f64::NAN, 30.0),
            Err(MlError::InvalidParameter(_))
        ));
    }

    #[test]
    fn test_initial_neutral_state() {
        let mut rsi = DifferentiableRsi::new(14.0, 70.0, 30.0).expect("valid rsi");
        let out = rsi.next(100.0);

        assert_eq!(out.rsi, 50.0);
        assert_eq!(out.grad_span, 0.0);
        // At RSI = 50 with symmetric thresholds 70/30:
        // z_ob = 0.1 * (50 - 70) = -2.0
        // z_os = 0.1 * (30 - 50) = -2.0
        // signal = sig(-2) - sig(-2) = 0.0
        assert!((out.signal).abs() < 1e-12);
        assert_eq!(rsi.bars_consumed(), 1);
    }

    #[test]
    fn test_directional_signals() {
        let mut rsi = DifferentiableRsi::new(5.0, 70.0, 30.0).expect("valid rsi");

        // Consecutive upward moves
        for p in [100.0, 105.0, 110.0, 116.0, 123.0, 131.0] {
            rsi.next(p);
        }
        let out_bullish = rsi.last_output().expect("has output");
        assert!(
            out_bullish.rsi > 70.0,
            "RSI should be overbought, got {}",
            out_bullish.rsi
        );
        assert!(
            out_bullish.signal > 0.5,
            "Signal should be strongly positive, got {}",
            out_bullish.signal
        );

        // Consecutive downward moves
        rsi.reset();
        for p in [100.0, 95.0, 90.0, 84.0, 77.0, 69.0] {
            rsi.next(p);
        }
        let out_bearish = rsi.last_output().expect("has output");
        assert!(
            out_bearish.rsi < 30.0,
            "RSI should be oversold, got {}",
            out_bearish.rsi
        );
        assert!(
            out_bearish.signal < -0.5,
            "Signal should be strongly negative, got {}",
            out_bearish.signal
        );
    }

    #[test]
    fn test_finite_difference_rsi_grad_span() {
        let prices = [
            100.0, 102.0, 101.5, 104.0, 103.0, 105.5, 104.2, 107.0, 106.5, 108.0, 110.0, 107.5,
            111.0, 112.5, 111.0, 113.5, 115.0, 114.2, 116.5, 118.0,
        ];
        let span = 10.0;
        let mut model = DifferentiableRsi::new(span, 70.0, 30.0).expect("valid rsi");

        let mut analytical_grad = 0.0;
        for &p in &prices {
            let out = model.next(p);
            analytical_grad = out.grad_span;
        }

        // Numerical finite difference
        let eps = 1e-6;
        let eval_rsi = |target_span: f64| -> f64 {
            let mut r = DifferentiableRsi::new(target_span, 70.0, 30.0).expect("valid rsi");
            let mut last_rsi = 50.0;
            for &p in &prices {
                last_rsi = r.next(p).rsi;
            }
            last_rsi
        };

        let rsi_plus = eval_rsi(span + eps);
        let rsi_minus = eval_rsi(span - eps);
        let numerical_grad = (rsi_plus - rsi_minus) / (2.0 * eps);

        let rel_diff = (analytical_grad - numerical_grad).abs() / analytical_grad.abs().max(1.0);
        assert!(
            rel_diff < 1e-4,
            "RSI grad_span mismatch: analytical={analytical_grad}, numerical={numerical_grad}, rel_diff={rel_diff}"
        );
    }

    #[test]
    fn test_reset_and_determinism() {
        let prices = [10.0, 12.0, 11.0, 13.0, 15.0, 14.0];
        let mut rsi = DifferentiableRsi::new(6.0, 70.0, 30.0).expect("valid rsi");

        let outputs1: Vec<_> = prices.iter().map(|&p| rsi.next(p)).collect();
        rsi.reset();
        assert_eq!(rsi.bars_consumed(), 0);
        assert!(rsi.last_output().is_none());

        let outputs2: Vec<_> = prices.iter().map(|&p| rsi.next(p)).collect();
        assert_eq!(outputs1, outputs2);
    }

    #[test]
    fn test_serde() {
        let mut rsi = DifferentiableRsi::new(14.0, 70.0, 30.0).expect("valid rsi");
        rsi.next(100.0);
        rsi.next(105.0);

        let serialized = serde_json::to_string(&rsi).expect("serialize");
        let deserialized: DifferentiableRsi =
            serde_json::from_str(&serialized).expect("deserialize");

        assert_eq!(rsi.span(), deserialized.span());
        assert_eq!(rsi.overbought(), deserialized.overbought());
        assert_eq!(rsi.oversold(), deserialized.oversold());
        assert_eq!(rsi.bars_consumed(), deserialized.bars_consumed());

        let out1 = rsi.last_output().expect("out1");
        let out2 = deserialized.last_output().expect("out2");
        assert!((out1.rsi - out2.rsi).abs() < 1e-9);
        assert!((out1.signal - out2.signal).abs() < 1e-9);
        assert!((out1.grad_span - out2.grad_span).abs() < 1e-9);
    }
}
