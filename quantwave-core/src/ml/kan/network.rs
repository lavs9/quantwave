//! Kolmogorov-Arnold Network (KAN) multi-layer architecture and streaming indicator.
//!
//! # Overview
//! A [`KanNetwork`] chains multiple [`KanLayer`] instances into a deep
//! Kolmogorov-Arnold Network. Each layer non-linearly maps inputs using learnable
//! univariate spline activations along each edge.
//!
//! A [`KanIndicator`] wraps a scalar-output [`KanNetwork`] (where final `out_features == 1`)
//! and implements the QuantWave [`Next<&[f64]>`](crate::traits::Next) streaming contract
//! with zero heap allocations during the hot path.
//!
//! # References
//! - Liu, Z. et al. (2024). *KAN: Kolmogorov-Arnold Networks*. arXiv:2404.19756.
//! - Oad et al. (2026). *KASPER: Kolmogorov Arnold Networks for Stock Prediction*. arXiv:2507.18983.

use crate::ml::error::MlError;
use crate::ml::kan::layer::KanLayer;
use crate::traits::Next;
use serde::{Deserialize, Serialize};

/// A deep Kolmogorov-Arnold Network composed of sequential [`KanLayer`]s.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KanNetwork {
    layers: Vec<KanLayer>,
}

impl KanNetwork {
    /// Creates a new `KanNetwork` from a sequential list of [`KanLayer`]s.
    ///
    /// # Errors
    /// Returns `MlError::EmptyNetwork` if `layers` is empty.
    /// Returns `MlError::DimensionMismatch` if the output dimension of layer $l$
    /// does not match the input dimension of layer $l+1$.
    pub fn new(layers: Vec<KanLayer>) -> Result<Self, MlError> {
        if layers.is_empty() {
            return Err(MlError::EmptyNetwork);
        }

        for i in 0..layers.len() - 1 {
            let out_dim = layers[i].out_features();
            let next_in_dim = layers[i + 1].in_features();
            if out_dim != next_in_dim {
                return Err(MlError::DimensionMismatch {
                    expected: out_dim,
                    got: next_in_dim,
                });
            }
        }

        Ok(Self { layers })
    }

    /// Returns a slice of the layers in this network.
    #[inline]
    #[must_use]
    pub fn layers(&self) -> &[KanLayer] {
        &self.layers
    }

    /// Returns a mutable slice of the layers in this network.
    #[inline]
    pub fn layers_mut(&mut self) -> &mut [KanLayer] {
        &mut self.layers
    }

    /// Total number of layers in the network.
    #[inline]
    #[must_use]
    pub fn num_layers(&self) -> usize {
        self.layers.len()
    }

    /// Number of input features to the first layer.
    #[inline]
    #[must_use]
    pub fn in_features(&self) -> usize {
        self.layers[0].in_features()
    }

    /// Number of output features from the final layer.
    #[inline]
    #[must_use]
    pub fn out_features(&self) -> usize {
        self.layers[self.layers.len() - 1].out_features()
    }

    /// Evaluates the network in-place, writing outputs to `out`.
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if `inputs.len() != self.in_features()`
    /// or `out.len() != self.out_features()`.
    pub fn forward_into(&self, inputs: &[f64], out: &mut [f64]) -> Result<(), MlError> {
        if inputs.len() != self.in_features() {
            return Err(MlError::DimensionMismatch {
                expected: self.in_features(),
                got: inputs.len(),
            });
        }
        if out.len() != self.out_features() {
            return Err(MlError::DimensionMismatch {
                expected: self.out_features(),
                got: out.len(),
            });
        }

        let num_layers = self.layers.len();
        if num_layers == 1 {
            return self.layers[0].forward_into(inputs, out);
        }

        // Multi-layer evaluation ping-ponging between two intermediate buffers
        let max_inter = self.layers[..num_layers - 1]
            .iter()
            .map(|l| l.out_features())
            .max()
            .unwrap_or(0);

        let mut buf_a = vec![0.0; max_inter];
        let mut buf_b = vec![0.0; max_inter];

        let out0_dim = self.layers[0].out_features();
        self.layers[0].forward_into(inputs, &mut buf_a[..out0_dim])?;

        let mut in_b = false;
        for l in 1..num_layers - 1 {
            let in_dim = self.layers[l].in_features();
            let out_dim = self.layers[l].out_features();
            if in_b {
                self.layers[l].forward_into(&buf_b[..in_dim], &mut buf_a[..out_dim])?;
                in_b = false;
            } else {
                self.layers[l].forward_into(&buf_a[..in_dim], &mut buf_b[..out_dim])?;
                in_b = true;
            }
        }

        let last_layer = &self.layers[num_layers - 1];
        let in_dim = last_layer.in_features();
        let last_in = if in_b {
            &buf_b[..in_dim]
        } else {
            &buf_a[..in_dim]
        };
        last_layer.forward_into(last_in, out)?;

        Ok(())
    }

