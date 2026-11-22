//! PyO3 bindings for Kolmogorov-Arnold Networks (KAN) and Technical Indicator Networks (TIN).
//!
//! Provides fast, pure-Rust implementations of learnable indicator architectures
//! and differentiable indicators with analytical recurrence gradients.

use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use quantwave_core::ml::{
    DifferentiableEma, DifferentiableMacd, DifferentiableRsi, KanIndicator, KanLayer, KanNetwork,
    MlError,
};
use quantwave_core::traits::Next;

fn ml_err_to_py(err: MlError) -> PyErr {
    match err {
        MlError::DimensionMismatch { expected, got } => PyValueError::new_err(format!(
            "Dimension mismatch: expected {expected}, got {got}"
        )),
        MlError::InvalidParameter(val) => {
            PyValueError::new_err(format!("Invalid parameter value: {val}"))
        }
        MlError::InvalidGridRange(min, max) => PyValueError::new_err(format!(
            "Invalid grid range: [{min}, {max}] (min must be < max)"
        )),
        MlError::EmptyNetwork => {
            PyValueError::new_err("KAN network cannot be empty (at least 1 layer required)")
        }
        MlError::InvalidGridIntervals(n) => {
            PyValueError::new_err(format!("Grid intervals must be >= 1, got {n}"))
        }
        MlError::InvalidSplineDegree(deg) => {
            PyValueError::new_err(format!("Spline degree must be <= 10, got {deg}"))
        }
        MlError::CoefficientMismatch { expected, got } => PyValueError::new_err(format!(
            "Coefficient count mismatch: expected {expected}, got {got}"
        )),
        MlError::InsufficientData => {
            PyValueError::new_err("Empty input series or insufficient data")
        }
        MlError::Serialization(msg) => PyRuntimeError::new_err(msg),
    }
}

/// A deep Kolmogorov-Arnold Network (KAN) composed of sequential B-spline layers.
#[pyclass(name = "KanNetwork", module = "quantwave.ml")]
#[derive(Clone)]
pub struct PyKanNetwork {
    pub(crate) inner: KanNetwork,
}

#[pymethods]
impl PyKanNetwork {
    /// Creates a new `KanNetwork` from a list of layer dimensions.
    ///
    /// # Arguments
    /// * `layers` - Sequence of dimensions, e.g. `[4, 8, 1]` for 4 inputs, 8 hidden, 1 output.
    /// * `grid_size` - Number of B-spline intervals per edge (default: 5).
    /// * `spline_order` - Polynomial degree $k$ of B-splines (default: 3).
    /// * `x_min` - Minimum domain bound for the spline grid (default: -1.0).
    /// * `x_max` - Maximum domain bound for the spline grid (default: 1.0).
    #[new]
    #[pyo3(signature = (layers, grid_size=5, spline_order=3, x_min=-1.0, x_max=1.0))]
    pub fn new(
        layers: Vec<usize>,
        grid_size: usize,
        spline_order: usize,
        x_min: f64,
        x_max: f64,
    ) -> PyResult<Self> {
        if layers.len() < 2 {
            return Err(PyValueError::new_err(
                "layers must have at least 2 dimensions [in_features, ..., out_features]",
            ));
        }

        let mut kan_layers = Vec::with_capacity(layers.len() - 1);
        for i in 0..layers.len() - 1 {
            let in_dim = layers[i];
            let out_dim = layers[i + 1];
            let layer = KanLayer::new(in_dim, out_dim, grid_size, spline_order, x_min, x_max)
                .map_err(ml_err_to_py)?;
            kan_layers.push(layer);
        }

        let network = KanNetwork::new(kan_layers).map_err(ml_err_to_py)?;
        Ok(Self { inner: network })
    }

    /// Evaluates the network forward pass for a single input vector.
    pub fn forward(&self, inputs: Vec<f64>) -> PyResult<Vec<f64>> {
        self.inner.forward(&inputs).map_err(ml_err_to_py)
    }

    /// Evaluates the network forward pass for a 2D batch of samples.
    pub fn forward_batch(&self, inputs: Vec<Vec<f64>>) -> PyResult<Vec<Vec<f64>>> {
        let mut results = Vec::with_capacity(inputs.len());
        for row in inputs {
            let out = self.inner.forward(&row).map_err(ml_err_to_py)?;
            results.push(out);
        }
        Ok(results)
    }

    /// Returns the number of input features expected by the first layer.
    pub fn in_features(&self) -> usize {
        self.inner.in_features()
    }

    /// Returns the number of output features produced by the final layer.
    pub fn out_features(&self) -> usize {
        self.inner.out_features()
    }

