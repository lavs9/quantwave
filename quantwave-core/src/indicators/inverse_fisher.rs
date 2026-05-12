use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;

/// Inverse Fisher Transform (IFT)
/// 
/// Based on John Ehlers' "The Inverse Fisher Transform".
/// The Inverse Fisher Transform is a compressive transform that alters the 
/// Probability Distribution Function (PDF) of an oscillator to produce clear 
/// black-or-white signals.
/// 
/// It is mathematically equivalent to the Hyperbolic Tangent (tanh) function.
#[derive(Debug, Clone, Default)]
pub struct InverseFisherTransform;

impl InverseFisherTransform {
    pub fn new() -> Self {
        Self
    }
}

impl Next<f64> for InverseFisherTransform {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        // IFT(x) = (exp(2*x) - 1) / (exp(2*x) + 1)
        // This is exactly tanh(x)
        input.tanh()
    }
}

pub const INVERSE_FISHER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Inverse Fisher Transform",
    description: "A compressive transform that forces oscillator values towards +1 or -1, creating clear buy/sell signals.",
    params: &[],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TheInverseFisherTransform.pdf",
    formula_latex: r#"
\[
IFT(x) = \frac{e^{2x} - 1}{e^{2x} + 1} = \tanh(x)
\]
"#,
    gold_standard_file: "inverse_fisher.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_inverse_fisher_basic() {
        let mut ift = InverseFisherTransform::new();
        // Values > 2 should be close to 1
        approx::assert_relative_eq!(ift.next(2.0), 0.96402758, epsilon = 1e-6);
        approx::assert_relative_eq!(ift.next(5.0), 0.9999092, epsilon = 1e-6);
        // Values < -2 should be close to -1
        approx::assert_relative_eq!(ift.next(-2.0), -0.96402758, epsilon = 1e-6);
        // Value 0 should be 0
        approx::assert_relative_eq!(ift.next(0.0), 0.0, epsilon = 1e-6);
    }

    proptest! {
        #[test]
        fn test_inverse_fisher_parity(input in prop::collection::vec(-5.0..5.0, 1..100)) {
            let mut ift = InverseFisherTransform::new();
            for &val in &input {
                let s = ift.next(val);
                let b = val.tanh();
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
