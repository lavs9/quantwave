use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// HighPass Filter
/// 
/// Based on John Ehlers' "The Ultimate Smoother"
/// A second-order High Pass filter that rejects low-frequency components 
/// and passes high-frequency components unattenuated.
#[derive(Debug, Clone)]
pub struct HighPass {
    c1: f64,
    c2: f64,
    c3: f64,
    price_history: [f64; 2],
    hp_history: [f64; 2],
    count: usize,
}

impl HighPass {
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
            hp_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Next<f64> for HighPass {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let res = if self.count < 4 {
            0.0
        } else {
            self.c1 * (input - 2.0 * self.price_history[0] + self.price_history[1])
                + self.c2 * self.hp_history[0]
                + self.c3 * self.hp_history[1]
        };
        
        self.hp_history[1] = self.hp_history[0];
        self.hp_history[0] = res;
        self.price_history[1] = self.price_history[0];
        self.price_history[0] = input;
        res
    }
}

pub const HIGH_PASS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "HighPass",
    description: "A second-order High Pass filter that rejects low-frequency components.",
    params: &[
        ParamDef { name: "period", default: "20", description: "Critical period (wavelength)" },
    ],
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
HP = c_1 (Price - 2 Price_{t-1} + Price_{t-2}) + c_2 HP_{t-1} + c_3 HP_{t-2}
\]
"#,
    gold_standard_file: "high_pass.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_high_pass_basic() {
        let mut hp = HighPass::new(20);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = hp.next(input);
            println!("Input: {}, Output: {}", input, res);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_high_pass_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let period = 20;
            let mut hp = HighPass::new(period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| hp.next(x)).collect();
            
            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let period_f = period as f64;
            let a1 = (-1.414 * PI / period_f).exp();
            let c2 = 2.0 * a1 * (1.414 * PI / period_f).cos();
            let c3 = -a1 * a1;
            let c1 = (1.0 + c2 - c3) / 4.0;
            
            let mut hp_hist = [0.0; 2];
            let mut price_hist = [0.0; 2];
            
            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let res = if bar < 4 {
                    0.0
                } else {
                    c1 * (input - 2.0 * price_hist[0] + price_hist[1]) + c2 * hp_hist[0] + c3 * hp_hist[1]
                };
                hp_hist[1] = hp_hist[0];
                hp_hist[0] = res;
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
