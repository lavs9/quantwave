use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Projected Moving Average (PMA)
///
/// Based on John Ehlers' "Removing Moving Average Lag" (TASC March 2025).
/// Adds the linear regression slope multiplied by half the length to a simple moving average
/// to compensate for the lag inherent in moving averages.
/// Returns (PMA, Predict).
#[derive(Debug, Clone)]
pub struct ProjectedMovingAverage {
    length: usize,
    window: VecDeque<f64>,
    slope_history: VecDeque<f64>,
    sum_x: f64,
    sum_x2: f64,
}

impl ProjectedMovingAverage {
    pub fn new(length: usize) -> Self {
        let mut sum_x = 0.0;
        let mut sum_x2 = 0.0;
        for i in 1..=length {
            let x = i as f64;
            sum_x += x;
            sum_x2 += x * x;
        }
        Self {
            length,
            window: VecDeque::with_capacity(length),
            slope_history: VecDeque::from(vec![0.0; 3]),
            sum_x,
            sum_x2,
        }
    }
}

impl Default for ProjectedMovingAverage {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Next<f64> for ProjectedMovingAverage {
    type Output = (f64, f64);

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_front(input);
        if self.window.len() > self.length {
            self.window.pop_back();
        }

        if self.window.len() < self.length {
            return (input, input);
        }

        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;

        for i in 0..self.length {
            let y = self.window[i];
            let x = (i + 1) as f64;
            sum_y += y;
            sum_xy += x * y;
        }

        let n = self.length as f64;
        let denom = n * self.sum_x2 - self.sum_x * self.sum_x;
        let slope = if denom != 0.0 {
            -(n * sum_xy - self.sum_x * sum_y) / denom
        } else {
            0.0
        };
        let sma = sum_y / n;
        let pma = sma + slope * n / 2.0;

        self.slope_history.pop_back();
        self.slope_history.push_front(slope);

        let predict = pma + 0.5 * (slope - self.slope_history[2]) * n;

        (pma, predict)
    }
}

pub const PROJECTED_MOVING_AVERAGE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Projected Moving Average",
    description: "A lag-compensated moving average that uses linear regression slope to project the average forward.",
    params: &[ParamDef {
        name: "length",
        default: "20",
        description: "Calculation length",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20MARCH%202025.html",
    formula_latex: r#"
\[
Slope = -\frac{n \sum xy - \sum x \sum y}{n \sum x^2 - (\sum x)^2}
\]
\[
PMA = SMA + Slope \cdot \frac{n}{2}
\]
\[
Predict = PMA + 0.5 \cdot (Slope - Slope_{t-2}) \cdot n
\]
"#,
    gold_standard_file: "pma.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_pma_basic() {
        let mut pma = ProjectedMovingAverage::new(20);
        let inputs = vec![10.0; 40];
        for input in inputs {
            let (p, pr) = pma.next(input);
            assert_eq!(p, 10.0);
            assert_eq!(pr, 10.0);
        }
    }

    proptest! {
        #[test]
        fn test_pma_parity(
            inputs in prop::collection::vec(1.0..100.0, 40..100),
        ) {
            let length = 20;
            let mut pma = ProjectedMovingAverage::new(length);
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| pma.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut slope_hist = vec![0.0; 3];
            let mut sum_x = 0.0;
            let mut sum_x2 = 0.0;
            for i in 1..=length {
                let x = i as f64;
                sum_x += x;
                sum_x2 += x * x;
            }

            for i in 0..inputs.len() {
                if i < length - 1 {
                    batch_results.push((inputs[i], inputs[i]));
                    continue;
                }

                let mut sum_y = 0.0;
                let mut sum_xy = 0.0;
                for j in 0..length {
                    let y = inputs[i - j];
                    let x = (j + 1) as f64;
                    sum_y += y;
                    sum_xy += x * y;
                }

                let n = length as f64;
                let denom = n * sum_x2 - sum_x * sum_x;
                let slope = if denom != 0.0 {
                    -(n * sum_xy - sum_x * sum_y) / denom
                } else {
                    0.0
                };
                let sma = sum_y / n;
                let pma_val = sma + slope * n / 2.0;

                slope_hist.insert(0, slope);
                if slope_hist.len() > 3 {
                    slope_hist.pop();
                }

                let predict = pma_val + 0.5 * (slope - slope_hist[2]) * n;
                batch_results.push((pma_val, predict));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
