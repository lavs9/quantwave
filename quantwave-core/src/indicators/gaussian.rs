use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// Gaussian Filter
///
/// Based on John Ehlers' "Gaussian and Other Low Lag Filters".
/// A family of low-pass filters with N poles at the same location.
/// Provides approximately half the lag of an equivalent Butterworth filter.
#[derive(Debug, Clone)]
pub struct GaussianFilter {
    poles: usize,
    alpha: f64,
    // a^n
    alpha_pow: f64,
    // (1-a)^n
    one_minus_alpha: f64,
    price_history: Vec<f64>,
    filt_history: Vec<f64>,
    count: usize,
}

impl GaussianFilter {
    pub fn new(period: usize, poles: usize) -> Self {
        let poles = poles.clamp(1, 4);
        let p = period as f64;
        let omega = 2.0 * PI / p;
        // beta = (1 - cos(omega)) / (2^(1/(2N)) - 1)
        // 1.4142 is sqrt(2), so 2^(1/(2N))
        let beta = (1.0 - omega.cos()) / (2.0_f64.powf(1.0 / (2.0 * poles as f64)) - 1.0);
        let alpha = -beta + (beta * beta + 2.0 * beta).sqrt();
        
        Self {
            poles,
            alpha,
            alpha_pow: alpha.powi(poles as i32),
            one_minus_alpha: 1.0 - alpha,
            price_history: vec![0.0; poles + 1],
            filt_history: vec![0.0; poles + 1],
            count: 0,
        }
    }
}

impl Default for GaussianFilter {
    fn default() -> Self {
        Self::new(14, 4)
    }
}

impl Next<f64> for GaussianFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        
        let res = match self.poles {
            1 => {
                // f = a*g + (1-a)f[1]
                if self.count < 2 {
                    input
                } else {
                    self.alpha * input + self.one_minus_alpha * self.filt_history[0]
                }
            }
            2 => {
                // f = a^2*g + 2(1-a)f[1] - (1-a)^2f[2]
                if self.count < 3 {
                    input
                } else {
                    self.alpha_pow * input
                        + 2.0 * self.one_minus_alpha * self.filt_history[0]
                        - self.one_minus_alpha.powi(2) * self.filt_history[1]
                }
            }
            3 => {
                // f = a^3*g + 3(1-a)f[1] - 3(1-a)^2f[2] + (1-a)^3f[3]
                if self.count < 4 {
                    input
                } else {
                    self.alpha_pow * input
                        + 3.0 * self.one_minus_alpha * self.filt_history[0]
                        - 3.0 * self.one_minus_alpha.powi(2) * self.filt_history[1]
                        + self.one_minus_alpha.powi(3) * self.filt_history[2]
                }
            }
            4 => {
                // f = a^4*g + 4(1-a)f[1] - 6(1-a)^2f[2] + 4(1-a)^3f[3] - (1-a)^4f[4]
                if self.count < 5 {
                    input
                } else {
                    self.alpha_pow * input
                        + 4.0 * self.one_minus_alpha * self.filt_history[0]
                        - 6.0 * self.one_minus_alpha.powi(2) * self.filt_history[1]
                        + 4.0 * self.one_minus_alpha.powi(3) * self.filt_history[2]
                        - self.one_minus_alpha.powi(4) * self.filt_history[3]
                }
            }
            _ => input,
        };

        // Shift history
        for i in (1..self.poles).rev() {
            self.filt_history[i] = self.filt_history[i - 1];
            self.price_history[i] = self.price_history[i - 1];
        }
        self.filt_history[0] = res;
        self.price_history[0] = input;
        
        res
    }
}

pub const GAUSSIAN_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "GaussianFilter",
    description: "Multi-pole Gaussian low-pass filter for reduced lag.",
    params: &[
        ParamDef {
            name: "period",
            default: "14",
            description: "Critical period",
        },
        ParamDef {
            name: "poles",
            default: "4",
            description: "Number of poles (1-4)",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/GaussianFilters.pdf",
    formula_latex: r#"
\[
\alpha = -\beta + \sqrt{\beta^2 + 2\beta}
\]
\[
\beta = \frac{1 - \cos(2\pi/P)}{2^{1/(2N)} - 1}
\]
"#,
    gold_standard_file: "gaussian_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_gaussian_basic() {
        let mut filter = GaussianFilter::new(14, 4);
        for i in 0..50 {
            let val = filter.next(100.0);
            if i > 20 {
                approx::assert_relative_eq!(val, 100.0, epsilon = 1.0);
            }
        }
    }

    proptest! {
        #[test]
        fn test_gaussian_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
            poles in 1usize..4usize,
        ) {
            let p = 14;
            let mut filter = GaussianFilter::new(p, poles);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| filter.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let p_f = p as f64;
            let omega = 2.0 * PI / p_f;
            let beta = (1.0 - omega.cos()) / (2.0_f64.powf(1.0 / (2.0 * poles as f64)) - 1.0);
            let alpha = -beta + (beta * beta + 2.0 * beta).sqrt();
            let alpha_pow = alpha.powi(poles as i32);
            let oma = 1.0 - alpha;

            let mut f_hist = vec![0.0; poles];
            
            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let res = if bar < poles + 1 {
                    input
                } else {
                    match poles {
                        1 => alpha_pow * input + oma * f_hist[0],
                        2 => alpha_pow * input + 2.0 * oma * f_hist[0] - oma.powi(2) * f_hist[1],
                        3 => alpha_pow * input + 3.0 * oma * f_hist[0] - 3.0 * oma.powi(2) * f_hist[1] + oma.powi(3) * f_hist[2],
                        4 => alpha_pow * input + 4.0 * oma * f_hist[0] - 6.0 * oma.powi(2) * f_hist[1] + 4.0 * oma.powi(3) * f_hist[2] - oma.powi(4) * f_hist[3],
                        _ => input,
                    }
                };
                
                for j in (1..poles).rev() {
                    f_hist[j] = f_hist[j-1];
                }
                f_hist[0] = res;
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
