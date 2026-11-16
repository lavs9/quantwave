//! Technical Indicator Networks (TIN) for differentiable technical analysis.
//!
//! Technical Indicator Networks (TINs, Lu 2025/2026) replace fixed, non-differentiable lookback windows
//! with continuous parameterizations $\tau \in \mathbb{R}^+$ and closed-form analytical gradients.
//! This allows indicators to be embedded into gradient-based neural architectures or calibrated
//! directly via continuous optimization.
//!
//! # Submodules
//! - [`smoothing`]: Differentiable Exponential Moving Average ([`DifferentiableEma`]).
//! - [`rsi`]: Differentiable Relative Strength Index with sigmoidal thresholding ([`DifferentiableRsi`]).
//! - [`macd`]: Differentiable Moving Average Convergence Divergence ([`DifferentiableMacd`]).

pub mod macd;
pub mod rsi;
pub mod smoothing;

pub use macd::{DifferentiableMacd, DifferentiableMacdOutput};
pub use rsi::{DifferentiableRsi, DifferentiableRsiOutput};
pub use smoothing::DifferentiableEma;
