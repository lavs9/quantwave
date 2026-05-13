use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Gaussian Filter
/// 
/// Based on John Ehlers' "Gaussian and Other Low Lag Filters".
/// A Gaussian filter is constructed by cascading multiple Exponential Moving Averages (EMA).
/// It provides significant smoothing with less lag than a Butterworth filter of the same order.
/// This implementation supports 1 to 4 poles.
#[derive(Debug, Clone)]
pub struct GaussianFilter {
    poles: usize,
    alpha_n: f64,
    coeffs: Vec<f64>,
    output_history: VecDeque<f64>,
    count: usize,
}

impl GaussianFilter {
    pub fn new(period: usize, poles: usize) -> Self {
        assert!(poles >= 1 && poles <= 4, "Gaussian Filter supports 1-4 poles");
        
        let w = 2.0 * PI / period as f64;
        let beta = (1.0 - w.cos()) / (2.0f64.powf(1.0 / poles as f64) - 1.0);
        let alpha = -beta + (beta * beta + 2.0 * beta).sqrt();
        
        let alpha_n = alpha.powi(poles as i32);
        let mut coeffs = Vec::with_capacity(poles);
        let c = 1.0 - alpha;
        
        match poles {
            1 => {
                coeffs.push(c);
            }
            2 => {
                coeffs.push(2.0 * c);
                coeffs.push(-c * c);
            }
            3 => {
                coeffs.push(3.0 * c);
                coeffs.push(-3.0 * c * c);
                coeffs.push(c * c * c);
            }
            4 => {
                coeffs.push(4.0 * c);
                coeffs.push(-6.0 * c * c);
                coeffs.push(4.0 * c * c * c);
                coeffs.push(-c * c * c * c);
            }
            _ => unreachable!(),
        }

        Self {
            poles,
            alpha_n,
            coeffs,
            output_history: VecDeque::from(vec![0.0; poles]),
            count: 0,
        }
    }
}

impl Next<f64> for GaussianFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;

        if self.count == 1 {
            for val in self.output_history.iter_mut() {
                *val = input;
            }
            return input;
        }

        let mut res = self.alpha_n * input;
        for i in 0..self.poles {
            res += self.coeffs[i] * self.output_history[i];
        }

        self.output_history.pop_back();
        self.output_history.push_front(res);

        res
    }
}

pub const GAUSSIAN_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Gaussian Filter",
    description: "A multi-pole low-pass filter with less lag than Butterworth filters. Cascades EMA filters to achieve Gaussian response.",
    params: &[
        ParamDef { name: "period", default: "20", description: "Filter period" },
        ParamDef { name: "poles", default: "2", description: "Number of poles (1-4)" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/GaussianFilters.pdf",
    formula_latex: r#"
\[
\beta = \frac{1 - \cos(2\pi/P)}{2^{1/N} - 1}, \alpha = -\beta + \sqrt{\beta^2 + 2\beta}
\]
\[
y_t = \alpha^N x_t + \sum_{k=1}^N \binom{N}{k} (-1)^{k-1} (1-\alpha)^k y_{t-k}
\]
"#,
    gold_standard_file: "gaussian.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_gaussian_basic() {
        let mut gf = GaussianFilter::new(20, 2);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = gf.next(input);
            assert!(!res.is_nan());
        }
    }

    #[test]
    fn test_gaussian_table_parity() {
        // Test against 2-pole table values from the PDF: Period 20, B[0]=0.146017, A[1]=1.235757, A[2]=-0.381774
        let period = 20;
        let poles = 2;
        let gf = GaussianFilter::new(period, poles);
        
        approx::assert_relative_eq!(gf.alpha_n, 0.146017, epsilon = 1e-5);
        approx::assert_relative_eq!(gf.coeffs[0], 1.235757, epsilon = 1e-5);
        approx::assert_relative_eq!(gf.coeffs[1], -0.381774, epsilon = 1e-5);
    }

    proptest! {
        #[test]
        fn test_gaussian_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
            poles in 1usize..4usize,
        ) {
            let period = 20;
            let mut gf = GaussianFilter::new(period, poles);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| gf.next(x)).collect();
            
            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let w = 2.0 * PI / period as f64;
            let beta = (1.0 - w.cos()) / (2.0f64.powf(1.0 / poles as f64) - 1.0);
            let alpha = -beta + (beta * beta + 2.0 * beta).sqrt();
            let alpha_n = alpha.powi(poles as i32);
            let c = 1.0 - alpha;
            
            let mut coeffs = Vec::new();
            match poles {
                1 => { coeffs.push(c); }
                2 => { coeffs.push(2.0 * c); coeffs.push(-c * c); }
                3 => { coeffs.push(3.0 * c); coeffs.push(-3.0 * c * c); coeffs.push(c * c * c); }
                4 => { coeffs.push(4.0 * c); coeffs.push(-6.0 * c * c); coeffs.push(4.0 * c * c * c); coeffs.push(-c * c * c * c); }
                _ => unreachable!(),
            }
            
            let mut hist = vec![0.0; poles];
            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                if bar == 1 {
                    for h in hist.iter_mut() { *h = input; }
                    batch_results.push(input);
                    continue;
                }
                
                let mut res = alpha_n * input;
                for j in 0..poles {
                    res += coeffs[j] * hist[j];
                }
                
                for j in (1..poles).rev() {
                    hist[j] = hist[j-1];
                }
                hist[0] = res;
                batch_results.push(res);
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
