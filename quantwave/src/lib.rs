//! QuantWave: A high-performance technical analysis library.
//! 
//! This is the main umbrella crate that re-exports the core engine and Polars integration.

pub use quantwave_core as core;

#[cfg(feature = "polars")]
pub use quantwave_polars as polars;

/// Prelude for common traits and types.
pub mod prelude {
    pub use quantwave_core::traits::*;
    pub use quantwave_core::indicators::*;
    
    #[cfg(feature = "polars")]
    pub use quantwave_polars::prelude::*;
}
