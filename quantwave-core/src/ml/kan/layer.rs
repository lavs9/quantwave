//! Kolmogorov-Arnold Network (KAN) Layer implementation.
//!
//! A KAN layer applies non-linear learnable 1D edge activation functions $\phi_{i,j}(x_i)$
//! on each incoming connection $(i \to j)$ instead of fixed node activations:
//!
//! $$\phi_{i,j}(x_i) = w_{j,i} \cdot \text{SiLU}(x_i) + \sum_{g=0}^{G-1} c_{j,i,g} B_g(x_i)$$
//!
//! The output of node $j$ is the sum of activations across all input features:
//!
//! $$y_j = \sum_{i=0}^{\text{in\_features}-1} \phi_{i,j}(x_i)$$
//!
//! # References
//! - Liu, Z. et al. (2024). *KAN: Kolmogorov-Arnold Networks*. arXiv:2404.19756.
//! - Oad et al. (2026). *KASPER: Kolmogorov Arnold Networks for Stock Prediction*. arXiv:2507.18983.

use crate::ml::bspline::BSplineBasis;
use crate::ml::error::MlError;
use serde::{Deserialize, Serialize};

/// Numerically stable Sigmoid Linear Unit (SiLU) activation function:
///
/// $$\text{SiLU}(x) = \frac{x}{1 + e^{-x}}$$
#[inline]
pub fn silu(x: f64) -> f64 {
    if !x.is_finite() {
        return x;
    }
    if x >= 0.0 {
        x / (1.0 + (-x).exp())
    } else {
        let exp_x = x.exp();
        x * exp_x / (1.0 + exp_x)
    }
}

/// A single layer in a Kolmogorov-Arnold Network (KAN).
///
/// Contains learnable 1D continuous functions along each edge $(i \to j)$ parameterised
/// by a base residual weight $w_{j,i}$ (acting on $\text{SiLU}$) and a linear combination
/// of B-spline basis functions $B_g(x)$ with coefficients $c_{j,i,g}$.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KanLayer {
    /// Number of input features $I \ge 1$.
    in_features: usize,
    /// Number of output features $J \ge 1$.
    out_features: usize,
    /// Shared 1D uniform B-spline basis on `[x_min, x_max]`.
    basis: BSplineBasis,
    /// Base linear residual weights $w_{j,i}$, size `out_features * in_features`.
    /// Flat row-major: index is `j * in_features + i`.
    base_weights: Vec<f64>,
    /// Spline expansion coefficients $c_{j,i,g}$, size `out_features * in_features * basis.num_basis()`.
    /// Flat row-major: index is `(j * in_features + i) * basis.num_basis() + g`.
    spline_weights: Vec<f64>,
}

impl KanLayer {
    /// Creates a new `KanLayer` with deterministic initialization.
    ///
    /// Base weights $w_{j,i}$ are initialized to `1.0`, and spline coefficients $c_{j,i,g}$
    /// are initialized to small deterministic pseudo-random values in $[-0.02, 0.02]$.
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if `in_features == 0` or `out_features == 0`.
    /// Returns `MlError` from [`BSplineBasis::new`] if basis parameters are invalid.
    pub fn new(
        in_features: usize,
        out_features: usize,
        num_intervals: usize,
        degree: usize,
        x_min: f64,
        x_max: f64,
    ) -> Result<Self, MlError> {
        if in_features == 0 {
            return Err(MlError::DimensionMismatch {
                expected: 1,
                got: 0,
            });
        }
        if out_features == 0 {
            return Err(MlError::DimensionMismatch {
                expected: 1,
                got: 0,
            });
        }

        let basis = BSplineBasis::new(num_intervals, degree, x_min, x_max)?;
        let num_edges = out_features * in_features;
        let base_weights = vec![1.0; num_edges];

        let num_basis = basis.num_basis();
        let num_spline_weights = num_edges * num_basis;
        let mut spline_weights = Vec::with_capacity(num_spline_weights);

        // Deterministic pseudo-random initialization (LCG)
        for idx in 0..num_spline_weights {
            let pseudo = (((idx.wrapping_mul(1_103_515_245).wrapping_add(12_345)) >> 16) & 0x7fff)
                as f64
                / 32767.0;
            spline_weights.push((pseudo - 0.5) * 0.04);
        }

        Ok(Self {
            in_features,
            out_features,
            basis,
            base_weights,
            spline_weights,
        })
    }

