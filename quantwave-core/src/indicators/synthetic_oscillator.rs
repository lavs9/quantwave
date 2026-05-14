use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::hann::HannFilter;
use crate::indicators::high_pass::HighPass;
use crate::indicators::super_smoother::SuperSmoother;
use crate::indicators::ultimate_smoother::UltimateSmoother;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Synthetic Oscillator
///
/// Based on John Ehlers' "A Synthetic Oscillator" (April 2026).
/// A nonlinear oscillator designed to reduce lag while maintaining smoothness.
/// It adapts to the instantaneous dominant cycle and uses phase accumulation.
#[derive(Debug, Clone)]
pub struct SyntheticOscillator {
    hann_price: HannFilter,
    hp: HighPass,
    ss: SuperSmoother,
    lp_window: VecDeque<f64>,
    lp_sum_sq: f64,
    re_prev: f64,
    roc_window: VecDeque<f64>,
    roc_sum_sq: f64,
    im_prev: f64,
    dc_prev: f64,
    hp2: HighPass,
    us: UltimateSmoother,
    bp_prev: f64,
    phase: f64,
    synth_prev: f64,
    lower_bound: f64,
    upper_bound: f64,
}

impl SyntheticOscillator {
    pub fn new(lower_bound: usize, upper_bound: usize) -> Self {
        let mid = ((lower_bound * upper_bound) as f64).sqrt();
        Self {
            hann_price: HannFilter::new(12),
            hp: HighPass::new(upper_bound),
            ss: SuperSmoother::new(lower_bound),
            lp_window: VecDeque::with_capacity(100),
            lp_sum_sq: 0.0,
            re_prev: 0.0,
            roc_window: VecDeque::with_capacity(100),
            roc_sum_sq: 0.0,
            im_prev: 0.0,
            dc_prev: lower_bound as f64,
            hp2: HighPass::new(mid as usize),
            us: UltimateSmoother::new(mid as usize),
            bp_prev: 0.0,
            phase: 0.0,
            synth_prev: 0.0,
            lower_bound: lower_bound as f64,
            upper_bound: upper_bound as f64,
        }
    }
}

impl Default for SyntheticOscillator {
    fn default() -> Self {
        Self::new(15, 25)
    }
}

impl Next<f64> for SyntheticOscillator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let price = self.hann_price.next(input);

        // Real component
        let hp = self.hp.next(price);
        let lp = self.ss.next(hp);

        self.lp_window.push_back(lp);
        self.lp_sum_sq += lp * lp;
        if self.lp_window.len() > 100 && let Some(old) = self.lp_window.pop_front() {
            self.lp_sum_sq -= old * old;
        }

        let rms_lp = (self.lp_sum_sq / self.lp_window.len() as f64).sqrt();
        let re = if rms_lp > 1e-10 { lp / rms_lp } else { 0.0 };

        // Imaginary component
        let roc = re - self.re_prev;
        self.roc_window.push_back(roc);
        self.roc_sum_sq += roc * roc;
        if self.roc_window.len() > 100 && let Some(old) = self.roc_window.pop_front() {
            self.roc_sum_sq -= old * old;
        }

        let qrms = (self.roc_sum_sq / self.roc_window.len() as f64).sqrt();
        let im = if qrms > 1e-10 { roc / qrms } else { 0.0 };

        // Dominant Cycle
        let denom = (re - self.re_prev) * im - (im - self.im_prev) * re;
        let mut dc = if denom.abs() > 1e-10 {
            (2.0 * PI * (re * re + im * im)) / denom
        } else {
            self.dc_prev
        };

        if dc < self.lower_bound { dc = self.lower_bound; }
        if dc > self.upper_bound { dc = self.upper_bound; }

        let hp2 = self.hp2.next(input);
        let bp = self.us.next(hp2);

        // Phase accumulation
        self.phase += 2.0 * PI / dc;

        // Reset at zero crossings
        if self.bp_prev <= 0.0 && bp > 0.0 {
            self.phase = PI / dc;
        } else if self.bp_prev >= 0.0 && bp < 0.0 {
            self.phase = PI + PI / dc;
        }

        let mut synth = self.phase.sin();

        // Glitch removal
        // Normalize phase to [0, 2*PI] for quadrant checks
        let norm_phase = self.phase % (2.0 * PI);
        if (norm_phase > 0.0 && norm_phase < PI / 2.0 && synth < self.synth_prev) || (norm_phase > PI && norm_phase < 1.5 * PI && synth > self.synth_prev) {
            synth = self.synth_prev;
        }

        self.re_prev = re;
        self.im_prev = im;
        self.dc_prev = dc;
        self.bp_prev = bp;
        self.synth_prev = synth;

        synth
    }
}

