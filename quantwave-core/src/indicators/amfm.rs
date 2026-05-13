use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// AM Detector (Volatility)
/// 
/// Based on John Ehlers' "A Technical Description of Market Data for Traders".
/// It recovers market volatility by detecting the envelope of the amplitude-modulated 
/// whitened price spectrum.
#[derive(Debug, Clone)]
pub struct AMDetector {
    highest_len: usize,
    avg_len: usize,
    deriv_history: VecDeque<f64>,
    envelope_history: VecDeque<f64>,
    sum_envelope: f64,
}

impl AMDetector {
    pub fn new(highest_len: usize, avg_len: usize) -> Self {
        Self {
            highest_len,
            avg_len,
            deriv_history: VecDeque::with_capacity(highest_len),
            envelope_history: VecDeque::with_capacity(avg_len),
            sum_envelope: 0.0,
        }
    }
}

impl Next<(f64, f64)> for AMDetector {
    type Output = f64;

    fn next(&mut self, (close, open): (f64, f64)) -> Self::Output {
        let deriv = (close - open).abs();
        self.deriv_history.push_front(deriv);
        if self.deriv_history.len() > self.highest_len {
            self.deriv_history.pop_back();
        }

        let envelope = self.deriv_history.iter().fold(f64::MIN, |a, &b| a.max(b));
        
        self.envelope_history.push_back(envelope);
        self.sum_envelope += envelope;
        if self.envelope_history.len() > self.avg_len {
            if let Some(old) = self.envelope_history.pop_front() {
                self.sum_envelope -= old;
            }
        }

        self.sum_envelope / self.envelope_history.len() as f64
    }
}

/// FM Demodulator (Timing)
/// 
/// Based on John Ehlers' "A Technical Description of Market Data for Traders".
/// It extracts market timing information by demodulating the frequency-modulated 
/// price spectrum using a hard limiter and a SuperSmoother filter.
#[derive(Debug, Clone)]
pub struct FMDemodulator {
    period: usize,
    c1: f64,
    c2: f64,
    c3: f64,
    hl_prev: f64,
    ss_history: [f64; 2],
    count: usize,
}

impl FMDemodulator {
    pub fn new(period: usize) -> Self {
        let a1 = (-1.414 * PI / period as f64).exp();
        let c2 = 2.0 * a1 * (1.414 * PI / period as f64).cos();
        let c3 = -a1 * a1;
        let c1 = 1.0 - c2 - c3;
        
        Self {
            period,
            c1,
            c2,
            c3,
            hl_prev: 0.0,
            ss_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Next<(f64, f64)> for FMDemodulator {
    type Output = f64;

    fn next(&mut self, (close, open): (f64, f64)) -> Self::Output {
        self.count += 1;
        let deriv = close - open;
        let mut hl = 10.0 * deriv;
        if hl > 1.0 { hl = 1.0; }
        if hl < -1.0 { hl = -1.0; }

        let ss = if self.count < 3 {
            deriv
        } else {
            self.c1 * (hl + self.hl_prev) / 2.0
                + self.c2 * self.ss_history[0]
                + self.c3 * self.ss_history[1]
        };

        self.ss_history[1] = self.ss_history[0];
        self.ss_history[0] = ss;
        self.hl_prev = hl;
        
        ss
    }
}

pub const AM_DETECTOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "AM Detector",
    description: "Recovers market volatility from the amplitude-modulated whitened price spectrum.",
    params: &[
        ParamDef { name: "highest_len", default: "4", description: "Envelope lookback length" },
        ParamDef { name: "avg_len", default: "8", description: "Smoothing length" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/AMFM.pdf",
    formula_latex: r#"
\[
Deriv = |Close - Open|, Envel = \max(Deriv, 4), Volatil = \text{Avg}(Envel, 8)
\]
"#,
    gold_standard_file: "am_detector.json",
    category: "Ehlers DSP",
};

pub const FM_DEMODULATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "FM Demodulator",
    description: "Extracts market timing information by demodulating the frequency-modulated price spectrum.",
    params: &[
        ParamDef { name: "period", default: "30", description: "SuperSmoother period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/AMFM.pdf",
    formula_latex: r#"
\[
Deriv = Close - Open, HL = \text{Clip}(10 \times Deriv, -1, 1)
\]
\[
SS = c_1 \frac{HL + HL_{t-1}}{2} + c_2 SS_{t-1} + c_3 SS_{t-2}
\]
"#,
    gold_standard_file: "fm_demodulator.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_am_detector_basic() {
        let mut am = AMDetector::new(4, 8);
        let inputs = vec![(10.0, 9.0), (11.0, 10.0), (12.0, 11.0)];
        for input in inputs {
            let res = am.next(input);
            assert!(res >= 0.0);
        }
    }

    #[test]
    fn test_fm_demodulator_basic() {
        let mut fm = FMDemodulator::new(30);
        let inputs = vec![(10.0, 9.0), (11.0, 10.0), (12.0, 11.0)];
        for input in inputs {
            let res = fm.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_am_detector_parity(
            closes in prop::collection::vec(1.0..100.0, 50..100),
            opens in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let h_len = 4;
            let a_len = 8;
            let mut am = AMDetector::new(h_len, a_len);
            let inputs: Vec<(f64, f64)> = closes.iter().zip(opens.iter()).map(|(&c, &o)| (c, o)).collect();
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| am.next(x)).collect();
            
            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut envelope_hist = VecDeque::new();
            let mut sum_env = 0.0;
            
            for i in 0..inputs.len() {
                let start = if i >= h_len { i + 1 - h_len } else { 0 };
                let mut max_deriv = f64::MIN;
                for j in start..=i {
                    let deriv = (inputs[j].0 - inputs[j].1).abs();
                    if deriv > max_deriv { max_deriv = deriv; }
                }
                
                envelope_hist.push_back(max_deriv);
                sum_env += max_deriv;
                if envelope_hist.len() > a_len {
                    if let Some(old) = envelope_hist.pop_front() {
                        sum_env -= old;
                    }
                }
                batch_results.push(sum_env / envelope_hist.len() as f64);
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }

        #[test]
        fn test_fm_demodulator_parity(
            closes in prop::collection::vec(1.0..100.0, 50..100),
            opens in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let period = 30;
            let mut fm = FMDemodulator::new(period);
            let inputs: Vec<(f64, f64)> = closes.iter().zip(opens.iter()).map(|(&c, &o)| (c, o)).collect();
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| fm.next(x)).collect();
            
            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let a1 = (-1.414 * PI / period as f64).exp();
            let c2 = 2.0 * a1 * (1.414 * PI / period as f64).cos();
            let c3 = -a1 * a1;
            let c1 = 1.0 - c2 - c3;
            
            let mut hl_prev = 0.0;
            let mut ss_hist = [0.0; 2];
            
            for (i, &input) in inputs.iter().enumerate() {
                let deriv = input.0 - input.1;
                let mut hl = 10.0 * deriv;
                if hl > 1.0 { hl = 1.0; }
                if hl < -1.0 { hl = -1.0; }
                
                let res = if i + 1 < 3 {
                    deriv
                } else {
                    c1 * (hl + hl_prev) / 2.0 + c2 * ss_hist[0] + c3 * ss_hist[1]
                };
                
                ss_hist[1] = ss_hist[0];
                ss_hist[0] = res;
                hl_prev = hl;
                batch_results.push(res);
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
