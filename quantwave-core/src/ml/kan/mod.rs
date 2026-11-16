//! Kolmogorov-Arnold Networks (KAN) for financial indicator synthesis.

pub mod layer;
pub mod network;
pub mod symbolic;

pub use layer::{KanLayer, silu};
pub use network::{KanIndicator, KanNetwork};
pub use symbolic::{SymbolicTerm, extract_feature_importance, extract_symbolic_formula};