pub const SYNTHETIC_OSCILLATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Synthetic Oscillator",
    description: "A nonlinear oscillator designed to reduce lag while maintaining smoothness by adapting to the dominant cycle.",
    usage: "Use to construct a synthetic oscillator from dominant cycle sine components when direct price oscillators are too noisy. Most effective in clearly cyclical markets.",
    keywords: &["oscillator", "ehlers", "dsp", "cycle", "synthetic"],
    ehlers_summary: "Ehlers constructs a Synthetic Oscillator by generating a synthetic sine wave at the measured dominant cycle period and comparing it to price. The phase difference between the synthetic sine and actual price reveals whether the market is ahead of or behind its expected cycle position.",
    params: &[
        ParamDef { name: "lower_bound", default: "15", description: "Lower bound of cycle period" },
        ParamDef { name: "upper_bound", default: "25", description: "Upper bound of cycle period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20APRIL%202026.html",
    formula_latex: r#"
\[
Price = \text{Hann}(Close, 12)
\]
\[
LP = \text{SuperSmoother}(\text{HighPass}(Price, UB), LB)
\]
\[
Re = \frac{LP}{RMS(LP, 100)}, \quad Im = \frac{Re - Re_{t-1}}{RMS(Re - Re_{t-1}, 100)}
\]
\[
DC = \frac{2\pi(Re^2 + Im^2)}{(Re - Re_{t-1})Im - (Im - Im_{t-1})Re}
\]
\[
BP = \text{UltimateSmoother}(\text{HighPass}(Close, Mid), Mid)
\]
\[
Phase = Phase_{t-1} + \frac{2\pi}{DC}
\]
\[
Synth = \sin(Phase)
\]
"#,
    gold_standard_file: "synthetic_oscillator.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_synthetic_oscillator_basic() {
        let mut so = SyntheticOscillator::default();
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = so.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_synthetic_oscillator_parity(
            inputs in prop::collection::vec(1.0..100.0, 200..300),
        ) {
            let lb = 15;
            let ub = 25;
            let mut so = SyntheticOscillator::new(lb, ub);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| so.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut hann = HannFilter::new(12);
            let mut hp = HighPass::new(ub);
            let mut ss = SuperSmoother::new(lb);
            let mut lp_win = VecDeque::new();
            let mut lp_sum_sq = 0.0;
            let mut re_prev = 0.0;
            let mut roc_win = VecDeque::new();
            let mut roc_sum_sq = 0.0;
            let mut im_prev = 0.0;
            let mut dc_prev = lb as f64;
            let mid = ((lb * ub) as f64).sqrt();
            let mut hp2 = HighPass::new(mid as usize);
            let mut us = UltimateSmoother::new(mid as usize);
            let mut bp_prev = 0.0;
            let mut phase = 0.0;
            let mut synth_prev = 0.0;

            for &input in &inputs {
                let p = hann.next(input);
                let h = hp.next(p);
                let l = ss.next(h);
                
                lp_win.push_back(l);
                lp_sum_sq += l * l;
                if lp_win.len() > 100 {
                    let old = lp_win.pop_front().unwrap();
                    lp_sum_sq -= old * old;
                }
                let rms_lp = (lp_sum_sq / lp_win.len() as f64).sqrt();
                let re = if rms_lp > 1e-10 { l / rms_lp } else { 0.0 };
                
                let roc = re - re_prev;
                roc_win.push_back(roc);
                roc_sum_sq += roc * roc;
                if roc_win.len() > 100 {
                    let old = roc_win.pop_front().unwrap();
                    roc_sum_sq -= old * old;
                }
                let qrms = (roc_sum_sq / roc_win.len() as f64).sqrt();
                let im = if qrms > 1e-10 { roc / qrms } else { 0.0 };
                
                let denom = (re - re_prev) * im - (im - im_prev) * re;
                let mut dc = if denom.abs() > 1e-10 {
                    (2.0 * PI * (re * re + im * im)) / denom
                } else {
                    dc_prev
                };
                if dc < lb as f64 { dc = lb as f64; }
                if dc > ub as f64 { dc = ub as f64; }
                
                let h2 = hp2.next(input);
                let bp = us.next(h2);
                
                phase += 2.0 * PI / dc;
                if bp_prev <= 0.0 && bp > 0.0 {
                    phase = PI / dc;
                } else if bp_prev >= 0.0 && bp < 0.0 {
                    phase = PI + PI / dc;
                }
                
                let mut synth = phase.sin();
                let norm_phase = phase % (2.0 * PI);
                if norm_phase > 0.0 && norm_phase < PI / 2.0 && synth < synth_prev {
                    synth = synth_prev;
                } else if norm_phase > PI && norm_phase < 1.5 * PI && synth > synth_prev {
                    synth = synth_prev;
                }
                
                batch_results.push(synth);
                
                re_prev = re;
                im_prev = im;
                dc_prev = dc;
                bp_prev = bp;
                synth_prev = synth;
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
