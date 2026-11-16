//! Symbolic mathematical formula extraction for Kolmogorov-Arnold Networks.
//!
//! KANs parameterize activations on edges via B-splines. By inspecting the $L_1$
//! norm of edge activations $\phi(x)$, dominant inputs can be pruned and extracted
//! into interpretable mathematical expressions.
//!
//! # References
//! - Liu, Z. et al. (2024). *KAN: Kolmogorov-Arnold Networks*. arXiv:2404.19756 (Section 2.5).

use crate::ml::error::MlError;
use crate::ml::kan::network::KanNetwork;

/// Structured representation of an extracted feature term in a KAN indicator.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolicTerm {
    /// Original input feature index.
    pub feature_index: usize,
    /// Name of the feature.
    pub feature_name: String,
    /// Sum of $L_1$ edge norms connected to this feature in the first layer.
    pub l1_norm: f64,
    /// Relative importance score normalized to $[0.0, 1.0]$.
    pub relative_importance: f64,
}

/// Extracts ranked feature importance terms from a [`KanNetwork`].
///
/// Computes the total $L_1$ norm of edge activations originating from each input feature
/// in the network's first layer, then sorts features in descending order of importance.
///
/// # Errors
/// Returns `MlError::EmptyNetwork` if the network contains no layers.
/// Returns `MlError::DimensionMismatch` if `feature_names.len() != network.in_features()`
/// or if `network.out_features() != 1`.
pub fn extract_feature_importance(
    network: &KanNetwork,
    feature_names: &[&str],
) -> Result<Vec<SymbolicTerm>, MlError> {
    if network.num_layers() == 0 {
        return Err(MlError::EmptyNetwork);
    }
    if feature_names.len() != network.in_features() {
        return Err(MlError::DimensionMismatch {
            expected: network.in_features(),
            got: feature_names.len(),
        });
    }
    if network.out_features() != 1 {
        return Err(MlError::DimensionMismatch {
            expected: 1,
            got: network.out_features(),
        });
    }

    let first_layer = &network.layers()[0];
    let in_features = network.in_features();
    let out_features = first_layer.out_features();

    let mut terms = Vec::with_capacity(in_features);
    let mut total_l1 = 0.0;

    for i in 0..in_features {
        let mut feature_l1 = 0.0;
        for j in 0..out_features {
            feature_l1 += first_layer.edge_l1_norm(i, j);
        }
        total_l1 += feature_l1;

        terms.push(SymbolicTerm {
            feature_index: i,
            feature_name: feature_names[i].to_string(),
            l1_norm: feature_l1,
            relative_importance: 0.0,
        });
    }

    if total_l1 > 1e-12 {
        for term in &mut terms {
            term.relative_importance = term.l1_norm / total_l1;
        }
    }

    // Sort descending by L1 norm
    terms.sort_by(|a, b| b.l1_norm.total_cmp(&a.l1_norm));

    Ok(terms)
}

