use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// Classic Laguerre Filter
///
/// Based on John Ehlers' "Time Warp – Without Space Travel" (2002).
/// This filter uses the Laguerre Transform to provide superior smoothing
/// with minimal lag using only four data samples.
#[derive(Debug, Clone)]
pub struct ClassicLaguerre {
    gamma: f64,
    l0: f64,
    l1: f64,
    l2: f64,
    l3: f64,
    count: usize,
}

impl ClassicLaguerre {
    pub fn new(gamma: f64) -> Self {
        Self {
            gamma,
            l0: 0.0,
            l1: 0.0,
            l2: 0.0,
            l3: 0.0,
            count: 0,
        }
    }
}

impl Default for ClassicLaguerre {
    fn default() -> Self {
        Self::new(0.8)
    }
}

impl Next<f64> for ClassicLaguerre {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;

        if self.count == 1 {
            self.l0 = input;
            self.l1 = input;
            self.l2 = input;
            self.l3 = input;
            return input;
        }

        let prev_l0 = self.l0;
        let prev_l1 = self.l1;
        let prev_l2 = self.l2;
        let prev_l3 = self.l3;

        self.l0 = (1.0 - self.gamma) * input + self.gamma * prev_l0;
        self.l1 = -self.gamma * self.l0 + prev_l0 + self.gamma * prev_l1;
        self.l2 = -self.gamma * self.l1 + prev_l1 + self.gamma * prev_l2;
        self.l3 = -self.gamma * self.l2 + prev_l2 + self.gamma * prev_l3;

        (self.l0 + 2.0 * self.l1 + 2.0 * self.l2 + self.l3) / 6.0
    }
}

pub const CLASSIC_LAGUERRE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Classic Laguerre Filter",
    description: "The original Laguerre filter from John Ehlers' 2002 'Time Warp' paper.",
    params: &[ParamDef {
        name: "gamma",
        default: "0.8",
        description: "Smoothing factor (0.0 to 1.0)",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TimeWarp.pdf",
    formula_latex: r#"
\[
L_0 = (1 - \gamma) \cdot Price + \gamma \cdot L_{0,t-1}
\]
\[
L_1 = -\gamma L_0 + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
L_2 = -\gamma L_1 + L_{1,t-1} + \gamma L_{2,t-1}
\]
\[
L_3 = -\gamma L_2 + L_{2,t-1} + \gamma L_{3,t-1}
\]
\[
Filt = \frac{L_0 + 2L_1 + 2L_2 + L_3}{6}
\]
"#,
    gold_standard_file: "classic_laguerre.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_classic_laguerre_basic() {
        let mut cl = ClassicLaguerre::new(0.8);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        for input in inputs {
            let res = cl.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_classic_laguerre_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let gamma = 0.8;
            let mut cl = ClassicLaguerre::new(gamma);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| cl.next(x)).collect();

            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut l0 = 0.0;
            let mut l1 = 0.0;
            let mut l2 = 0.0;
            let mut l3 = 0.0;

            for (i, &input) in inputs.iter().enumerate() {
                if i == 0 {
                    l0 = input; l1 = input; l2 = input; l3 = input;
                    batch_results.push(input);
                } else {
                    let prev_l0 = l0;
                    let prev_l1 = l1;
                    let prev_l2 = l2;
                    let prev_l3 = l3;

                    l0 = (1.0 - gamma) * input + gamma * prev_l0;
                    l1 = -gamma * l0 + prev_l0 + gamma * prev_l1;
                    l2 = -gamma * l1 + prev_l1 + gamma * prev_l2;
                    l3 = -gamma * l2 + prev_l2 + gamma * prev_l3;

                    let res = (l0 + 2.0 * l1 + 2.0 * l2 + l3) / 6.0;
                    batch_results.push(res);
                }
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
