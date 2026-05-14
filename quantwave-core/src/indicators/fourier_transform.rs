use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Fourier Dominant Cycle
///
/// Based on John Ehlers' "Fourier Transform for Traders".
/// Estimates the dominant cycle period using a Discrete Fourier Transform (DFT)
/// with a Kay & Demeure resolution-enhancing transformation, followed by a
/// center-of-gravity calculation.
#[derive(Debug, Clone)]
pub struct FourierDominantCycle {
    window_len: usize,
    hp: HighPass,
    hp_history: [f64; 6],
    cleaned_window: VecDeque<f64>,
    count: usize,
}

impl FourierDominantCycle {
    pub fn new(window_len: usize) -> Self {
        Self {
            window_len,
            hp: HighPass::new(40),
            hp_history: [0.0; 6],
            cleaned_window: VecDeque::with_capacity(window_len),
            count: 0,
        }
    }
}

impl Default for FourierDominantCycle {
    fn default() -> Self {
        Self::new(50)
    }
}

impl Next<f64> for FourierDominantCycle {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let hp_val = self.hp.next(input);

        // FIR Smoothing: CleanedData = (HP + 2*HP[1] + 3*HP[2] + 3*HP[3] + 2*HP[4] + HP[5])/12
        let cleaned = (hp_val 
            + 2.0 * self.hp_history[0] 
            + 3.0 * self.hp_history[1] 
            + 3.0 * self.hp_history[2] 
            + 2.0 * self.hp_history[3] 
            + self.hp_history[4]) / 12.0;

        // Shift HP history
        for i in (1..6).rev() {
            self.hp_history[i] = self.hp_history[i-1];
        }
        self.hp_history[0] = hp_val;

        self.cleaned_window.push_front(cleaned);
        if self.cleaned_window.len() > self.window_len {
            self.cleaned_window.pop_back();
        }

        if self.cleaned_window.len() < self.window_len {
            return 0.0;
        }

        // DFT
        let mut pwr = vec![0.0; 51]; // Periods 8 to 50
        for period_idx in 8..=50 {
            let period = period_idx as f64;
            let mut cos_part = 0.0;
            let mut sin_part = 0.0;
            for n in 0..self.window_len {
                let angle = 2.0 * PI * n as f64 / period;
                cos_part += self.cleaned_window[n] * angle.cos();
                sin_part += self.cleaned_window[n] * angle.sin();
            }
            pwr[period_idx] = cos_part * cos_part + sin_part * sin_part;
        }

        // Max Power
        let mut max_pwr = 0.0;
        for &p in &pwr[8..=50] {
            if p > max_pwr { max_pwr = p; }
        }

        // dB and CG
        let mut num = 0.0;
        let mut denom = 0.0;
        for period_idx in 8..=50 {
            let p = pwr[period_idx];
            let db = if max_pwr > 0.0 && p > 0.0 {
                let val = 0.01 / (1.0 - 0.99 * p / max_pwr);
                -10.0 * val.log10()
            } else {
                20.0
            }.min(20.0);

            if db < 3.0 {
                num += period_idx as f64 * (3.0 - db);
                denom += 3.0 - db;
            }
        }

        if denom != 0.0 {
            num / denom
        } else {
            0.0
        }
    }
}