    /// Returns the total number of layers in the network.
    pub fn num_layers(&self) -> usize {
        self.inner.num_layers()
    }

    /// Extracts an interpretable symbolic mathematical formula from the network.
    #[pyo3(signature = (feature_names=None))]
    pub fn extract_symbolic_formula(&self, feature_names: Option<Vec<String>>) -> PyResult<String> {
        let default_names: Vec<String>;
        let names_ref: Vec<&str> = match &feature_names {
            Some(names) => names.iter().map(|s| s.as_str()).collect(),
            None => {
                default_names = (0..self.inner.in_features())
                    .map(|i| format!("x{i}"))
                    .collect();
                default_names.iter().map(|s| s.as_str()).collect()
            }
        };

        quantwave_core::ml::extract_symbolic_formula(&self.inner, &names_ref).map_err(ml_err_to_py)
    }

    /// Extracts ranked feature importance terms with L1 norms and relative scores.
    #[pyo3(signature = (feature_names=None))]
    pub fn extract_feature_importance<'py>(
        &self,
        py: Python<'py>,
        feature_names: Option<Vec<String>>,
    ) -> PyResult<Vec<Bound<'py, PyDict>>> {
        let default_names: Vec<String>;
        let names_ref: Vec<&str> = match &feature_names {
            Some(names) => names.iter().map(|s| s.as_str()).collect(),
            None => {
                default_names = (0..self.inner.in_features())
                    .map(|i| format!("x{i}"))
                    .collect();
                default_names.iter().map(|s| s.as_str()).collect()
            }
        };

        let terms = quantwave_core::ml::extract_feature_importance(&self.inner, &names_ref)
            .map_err(ml_err_to_py)?;
        let mut result = Vec::with_capacity(terms.len());
        for t in terms {
            let dict = PyDict::new(py);
            dict.set_item("feature_index", t.feature_index)?;
            dict.set_item("feature_name", t.feature_name)?;
            dict.set_item("l1_norm", t.l1_norm)?;
            dict.set_item("relative_importance", t.relative_importance)?;
            result.push(dict);
        }
        Ok(result)
    }

    /// Serializes the network topology and weights to a JSON string.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyRuntimeError::new_err(format!("Serialization error: {e}")))
    }

    /// Deserializes a network topology and weights from a JSON string.
    #[staticmethod]
    pub fn from_json(json_str: &str) -> PyResult<Self> {
        let inner: KanNetwork = serde_json::from_str(json_str)
            .map_err(|e| PyValueError::new_err(format!("Deserialization error: {e}")))?;
        Ok(Self { inner })
    }

    fn __repr__(&self) -> String {
        format!(
            "KanNetwork(in_features={}, out_features={}, num_layers={})",
            self.inner.in_features(),
            self.inner.out_features(),
            self.inner.num_layers()
        )
    }
}

/// A streaming technical indicator backed by a scalar-output [`KanNetwork`].
#[pyclass(name = "KanIndicator", module = "quantwave.ml")]
pub struct PyKanIndicator {
    inner: KanIndicator,
}

#[pymethods]
impl PyKanIndicator {
    /// Creates a new `KanIndicator` wrapping a `KanNetwork` (must have out_features == 1).
    #[new]
    pub fn new(network: &PyKanNetwork) -> PyResult<Self> {
        let indicator = KanIndicator::new(network.inner.clone()).map_err(ml_err_to_py)?;
        Ok(Self { inner: indicator })
    }

    /// Evaluates the next indicator tick for a scalar input or feature vector.
    pub fn next(&mut self, input: &Bound<'_, PyAny>) -> PyResult<f64> {
        if let Ok(val) = input.extract::<f64>() {
            let slice = [val];
            return self.inner.try_next(&slice).map_err(ml_err_to_py);
        }
        if let Ok(vec) = input.extract::<Vec<f64>>() {
            return self.inner.try_next(&vec).map_err(ml_err_to_py);
        }
        Err(PyValueError::new_err(
            "Input to KanIndicator.next must be float or Sequence[float]",
        ))
    }

    /// Alias for `.next(input)`.
    pub fn update(&mut self, input: &Bound<'_, PyAny>) -> PyResult<f64> {
        self.next(input)
    }

    /// Number of expected input features.
    pub fn in_features(&self) -> usize {
        self.inner.in_features()
    }

    /// Resets internal scratchpad states.
    pub fn reset(&mut self) {
        // KanIndicator scratch buffers are ping-pong scratchpads re-zeroed during forward
    }