    /// Evaluates the network forward pass and returns an allocated [`Vec<f64>`].
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if `inputs.len() != self.in_features()`.
    pub fn forward(&self, inputs: &[f64]) -> Result<Vec<f64>, MlError> {
        let mut out = vec![0.0; self.out_features()];
        self.forward_into(inputs, &mut out)?;
        Ok(out)
    }
}

/// A streaming technical indicator backed by a scalar-output [`KanNetwork`].
///
/// Implements [`Next<&[f64]>`](crate::traits::Next) and [`Next<&Vec<f64>>`](crate::traits::Next),
/// producing an `f64` output at each step. Intermediate buffers are pre-allocated at
/// construction, ensuring zero heap allocations on the streaming hot path.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KanIndicator {
    network: KanNetwork,
    #[serde(skip)]
    intermediate_a: Vec<f64>,
    #[serde(skip)]
    intermediate_b: Vec<f64>,
}

impl KanIndicator {
    /// Creates a new `KanIndicator` wrapping a `KanNetwork`.
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if the network's output dimension is not 1.
    pub fn new(network: KanNetwork) -> Result<Self, MlError> {
        let out_dim = network.out_features();
        if out_dim != 1 {
            return Err(MlError::DimensionMismatch {
                expected: 1,
                got: out_dim,
            });
        }

        let num_layers = network.num_layers();
        let max_inter = if num_layers > 1 {
            network.layers()[..num_layers - 1]
                .iter()
                .map(|l| l.out_features())
                .max()
                .unwrap_or(0)
        } else {
            0
        };

        Ok(Self {
            network,
            intermediate_a: vec![0.0; max_inter],
            intermediate_b: vec![0.0; max_inter],
        })
    }

    /// Ensures scratch buffers are sized properly (e.g. after deserialization).
    fn ensure_buffers(&mut self) {
        let num_layers = self.network.num_layers();
        if num_layers > 1 {
            let max_inter = self.network.layers()[..num_layers - 1]
                .iter()
                .map(|l| l.out_features())
                .max()
                .unwrap_or(0);
            if self.intermediate_a.len() < max_inter {
                self.intermediate_a.resize(max_inter, 0.0);
            }
            if self.intermediate_b.len() < max_inter {
                self.intermediate_b.resize(max_inter, 0.0);
            }
        }
    }

    /// Reference to the underlying network.
    #[inline]
    #[must_use]
    pub fn network(&self) -> &KanNetwork {
        &self.network
    }

    /// Mutable reference to the underlying network.
    #[inline]
    pub fn network_mut(&mut self) -> &mut KanNetwork {
        &mut self.network
    }

    /// Number of expected input features.
    #[inline]
    #[must_use]
    pub fn in_features(&self) -> usize {
        self.network.in_features()
    }

    /// Evaluates the next indicator tick fallibly.
    ///
    /// # Performance
    /// Zero heap allocations. Uses pre-allocated intermediate ping-pong buffers.
    ///
    /// # Errors
    /// Returns `MlError::DimensionMismatch` if `input.len() != self.in_features()`.
    pub fn try_next(&mut self, input: &[f64]) -> Result<f64, MlError> {
        if input.len() != self.network.in_features() {
            return Err(MlError::DimensionMismatch {
                expected: self.network.in_features(),
                got: input.len(),
            });
        }

        self.ensure_buffers();

        let num_layers = self.network.num_layers();
        if num_layers == 1 {
            let mut out = [0.0; 1];
            self.network.layers[0].forward_into(input, &mut out)?;
            return Ok(out[0]);
        }

        let out0_dim = self.network.layers[0].out_features();
        self.network.layers[0].forward_into(input, &mut self.intermediate_a[..out0_dim])?;

        let mut in_b = false;
        for l in 1..num_layers - 1 {
            let in_dim = self.network.layers[l].in_features();
            let out_dim = self.network.layers[l].out_features();
            if in_b {
                self.network.layers[l].forward_into(
                    &self.intermediate_b[..in_dim],
                    &mut self.intermediate_a[..out_dim],
                )?;
                in_b = false;
            } else {
                self.network.layers[l].forward_into(
                    &self.intermediate_a[..in_dim],
                    &mut self.intermediate_b[..out_dim],
                )?;
                in_b = true;
            }
        }

        let last_layer = &self.network.layers[num_layers - 1];
        let in_dim = last_layer.in_features();
        let last_in = if in_b {
            &self.intermediate_b[..in_dim]
        } else {
            &self.intermediate_a[..in_dim]
        };
        let mut out = [0.0; 1];
        last_layer.forward_into(last_in, &mut out)?;
        Ok(out[0])
    }

    /// Evaluates the indicator on a slice of inputs.
    #[inline]
    pub fn eval(&mut self, input: &[f64]) -> f64 {
        self.next(input)
    }
}

impl Next<&[f64]> for KanIndicator {
    type Output = f64;

    fn next(&mut self, input: &[f64]) -> Self::Output {
        self.try_next(input).unwrap_or(f64::NAN)
    }
}

impl Next<&Vec<f64>> for KanIndicator {
    type Output = f64;

    fn next(&mut self, input: &Vec<f64>) -> Self::Output {
        self.next(input.as_slice())
    }
}