    /// Creates a new `KanLayer` with custom explicit weights.
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if `in_features == 0` or `out_features == 0`.
    /// Returns `MlError::CoefficientMismatch` if the lengths of `base_weights` or `spline_weights`
    /// do not match the expected layer dimensions.
    pub fn with_weights(
        in_features: usize,
        out_features: usize,
        basis: BSplineBasis,
        base_weights: Vec<f64>,
        spline_weights: Vec<f64>,
    ) -> Result<Self, MlError> {
        if in_features == 0 {
            return Err(MlError::DimensionMismatch {
                expected: 1,
                got: 0,
            });
        }
        if out_features == 0 {
            return Err(MlError::DimensionMismatch {
                expected: 1,
                got: 0,
            });
        }

        let expected_base = out_features * in_features;
        if base_weights.len() != expected_base {
            return Err(MlError::CoefficientMismatch {
                expected: expected_base,
                got: base_weights.len(),
            });
        }

        let expected_spline = expected_base * basis.num_basis();
        if spline_weights.len() != expected_spline {
            return Err(MlError::CoefficientMismatch {
                expected: expected_spline,
                got: spline_weights.len(),
            });
        }

        Ok(Self {
            in_features,
            out_features,
            basis,
            base_weights,
            spline_weights,
        })
    }

    /// Number of input features $I$.
    #[inline]
    #[must_use]
    pub fn in_features(&self) -> usize {
        self.in_features
    }

    /// Number of output features $J$.
    #[inline]
    #[must_use]
    pub fn out_features(&self) -> usize {
        self.out_features
    }

    /// Reference to the underlying B-spline basis.
    #[inline]
    #[must_use]
    pub fn basis(&self) -> &BSplineBasis {
        &self.basis
    }

    /// Slice of all base residual weights $w_{j,i}$.
    #[inline]
    #[must_use]
    pub fn base_weights(&self) -> &[f64] {
        &self.base_weights
    }

    /// Mutable slice of all base residual weights.
    #[inline]
    pub fn base_weights_mut(&mut self) -> &mut [f64] {
        &mut self.base_weights
    }

    /// Slice of all spline coefficients $c_{j,i,g}$.
    #[inline]
    #[must_use]
    pub fn spline_weights(&self) -> &[f64] {
        &self.spline_weights
    }

    /// Mutable slice of all spline coefficients.
    #[inline]
    pub fn spline_weights_mut(&mut self) -> &mut [f64] {
        &mut self.spline_weights
    }

    /// Returns the base residual weight for edge $(i \to j)$.
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if indices are out of bounds.
    pub fn base_weight(&self, in_idx: usize, out_idx: usize) -> Result<f64, MlError> {
        if in_idx >= self.in_features {
            return Err(MlError::DimensionMismatch {
                expected: self.in_features,
                got: in_idx,
            });
        }
        if out_idx >= self.out_features {
            return Err(MlError::DimensionMismatch {
                expected: self.out_features,
                got: out_idx,
            });
        }
        Ok(self.base_weights[out_idx * self.in_features + in_idx])
    }

    /// Returns the spline coefficients for edge $(i \to j)$.
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if indices are out of bounds.
    pub fn spline_weights_for_edge(
        &self,
        in_idx: usize,
        out_idx: usize,
    ) -> Result<&[f64], MlError> {
        if in_idx >= self.in_features {
            return Err(MlError::DimensionMismatch {
                expected: self.in_features,
                got: in_idx,
            });
        }
        if out_idx >= self.out_features {
            return Err(MlError::DimensionMismatch {
                expected: self.out_features,
                got: out_idx,
            });
        }
        let num_basis = self.basis.num_basis();
        let offset = (out_idx * self.in_features + in_idx) * num_basis;
        Ok(&self.spline_weights[offset..offset + num_basis])
    }

    /// Computes the $L_1$ norm of edge $(i \to j)$:
    ///
    /// $$L_1(\phi_{i,j}) = |w_{j,i}| + \sum_{g=0}^{G-1} |c_{j,i,g}|$$
    ///
    /// Returns `0.0` if indices are out of bounds.
    #[must_use]
    pub fn edge_l1_norm(&self, input_idx: usize, output_idx: usize) -> f64 {
        if input_idx >= self.in_features || output_idx >= self.out_features {
            return 0.0;
        }
        let base_idx = output_idx * self.in_features + input_idx;
        let num_basis = self.basis.num_basis();
        let spline_offset = base_idx * num_basis;

        let base_l1 = self.base_weights[base_idx].abs();
        let spline_slice = &self.spline_weights[spline_offset..spline_offset + num_basis];
        let spline_l1: f64 = spline_slice.iter().map(|w| w.abs()).sum();

        base_l1 + spline_l1
    }

