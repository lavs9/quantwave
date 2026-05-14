use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::EMA;
use crate::traits::Next;

/// ZeroLag Indicator
///
/// Based on John Ehlers' "Zero Lag (well, almost)"
/// The indicator acknowledgement that the EMA filter has an error term: Error = Price - EMA[1].
/// It introduces this error term into the equation in addition to the value of the new data sample,
/// and applies a gain term to minimize the lag.
#[derive(Debug, Clone)]
pub struct ZeroLag {
    alpha: f64,
    gain_limit: f64,
    ema: EMA,
    ec_prev: Option<f64>,
}

impl ZeroLag {
    pub fn new(length: usize, gain_limit: f64) -> Self {
        let alpha = 2.0 / (length as f64 + 1.0);
        Self {
            alpha,
            gain_limit,
            ema: EMA::new(length),
            ec_prev: None,
        }
    }
}

impl Next<f64> for ZeroLag {
    type Output = (f64, f64); // (EC, EMA)

    fn next(&mut self, input: f64) -> Self::Output {
        let ema_val = self.ema.next(input);

        let ec_prev = match self.ec_prev {
            Some(prev) => prev,
            None => {
                self.ec_prev = Some(input);
                return (input, ema_val);
            }
        };

        let mut least_error = f64::MAX;
        let mut best_gain = 0.0;

        let gain_limit_steps = (self.gain_limit) as i32;

        for i in -gain_limit_steps..=gain_limit_steps {
            let gain = i as f64 / 10.0;
            let ec =
                self.alpha * (ema_val + gain * (input - ec_prev)) + (1.0 - self.alpha) * ec_prev;
            let error = (input - ec).abs();
            if error < least_error {
                least_error = error;
                best_gain = gain;
            }
        }

        let ec =
            self.alpha * (ema_val + best_gain * (input - ec_prev)) + (1.0 - self.alpha) * ec_prev;
        self.ec_prev = Some(ec);

        (ec, ema_val)
    }
}

pub const ZERO_LAG_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Zero Lag EC",
    description: "Zero Lag Error Corrected EMA attempts to eliminate lag by adding an error term to the EMA.",
    usage: "Use as a near-zero-lag moving average for trend-following systems. The error-correction term removes the lag inherent in the standard EMA without introducing significant overshoot.",
    keywords: &["moving-average", "zero-lag", "ehlers", "ema", "smoothing"],
    ehlers_summary: "Ehlers introduces the Zero Lag indicator in Cybernetic Analysis as an EMA with an added error-correction term that subtracts the average lag from the output. The resulting EC (Error Corrected) line tracks price with near-zero delay while the ZL-EMA provides a smoothed reference, with crossovers between them providing trade signals.",
    params: &[
        ParamDef {
            name: "length",
            default: "20",
            description: "Equivalent SMA length",
        },
        ParamDef {
            name: "gain_limit",
            default: "50.0",
            description: "Gain limit (divided by 10 for actual gain)",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/implemented/ZeroLag.pdf",
    formula_latex: r#"
\[
\alpha = \frac{2}{Length + 1}
\]
\[
EMA = \alpha \times Close + (1 - \alpha) \times EMA_{t-1}
\]
\[
EC = \alpha \times (EMA + Gain \times (Close - EC_{t-1})) + (1 - \alpha) \times EC_{t-1}
\]
"#,
    gold_standard_file: "zero_lag.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_zero_lag_basic() {
        let mut zl = ZeroLag::new(20, 50.0);
        let inputs = vec![10.0, 11.0, 12.0, 11.0, 10.0];
        for input in inputs {
            let (ec, ema) = zl.next(input);
            println!("Input: {}, EC: {}, EMA: {}", input, ec, ema);
            assert!(!ec.is_nan());
            assert!(!ema.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_zero_lag_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let length = 20;
            let gain_limit = 50.0;
            let mut zl = ZeroLag::new(length, gain_limit);

            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| zl.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let alpha = 2.0 / (length as f64 + 1.0);
            let mut ema_prev = None;
            let mut ec_prev = None;

            for &input in &inputs {
                let ema = match ema_prev {
                    Some(prev) => alpha * input + (1.0 - alpha) * prev,
                    None => input,
                };
                ema_prev = Some(ema);

                let ec = match ec_prev {
                    Some(prev) => {
                        let mut least_err = f64::MAX;
                        let mut best_g = 0.0;
                        for i in -50..=50 {
                            let g = i as f64 / 10.0;
                            let ec_val: f64 = alpha * (ema + g * (input - prev)) + (1.0 - alpha) * prev;
                            let err = (input - ec_val).abs();
                            if err < least_err {
                                least_err = err;
                                best_g = g;
                            }
                        }
                        alpha * (ema + best_g * (input - prev)) + (1.0 - alpha) * prev
                    }
                    None => input,
                };
                ec_prev = Some(ec);
                batch_results.push((ec, ema));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
