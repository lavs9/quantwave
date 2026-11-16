//! Domain error types for QuantWave ML modules.

use thiserror::Error;

/// Errors arising from machine learning, KAN, and TIN operations.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum MlError {
    #[error("grid intervals must be at least 1, got {0}")]
    InvalidGridIntervals(usize),

    #[error("spline degree must be <= 10, got {0}")]
    InvalidSplineDegree(usize),

    #[error("grid range must be strictly increasing: min={0}, max={1}")]
    InvalidGridRange(f64, f64),

    #[error("input dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("coefficient count mismatch: expected {expected}, got {got}")]
    CoefficientMismatch { expected: usize, got: usize },

    #[error("parameter must be positive and finite, got {0}")]
    InvalidParameter(f64),

    #[error("layer list must not be empty")]
    EmptyNetwork,

    #[error("empty input series or insufficient data")]
    InsufficientData,

    #[error("model serialization/deserialization error: {0}")]
    Serialization(String),
}
