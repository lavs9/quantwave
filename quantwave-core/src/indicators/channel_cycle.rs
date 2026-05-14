use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// Channel Cycle Indicator
///
/// Based on John Ehlers' "Inferring Trading Strategies from Probability Distribution Functions".
/// Detrends price using a channel normalization, then applies a bandpass filter to extract
/// a near sine wave, and computes a leading cosine-like function.
#[derive(Debug, Clone)]
pub struct ChannelCycle {
    period: usize,
    alpha: f64,
    beta: f64,
    price_window: Vec<f64>,
    bp_history: [f64; 2],
    detrended_prev: [f64; 2],
    count: usize,
}

impl ChannelCycle {
    pub fn new(period: usize) -> Self {
        // Standard Ehlers Bandpass for delta=0.1
        let delta = 0.1;
        let beta = (2.0 * PI / period as f64).cos();
        let gamma = 1.0 / (4.0 * PI * delta / period as f64).cos();
        let alpha = gamma - (gamma * gamma - 1.0).sqrt();

        Self {
            period,
            alpha,
            beta,
            price_window: Vec::with_capacity(period),
            bp_history: [0.0; 2],
            detrended_prev: [0.0; 2],
            count: 0,
        }
    }
}

impl Default for ChannelCycle {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Next<f64> for ChannelCycle {
    type Output = (f64, f64); // (Sine, Leading)

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        self.price_window.push(input);
        if self.price_window.len() > self.period {
            self.price_window.remove(0);
        }

        if self.price_window.len() < self.period {
            return (0.0, 0.0);
        }

        let mut high = f64::MIN;
        let mut low = f64::MAX;
        for &p in &self.price_window {
            if p > high { high = p; }
            if p < low { low = p; }
        }

        let detrended = if high != low {
            (input - low) / (high - low) - 0.5
        } else {
            0.0
        };

        // Bandpass Filter
        let bp = 0.5 * (1.0 - self.alpha) * (detrended - self.detrended_prev[1])
            + self.beta * (1.0 + self.alpha) * self.bp_history[0]
            - self.alpha * self.bp_history[1];

        // Leading Function: derivative corrected for angular frequency
        // leading = (BP - BP[1]) / (2*PI/Period)
        let omega = 2.0 * PI / self.period as f64;
        let leading = (bp - self.bp_history[0]) / omega;

        // Shift history
        self.bp_history[1] = self.bp_history[0];
        self.bp_history[0] = bp;
        self.detrended_prev[1] = self.detrended_prev[0];
        self.detrended_prev[0] = detrended;

        (bp, leading)
    }
}

pub const CHANNEL_CYCLE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "ChannelCycle",
    description: "Extracts cyclic components and a leading function using channel-normalized bandpass filtering.",
    usage: "Use to estimate the dominant cycle period from the width of price channels. Useful as a simpler alternative to Hilbert Transform cycle measurement when computational resources are limited.",
    keywords: &["cycle", "ehlers", "dsp", "dominant-cycle"],
    ehlers_summary: "Ehlers estimates the dominant cycle period by tracking successive peaks and troughs of price. The distance between turning points approximates half the cycle period, and smoothing this measurement across recent bars gives a stable period estimate for use in adaptive indicators.",
    params: &[
        ParamDef {
            name: "period",
            default: "20",
            description: "Channel and Bandpass period",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/InferringTradingStrategies.pdf",
    formula_latex: r#"
\[
Detrended = \frac{Price - Low}{High - Low} - 0.5
\]
\[
BP = \text{Bandpass}(Detrended, Period)
\]
\[
Leading = \frac{BP - BP_{t-1}}{2\pi/Period}
\]
"#,
    gold_standard_file: "channel_cycle.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_channel_cycle_basic() {
        let mut cc = ChannelCycle::new(20);
        for i in 0..100 {
            let (s, l) = cc.next(100.0 + (i as f64 * 0.1).sin());
            assert!(!s.is_nan());
            assert!(!l.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_channel_cycle_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..200),
        ) {
            let period = 20;
            let mut cc = ChannelCycle::new(period);
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| cc.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let delta = 0.1;
            let beta = (2.0 * PI / period as f64).cos();
            let gamma = 1.0 / (4.0 * PI * delta / period as f64).cos();
            let alpha = gamma - (gamma * gamma - 1.0).sqrt();
            let omega = 2.0 * PI / period as f64;

            let mut detrended_vals = Vec::new();
            let mut bp_vals = vec![0.0; inputs.len() + 2];
            let mut d_vals = vec![0.0; inputs.len() + 2];

            for (i, &input) in inputs.iter().enumerate() {
                let start = if i >= period - 1 { i + 1 - period } else { 0 };
                let window = &inputs[start..i + 1];
                
                if window.len() < period {
                    batch_results.push((0.0, 0.0));
                    detrended_vals.push(0.0);
                    continue;
                }

                let mut high = f64::MIN;
                let mut low = f64::MAX;
                for &p in window {
                    if p > high { high = p; }
                    if p < low { low = p; }
                }

                let detrended = if high != low {
                    (input - low) / (high - low) - 0.5
                } else {
                    0.0
                };
                detrended_vals.push(detrended);
                
                let idx = i + 2;
                d_vals[idx] = detrended;
                let bp = 0.5 * (1.0 - alpha) * (d_vals[idx] - d_vals[idx-2])
                    + beta * (1.0 + alpha) * bp_vals[idx-1]
                    - alpha * bp_vals[idx-2];
                bp_vals[idx] = bp;

                let leading = (bp - bp_vals[idx-1]) / omega;
                batch_results.push((bp, leading));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
