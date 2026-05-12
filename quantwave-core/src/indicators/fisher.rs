use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;

/// Fisher Transform
/// 
/// Based on John Ehlers' "Using The Fisher Transform".
/// The Fisher Transform changes the Probability Density Function (PDF) of any 
/// waveform so that the transformed output has an approximately Gaussian PDF.
/// This accentuates the largest deviations from the mean, providing sharp 
/// and easy to identify turning points.
#[derive(Debug, Clone, Default)]
pub struct FisherTransform;

impl FisherTransform {
    pub fn new() -> Self {
        Self
    }
}

impl Next<f64> for FisherTransform {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        // y = 0.5 * ln((1 + x) / (1 - x))
        // This is exactly atanh(x)
        // input must be in range (-1, 1)
        let x = input.clamp(-0.999, 0.999);
        0.5 * ((1.0 + x) / (1.0 - x)).ln()
    }
}

pub const FISHER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Fisher Transform",
    description: "Converts inputs to a nearly Gaussian probability distribution, creating sharp peaks at turning points.",
    params: &[],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UsingTheFisherTransform.pdf",
    formula_latex: r#"
\[
Fish(x) = 0.5 \times \ln\left(\frac{1 + x}{1 - x}\right) = \text{atanh}(x)
\]
"#,
    gold_standard_file: "fisher.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_fisher_basic() {
        let mut fish = FisherTransform::new();
        // Values close to 1 should be large positive
        assert!(fish.next(0.9) > 1.0);
        // Values close to -1 should be large negative
        assert!(fish.next(-0.9) < -1.0);
        // Value 0 should be 0
        approx::assert_relative_eq!(fish.next(0.0), 0.0, epsilon = 1e-6);
    }

    proptest! {
        #[test]
        fn test_fisher_parity(input in prop::collection::vec(-0.99..0.99, 1..100)) {
            let mut fish = FisherTransform::new();
            for &val in &input {
                let s = fish.next(val);
                let b = 0.5 * ((1.0 + val) / (1.0 - val)).ln();
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