    /// Fallible version of [`Self::edge_l1_norm`] returning explicit [`MlError`]
    /// on index mismatch.
    pub fn try_edge_l1_norm(&self, input_idx: usize, output_idx: usize) -> Result<f64, MlError> {
        if input_idx >= self.in_features {
            return Err(MlError::DimensionMismatch {
                expected: self.in_features,
                got: input_idx,
            });
        }
        if output_idx >= self.out_features {
            return Err(MlError::DimensionMismatch {
                expected: self.out_features,
                got: output_idx,
            });
        }
        Ok(self.edge_l1_norm(input_idx, output_idx))
    }

    /// Evaluates the layer activations in-place into `out`.
    ///
    /// # Performance
    /// Zero heap allocations when `basis.num_basis() <= 64`. For each input feature $x_i$,
    /// both $\text{SiLU}(x_i)$ and B-spline basis functions $B_g(x_i)$ are evaluated exactly once
    /// and reused across all output nodes.
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if `inputs.len() != self.in_features` or
    /// `out.len() != self.out_features`.
    pub fn forward_into(&self, inputs: &[f64], out: &mut [f64]) -> Result<(), MlError> {
        if inputs.len() != self.in_features {
            return Err(MlError::DimensionMismatch {
                expected: self.in_features,
                got: inputs.len(),
            });
        }
        if out.len() != self.out_features {
            return Err(MlError::DimensionMismatch {
                expected: self.out_features,
                got: out.len(),
            });
        }

        out[..self.out_features].fill(0.0);
        let num_basis = self.basis.num_basis();

        let mut stack_basis = [0.0; 64];
        let mut heap_basis = if num_basis > 64 {
            vec![0.0; num_basis]
        } else {
            Vec::new()
        };

        for (i, &x_i) in inputs.iter().enumerate() {
            let silu_x = silu(x_i);
            let basis_buf: &mut [f64] = if num_basis <= 64 {
                &mut stack_basis[..num_basis]
            } else {
                &mut heap_basis[..num_basis]
            };
            self.basis.eval_into(x_i, basis_buf);

            for j in 0..self.out_features {
                let base_idx = j * self.in_features + i;
                let base_w = self.base_weights[base_idx];
                let spline_offset = base_idx * num_basis;
                let spline_slice = &self.spline_weights[spline_offset..spline_offset + num_basis];

                let mut spline_sum = 0.0;
                for (&w, &b) in spline_slice.iter().zip(basis_buf.iter()) {
                    spline_sum += w * b;
                }

                let phi = base_w * silu_x + spline_sum;
                out[j] += phi;
            }
        }

        Ok(())
    }

