use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::ultimate_smoother::UltimateSmoother;
use crate::traits::Next;
use std::collections::VecDeque;

/// Ehlers Autocorrelation
///
/// Based on John Ehlers' "Drunkard’s Walk: Theory And Measurement By Autocorrelation" (2025).
/// It computes the Pearson correlation of smoothed price data with its own lagged versions.
/// This is typically displayed as a heatmap to identify cycles and trends.
#[derive(Debug, Clone)]
pub struct EhlersAutocorrelation {
    length: usize,
    num_lags: usize,
    smoother: UltimateSmoother,
    filt_history: VecDeque<f64>,
}

impl EhlersAutocorrelation {
    pub fn new(length: usize, num_lags: usize) -> Self {
        Self {
            length,
            num_lags,
            smoother: UltimateSmoother::new(20), // Default smoother period from paper
            filt_history: VecDeque::from(vec![0.0; length + num_lags]),
        }
    }

    pub fn with_smoother_period(length: usize, num_lags: usize, smoother_period: usize) -> Self {
        Self {
            length,
            num_lags,
            smoother: UltimateSmoother::new(smoother_period),
            filt_history: VecDeque::from(vec![0.0; length + num_lags]),
        }
    }
}

impl Next<f64> for EhlersAutocorrelation {
    type Output = Vec<f64>; // Correlation for each lag from 0 to num_lags-1

    fn next(&mut self, input: f64) -> Self::Output {
        let filt = self.smoother.next(input);
        self.filt_history.push_front(filt);
        self.filt_history.pop_back();

        let mut results = Vec::with_capacity(self.num_lags);
        let len_f = self.length as f64;

        for lag in 0..self.num_lags {
            let mut sx = 0.0;
            let mut sy = 0.0;
            let mut sxx = 0.0;
            let mut sxy = 0.0;
            let mut syy = 0.0;

            for j in 0..self.length {
                let x = self.filt_history[j];
                let y = self.filt_history[lag + j];
                sx += x;
                sy += y;
                sxx += x * x;
                sxy += x * y;
                syy += y * y;
            }

            let denom_x = len_f * sxx - sx * sx;
            let denom_y = len_f * syy - sy * sy;

            let corr = if denom_x > 0.0 && denom_y > 0.0 {
                (len_f * sxy - sx * sy) / (denom_x * denom_y).sqrt()
            } else if lag == 0 {
                1.0
            } else {
                0.0
            };

            results.push(corr);
        }

        results
    }
}

pub const EHLERS_AUTOCORRELATION_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ehlers Autocorrelation",
    description: "Computes Pearson correlation of smoothed price with its lags to identify market structure.",
    usage: "Use to generate an autocorrelation periodogram showing which cycle periods are currently dominant. Visualise as a heatmap to track cycle period shifts over time.",
    keywords: &["cycle", "spectral", "ehlers", "dsp", "dominant-cycle"],
    ehlers_summary: "Ehlers introduces autocorrelation-based cycle measurement in Cycle Analytics for Traders (2013) as a more robust alternative to DFT. By computing autocorrelation of Roofing-filtered price at each lag, then applying a spectral DFT to the lag series, he obtains a periodogram insensitive to amplitude variations.",
    params: &[
        ParamDef {
            name: "length",
            default: "20",
            description: "Correlation window length",
        },
        ParamDef {
            name: "num_lags",
            default: "100",
            description: "Number of lags to compute",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’ TIPS - FEBRUARY 2025.html",
    formula_latex: r#"
\[
\rho(lag) = \frac{N \sum X Y - \sum X \sum Y}{\sqrt{(N \sum X^2 - (\sum X)^2)(N \sum Y^2 - (\sum Y)^2)}}
\]
"#,
    gold_standard_file: "ehlers_autocorrelation.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use crate::test_utils::{load_gold_standard_vec, assert_indicator_parity_vec};
    use proptest::prelude::*;

    #[test]
    fn test_ehlers_autocorrelation_gold_standard() {
        let case = load_gold_standard_vec("ehlers_autocorrelation");
        let ac = EhlersAutocorrelation::new(20, 10);
        assert_indicator_parity_vec(ac, &case.input, &case.expected);
    }

    #[test]
    fn test_ehlers_autocorrelation_basic() {
        let mut ac = EhlersAutocorrelation::new(20, 10);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = ac.next(input);
            assert_eq!(res.len(), 10);
            approx::assert_relative_eq!(res[0], 1.0, epsilon = 1e-10);
        }
    }

    proptest! {
        #[test]
        fn test_ehlers_autocorrelation_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 20;
            let num_lags = 10;
            let mut ac = EhlersAutocorrelation::new(length, num_lags);
            let streaming_results: Vec<Vec<f64>> = inputs.iter().map(|&x| ac.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut smoother = UltimateSmoother::new(20);
            let filtered: Vec<f64> = inputs.iter().map(|&x| smoother.next(x)).collect();
            
            for i in 0..inputs.len() {
                let mut bar_results = Vec::with_capacity(num_lags);
                for lag in 0..num_lags {
                    let mut sx = 0.0;
                    let mut sy = 0.0;
                    let mut sxx = 0.0;
                    let mut sxy = 0.0;
                    let mut syy = 0.0;
                    
                    for j in 0..length {
                        let idx_x = i as i32 - j as i32;
                        let idx_y = i as i32 - (lag + j) as i32;
                        
                        let x = if idx_x >= 0 { filtered[idx_x as usize] } else { 0.0 };
                        let y = if idx_y >= 0 { filtered[idx_y as usize] } else { 0.0 };
                        
                        sx += x;
                        sy += y;
                        sxx += x * x;
                        sxy += x * y;
                        syy += y * y;
                    }
                    
                    let len_f = length as f64;
                    let denom_x = len_f * sxx - sx * sx;
                    let denom_y = len_f * syy - sy * sy;
                    
                    let corr = if denom_x > 0.0 && denom_y > 0.0 {
                        (len_f * sxy - sx * sy) / (denom_x * denom_y).sqrt()
                    } else if lag == 0 {
                        1.0
                    } else {
                        0.0
                    };
                    bar_results.push(corr);
                }
                batch_results.push(bar_results);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                for (sv, bv) in s.iter().zip(b.iter()) {
                    approx::assert_relative_eq!(sv, bv, epsilon = 1e-10);
                }
            }
        }
    }
}
