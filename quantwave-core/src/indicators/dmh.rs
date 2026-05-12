use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;
use std::collections::VecDeque;

/// The DMH: An Improved Directional Movement Indicator
/// 
/// Based on John Ehlers' article "The DMH: An Improved Directional Movement Indicator" (TASC Dec 2021).
/// This indicator modernizes Wilder's Directional Movement by applying Hann windowing to an EMA
/// of the directional difference, significantly reducing lag and noise compared to the classic ADX/DMI.
#[derive(Debug, Clone)]
pub struct DMH {
    length: usize,
    sf: f64,
    ema: f64,
    ema_history: VecDeque<f64>,
    prev_high: Option<f64>,
    prev_low: Option<f64>,
    hann_coeffs: Vec<f64>,
    sum_coeffs: f64,
    count: usize,
}

impl DMH {
    pub fn new(length: usize) -> Self {
        let sf = 1.0 / (length as f64);
        
        // Pre-calculate Hann coefficients
        let mut hann_coeffs = Vec::with_capacity(length);
        let mut sum_coeffs = 0.0;
        let length_plus_1 = (length + 1) as f64;
        
        for i in 1..=length {
            let coef = 1.0 - (2.0 * PI * (i as f64) / length_plus_1).cos();
            hann_coeffs.push(coef);
            sum_coeffs += coef;
        }

        Self {
            length,
            sf,
            ema: 0.0,
            ema_history: VecDeque::with_capacity(length),
            prev_high: None,
            prev_low: None,
            hann_coeffs,
            sum_coeffs,
            count: 0,
        }
    }
}

impl Next<(f64, f64)> for DMH {
    type Output = f64;

    fn next(&mut self, (high, low): (f64, f64)) -> Self::Output {
        self.count += 1;

        let (plus_dm, minus_dm) = match (self.prev_high, self.prev_low) {
            (Some(ph), Some(pl)) => {
                let upper_move = high - ph;
                let lower_move = pl - low;
                
                let mut p_dm = 0.0;
                let mut m_dm = 0.0;
                
                if upper_move > lower_move && upper_move > 0.0 {
                    p_dm = upper_move;
                } else if lower_move > upper_move && lower_move > 0.0 {
                    m_dm = lower_move;
                }
                (p_dm, m_dm)
            },
            _ => (0.0, 0.0),
        };

        self.prev_high = Some(high);
        self.prev_low = Some(low);

        // Wilder's EMA (Smoothing Factor = 1/Length)
        let diff = plus_dm - minus_dm;
        if self.count == 1 {
            self.ema = diff;
        } else {
            self.ema = self.sf * diff + (1.0 - self.sf) * self.ema;
        }

        self.ema_history.push_front(self.ema);
        if self.ema_history.len() > self.length {
            self.ema_history.pop_back();
        }

        // Apply Hann Windowed FIR filter
        if self.ema_history.len() < self.length {
            // During startup, we can still calculate but we'll use a partial window
            let mut dm_sum = 0.0;
            let mut partial_sum_coeffs = 0.0;
            for (i, &val) in self.ema_history.iter().enumerate() {
                let coef = self.hann_coeffs[i];
                dm_sum += coef * val;
                partial_sum_coeffs += coef;
            }
            if partial_sum_coeffs != 0.0 { dm_sum / partial_sum_coeffs } else { 0.0 }
        } else {
            let mut dm_sum = 0.0;
            for (i, &val) in self.ema_history.iter().enumerate() {
                dm_sum += self.hann_coeffs[i] * val;
            }
            if self.sum_coeffs != 0.0 { dm_sum / self.sum_coeffs } else { 0.0 }
        }
    }
}

pub const DMH_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "DMH",
    description: "An improved Directional Movement indicator using Hann windowing for smoother signals and reduced lag.",
    params: &[
        ParamDef { name: "length", default: "14", description: "Smoothing period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/implemented/TRADERS%E2%80%99%20TIPS%20-%20DECEMBER%202021.html",
    formula_latex: r#"
\[
\text{PlusDM} = \text{High} - \text{High}_{t-1} \text{ if } > (\text{Low}_{t-1} - \text{Low}) \text{ and } > 0, \text{ else } 0
\]
\[
\text{MinusDM} = \text{Low}_{t-1} - \text{Low} \text{ if } > (\text{High} - \text{High}_{t-1}) \text{ and } > 0, \text{ else } 0
\]
\[
\text{EMA} = \frac{1}{L}(\text{PlusDM} - \text{MinusDM}) + (1 - \frac{1}{L})\text{EMA}_{t-1}
\]
\[
\text{DMH} = \frac{\sum_{i=1}^{L} w_i \text{EMA}_{t-i+1}}{\sum_{i=1}^{L} w_i}, \text{ where } w_i = 1 - \cos\left(\frac{2\pi i}{L+1}\right)
\]
"#,
    gold_standard_file: "dmh.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_dmh_basic() {
        let mut dmh = DMH::new(14);
        let inputs = vec![
            (10.0, 9.0),
            (11.0, 10.0),
            (12.0, 11.0),
            (13.0, 12.0),
            (12.0, 11.0),
            (11.0, 10.0),
        ];
        for input in inputs {
            let res = dmh.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_dmh_parity(
            highs in prop::collection::vec(10.0..20.0, 50..100),
            lows in prop::collection::vec(5.0..15.0, 50..100),
        ) {
            let len = highs.len().min(lows.len());
            let inputs: Vec<(f64, f64)> = (0..len).map(|i| {
                let h: f64 = highs[i];
                let l: f64 = lows[i];
                (h.max(l), h.min(l))
            }).collect();
            
            let length = 14;
            let mut dmh = DMH::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&val| dmh.next(val)).collect();
            
            // Reference implementation
            let mut ema = 0.0;
            let mut ema_hist = Vec::new();
            let mut batch_results = Vec::with_capacity(len);
            
            let sf = 1.0 / length as f64;
            let mut hann_coeffs = Vec::new();
            for i in 1..=length {
                let c = 1.0 - (2.0 * PI * i as f64 / (length + 1) as f64).cos();
                hann_coeffs.push(c);
            }

            for i in 0..len {
                let (plus_dm, minus_dm) = if i == 0 {
                    (0.0, 0.0)
                } else {
                    let um = inputs[i].0 - inputs[i-1].0;
                    let lm = inputs[i-1].1 - inputs[i].1;
                    let mut p = 0.0;
                    let mut m = 0.0;
                    if um > lm && um > 0.0 { p = um; }
                    else if lm > um && lm > 0.0 { m = lm; }
                    (p, m)
                };
                
                let diff = plus_dm - minus_dm;
                if i == 0 {
                    ema = diff;
                } else {
                    ema = sf * diff + (1.0 - sf) * ema;
                }
                
                ema_hist.push(ema);
                
                let mut dm_sum = 0.0;
                let mut cur_sum_coeffs = 0.0;
                
                let start = if i + 1 > length { i + 1 - length } else { 0 };
                let window = &ema_hist[start..i+1];
                
                for (j, &val) in window.iter().rev().enumerate() {
                    let c = hann_coeffs[j];
                    dm_sum += c * val;
                    cur_sum_coeffs += c;
                }
                
                batch_results.push(dm_sum / cur_sum_coeffs);
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
