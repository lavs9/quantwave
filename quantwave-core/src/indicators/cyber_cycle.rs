use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// John Ehlers' Cyber Cycle
/// As described in "Cybernetic Analysis for Stocks and Futures" (2004), Chapter 4, Page 33-34.
///
/// The Cyber Cycle is an indicator that models the cyclical component of price movement.
/// It uses a 4-bar symmetrical finite impulse response (FIR) filter for smoothing
/// and an alpha calculation to isolate the cycle.
#[derive(Debug, Clone)]
pub struct CyberCycle {
    alpha: f64,
    x: [f64; 4],     // X[t], X[t-1], X[t-2], X[t-3]
    x_s: [f64; 3],   // X_S[t], X_S[t-1], X_S[t-2]
    cc: [f64; 3],    // CC[t], CC[t-1], CC[t-2]
    trigger: f64,
    t: usize,
}

impl CyberCycle {
    pub fn new(length: usize) -> Self {
        let alpha = 2.0 / ((length as f64) + 1.0);
        Self {
            alpha,
            x: [0.0; 4],
            x_s: [0.0; 3],
            cc: [0.0; 3],
            trigger: 0.0,
            t: 0,
        }
    }
}

impl Next<f64> for CyberCycle {
    type Output = (f64, f64); // (CyberCycle, Trigger)

    fn next(&mut self, input: f64) -> Self::Output {
        self.x[3] = self.x[2];
        self.x[2] = self.x[1];
        self.x[1] = self.x[0];
        self.x[0] = input;

        let smooth = (self.x[0] + 2.0 * self.x[1] + 2.0 * self.x[2] + self.x[3]) / 6.0;

        self.x_s[2] = self.x_s[1];
        self.x_s[1] = self.x_s[0];
        self.x_s[0] = smooth;

        self.cc[2] = self.cc[1];
        self.cc[1] = self.cc[0];

        // Ehlers' typical trigger is CC delayed by 1 bar
        self.trigger = self.cc[1];

        if self.t < 6 {
            self.cc[0] = (self.x[0] - 2.0 * self.x[1] + self.x[2]) / 4.0;
        } else {
            let part1 = (1.0 - 0.5 * self.alpha).powi(2) * (self.x_s[0] - 2.0 * self.x_s[1] + self.x_s[2]);
            let part2 = 2.0 * (1.0 - self.alpha) * self.cc[1];
            let part3 = (1.0 - self.alpha).powi(2) * self.cc[2];
            self.cc[0] = part1 + part2 - part3;
        }

        self.t += 1;

        (self.cc[0], self.trigger)
    }
}

pub const CYBER_CYCLE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Cyber Cycle",
    description: "An oscillator introduced by John Ehlers that models the cyclical component of a time series using FIR smoothing.",
    params: &[
        ParamDef { name: "length", default: "14", description: "Alpha smoothing length parameter" },
    ],
    formula_source: "Cybernetic Analysis for Stocks and Futures, John Ehlers, 2004, Chapter 4",
    formula_latex: r#"
\[
\alpha = \frac{2}{\text{Length} + 1}
\]
\[
\text{Smooth} = \frac{X_t + 2X_{t-1} + 2X_{t-2} + X_{t-3}}{6}
\]
\[
CC_t = \left(1 - \frac{\alpha}{2}\right)^2 (\text{Smooth}_t - 2\text{Smooth}_{t-1} + \text{Smooth}_{t-2}) + 2(1 - \alpha)CC_{t-1} - (1 - \alpha)^2 CC_{t-2}
\]
"#,
    gold_standard_file: "cyber_cycle.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn cyber_cycle_batch(data: &[f64], length: usize) -> Vec<(f64, f64)> {
        let mut cc = CyberCycle::new(length);
        data.iter().map(|&x| cc.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_cyber_cycle_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let length = 14;
            let mut streaming_cc = CyberCycle::new(length);
            let streaming_results: Vec<(f64, f64)> = input.iter().map(|&x| streaming_cc.next(x)).collect();
            let batch_results = cyber_cycle_batch(&input, length);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
            }
        }
    }
}