impl Next<f64> for KanIndicator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.next(&[input][..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_validation() {
        assert!(KanNetwork::new(vec![]).is_err());

        let layer1 = KanLayer::new(3, 4, 4, 3, -2.0, 2.0).unwrap();
        let layer2_wrong = KanLayer::new(5, 1, 4, 3, -2.0, 2.0).unwrap();
        // 4 out != 5 in
        assert!(KanNetwork::new(vec![layer1.clone(), layer2_wrong]).is_err());

        let layer2_ok = KanLayer::new(4, 1, 4, 3, -2.0, 2.0).unwrap();
        let net = KanNetwork::new(vec![layer1, layer2_ok]).unwrap();
        assert_eq!(net.num_layers(), 2);
        assert_eq!(net.in_features(), 3);
        assert_eq!(net.out_features(), 1);
    }

    #[test]
    fn test_indicator_validation() {
        // Multi-output network cannot become a KanIndicator
        let layer = KanLayer::new(2, 3, 4, 3, -1.0, 1.0).unwrap();
        let net = KanNetwork::new(vec![layer]).unwrap();
        assert!(KanIndicator::new(net).is_err());

        // Single-output network creates KanIndicator successfully
        let layer1 = KanLayer::new(2, 3, 4, 3, -1.0, 1.0).unwrap();
        let layer2 = KanLayer::new(3, 1, 4, 3, -1.0, 1.0).unwrap();
        let net = KanNetwork::new(vec![layer1, layer2]).unwrap();
        let indicator = KanIndicator::new(net);
        assert!(indicator.is_ok());
    }

    #[test]
    fn test_network_and_indicator_forward_parity() {
        let layer1 = KanLayer::new(3, 4, 4, 2, -2.0, 2.0).unwrap();
        let layer2 = KanLayer::new(4, 2, 4, 2, -2.0, 2.0).unwrap();
        let layer3 = KanLayer::new(2, 1, 4, 2, -2.0, 2.0).unwrap();
        let net = KanNetwork::new(vec![layer1, layer2, layer3]).unwrap();

        let inputs = [0.5, -1.2, 1.8];
        let net_out = net.forward(&inputs).unwrap();
        assert_eq!(net_out.len(), 1);

        let mut indicator = KanIndicator::new(net).unwrap();
        let ind_out = indicator.next(&inputs[..]);
        assert_eq!(net_out[0], ind_out);

        // Test Next<&Vec<f64>>
        let vec_input = inputs.to_vec();
        let ind_vec_out = indicator.next(&vec_input);
        assert_eq!(ind_out, ind_vec_out);
    }

    #[test]
    fn test_indicator_invalid_input_returns_nan() {
        let layer = KanLayer::new(3, 1, 4, 2, -1.0, 1.0).unwrap();
        let net = KanNetwork::new(vec![layer]).unwrap();
        let mut indicator = KanIndicator::new(net).unwrap();

        // Pass 2 inputs when 3 are expected
        let out = indicator.next(&[1.0, 2.0][..]);
        assert!(out.is_nan());
    }

    #[test]
    fn test_single_input_indicator_next_f64() {
        let layer = KanLayer::new(1, 1, 4, 2, -2.0, 2.0).unwrap();
        let net = KanNetwork::new(vec![layer]).unwrap();
        let mut indicator = KanIndicator::new(net).unwrap();

        let out_f64 = indicator.next(1.5);
        let out_slice = indicator.next(&[1.5][..]);
        assert_eq!(out_f64, out_slice);
    }

    #[test]
    fn test_network_and_indicator_serde() {
        let layer1 = KanLayer::new(2, 3, 3, 2, -5.0, 5.0).unwrap();
        let layer2 = KanLayer::new(3, 1, 3, 2, -5.0, 5.0).unwrap();
        let net = KanNetwork::new(vec![layer1, layer2]).unwrap();
        let mut indicator = KanIndicator::new(net).unwrap();

        let json = serde_json::to_string(&indicator).unwrap();
        let mut deserialized: KanIndicator = serde_json::from_str(&json).unwrap();

        let input = [1.0, -2.0];
        assert_eq!(indicator.next(&input[..]), deserialized.next(&input[..]));
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_indicator_streaming_batch_parity(
            raw_inputs in prop::collection::vec(
                prop::collection::vec(-5.0f64..5.0, 2..3),
                10..30
            )
        ) {
            let layer1 = KanLayer::new(2, 3, 3, 2, -5.0, 5.0).expect("layer1");
            let layer2 = KanLayer::new(3, 1, 3, 2, -5.0, 5.0).expect("layer2");
            let net = KanNetwork::new(vec![layer1, layer2]).expect("network");
            let mut indicator = KanIndicator::new(net.clone()).expect("indicator");

            for tick_input in &raw_inputs {
                let net_out = net.forward(tick_input).expect("forward");
                let ind_out = indicator.next(tick_input.as_slice());
                prop_assert!(ind_out.is_finite());
                prop_assert!((net_out[0] - ind_out).abs() < 1e-12);
            }
        }
    }
}