    /// Returns a copy of the underlying `KanNetwork`.
    pub fn network(&self) -> PyKanNetwork {
        PyKanNetwork {
            inner: self.inner.network().clone(),
        }
    }

    /// Evaluates a 2D batch of samples sequentially.
    pub fn batch(&mut self, inputs: Vec<Vec<f64>>) -> PyResult<Vec<f64>> {
        let mut out = Vec::with_capacity(inputs.len());
        for row in inputs {
            let val = self.inner.try_next(&row).map_err(ml_err_to_py)?;
            out.push(val);
        }
        Ok(out)
    }

    fn __repr__(&self) -> String {
        format!(
            "KanIndicator(in_features={})",
            self.inner.network().in_features()
        )
    }
}

/// Continuous differentiable Exponential Moving Average with analytical parameter gradients.
#[pyclass(name = "DifferentiableEma", module = "quantwave.ml")]
pub struct PyDifferentiableEma {
    inner: DifferentiableEma,
}

#[pymethods]
impl PyDifferentiableEma {
    /// Creates a new `DifferentiableEma` with continuous span $\tau > 0$ (default: 10.0).
    #[new]
    #[pyo3(signature = (span=10.0))]
    pub fn new(span: f64) -> PyResult<Self> {
        let inner = DifferentiableEma::new(span).map_err(ml_err_to_py)?;
        Ok(Self { inner })
    }

    /// Steps the EMA by one bar, returning `(smoothed_value, grad_span)`.
    pub fn step(&mut self, input: f64) -> (f64, f64) {
        let val = self.inner.next(input);
        let grad = self.inner.grad_span();
        (val, grad)
    }

    /// Evaluates the next bar and returns only the smoothed value.
    pub fn next(&mut self, input: f64) -> f64 {
        self.inner.next(input)
    }

    /// Current continuous span $\tau$.
    #[getter]
    pub fn span(&self) -> f64 {
        self.inner.span()
    }

    /// Sets the continuous span $\tau$.
    #[setter]
    pub fn set_span(&mut self, span: f64) -> PyResult<()> {
        self.inner.set_span(span).map_err(ml_err_to_py)
    }

    /// Current smoothing factor $\alpha = \frac{2}{\tau + 1}$.
    #[getter]
    pub fn alpha(&self) -> f64 {
        self.inner.alpha()
    }

    /// Current analytical gradient $\frac{\partial y_t}{\partial \tau}$ with respect to span.
    #[getter]
    pub fn grad_span(&self) -> f64 {
        self.inner.grad_span()
    }

    /// Current analytical gradient $\frac{\partial y_t}{\partial \alpha}$ with respect to alpha.
    #[getter]
    pub fn grad_alpha(&self) -> f64 {
        self.inner.grad_alpha()
    }

    /// Number of bars consumed since initialization or last reset.
    #[getter]
    pub fn bars_consumed(&self) -> usize {
        self.inner.bars_consumed()
    }

    /// Updates span $\tau$ along the negative gradient of a downstream loss via online SGD.
    #[pyo3(signature = (learning_rate, loss_gradient, min_span=1.0, max_span=100.0))]
    pub fn update_span(
        &mut self,
        learning_rate: f64,
        loss_gradient: f64,
        min_span: f64,
        max_span: f64,
    ) -> f64 {
        self.inner
            .update_span(learning_rate, loss_gradient, min_span, max_span);
        self.inner.span()
    }

    /// Resets internal accumulator states, clearing history while preserving span.
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Evaluates a 1D sequence of prices in batch, returning `(smoothed_values, grad_spans)`.
    pub fn batch(&mut self, inputs: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
        let n = inputs.len();
        let mut values = Vec::with_capacity(n);
        let mut grads = Vec::with_capacity(n);
        for x in inputs {
            let val = self.inner.next(x);
            values.push(val);
            grads.push(self.inner.grad_span());
        }
        (values, grads)
    }

    fn __repr__(&self) -> String {
        format!(
            "DifferentiableEma(span={:.4}, alpha={:.4}, bars={})",
            self.inner.span(),
            self.inner.alpha(),
            self.inner.bars_consumed()
        )
    }
}

/// Continuous differentiable Relative Strength Index with analytical parameter gradients.
#[pyclass(name = "DifferentiableRsi", module = "quantwave.ml")]
pub struct PyDifferentiableRsi {
    inner: DifferentiableRsi,
}

