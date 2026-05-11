/// The core trait for streaming indicators.
/// Every indicator maintains an internal state and processes data points one by one.
pub trait Next<Input> {
    type Output;

    /// Process the next input and return the updated output.
    fn next(&mut self, input: Input) -> Self::Output;
}

/// A trait for algorithms that smooth a series of values (e.g., SMA, EMA).
/// This allows indicators like SuperTrend or Keltner Channels to be generic over their MA type.
pub trait SmoothingAlgorithm: Next<f64, Output = f64> + Clone + Send + Sync {}

/// A trait for indicator configurations that can build their respective streaming state machines.
pub trait IndicatorConfig {
    type Indicator: Next<f64>;

    /// Build a new instance of the indicator state machine.
    fn build(&self) -> Self::Indicator;
}

/// Blanket implementation for types that satisfy the SmoothingAlgorithm requirements.
impl<T> SmoothingAlgorithm for T where T: Next<f64, Output = f64> + Clone + Send + Sync {}
