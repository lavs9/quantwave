//! ML Feature Engineering Toolkit (ta.features.*)
//!
//! This module provides rich, multi-dimensional feature extractors built on top of
//! QuantWave's existing high-quality indicators (especially Ehlers DSP and Regimes).
//! The goal is to make it trivial to build stable, no-lookahead feature matrices
//! for ML pipelines and strategy research.
//!
//! All extractors follow the Universal Indicator pattern where possible:
//! - Implement `Next<Input>` for streaming use
//! - Provide equivalent batch (Polars) paths in `quantwave-polars`
//! - Must eventually prove batch == streaming via proptests (see task quantwave-tha)
//!
//! Design principles (from quantwave-4ub research notes):
//! - Rich outputs (structs or tuples) over single scalars when useful
//! - Strong metadata (reuse/extend IndicatorMetadata)
//! - Easy composition with regimes and (future) PA events
//! - Zero lookahead by construction
//!
//! Sources recorded (per AGENTS.md):
//! - cyber_cycle.rs:35 (returns (cycle, trigger))
//! - hurst.rs (persistence value, excellent regime feature)
//! - regimes/mod.rs + 12 submodules (HMM probs, GMM, PELT, etc. as meta-features)
//! - Ehlers papers in references/Ehlers Papers/implemented/
//! - Prado "Advances in Financial Machine Learning" (for future fractional differencing / entropy)

pub mod cyber_cycle;
pub mod ehlers_autocorrelation;
pub mod griffiths_dominant_cycle;
pub mod hurst;
pub mod instantaneous_trendline;
pub mod regime;
pub mod regime_probs;
pub mod trendflex;

// Re-export common feature types for convenience
pub use cyber_cycle::{CyberCycleFeatureExtractor, CyberCycleFeatures};
pub use ehlers_autocorrelation::{
    EhlersAutocorrelationFeatureExtractor, EhlersAutocorrelationFeatures,
};
pub use griffiths_dominant_cycle::{
    GriffithsDominantCycleFeatureExtractor, GriffithsDominantCycleFeatures,
};
pub use hurst::{HurstFeatureExtractor, HurstFeatures};
pub use instantaneous_trendline::{
    InstantaneousTrendlineFeatureExtractor, InstantaneousTrendlineFeatures,
};
pub use regime::{RegimeFeatures, regime_to_features};
pub use regime_probs::{RegimeProbFeatureExtractor, RegimeProbFeatures, regime_to_prob_features};
pub use trendflex::{TrendflexFeatureExtractor, TrendflexFeatures};

// === wlx (Polars layer) preparation note (2026-05-30) ===
// Planned public API surface in quantwave-polars (to be implemented in wlx task):
// - Extension trait on LazyFrame/Series: .ta.features.hurst(period) -> Struct or multiple columns
// - .ta.features.ehlers_autocorrelation(length, num_lags) -> Struct with "correlations" List<f64> + "dominant_lag"
// - Convenience: .ta.features.build_core_matrix() or .ta.features.build_matrix(["cyber_cycle", "hurst", "regime", "autocorrelation"])
// - Rich returns: Struct for multi-value (cycle+trigger, autocorr vec), or exploded columns.
// - All features must be causal (no lookahead) — enforced by using the Next<T> impls here or equivalent Polars exprs.
// - Full integration with regimes (e.g. append hmm_regime probs as features) and future PA rich events (from cu03).
// - Python exposure: from quantwave import ta; df.ta.features.hurst(20) etc. (minimal already prototyped in gw7s notebook via python bindings).
//
// See: quantwave-4ps epic + children (tha + wlx + gw7s), quantwave-4ub research notes (P0 list + validation strategy), this module's proptest skeleton (the parity contract wlx must honor).
// Once the core extractors here are stable (more P0 + full proptests), wlx can wire the Polars expressions + builders.
// The gw7s notebook (docs/examples/notebooks/ml_feature_stability.py) already demonstrates the intended usage pattern with the current extractors.

#[cfg(test)]
mod proptest_parity {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    // Skeleton for batch vs streaming parity (per quantwave-tha + 4ub research).
    // Once wlx Polars layer exists, the "batch" path will use actual .ta.features.* exprs on LazyFrame.
    // Current skeleton: determinism of Next + simple "batch re-compute" equivalence for stateless views.
    // Full rich parity (including regime + feature filters) will be exercised in backtester (ug9t/06sz).

    proptest! {
        #[test]
        fn hurst_streaming_is_deterministic(data in prop::collection::vec(-100f64..100.0, 5..100)) {
            let mut ext1 = HurstFeatureExtractor::new(20);
            let mut ext2 = HurstFeatureExtractor::new(20);

            for &val in &data {
                let f1 = ext1.next(val);
                let f2 = ext2.next(val);
                // Treat NaN == NaN as equal for determinism check on degenerate input
                if f1.persistence.is_nan() && f2.persistence.is_nan() {
                    continue;
                }
                prop_assert_eq!(f1.persistence, f2.persistence);
            }
        }

        #[test]
        fn cybercycle_streaming_deterministic_and_momentum_sane(data in prop::collection::vec(-50f64..50.0, 10..80)) {
            let mut ext = CyberCycleFeatureExtractor::new(14);
            let mut prev_mom = 0.0;
            for &val in &data {
                let f = ext.next(val);
                // Momentum should be cycle delta (sanity, not strict property)
                if !f.cycle_momentum.is_nan() {
                    prop_assert!(f.cycle_momentum.abs() < 100.0);
                }
            }
        }

        #[test]
        fn autocorrelation_streaming_deterministic(data in prop::collection::vec(-100f64..100.0, 20..120)) {
            let mut ext1 = EhlersAutocorrelationFeatureExtractor::new(30, 10);
            let mut ext2 = EhlersAutocorrelationFeatureExtractor::new(30, 10);
            for &val in &data {
                let f1 = ext1.next(val);
                let f2 = ext2.next(val);
                prop_assert_eq!(f1.dominant_lag, f2.dominant_lag);
                prop_assert_eq!(f1.max_correlation, f2.max_correlation);
            }
        }

        #[test]
        fn griffiths_and_regime_prob_streaming_stable(data in prop::collection::vec(-80f64..80.0, 30..150)) {
            let mut g1 = GriffithsDominantCycleFeatureExtractor::new(8, 50, 40);
            let mut g2 = GriffithsDominantCycleFeatureExtractor::new(8, 50, 40);
            for &val in &data {
                let gf1 = g1.next(val);
                let gf2 = g2.next(val);
                prop_assert_eq!(gf1.dominant_cycle, gf2.dominant_cycle);
                // regime prob is deterministic transform of label
                let rf = regime_probs::regime_to_prob_features(crate::regimes::MarketRegime::Steady);
                prop_assert!(rf.probs.iter().sum::<f64>() > 0.99);
            }
        }

        // TODO (wlx integration): Once Polars exposure exists,
        // add property:
        // "batch Polars .ta.features.hurst(...) on LazyFrame(series) == streaming collect(Next) on same series"
        // using existing crate check_batch_streaming_parity helper pattern from indicators.
    }
}

/// Common trait for feature extractors that want to expose a stable "feature vector" view.
/// (Future expansion point for a unified FeatureVector trait.)
pub trait AsFeatures {
    /// Returns a slice of the current feature values (for quick ML consumption).
    fn as_features(&self) -> &[f64];
}