pub const FOURIER_DOMINANT_CYCLE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "FourierDominantCycle",
    description: "Dominant cycle period estimation using resolution-enhanced DFT and center of gravity.",
    usage: "Use to compute the dominant market cycle period via DFT. Feed the output period into adaptive indicators like DSMA or Ehlers Stochastic to make them cycle-synchronized.",
    keywords: &["cycle", "spectral", "ehlers", "dsp", "dominant-cycle", "fourier"],
    ehlers_summary: "Ehlers implements a Discrete Fourier Transform cycle measurement in Cybernetic Analysis using a Hann-windowed data segment. The DFT computes power across periods from 6 to 50 bars, and the peak power identifies the dominant cycle period driving price movement.",
    params: &[
        ParamDef {
            name: "window_len",
            default: "50",
            description: "DFT window length",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/FourierTransformForTraders.pdf",
    formula_latex: r#"
\[
HP = \text{HighPass}(Price, 40)
\]
\[
Cleaned = \frac{HP + 2HP_{t-1} + 3HP_{t-2} + 3HP_{t-3} + 2HP_{t-4} + HP_{t-5}}{12}
\]
\[
Pwr(P) = \left(\sum_{n=0}^{W-1} Cleaned_{t-n} \cos\left(\frac{2\pi n}{P}\right)\right)^2 + \left(\sum_{n=0}^{W-1} Cleaned_{t-n} \sin\left(\frac{2\pi n}{P}\right)\right)^2
\]
\[
DB(P) = \min\left(20, -10 \log_{10}\left(\frac{0.01}{1 - 0.99 \frac{Pwr(P)}{\max(Pwr)}}\right)\right)
\]
\[
DC = \frac{\sum_{P=8}^{50} P \cdot (3 - DB(P)) \text{ where } DB(P) < 3}{\sum (3 - DB(P))}
\]
"#,
    gold_standard_file: "fourier_dominant_cycle.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_fourier_dc_basic() {
        let mut fdc = FourierDominantCycle::new(50);
        for i in 0..200 {
            // Sine wave with period 20
            let val = fdc.next((2.0 * PI * i as f64 / 20.0).sin());
            if i > 150 {
                // Should be around 20
                assert!(val > 15.0 && val < 25.0);
            }
        }
    }

    proptest! {
        #[test]
        fn test_fourier_dc_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..150),
        ) {
            let window_len = 50;
            let mut fdc = FourierDominantCycle::new(window_len);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| fdc.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut hp = HighPass::new(40);
            let hp_vals: Vec<f64> = inputs.iter().map(|&x| hp.next(x)).collect();
            let mut cleaned_vals = Vec::new();

            for i in 0..hp_vals.len() {
                let c = (hp_vals[i] 
                    + 2.0 * (if i > 0 { hp_vals[i-1] } else { 0.0 })
                    + 3.0 * (if i > 1 { hp_vals[i-2] } else { 0.0 })
                    + 3.0 * (if i > 2 { hp_vals[i-3] } else { 0.0 })
                    + 2.0 * (if i > 3 { hp_vals[i-4] } else { 0.0 })
                    + (if i > 4 { hp_vals[i-5] } else { 0.0 })) / 12.0;
                cleaned_vals.push(c);

                if i < window_len + 5 { // account for HP and FIR delay
                    batch_results.push(0.0);
                    continue;
                }

                let mut pwr = vec![0.0; 51];
                for period_idx in 8..=50 {
                    let period = period_idx as f64;
                    let mut cos_part = 0.0;
                    let mut sin_part = 0.0;
                    for n in 0..window_len {
                        let angle = 2.0 * PI * n as f64 / period;
                        cos_part += cleaned_vals[i-n] * angle.cos();
                        sin_part += cleaned_vals[i-n] * angle.sin();
                    }
                    pwr[period_idx] = cos_part * cos_part + sin_part * sin_part;
                }

                let mut max_p = 0.0;
                for &p in &pwr[8..=50] {
                    if p > max_p { max_p = p; }
                }

                let mut num = 0.0;
                let mut den = 0.0;
                for period_idx in 8..=50 {
                    let p = pwr[period_idx];
                    let db = if max_p > 0.0 && p > 0.0 {
                        let val = 0.01 / (1.0 - 0.99 * p / max_p);
                        -10.0 * val.log10()
                    } else {
                        20.0
                    }.min(20.0);

                    if db < 3.0 {
                        num += period_idx as f64 * (3.0 - db);
                        den += 3.0 - db;
                    }
                }

                batch_results.push(if den != 0.0 { num / den } else { 0.0 });
            }

            // Note: streaming uses front-pushing window, batch uses indexing.
            // There might be minor startup differences due to history initialization.
            for i in 60..inputs.len() {
                approx::assert_relative_eq!(streaming_results[i], batch_results[i], epsilon = 1e-10);
            }
        }
    }
}