    /// Evaluates the layer activations and returns an allocated [`Vec<f64>`].
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if `inputs.len() != self.in_features`.
    pub fn forward(&self, inputs: &[f64]) -> Result<Vec<f64>, MlError> {
        let mut out = vec![0.0; self.out_features];
        self.forward_into(inputs, &mut out)?;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_construction_validation() {
        assert!(KanLayer::new(0, 2, 5, 3, -1.0, 1.0).is_err());
        assert!(KanLayer::new(2, 0, 5, 3, -1.0, 1.0).is_err());
        assert!(KanLayer::new(2, 2, 0, 3, -1.0, 1.0).is_err());
        assert!(KanLayer::new(2, 2, 5, 12, -1.0, 1.0).is_err());

        let layer = KanLayer::new(2, 3, 5, 3, -2.0, 2.0).expect("valid layer");
        assert_eq!(layer.in_features(), 2);
        assert_eq!(layer.out_features(), 3);
        assert_eq!(layer.base_weights().len(), 6);
        assert_eq!(layer.spline_weights().len(), 6 * 8); // 5 + 3 = 8 basis funcs
    }

    #[test]
    fn test_with_weights_mismatch() {
        let basis = BSplineBasis::new(5, 3, -1.0, 1.0).unwrap();
        let base_correct = vec![1.0; 2]; // 1 in, 2 out -> 2 base weights
        let spline_correct = vec![0.1; 2 * 8]; // 2 * 8 = 16

        // Wrong base length
        assert!(
            KanLayer::with_weights(1, 2, basis.clone(), vec![1.0; 3], spline_correct.clone())
                .is_err()
        );

        // Wrong spline length
        assert!(KanLayer::with_weights(1, 2, basis, base_correct, vec![0.1; 15]).is_err());
    }

    #[test]
    fn test_edge_l1_norm() {
        let basis = BSplineBasis::new(3, 1, -1.0, 1.0).unwrap(); // 3 + 1 = 4 basis
        let base_weights = vec![1.5, -2.0]; // 2 in, 1 out
        let spline_weights = vec![
            0.1, -0.2, 0.3, -0.4, // edge (0 -> 0): sum = 1.0
            -0.5, 0.5, -0.5, 0.5, // edge (1 -> 0): sum = 2.0
        ];
        let layer =
            KanLayer::with_weights(2, 1, basis, base_weights, spline_weights).expect("valid layer");

        // Edge 0 -> 0: |1.5| + (0.1 + 0.2 + 0.3 + 0.4) = 1.5 + 1.0 = 2.5
        assert!((layer.edge_l1_norm(0, 0) - 2.5).abs() < 1e-12);
        // Edge 1 -> 0: |-2.0| + 2.0 = 4.0
        assert!((layer.edge_l1_norm(1, 0) - 4.0).abs() < 1e-12);

        // Out of bounds returns 0.0 or Err
        assert_eq!(layer.edge_l1_norm(5, 0), 0.0);
        assert!(layer.try_edge_l1_norm(5, 0).is_err());
    }

    #[test]
    fn test_forward_dimension_validation() {
        let layer = KanLayer::new(2, 3, 5, 3, -1.0, 1.0).unwrap();
        // Dimension mismatch on inputs
        assert!(layer.forward(&[1.0]).is_err());
        assert!(layer.forward(&[1.0, 2.0, 3.0]).is_err());

        // Dimension mismatch on forward_into out
        let mut out_small = [0.0; 2];
        assert!(layer.forward_into(&[1.0, 2.0], &mut out_small).is_err());
    }

    #[test]
    fn test_forward_computation_single_edge() {
        // Construct 1 in -> 1 out with degree 0, 1 interval -> 1 basis func = 1.0 everywhere
        let basis = BSplineBasis::new(1, 0, -5.0, 5.0).unwrap();
        let base_weights = vec![2.0];
        let spline_weights = vec![0.5];
        let layer =
            KanLayer::with_weights(1, 1, basis, base_weights, spline_weights).expect("valid");

        // At x = 0.0: silu(0) = 0.0. Spline sum = 0.5 * 1.0 = 0.5.
        let out = layer.forward(&[0.0]).unwrap();
        assert!((out[0] - 0.5).abs() < 1e-10);

        // At x = 1.0: silu(1.0) = 1.0 / (1.0 + exp(-1.0)) approx 0.7310585786
        // phi(1.0) = 2.0 * silu(1.0) + 0.5
        let expected = 2.0 * silu(1.0) + 0.5;
        let out = layer.forward(&[1.0]).unwrap();
        assert!((out[0] - expected).abs() < 1e-10);
    }

    #[test]
    fn test_forward_and_forward_into_parity() {
        let layer = KanLayer::new(3, 2, 4, 2, -3.0, 3.0).unwrap();
        let inputs = [-1.5, 0.4, 2.2];

        let out_alloc = layer.forward(&inputs).unwrap();
        let mut out_into = [0.0; 2];
        layer.forward_into(&inputs, &mut out_into).unwrap();

        assert_eq!(out_alloc.len(), 2);
        for j in 0..2 {
            assert_eq!(out_alloc[j], out_into[j]);
        }
    }

    #[test]
    fn test_layer_serde() {
        let layer = KanLayer::new(2, 2, 4, 3, -2.0, 2.0).unwrap();
        let serialized = serde_json::to_string(&layer).unwrap();
        let deserialized: KanLayer = serde_json::from_str(&serialized).unwrap();

        assert_eq!(layer.in_features(), deserialized.in_features());
        assert_eq!(layer.out_features(), deserialized.out_features());

        let inputs = [0.5, -1.0];
        let out_orig = layer.forward(&inputs).unwrap();
        let out_deser = deserialized.forward(&inputs).unwrap();
        for (a, b) in out_orig.iter().zip(out_deser.iter()) {
            assert!((a - b).abs() < 1e-12);
        }
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_layer_proptest(
            x0 in -10.0f64..10.0,
            x1 in -10.0f64..10.0,
            x2 in -10.0f64..10.0,
        ) {
            let layer = KanLayer::new(3, 2, 4, 3, -10.0, 10.0).expect("valid layer");
            let inputs = [x0, x1, x2];

            let out_alloc = layer.forward(&inputs).expect("forward succeeds");
            let mut out_into = [0.0; 2];
            layer.forward_into(&inputs, &mut out_into).expect("forward_into succeeds");

            prop_assert_eq!(out_alloc.len(), 2);
            for j in 0..2 {
                prop_assert!(out_alloc[j].is_finite());
                prop_assert!((out_alloc[j] - out_into[j]).abs() < 1e-12);
            }

            for i in 0..3 {
                for j in 0..2 {
                    let norm = layer.edge_l1_norm(i, j);
                    prop_assert!(norm >= 0.0);
                    prop_assert!(norm.is_finite());
                }
            }
        }
    }
}
