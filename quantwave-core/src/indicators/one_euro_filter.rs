use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// One Euro Filter
///
/// Based on John Ehlers' "The One Euro Filter" (TASC December 2025).
/// A speed-based adaptive low-pass filter that dynamically adjusts its
/// smoothing coefficient based on the rate of change of the input signal.
#[derive(Debug, Clone)]
pub struct OneEuroFilter {
    period_min: f64,
    beta: f64,
    prev_price: f64,
    smoothed_dx: f64,
    smoothed: f64,
    alpha_dx: f64,
    count: usize,
}

impl OneEuroFilter {
    pub fn new(period_min: usize, beta: f64) -> Self {
        let period_dx = 10.0;
        let alpha_dx = 2.0 * PI / (4.0 * PI + period_dx);
        Self {
            period_min: period_min as f64,
            beta,
            prev_price: 0.0,
            smoothed_dx: 0.0,
            smoothed: 0.0,
            alpha_dx,
            count: 0,
        }
    }
}

impl Default for OneEuroFilter {
    fn default() -> Self {
        Self::new(10, 0.2)
    }
}

impl Next<f64> for OneEuroFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        if self.count == 1 {
            self.smoothed = input;
            self.prev_price = input;
            return input;
        }

        // EMA the Delta Price
        self.smoothed_dx = self.alpha_dx * (input - self.prev_price) + (1.0 - self.alpha_dx) * self.smoothed_dx;
        
        // Adjust cutoff period based on fraction of the rate of change
        let cutoff = self.period_min + self.beta * self.smoothed_dx.abs();
        
        // Compute adaptive alpha
        let alpha3 = 2.0 * PI / (4.0 * PI + cutoff);
        
        // Adaptive smoothing
        self.smoothed = alpha3 * input + (1.0 - alpha3) * self.smoothed;
        
        self.prev_price = input;
        self.smoothed
    }
}

pub const ONE_EURO_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "One Euro Filter",
    description: "A speed-based adaptive low-pass filter that dynamically adjusts its smoothing coefficient.",
    usage: "Use in real-time systems where you need low lag at high speeds and low noise at low speeds. The adaptive cutoff frequency makes it self-tuning for different signal velocities.",
    keywords: &["filter", "smoothing", "adaptive", "real-time", "low-pass"],
    ehlers_summary: "The One Euro Filter, developed by Casiez et al. (2012), is an adaptive lowpass filter that adjusts its cutoff frequency based on the signal derivative. When the signal changes quickly (high speed) the cutoff is raised to reduce lag; when it changes slowly the cutoff is lowered to reduce noise — automatically balancing the speed-accuracy trade-off.",
    params: &[
        ParamDef {
            name: "period_min",
            default: "10",
            description: "Minimum cutoff period",
        },
        ParamDef {
            name: "beta",
            default: "0.2",
            description: "Responsiveness factor",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20DECEMBER%202025.html",
    formula_latex: r#"
\[
\alpha_{dx} = \frac{2\pi}{4\pi + 10}
\]
\[
SmoothedDX = \alpha_{dx}(Price - Price_{t-1}) + (1 - \alpha_{dx})SmoothedDX_{t-1}
\]
\[
Cutoff = PeriodMin + \beta |SmoothedDX|
\]
\[
\alpha_3 = \frac{2\pi}{4\pi + Cutoff}
\]
\[
Smoothed = \alpha_3 Price + (1 - \alpha_3)Smoothed_{t-1}
\]
"#,
    gold_standard_file: "one_euro_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_one_euro_filter_basic() {
        let mut oef = OneEuroFilter::new(10, 0.2);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        for input in inputs {
            let res = oef.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_one_euro_filter_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let p_min = 10;
            let beta = 0.2;
            let mut oef = OneEuroFilter::new(p_min, beta);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| oef.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut smoothed = 0.0;
            let mut smoothed_dx = 0.0;
            let mut prev_price = 0.0;
            let alpha_dx = 2.0 * PI / (4.0 * PI + 10.0);

            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                if bar == 1 {
                    smoothed = input;
                    prev_price = input;
                    batch_results.push(input);
                    continue;
                }

                smoothed_dx = alpha_dx * (input - prev_price) + (1.0 - alpha_dx) * smoothed_dx;
                let cutoff = (p_min as f64) + beta * smoothed_dx.abs();
                let alpha3 = 2.0 * PI / (4.0 * PI + cutoff);
                smoothed = alpha3 * input + (1.0 - alpha3) * smoothed;
                prev_price = input;
                batch_results.push(smoothed);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