/// Extracts an interpretable symbolic formula string from a trained [`KanNetwork`].
///
/// Identifies dominant input features by $L_1$ edge norms, prunes negligible connections,
/// and formats surviving terms into a human-readable mathematical representation:
///
/// `"0.85 * f_1(rsi) + 0.42 * f_2(volume_imbalance)"`
///
/// # Errors
/// Returns `MlError::EmptyNetwork` if the network contains no layers.
/// Returns `MlError::DimensionMismatch` if `feature_names.len() != network.in_features()`
/// or if `network.out_features() != 1`.
pub fn extract_symbolic_formula(
    network: &KanNetwork,
    feature_names: &[&str],
) -> Result<String, MlError> {
    let ranked_terms = extract_feature_importance(network, feature_names)?;

    if ranked_terms.is_empty() {
        return Ok("0".to_string());
    }

    let max_l1 = ranked_terms[0].l1_norm;
    if max_l1 < 1e-12 {
        return Ok("0".to_string());
    }

    // Prune features with < 1% relative magnitude compared to dominant feature
    let threshold = (0.01 * max_l1).max(1e-6);
    let mut active_terms: Vec<&SymbolicTerm> = ranked_terms
        .iter()
        .filter(|t| t.l1_norm >= threshold)
        .collect();

    if active_terms.is_empty() {
        active_terms.push(&ranked_terms[0]);
    }

    let formula_parts: Vec<String> = active_terms
        .iter()
        .enumerate()
        .map(|(rank, term)| {
            format!(
                "{:.2} * f_{}({})",
                term.l1_norm,
                rank + 1,
                term.feature_name
            )
        })
        .collect();

    Ok(formula_parts.join(" + "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::bspline::BSplineBasis;
    use crate::ml::kan::layer::KanLayer;

    #[test]
    fn test_extract_symbolic_formula_validation() {
        let basis = BSplineBasis::new(2, 1, -1.0, 1.0).unwrap();
        let layer = KanLayer::with_weights(
            2,
            1,
            basis,
            vec![1.0, 0.5],
            vec![0.0; 2 * 3], // 2 edges * 3 basis
        )
        .unwrap();
        let net = KanNetwork::new(vec![layer]).unwrap();

        // Wrong number of feature names
        assert!(extract_symbolic_formula(&net, &["rsi"]).is_err());
        assert!(extract_symbolic_formula(&net, &["rsi", "macd", "vol"]).is_err());

        // Correct number of feature names
        let formula = extract_symbolic_formula(&net, &["rsi", "vol"]);
        assert!(formula.is_ok());
    }

    #[test]
    fn test_extract_symbolic_formula_formatting_and_ordering() {
        let basis = BSplineBasis::new(1, 0, -1.0, 1.0).unwrap(); // 1 basis function
        // Feature 0: base=0.42, spline=0.0 -> L1 = 0.42
        // Feature 1: base=0.85, spline=0.0 -> L1 = 0.85
        // Feature 2: base=0.00, spline=0.0 -> L1 = 0.00 (pruned)
        let base_weights = vec![0.42, 0.85, 0.0];
        let spline_weights = vec![0.0, 0.0, 0.0];
        let layer = KanLayer::with_weights(3, 1, basis, base_weights, spline_weights).unwrap();
        let net = KanNetwork::new(vec![layer]).unwrap();

        let feature_names = ["volume_imbalance", "rsi", "noise"];
        let formula = extract_symbolic_formula(&net, &feature_names).unwrap();

        // rsi (0.85) is dominant -> rank 1
        // volume_imbalance (0.42) is second -> rank 2
        // noise (0.0) is pruned
        assert_eq!(formula, "0.85 * f_1(rsi) + 0.42 * f_2(volume_imbalance)");

        // Check structured terms
        let terms = extract_feature_importance(&net, &feature_names).unwrap();
        assert_eq!(terms.len(), 3);
        assert_eq!(terms[0].feature_name, "rsi");
        assert!((terms[0].l1_norm - 0.85).abs() < 1e-10);
        assert_eq!(terms[1].feature_name, "volume_imbalance");
        assert!((terms[1].l1_norm - 0.42).abs() < 1e-10);
        assert_eq!(terms[2].feature_name, "noise");
    }

    #[test]
    fn test_all_zero_weights() {
        let basis = BSplineBasis::new(2, 1, -1.0, 1.0).unwrap();
        let layer = KanLayer::with_weights(2, 1, basis, vec![0.0, 0.0], vec![0.0; 2 * 3]).unwrap();
        let net = KanNetwork::new(vec![layer]).unwrap();

        let formula = extract_symbolic_formula(&net, &["f1", "f2"]).unwrap();
        assert_eq!(formula, "0");
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_symbolic_monotonicity_proptest(
            w0 in 0.0f64..10.0,
            w1 in 0.0f64..10.0,
            w2 in 0.0f64..10.0,
        ) {
            let basis = BSplineBasis::new(2, 1, -1.0, 1.0).unwrap();
            let layer = KanLayer::with_weights(
                3,
                1,
                basis,
                vec![w0, w1, w2],
                vec![0.0; 3 * 3],
            ).unwrap();
            let net = KanNetwork::new(vec![layer]).unwrap();

            let names = ["f0", "f1", "f2"];
            let terms = extract_feature_importance(&net, &names).unwrap();
            prop_assert_eq!(terms.len(), 3);
            prop_assert!(terms[0].l1_norm >= terms[1].l1_norm);
            prop_assert!(terms[1].l1_norm >= terms[2].l1_norm);

            let formula = extract_symbolic_formula(&net, &names).unwrap();
            prop_assert!(!formula.is_empty());
        }
    }
}