#[pymethods]
impl PyDifferentiableRsi {
    /// Creates a new `DifferentiableRsi` with continuous span $\tau > 0$ and thresholds.
    #[new]
    #[pyo3(signature = (span=14.0, overbought=70.0, oversold=30.0, steepness=0.1))]
    pub fn new(span: f64, overbought: f64, oversold: f64, steepness: f64) -> PyResult<Self> {
        let mut inner = DifferentiableRsi::new(span, overbought, oversold).map_err(ml_err_to_py)?;
        inner.set_steepness(steepness).map_err(ml_err_to_py)?;
        Ok(Self { inner })
    }

    /// Steps the RSI by one price bar, returning `(rsi, signal, grad_span)`.
    pub fn step(&mut self, price: f64) -> (f64, f64, f64) {
        let out = self.inner.next(price);
        (out.rsi, out.signal, out.grad_span)
    }

    /// Evaluates the next bar and returns only the RSI value.
    pub fn next(&mut self, price: f64) -> f64 {
        self.inner.next(price).rsi
    }

    /// Current continuous span $\tau$.
    #[getter]
    pub fn span(&self) -> f64 {
        self.inner.span()
    }

    /// Sets the continuous span $\tau$.
    #[setter]
    pub fn set_span(&mut self, span: f64) -> PyResult<()> {
        self.inner.set_span(span).map_err(ml_err_to_py)
    }

    /// Upper overbought threshold $\theta_{\text{ob}}$.
    #[getter]
    pub fn overbought(&self) -> f64 {
        self.inner.overbought()
    }

    /// Lower oversold threshold $\theta_{\text{os}}$.
    #[getter]
    pub fn oversold(&self) -> f64 {
        self.inner.oversold()
    }

    /// Sigmoid steepness parameter $\beta > 0$.
    #[getter]
    pub fn steepness(&self) -> f64 {
        self.inner.steepness()
    }

    /// Sets the sigmoid steepness parameter $\beta$.
    #[setter]
    pub fn set_steepness(&mut self, steepness: f64) -> PyResult<()> {
        self.inner.set_steepness(steepness).map_err(ml_err_to_py)
    }

    /// Current analytical gradient $\frac{\partial \text{RSI}}{\partial \tau}$ with respect to span.
    #[getter]
    pub fn grad_span(&self) -> f64 {
        self.inner.grad_span()
    }

    /// Most recent continuous trading signal $\in [-1.0, 1.0]$.
    #[getter]
    pub fn signal(&self) -> f64 {
        self.inner.last_output().map(|o| o.signal).unwrap_or(0.0)
    }

    /// Number of bars consumed since initialization or last reset.
    #[getter]
    pub fn bars_consumed(&self) -> usize {
        self.inner.bars_consumed()
    }

    /// Updates span $\tau$ along the negative gradient of a downstream loss via online SGD.
    #[pyo3(signature = (learning_rate, loss_gradient, min_span=2.0, max_span=100.0))]
    pub fn update_span(
        &mut self,
        learning_rate: f64,
        loss_gradient: f64,
        min_span: f64,
        max_span: f64,
    ) -> f64 {
        self.inner
            .update_span(learning_rate, loss_gradient, min_span, max_span);
        self.inner.span()
    }

    /// Resets internal smoothers, prices, and bar counters.
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Evaluates a 1D sequence of prices in batch, returning `(rsi_values, signal_values, grad_spans)`.
    pub fn batch(&mut self, prices: Vec<f64>) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let n = prices.len();
        let mut rsi_vals = Vec::with_capacity(n);
        let mut sig_vals = Vec::with_capacity(n);
        let mut grad_vals = Vec::with_capacity(n);
        for p in prices {
            let out = self.inner.next(p);
            rsi_vals.push(out.rsi);
            sig_vals.push(out.signal);
            grad_vals.push(out.grad_span);
        }
        (rsi_vals, sig_vals, grad_vals)
    }

    fn __repr__(&self) -> String {
        format!(
            "DifferentiableRsi(span={:.4}, ob={:.1}, os={:.1}, bars={})",
            self.inner.span(),
            self.inner.overbought(),
            self.inner.oversold(),
            self.inner.bars_consumed()
        )
    }
}

/// Continuous differentiable Moving Average Convergence Divergence with analytical parameter gradients.
#[pyclass(name = "DifferentiableMacd", module = "quantwave.ml")]
pub struct PyDifferentiableMacd {
    inner: DifferentiableMacd,
}

#[pymethods]
impl PyDifferentiableMacd {
    /// Creates a new `DifferentiableMacd` with continuous fast, slow, and signal spans.
    #[new]
    #[pyo3(signature = (fast_span=12.0, slow_span=26.0, signal_span=9.0))]
    pub fn new(fast_span: f64, slow_span: f64, signal_span: f64) -> PyResult<Self> {
        let inner =
            DifferentiableMacd::new(fast_span, slow_span, signal_span).map_err(ml_err_to_py)?;
        Ok(Self { inner })
    }

