//! Property tests for the public indicator-metadata registry (quantwave-p2k0.1).
//!
//! The registry (`ALL_INDICATOR_METADATA`) is the single source of truth that feeds
//! the Python introspection API (`get_functions` / `abstract.Function`), the docs
//! catalog, and the canonical indicator count. These invariants keep it well-formed.

use quantwave_core::indicators::metadata_registry::{ALL_INDICATOR_METADATA, METADATA_COUNT};

#[test]
fn registry_len_matches_declared_count() {
    assert_eq!(
        ALL_INDICATOR_METADATA.len(),
        METADATA_COUNT,
        "ALL_INDICATOR_METADATA length must equal METADATA_COUNT"
    );
}

#[test]
fn every_entry_is_well_formed() {
    for m in ALL_INDICATOR_METADATA {
        assert!(!m.name.trim().is_empty(), "indicator has empty name");
        assert!(!m.category.trim().is_empty(), "{}: empty category", m.name);
        assert!(
            !m.description.trim().is_empty(),
            "{}: empty description",
            m.name
        );
    }
}

#[test]
fn params_are_well_formed_with_parseable_defaults() {
    for m in ALL_INDICATOR_METADATA {
        for p in m.params {
            assert!(
                !p.name.trim().is_empty(),
                "{}: parameter with empty name",
                m.name
            );
            // A declared default must be present and non-empty so the abstract API
            // can surface it. (Numeric defaults are validated by the Python parity
            // tests against the actual .ta signatures.)
            assert!(
                !p.default.trim().is_empty(),
                "{}: parameter {} has empty default",
                m.name,
                p.name
            );
        }
    }
}

#[test]
fn indicator_names_are_unique() {
    let mut seen = std::collections::HashSet::new();
    for m in ALL_INDICATOR_METADATA {
        assert!(
            seen.insert(m.name),
            "duplicate indicator name in registry: {}",
            m.name
        );
    }
}
