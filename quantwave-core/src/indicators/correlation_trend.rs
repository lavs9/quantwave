use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Correlation Trend Indicator
///
/// Based on John Ehlers' "Correlation as a Trend Indicator" (2020).
/// It calculates the Pearson correlation coefficient between price and a downward-sloping linear ramp
/// (representing the time axis going backwards).
/// A value near +1 indicates a strong uptrend, and -1 indicates a strong downtrend.
#[derive(Debug, Clone)]
pub struct CorrelationTrend {
    length: usize,
    window: VecDeque<f64>,
    sy: f64,
    syy: f64,
}

impl CorrelationTrend {
    pub fn new(length: usize) -> Self {
        let mut sy = 0.0;
        let mut syy = 0.0;
        for i in 0..length {
            let y = -(i as f64);
            sy += y;
            syy += y * y;
        }

        Self {
            length,
            window: VecDeque::with_capacity(length),
            sy,
            syy,
        }
    }
}

impl Next<f64> for CorrelationTrend {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_front(input);
        if self.window.len() > self.length {
            self.window.pop_back();
        }

        if self.window.len() < self.length {
            return 0.0;
        }

        let mut sx = 0.0;
        let mut sxx = 0.0;
        let mut sxy = 0.0;
        for (i, &x) in self.window.iter().enumerate() {
            let y = -(i as f64);
            sx += x;
            sxx += x * x;
            sxy += x * y;
        }

        let l_f = self.length as f64;
        let div1 = l_f * sxx - sx * sx;
        let div2 = l_f * self.syy - self.sy * self.sy;

        if div1 > 0.0 && div2 > 0.0 {
            (l_f * sxy - sx * self.sy) / (div1 * div2).sqrt()
        } else {
            0.0
        }
    }
}

pub const CORRELATION_TREND_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Correlation Trend",
    description: "Calculates the Pearson correlation between price and a linear time ramp to identify trends.",
    usage: "Use to confirm whether price is trending or cycling before applying directional strategies. High correlation indicates a strong trend; low correlation indicates a cycling market.",
    keywords: &["trend", "correlation", "ehlers", "statistics"],
    ehlers_summary: "In 'Correlation As A Trend Indicator' (2020), Ehlers uses the Pearson correlation coefficient between price and a linear ramp to identify trend strength. A coefficient near +1.0 indicates a consistent uptrend, while -1.0 indicates a consistent downtrend. Unlike standard moving averages, this approach is independent of price amplitude and focuses purely on the linearity of the move.",
    params: &[ParamDef {
        name: "length",
        default: "20",
        description: "Correlation window length",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/CORRELATION%20AS%20A%20TREND%20INDICATOR.pdf",
    formula_latex: r#"
\[
X_i = Price_{t-i}, Y_i = -i
\]
\[
R = \frac{n \sum X_i Y_i - \sum X_i \sum Y_i}{\sqrt{(n \sum X_i^2 - (\sum X_i)^2)(n \sum Y_i^2 - (\sum Y_i)^2)}}
\]
"#,
    gold_standard_file: "correlation_trend.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_correlation_trend_basic() {
        let mut ct = CorrelationTrend::new(20);
        for i in 0..30 {
            let res = ct.next(i as f64);
            if i >= 19 {
                // For a perfectly straight line, correlation should be 1.0 (or -1.0 depending on Y direction)
                // Here Y = -i, so as i increases, Y decreases.
                // Price = i, so as i increases, Price increases.
                // This is anti-correlation? No, wait.
                // count=0: Close[0]=20, Y=0
                // count=1: Close[1]=19, Y=-1
                // Price decreases as Y decreases. So positive correlation.
                assert!(res > 0.99);
            }
        }
    }

    proptest! {
        #[test]
        fn test_correlation_trend_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let length = 20;
            let mut ct = CorrelationTrend::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| ct.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let l_f = length as f64;
            let mut sy = 0.0;
            let mut syy = 0.0;
            for i in 0..length {
                let y = -(i as f64);
                sy += y;
                syy += y * y;
            }

            for i in 0..inputs.len() {
                if i < length - 1 {
                    batch_results.push(0.0);
                    continue;
                }

                let mut sx = 0.0;
                let mut sxx = 0.0;
                let mut sxy = 0.0;
                for j in 0..length {
                    let x = inputs[i-j];
                    let y = -(j as f64);
                    sx += x;
                    sxx += x * x;
                    sxy += x * y;
                }

                let div1 = l_f * sxx - sx * sx;
                let div2 = l_f * syy - sy * sy;
                let res = if div1 > 0.0 && div2 > 0.0 {
                    (l_f * sxy - sx * sy) / (div1 * div2).sqrt()
                } else {
                    0.0
                };
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
