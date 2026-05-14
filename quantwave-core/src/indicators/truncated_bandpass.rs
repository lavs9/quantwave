use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Truncated Bandpass Filter
///
/// Based on John Ehlers' "Truncated Indicators" (TASC July 2020).
/// It applies a Bandpass filter but truncates the feedback loop at a specific length
/// to prevent "ringing" and better handle sharp price movements.
#[derive(Debug, Clone)]
pub struct TruncatedBandpass {
    _period: f64,
    _bandwidth: f64,
    length: usize,
    prices: VecDeque<f64>,
    l1: f64,
    s1: f64,
}

impl TruncatedBandpass {
    pub fn new(period: usize, bandwidth: f64, length: usize) -> Self {
        let p = period as f64;
        let deg_to_rad = std::f64::consts::PI / 180.0;
        let l1 = (360.0 / p * deg_to_rad).cos();
        let g1 = (bandwidth * 360.0 / p * deg_to_rad).cos();
        let s1 = 1.0 / g1 - (1.0 / (g1 * g1) - 1.0).sqrt();

        Self {
            _period: p,
            _bandwidth: bandwidth,
            length,
            prices: VecDeque::with_capacity(length + 2),
            l1,
            s1,
        }
    }
}

impl Default for TruncatedBandpass {
    fn default() -> Self {
        Self::new(20, 0.1, 10)
    }
}

impl Next<f64> for TruncatedBandpass {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.prices.push_front(input);
        if self.prices.len() > self.length + 2 {
            self.prices.pop_back();
        }

        if self.prices.len() < self.length + 2 {
            return 0.0;
        }

        // Truncated calculation
        // Trunc[Length + 2] = 0;
        // Trunc[Length + 1] = 0;
        let mut t2 = 0.0;
        let mut t1 = 0.0;
        let mut bpt = 0.0;

        // In the EasyLanguage code:
        // for count = Length downto 1
        // Trunc[count] = .5*(1-S1)*(Close[count-1] - Close[count+1]) + L1*(1+S1)*Trunc[count+1] - S1*Trunc[count+2]
        
        // Let's iterate from the oldest relevant bar (Length) to the newest (1)
        for i in (0..self.length).rev() {
            // Close[i] in EL is price at index i (0 is current)
            // Close[count-1] -> prices[i]
            // Close[count+1] -> prices[i+2]
            let val = 0.5 * (1.0 - self.s1) * (self.prices[i] - self.prices[i + 2])
                + self.l1 * (1.0 + self.s1) * t1
                - self.s1 * t2;
            
            t2 = t1;
            t1 = val;
            bpt = val;
        }

        bpt
    }
}

pub const TRUNCATED_BANDPASS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "TruncatedBandpass",
    description: "Truncated Bandpass filter for handling sharp price movements.",
    usage: "Use to isolate cyclic components while minimizing 'ringing' effects caused by sudden price shocks. Ideal for cycle-based trading systems in volatile markets.",
    keywords: &["filter", "ehlers", "dsp", "bandpass", "cycle", "robust"],
    ehlers_summary: "Finite Impulse Response (FIR) filters have a fixed history, while Infinite Impulse Response (IIR) filters technically have an infinite history. Truncation limits the IIR feedback loop to a specific length, combining the sharp selectivity of IIR with the outlier-rejection of FIR.",
    params: &[
        ParamDef {
            name: "period",
            default: "20",
            description: "Cycle period to isolate",
        },
        ParamDef {
            name: "bandwidth",
            default: "0.1",
            description: "Bandwidth of the filter",
        },
        ParamDef {
            name: "length",
            default: "10",
            description: "Truncation length",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2020/07/TradersTips.html",
    formula_latex: r#"
\[
L1 = \cos(360/P), \quad G1 = \cos(BW \cdot 360/P), \quad S1 = 1/G1 - \sqrt{1/G1^2 - 1}
\]
\[
BPT_t = \text{IIR window of length } L \text{ with zero initial conditions}
\]
"#,
    gold_standard_file: "truncated_bandpass.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_truncated_bandpass_basic() {
        let mut tbp = TruncatedBandpass::new(20, 0.1, 10);
        for _ in 0..50 {
            let _ = tbp.next(100.0);
        }
        let val = tbp.next(100.0);
        // On constant input, Bandpass should output 0
        approx::assert_relative_eq!(val, 0.0, epsilon = 1e-10);
    }

    proptest! {
        #[test]
        fn test_truncated_bandpass_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let period = 20;
            let bandwidth = 0.1;
            let length = 10;
            let mut tbp = TruncatedBandpass::new(period, bandwidth, length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| tbp.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let p = period as f64;
            let deg_to_rad = std::f64::consts::PI / 180.0;
            let l1 = (360.0 / p * deg_to_rad).cos();
            let g1 = (bandwidth * 360.0 / p * deg_to_rad).cos();
            let s1 = 1.0 / g1 - (1.0 / (g1 * g1) - 1.0).sqrt();

            for i in 0..inputs.len() {
                if i < length + 1 {
                    batch_results.push(0.0);
                    continue;
                }
                
                let mut t2 = 0.0;
                let mut t1 = 0.0;
                let mut bpt = 0.0;
                for k in (0..length).rev() {
                    let val = 0.5 * (1.0 - s1) * (inputs[i - k] - inputs[i - k - 2])
                        + l1 * (1.0 + s1) * t1
                        - s1 * t2;
                    t2 = t1;
                    t1 = val;
                    bpt = val;
                }
                batch_results.push(bpt);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
