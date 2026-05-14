use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Hurst Exponent
///
/// Measures the long-term memory of time series.
/// H < 0.5: Mean reverting (anti-persistent)
/// H = 0.5: Random walk
/// H > 0.5: Trending (persistent)
///
/// This implementation uses the Rescaled Range (R/S) analysis over a fixed window.
#[derive(Debug, Clone)]
pub struct HurstExponent {
    period: usize,
    window: VecDeque<f64>,
}

impl HurstExponent {
    pub fn new(period: usize) -> Self {
        Self {
            period: period.max(2),
            window: VecDeque::with_capacity(period),
        }
    }
}

impl Next<f64> for HurstExponent {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_back(input);
        if self.window.len() > self.period {
            self.window.pop_front();
        }

        if self.window.len() < self.period {
            return f64::NAN;
        }

        // 1. Calculate Mean
        let sum: f64 = self.window.iter().sum();
        let mean = sum / self.period as f64;

        // 2. Mean-adjusted series and cumulative deviates
        // 3. Calculate Range
        let mut cumulative_deviate = 0.0;
        let mut max_z = f64::MIN;
        let mut min_z = f64::MAX;

        for &val in self.window.iter() {
            cumulative_deviate += val - mean;
            if cumulative_deviate > max_z {
                max_z = cumulative_deviate;
            }
            if cumulative_deviate < min_z {
                min_z = cumulative_deviate;
            }
        }

        let range = max_z - min_z;

        // 4. Calculate Standard Deviation
        let mut variance_sum = 0.0;
        for &val in self.window.iter() {
            let diff = val - mean;
            variance_sum += diff * diff;
        }
        let std_dev = (variance_sum / self.period as f64).sqrt();

        if std_dev == 0.0 {
            return 0.5; // Random walk if no variance
        }

        // 5. Calculate Hurst Exponent
        // H = log(R/S) / log(N)
        let rs = range / std_dev;
        (rs.ln()) / (self.period as f64).ln()
    }
}

pub const HURST_EXPONENT_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hurst Exponent",
    description: "Measures the persistence or anti-persistence of a time series using R/S analysis.",
    usage: "Use to classify the current market regime. H > 0.5 suggests a trending market (persistent); H < 0.5 suggests a mean-reverting market (anti-persistent). Useful as a filter for trend-following or mean-reversion strategies.",
    keywords: &["statistics", "regime-detection", "hurst", "ml", "trending", "mean-reversion"],
    ehlers_summary: "The Hurst Exponent, pioneered by Harold Edwin Hurst in 1951, quantifies the 'memory' of a time series. In technical analysis, it distinguishes between trending, mean-reverting, and random walk price action. It is a critical feature for machine learning models to adapt their logic to the underlying market structure.",
    params: &[
        ParamDef {
            name: "period",
            default: "100",
            description: "Lookback period for R/S analysis",
        },
    ],
    formula_source: "https://en.wikipedia.org/wiki/Hurst_exponent",
    formula_latex: r#"
\[
H = \frac{\ln(R/S)}{\ln(N)}
\]
"#,
    gold_standard_file: "hurst_exponent.json",
    category: "ML Features",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_hurst_basic() {
        let period = 10;
        let mut hurst = HurstExponent::new(period);
        // Upward trend should have H > 0.5
        for i in 0..period {
            let res = hurst.next(i as f64);
            if i < period - 1 {
                assert!(res.is_nan());
            } else {
                assert!(res > 0.5);
            }
        }
    }

    proptest! {
        #[test]
        fn test_hurst_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let period = 20;
            let mut hurst = HurstExponent::new(period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| hurst.next(x)).collect();

            // Reference implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            for i in 0..inputs.len() {
                if i < period - 1 {
                    batch_results.push(f64::NAN);
                    continue;
                }

                let slice = &inputs[i + 1 - period..=i];
                let sum: f64 = slice.iter().sum();
                let mean = sum / period as f64;

                let mut cum_dev = 0.0;
                let mut max_z = f64::MIN;
                let mut min_z = f64::MAX;
                for &val in slice {
                    cum_dev += val - mean;
                    if cum_dev > max_z { max_z = cum_dev; }
                    if cum_dev < min_z { min_z = cum_dev; }
                }
                let range = max_z - min_z;

                let var_sum: f64 = slice.iter().map(|&v| (v - mean).powi(2)).sum();
                let std_dev = (var_sum / period as f64).sqrt();

                let res = if std_dev == 0.0 { 0.5 } else { (range / std_dev).ln() / (period as f64).ln() };
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-10);
                }
            }
        }
    }
}
