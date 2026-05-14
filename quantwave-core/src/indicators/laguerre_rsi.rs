use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// Laguerre RSI
///
/// Based on John Ehlers' "Time Warp – Without Space Travel" (2002).
/// An RSI indicator generated over Laguerre time rather than linear time,
/// providing rapid response to price changes with minimal lag.
#[derive(Debug, Clone)]
pub struct LaguerreRSI {
    gamma: f64,
    l0: f64,
    l1: f64,
    l2: f64,
    l3: f64,
    count: usize,
}

impl LaguerreRSI {
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

impl Default for LaguerreRSI {
    fn default() -> Self {
        Self::new(0.5)
    }
}

impl Next<f64> for LaguerreRSI {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;

        if self.count == 1 {
            self.l0 = input;
            self.l1 = input;
            self.l2 = input;
            self.l3 = input;
            return 0.0;
        }

        let prev_l0 = self.l0;
        let prev_l1 = self.l1;
        let prev_l2 = self.l2;
        let prev_l3 = self.l3;

        self.l0 = (1.0 - self.gamma) * input + self.gamma * prev_l0;
        self.l1 = -self.gamma * self.l0 + prev_l0 + self.gamma * prev_l1;
        self.l2 = -self.gamma * self.l1 + prev_l1 + self.gamma * prev_l2;
        self.l3 = -self.gamma * self.l2 + prev_l2 + self.gamma * prev_l3;

        let mut cu = 0.0;
        let mut cd = 0.0;

        if self.l0 >= self.l1 {
            cu += self.l0 - self.l1;
        } else {
            cd += self.l1 - self.l0;
        }
        if self.l1 >= self.l2 {
            cu += self.l1 - self.l2;
        } else {
            cd += self.l2 - self.l1;
        }
        if self.l2 >= self.l3 {
            cu += self.l2 - self.l3;
        } else {
            cd += self.l3 - self.l2;
        }

        let rsi = if cu + cd != 0.0 {
            cu / (cu + cd)
        } else {
            0.0
        };

        rsi.clamp(0.0, 1.0)
    }
}

pub const LAGUERRE_RSI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Laguerre RSI",
    description: "RSI calculated over Laguerre-warped time for faster response.",
    usage: "Use as a faster lower-lag alternative to traditional RSI. Laguerre smoothing produces fewer whipsaws while remaining responsive to genuine momentum shifts.",
    keywords: &["oscillator", "rsi", "ehlers", "dsp", "laguerre", "momentum"],
    ehlers_summary: "Ehlers constructs the Laguerre RSI in Cybernetic Analysis by computing RSI on the four outputs of a Laguerre filter bank. The result has RSI-like scaling (0 to 1) but dramatically less lag and smoother behaviour than conventional RSI.",
    params: &[ParamDef {
        name: "gamma",
        default: "0.5",
        description: "Smoothing factor (0.0 to 1.0)",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/TimeWarp.pdf",
    formula_latex: r#"
\[
L_0 = (1 - \gamma) \cdot Close + \gamma \cdot L_{0,t-1}
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
CU = \sum \max(L_{i} - L_{i+1}, 0)
\]
\[
CD = \sum \max(L_{i+1} - L_{i}, 0)
\]
\[
RSI = \frac{CU}{CU + CD}
\]
"#,
    gold_standard_file: "laguerre_rsi.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_laguerre_rsi_basic() {
        let mut lrsi = LaguerreRSI::new(0.5);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        for input in inputs {
            let res = lrsi.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_laguerre_rsi_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let gamma = 0.5;
            let mut lrsi = LaguerreRSI::new(gamma);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| lrsi.next(x)).collect();

            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut l0 = 0.0;
            let mut l1 = 0.0;
            let mut l2 = 0.0;
            let mut l3 = 0.0;

            for (i, &input) in inputs.iter().enumerate() {
                if i == 0 {
                    l0 = input; l1 = input; l2 = input; l3 = input;
                    batch_results.push(0.0);
                } else {
                    let prev_l0 = l0;
                    let prev_l1 = l1;
                    let prev_l2 = l2;
                    let prev_l3 = l3;

                    l0 = (1.0 - gamma) * input + gamma * prev_l0;
                    l1 = -gamma * l0 + prev_l0 + gamma * prev_l1;
                    l2 = -gamma * l1 + prev_l1 + gamma * prev_l2;
                    l3 = -gamma * l2 + prev_l2 + gamma * prev_l3;

                    let mut cu = 0.0;
                    let mut cd = 0.0;

                    if l0 >= l1 { cu += l0 - l1; } else { cd += l1 - l0; }
                    if l1 >= l2 { cu += l1 - l2; } else { cd += l2 - l1; }
                    if l2 >= l3 { cu += l2 - l3; } else { cd += l3 - l2; }

                    let res = if cu + cd != 0.0 { cu / (cu + cd) } else { 0.0 };
                    batch_results.push(res);
                }
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