    /// Steps the MACD by one price bar, returning `(macd_line, signal_line, histogram, grad_fast, grad_slow, grad_signal)`.
    pub fn step(&mut self, price: f64) -> (f64, f64, f64, f64, f64, f64) {
        let out = self.inner.next(price);
        (
            out.macd_line,
            out.signal_line,
            out.histogram,
            self.inner.grad_macd_fast_span(),
            self.inner.grad_macd_slow_span(),
            self.inner.grad_signal_span(),
        )
    }

    /// Evaluates the next bar and returns only the MACD line.
    pub fn next(&mut self, price: f64) -> f64 {
        self.inner.next(price).macd_line
    }

    /// Fast EMA span $\tau_{\text{fast}}$.
    #[getter]
    pub fn fast_span(&self) -> f64 {
        self.inner.fast_span()
    }

    /// Slow EMA span $\tau_{\text{slow}}$.
    #[getter]
    pub fn slow_span(&self) -> f64 {
        self.inner.slow_span()
    }

    /// Signal EMA span $\tau_{\text{signal}}$.
    #[getter]
    pub fn signal_span(&self) -> f64 {
        self.inner.signal_span()
    }

    /// Analytical gradient $\frac{\partial M_t}{\partial \tau_{\text{fast}}}$.
    #[getter]
    pub fn grad_fast(&self) -> f64 {
        self.inner.grad_macd_fast_span()
    }

    /// Analytical gradient $\frac{\partial M_t}{\partial \tau_{\text{slow}}}$.
    #[getter]
    pub fn grad_slow(&self) -> f64 {
        self.inner.grad_macd_slow_span()
    }

    /// Analytical gradient $\frac{\partial S_t}{\partial \tau_{\text{signal}}}$.
    #[getter]
    pub fn grad_signal(&self) -> f64 {
        self.inner.grad_signal_span()
    }

    /// Number of bars consumed since initialization or last reset.
    #[getter]
    pub fn bars_consumed(&self) -> usize {
        self.inner.bars_consumed()
    }

    /// Updates all three spans simultaneously, validating parameters.
    pub fn set_parameters(
        &mut self,
        fast_span: f64,
        slow_span: f64,
        signal_span: f64,
    ) -> PyResult<()> {
        self.inner
            .set_parameters(fast_span, slow_span, signal_span)
            .map_err(ml_err_to_py)
    }

    /// Resets all internal smoothers, prices, and bar counters.
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Evaluates a 1D sequence of prices in batch, returning `(macd_lines, signal_lines, histograms)`.
    pub fn batch(&mut self, prices: Vec<f64>) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let n = prices.len();
        let mut macd_line = Vec::with_capacity(n);
        let mut signal_line = Vec::with_capacity(n);
        let mut histogram = Vec::with_capacity(n);
        for p in prices {
            let out = self.inner.next(p);
            macd_line.push(out.macd_line);
            signal_line.push(out.signal_line);
            histogram.push(out.histogram);
        }
        (macd_line, signal_line, histogram)
    }

    fn __repr__(&self) -> String {
        format!(
            "DifferentiableMacd(fast={:.2}, slow={:.2}, signal={:.2}, bars={})",
            self.inner.fast_span(),
            self.inner.slow_span(),
            self.inner.signal_span(),
            self.inner.bars_consumed()
        )
    }
}

#[pyfunction]
#[pyo3(signature = (network, feature_names=None))]
fn py_extract_symbolic_formula(
    network: &PyKanNetwork,
    feature_names: Option<Vec<String>>,
) -> PyResult<String> {
    network.extract_symbolic_formula(feature_names)
}

#[pyfunction]
#[pyo3(signature = (network, feature_names=None))]
fn py_extract_feature_importance<'py>(
    py: Python<'py>,
    network: &PyKanNetwork,
    feature_names: Option<Vec<String>>,
) -> PyResult<Vec<Bound<'py, PyDict>>> {
    network.extract_feature_importance(py, feature_names)
}

/// Registers the `quantwave._ml` classes and module functions into PyO3.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyKanNetwork>()?;
    m.add_class::<PyKanIndicator>()?;
    m.add_class::<PyDifferentiableEma>()?;
    m.add_class::<PyDifferentiableRsi>()?;
    m.add_class::<PyDifferentiableMacd>()?;

    m.add_function(wrap_pyfunction!(py_extract_symbolic_formula, m)?)?;
    m.add_function(wrap_pyfunction!(py_extract_feature_importance, m)?)?;

    Ok(())
}
