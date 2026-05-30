//! ML Feature Engineering Polars layer (ta.features.*)
//!
//! Wires the rich Rust feature extractors from quantwave-core::features into
//! zero-copy Polars expressions following the existing .ta. namespace pattern.
//!
//! Spec sources (recorded):
//! - quantwave-4ps epic children (tha + wlx + gw7s)
//! - features/mod.rs wlx prep note (2026-05-30)
//! - gw7s canonical notebook (docs/examples/notebooks/ml_feature_stability.py)
//! - quantwave-4ub research notes (P0 list + validation strategy)
//!
//! Current scope (MVP start):
//! - Expose Hurst, CyberCycle, GriffithsDominantCycle, and basic regime features.
//! - Return rich Structs where multi-value (or exploded columns).
//! - No-lookahead by construction (delegates to Next<T> core).
//!
//! Future (after this initial wiring):
//! - .ta.features.build_matrix(...)
//! - Full Python surface
//! - Integration with regimes soft probs + PA events

use polars::prelude::*;
use quantwave_core::features::{self as rust_features};
use quantwave_core::traits::Next;

/// Extension trait for LazyFrame / Series to access the ML feature namespace.
pub trait FeatureExt {
    /// Hurst persistence + optional regime label.
    fn ta_hurst(&self, period: usize) -> PolarsResult<Expr>;

    /// Cyber Cycle rich features (cycle, trigger, momentum, signal).
    fn ta_cyber_cycle(&self, length: usize) -> PolarsResult<Expr>;

    /// Griffiths Dominant Cycle estimate.
    fn ta_griffiths_dominant_cycle(&self, lower: usize, upper: usize, length: usize) -> PolarsResult<Expr>;

    /// Basic regime one-hot / prob features (extendable to soft probs later).
    fn ta_regime_features(&self) -> PolarsResult<Expr>;
}

impl FeatureExt for LazyFrame {
    fn ta_hurst(&self, period: usize) -> PolarsResult<Expr> {
        // For MVP we use map_batches with a stateful closure.
        // In production this would be a proper plugin for zero-copy.
        let extractor = std::sync::Arc::new(std::sync::Mutex::new(
            rust_features::HurstFeatureExtractor::new(period),
        ));

        Ok(col("close").map(
            move |s| {
                let mut guard = extractor.lock().unwrap();
                let ca: &Float64Chunked = s.f64()?;
                let out: Float64Chunked = ca.apply(|opt_v| {
                    opt_v.map(|v| guard.next(v).persistence)
                });
                Ok(Some(out.into_series().into()))
            },
            GetOutput::from_type(DataType::Float64),
        ).alias(&format!("hurst_{}", period)))
    }

    fn ta_cyber_cycle(&self, length: usize) -> PolarsResult<Expr> {
        let extractor = std::sync::Arc::new(std::sync::Mutex::new(
            rust_features::CyberCycleFeatureExtractor::new(length),
        ));

        // Return a Struct for the rich output (cycle, trigger, momentum, signal)
        Ok(col("close").map(
            move |s| {
                let mut guard = extractor.lock().unwrap();
                let ca: &Float64Chunked = s.f64()?;
                let (cycle, trigger, momentum, signal): (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) = ca
                    .into_iter()
                    .map(|opt_v| {
                        if let Some(v) = opt_v {
                            let f = guard.next(v);
                            (f.cycle, f.trigger, f.cycle_momentum, f.trigger_signal)
                        } else {
                            (f64::NAN, f64::NAN, f64::NAN, f64::NAN)
                        }
                    })
                    .unzip();

                let struct_series = StructChunked::new(
                    "cyber_cycle",
                    &[
                        Series::new("cycle".into(), cycle),
                        Series::new("trigger".into(), trigger),
                        Series::new("momentum".into(), momentum),
                        Series::new("signal".into(), signal),
                    ],
                )?
                .into_series();

                Ok(Some(struct_series))
            },
            GetOutput::from_type(DataType::Struct(vec![])), // simplified
        ))
    }

    fn ta_griffiths_dominant_cycle(&self, lower: usize, upper: usize, length: usize) -> PolarsResult<Expr> {
        let extractor = std::sync::Arc::new(std::sync::Mutex::new(
            rust_features::GriffithsDominantCycleFeatureExtractor::new(lower, upper, length),
        ));

        Ok(col("close").map(
            move |s| {
                let mut guard = extractor.lock().unwrap();
                let ca: &Float64Chunked = s.f64()?;
                let out: Float64Chunked = ca.apply(|opt_v| {
                    opt_v.map(|v| guard.next(v).dominant_cycle)
                });
                Ok(Some(out.into_series().into()))
            },
            GetOutput::from_type(DataType::Float64),
        ).alias(&format!("griffiths_dc_{}_{}_{}", lower, upper, length)))
    }

    fn ta_regime_features(&self) -> PolarsResult<Expr> {
        // For MVP we use a constant "Steady" as placeholder.
        // Real version will take regime column or compute on the fly.
        Ok(lit(0i64).alias("regime_placeholder"))
    }
}

// Re-export for easy use in the main lib.rs ta. namespace
pub use FeatureExt as TaFeaturesExt;
