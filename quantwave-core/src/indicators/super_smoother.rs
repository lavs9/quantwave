use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// SuperSmoother Filter
///
/// Based on John Ehlers' "The Ultimate Smoother"
/// A second-order IIR filter with a maximally flat Butterworth response.
/// It provides superior smoothing compared to a first-order EMA with equivalent lag.
#[derive(Debug, Clone)]
pub struct SuperSmoother {
    c1: f64,
    c2: f64,
    c3: f64,
    price_prev: f64,
    ss_history: [f64; 2],
    count: usize,
}

impl SuperSmoother {
    pub fn new(period: usize) -> Self {
        let period_f = period as f64;
        let a1 = (-1.414 * PI / period_f).exp();
        let c2 = 2.0 * a1 * (1.414 * PI / period_f).cos();
        let c3 = -a1 * a1;
        let c1 = 1.0 - c2 - c3;
        Self {
            c1,
            c2,
            c3,
            price_prev: 0.0,
            ss_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Next<f64> for SuperSmoother {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let res = if self.count < 4 {
            input
        } else {
            self.c1 * (input + self.price_prev) / 2.0
                + self.c2 * self.ss_history[0]
                + self.c3 * self.ss_history[1]
        };

        self.ss_history[1] = self.ss_history[0];
        self.ss_history[0] = res;
        self.price_prev = input;
        res
    }
}

pub const SUPER_SMOOTHER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "SuperSmoother",
    description: "A second-order IIR filter with a maximally flat Butterworth response for superior smoothing with minimal lag.",
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
c_1 = 1 - c_2 - c_3
\]
\[
SS = c_1 \frac{Price + Price_{t-1}}{2} + c_2 SS_{t-1} + c_3 SS_{t-2}
\]
"#,
    gold_standard_file: "super_smoother.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_super_smoother_basic() {
        let mut ss = SuperSmoother::new(20);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = ss.next(input);
            println!("Input: {}, Output: {}", input, res);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_super_smoother_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let period = 20;
            let mut ss = SuperSmoother::new(period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| ss.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let period_f = period as f64;
            let a1 = (-1.414 * PI / period_f).exp();
            let c2 = 2.0 * a1 * (1.414 * PI / period_f).cos();
            let c3 = -a1 * a1;
            let c1 = 1.0 - c2 - c3;

            let mut ss_hist = [0.0; 2];
            let mut price_prev = 0.0;

            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let res = if bar < 4 {
                    input
                } else {
                    c1 * (input + price_prev) / 2.0 + c2 * ss_hist[0] + c3 * ss_hist[1]
                };
                ss_hist[1] = ss_hist[0];
                ss_hist[0] = res;
                price_prev = input;
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
