use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// UltimateSmoother Filter
///
/// Based on John Ehlers' "The Ultimate Smoother"
/// It conceptually has zero lag in the Pass Band and has minimum lag in the transition band.
/// It is constructed by subtracting the High Pass response from the input data (cancellation).
#[derive(Debug, Clone)]
pub struct UltimateSmoother {
    c1: f64,
    c2: f64,
    c3: f64,
    price_history: [f64; 2],
    us_history: [f64; 2],
    count: usize,
}

impl UltimateSmoother {
    pub fn new(period: usize) -> Self {
        let period_f = period as f64;
        let a1 = (-1.414 * PI / period_f).exp();
        let c2 = 2.0 * a1 * (1.414 * PI / period_f).cos();
        let c3 = -a1 * a1;
        let c1 = (1.0 + c2 - c3) / 4.0;
        Self {
            c1,
            c2,
            c3,
            price_history: [0.0; 2],
            us_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Next<f64> for UltimateSmoother {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let res = if self.count < 4 {
            input
        } else {
            (1.0 - self.c1) * input + (2.0 * self.c1 - self.c2) * self.price_history[0]
                - (self.c1 + self.c3) * self.price_history[1]
                + self.c2 * self.us_history[0]
                + self.c3 * self.us_history[1]
        };

        self.us_history[1] = self.us_history[0];
        self.us_history[0] = res;
        self.price_history[1] = self.price_history[0];
        self.price_history[0] = input;
        res
    }
}

pub const ULTIMATE_SMOOTHER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "UltimateSmoother",
    description: "An Ehlers filter with zero lag in the Pass Band, constructed by subtracting High Pass response from the input data.",
    params: &[ParamDef {
        name: "period",
        default: "20",
        description: "Critical period (wavelength)",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/UltimateSmoother.pdf",
    formula_latex: r#"
\[
a_1 = \exp\left(-\frac{1.414\pi}{Period}\right)
\]
\[
c_2 = 2a_1 \cos\left(\frac{1.414\pi}{Period}\right)
\]
\[
c_3 = -a_1^2
\]
\[
c_1 = (1 + c_2 - c_3) / 4
\]
\[
US = (1 - c_1) Price + (2c_1 - c_2) Price_{t-1} - (c_1 + c_3) Price_{t-2} + c_2 US_{t-1} + c_3 US_{t-2}
\]
"#,
    gold_standard_file: "ultimate_smoother.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_ultimate_smoother_basic() {
        let mut us = UltimateSmoother::new(20);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = us.next(input);
            println!("Input: {}, Output: {}", input, res);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_ultimate_smoother_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let period = 20;
            let mut us = UltimateSmoother::new(period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| us.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let period_f = period as f64;
            let a1 = (-1.414 * PI / period_f).exp();
            let c2 = 2.0 * a1 * (1.414 * PI / period_f).cos();
            let c3 = -a1 * a1;
            let c1 = (1.0 + c2 - c3) / 4.0;

            let mut us_hist = [0.0; 2];
            let mut price_hist = [0.0; 2];

            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let res = if bar < 4 {
                    input
                } else {
                    (1.0 - c1) * input + (2.0 * c1 - c2) * price_hist[0] - (c1 + c3) * price_hist[1] + c2 * us_hist[0] + c3 * us_hist[1]
                };
                us_hist[1] = us_hist[0];
                us_hist[0] = res;
                price_hist[1] = price_hist[0];
                price_hist[0] = input;
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
