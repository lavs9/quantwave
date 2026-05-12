use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::ultimate_smoother::UltimateSmoother;

/// Generalized Laguerre Filter
/// 
/// Based on John Ehlers' "The Continuation Index" (TASC September 2025).
/// This is a generalized Laguerre filter of arbitrary order (up to 10), 
/// using an UltimateSmoother as the first Laguerre component.
#[derive(Debug, Clone)]
pub struct GeneralizedLaguerre {
    us: UltimateSmoother,
    gamma: f64,
    order: usize,
    lg_curr: [f64; 11], // 1-indexed to match formula (LG[1..order])
    lg_prev: [f64; 11],
    count: usize,
}

impl GeneralizedLaguerre {
    pub fn new(length: usize, gamma: f64, order: usize) -> Self {
        let order = order.min(10).max(1);
        Self {
            us: UltimateSmoother::new(length),
            gamma,
            order,
            lg_curr: [0.0; 11],
            lg_prev: [0.0; 11],
            count: 0,
        }
    }
}

impl Next<f64> for GeneralizedLaguerre {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        
        // Update previous values
        for i in 1..=self.order {
            self.lg_prev[i] = self.lg_curr[i];
        }

        // Calculate current component LG[1] using UltimateSmoother
        self.lg_curr[1] = self.us.next(input);

        // Calculate subsequent components LG[2..order]
        for i in 2..=self.order {
            self.lg_curr[i] = -self.gamma * self.lg_prev[i - 1] 
                             + self.lg_prev[i - 1] 
                             + self.gamma * self.lg_prev[i];
        }

        if self.count == 1 {
            // Initialization: set all components to the first value
            let first_val = self.lg_curr[1];
            for i in 1..=self.order {
                self.lg_curr[i] = first_val;
            }
            return first_val;
        }

        // Simple average of components
        let mut fir = 0.0;
        for i in 1..=self.order {
            fir += self.lg_curr[i];
        }

        fir / (self.order as f64)
    }
}

pub const GENERALIZED_LAGUERRE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Generalized Laguerre",
    description: "A generalized Laguerre filter of arbitrary order using an UltimateSmoother as the primary component.",
    params: &[
        ParamDef { name: "length", default: "40", description: "UltimateSmoother period" },
        ParamDef { name: "gamma", default: "0.8", description: "Smoothing factor (0.0 to 1.0)" },
        ParamDef { name: "order", default: "8", description: "Filter order (1 to 10)" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202025.html",
    formula_latex: r#"
\[
LG_1 = UltimateSmoother(Price, Length)
\]
\[
LG_i = -\gamma LG_{i-1,t-1} + LG_{i-1,t-1} + \gamma LG_{i,t-1} \text{ for } i=2 \dots Order
\]
\[
Filter = \frac{1}{Order} \sum_{i=1}^{Order} LG_i
\]
"#,
    gold_standard_file: "generalized_laguerre.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_generalized_laguerre_basic() {
        let mut gl = GeneralizedLaguerre::new(40, 0.8, 8);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0];
        for input in inputs {
            let res = gl.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_generalized_laguerre_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 40;
            let gamma = 0.8;
            let order = 8;
            let mut gl = GeneralizedLaguerre::new(length, gamma, order);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| gl.next(x)).collect();
            
            // Reference implementation
            let mut us = UltimateSmoother::new(length);
            let mut lg_curr = vec![0.0; order + 1];
            let mut lg_prev = vec![0.0; order + 1];
            let mut batch_results = Vec::with_capacity(inputs.len());
            
            for (t, &input) in inputs.iter().enumerate() {
                for i in 1..=order {
                    lg_prev[i] = lg_curr[i];
                }
                
                lg_curr[1] = us.next(input);
                
                for i in 2..=order {
                    lg_curr[i] = -gamma * lg_prev[i-1] + lg_prev[i-1] + gamma * lg_prev[i];
                }
                
                if t == 0 {
                    let first = lg_curr[1];
                    for i in 1..=order { lg_curr[i] = first; }
                }
                
                let res = lg_curr[1..=order].iter().sum::<f64>() / (order as f64);
                batch_results.push(res);
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
