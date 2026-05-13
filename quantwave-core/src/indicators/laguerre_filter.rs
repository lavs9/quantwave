use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::ultimate_smoother::UltimateSmoother;
use crate::traits::Next;

/// Laguerre Filter
///
/// Based on John Ehlers' "Laguerre Filters" (TASC July 2025).
/// This version of the Laguerre Filter uses an UltimateSmoother as the input (L0)
/// to provide enhanced trend-following characteristics with low lag.
#[derive(Debug, Clone)]
pub struct LaguerreFilter {
    us: UltimateSmoother,
    gamma: f64,
    l: [f64; 5], // L1, L2, L3, L4, L5 (using index 1-5 to match code)
    prev_l0: f64,
    count: usize,
}

impl LaguerreFilter {
    pub fn new(length: usize, gamma: f64) -> Self {
        Self {
            us: UltimateSmoother::new(length),
            gamma,
            l: [0.0; 5],
            prev_l0: 0.0,
            count: 0,
        }
    }
}

impl Next<f64> for LaguerreFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let l0 = self.us.next(input);
        self.count += 1;

        if self.count == 1 {
            self.prev_l0 = l0;
            for i in 0..5 {
                self.l[i] = l0;
            }
            return l0;
        }

        let l1 = -self.gamma * self.prev_l0 + self.prev_l0 + self.gamma * self.l[0];
        let l2 = -self.gamma * self.l[0] + self.l[0] + self.gamma * self.l[1];
        let l3 = -self.gamma * self.l[1] + self.l[1] + self.gamma * self.l[2];
        let l4 = -self.gamma * self.l[2] + self.l[2] + self.gamma * self.l[3];
        let l5 = -self.gamma * self.l[3] + self.l[3] + self.gamma * self.l[4];

        // Laguerre = (L0 + 4*L1 + 6*L2 + 4*L3 + L5) / 16
        // Note: The July 2025 code says L5 in the formula in Python/TradeStation code:
        // Laguerre[i] = (L0[i] + 4 * L1[i] + 6 * L2[i] + 4 * L3[i] + L5[i]) / 16
        let res = (l0 + 4.0 * l1 + 6.0 * l2 + 4.0 * l3 + l5) / 16.0;

        self.l[4] = l5;
        self.l[3] = l4;
        self.l[2] = l3;
        self.l[1] = l2;
        self.l[0] = l1;
        self.prev_l0 = l0;

        res
    }
}

pub const LAGUERRE_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Laguerre Filter",
    description: "A trend-following filter that excels at smoothing long-wavelength components using Laguerre polynomials and an UltimateSmoother base.",
    params: &[
        ParamDef {
            name: "length",
            default: "40",
            description: "UltimateSmoother period",
        },
        ParamDef {
            name: "gamma",
            default: "0.8",
            description: "Smoothing factor (0.0 to 1.0)",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JULY%202025.html",
    formula_latex: r#"
\[
L_0 = UltimateSmoother(Close, Length)
\]
\[
L_1 = -\gamma L_{0,t-1} + L_{0,t-1} + \gamma L_{1,t-1}
\]
\[
...
\]
\[
Laguerre = (L_0 + 4L_1 + 6L_2 + 4L_3 + L_5) / 16
\]
"#,
    gold_standard_file: "laguerre_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_laguerre_filter_basic() {
        let mut lf = LaguerreFilter::new(40, 0.8);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        for input in inputs {
            let res = lf.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_laguerre_filter_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 40;
            let gamma = 0.8;
            let mut lf = LaguerreFilter::new(length, gamma);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| lf.next(x)).collect();

            // Reference implementation (simplified)
            let mut us = UltimateSmoother::new(length);
            let l0_vals: Vec<f64> = inputs.iter().map(|&x| us.next(x)).collect();

            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut l1 = 0.0;
            let mut l2 = 0.0;
            let mut l3 = 0.0;
            let mut l4 = 0.0;
            let mut l5 = 0.0;

            for (i, &l0) in l0_vals.iter().enumerate() {
                if i == 0 {
                    l1 = l0; l2 = l0; l3 = l0; l4 = l0; l5 = l0;
                    batch_results.push(l0);
                } else {
                    let prev_l0 = l0_vals[i-1];
                    let pl1 = l1; let pl2 = l2; let pl3 = l3; let pl4 = l4;
                    l1 = -gamma * prev_l0 + prev_l0 + gamma * pl1;
                    l2 = -gamma * pl1 + pl1 + gamma * pl2;
                    l3 = -gamma * pl2 + pl2 + gamma * pl3;
                    l4 = -gamma * pl3 + pl3 + gamma * pl4;
                    l5 = -gamma * pl4 + pl4 + gamma * l5;

                    let res = (l0 + 4.0 * l1 + 6.0 * l2 + 4.0 * l3 + l5) / 16.0;
                    batch_results.push(res);
                }
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
