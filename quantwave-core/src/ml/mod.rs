//! Machine Learning, Neural Indicators, and Learnable Architectures.
//!
//! This module provides:
//! - **KAN (Kolmogorov-Arnold Networks)**: Spline-parameterized non-linear indicator synthesis
//!   and symbolic mathematical rule extraction ([`kan`]).
//! - **TIN (Technical Indicator Networks)**: Differentiable, learnable technical indicators
//!   with analytical parameter gradients ([`tin`]).
//! - **B-Spline Utilities**: Cox-de Boor B-spline basis evaluation ([`bspline`]).

pub mod bspline;
pub mod error;
pub mod kan;
pub mod tin;

pub use bspline::BSplineBasis;
pub use error::MlError;
pub use kan::{
    KanIndicator, KanLayer, KanNetwork, SymbolicTerm, extract_feature_importance,
    extract_symbolic_formula,
};
pub use tin::{
    DifferentiableEma, DifferentiableMacd, DifferentiableMacdOutput, DifferentiableRsi,
    DifferentiableRsiOutput,
};
