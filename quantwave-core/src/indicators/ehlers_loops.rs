use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// Ehlers Loops
///
/// Based on John Ehlers' "Ehlers Loops" (2022).
/// This indicator combines a 2-pole Butterworth Highpass filter and a SuperSmoother filter
/// to normalize price and volume data into "RMS" units (Standard Deviations).
/// Plotting PriceRMS vs VolRMS in a scatter plot creates "Ehlers Loops".
#[derive(Debug, Clone)]
pub struct EhlersLoops {
    price_filter: NormalizedRoofing,
    volume_filter: NormalizedRoofing,
}

#[derive(Debug, Clone)]
struct NormalizedRoofing {
    hpc1: f64,
    hpc2: f64,
    hpc3: f64,
    ssc1: f64,
    ssc2: f64,
    ssc3: f64,
    rms_alpha: f64,

    input_history: [f64; 2],
    hp_history: [f64; 2],
    ss_history: [f64; 2],
    ms: f64,
    count: usize,
}

impl NormalizedRoofing {
    fn new(lp_period: usize, hp_period: usize, rms_alpha: f64) -> Self {
        let hp_period_f = hp_period as f64;
        let lp_period_f = lp_period as f64;

        // 2 Pole Butterworth Highpass Filter coefficients
        let hpa1 = (-1.414 * PI / hp_period_f).exp();
        let hpb1 = 2.0 * hpa1 * (1.414 * PI / hp_period_f).cos();
        let hpc2 = hpb1;
        let hpc3 = -hpa1 * hpa1;
        let hpc1 = (1.0 + hpc2 - hpc3) / 4.0;

        // 2 Pole Super Smoother Filter coefficients
        let ssa1 = (-1.414 * PI / lp_period_f).exp();
        let ssb1 = 2.0 * ssa1 * (1.414 * PI / lp_period_f).cos();
        let ssc2 = ssb1;
        let ssc3 = -ssa1 * ssa1;
        let ssc1 = 1.0 - ssc2 - ssc3;

        Self {
            hpc1,
            hpc2,
            hpc3,
            ssc1,
            ssc2,
            ssc3,
            rms_alpha,
            input_history: [0.0; 2],
            hp_history: [0.0; 2],
            ss_history: [0.0; 2],
            ms: 0.0,
            count: 0,
        }
    }

    fn next(&mut self, input: f64) -> f64 {
        self.count += 1;

        // HP = hpc1 * ( Close - 2 * Close[1] + Close[2] ) + hpc2 * HP[1] + hpc3 * HP[2];
        let hp = if self.count < 3 {
            0.0
        } else {
            self.hpc1 * (input - 2.0 * self.input_history[0] + self.input_history[1])
                + self.hpc2 * self.hp_history[0]
                + self.hpc3 * self.hp_history[1]
        };

        // SS = ssc1 * ( HP + HP[1] ) / 2 + ssc2 * SS[1] + ssc3 * SS[2];
        let ss = if self.count < 3 {
            0.0
        } else {
            self.ssc1 * (hp + self.hp_history[0]) / 2.0
                + self.ssc2 * self.ss_history[0]
                + self.ssc3 * self.ss_history[1]
        };

        // Scale in terms of Standard Deviations using Fast RMS (EMA of squares)
        if self.count == 1 {
            self.ms = ss * ss;
        } else {
            self.ms = self.rms_alpha * (ss * ss) + (1.0 - self.rms_alpha) * self.ms;
        }

        let res = if self.ms > 0.0 {
            ss / self.ms.sqrt()
        } else {
            0.0
        };

        // Update history
        self.hp_history[1] = self.hp_history[0];
        self.hp_history[0] = hp;
        self.input_history[1] = self.input_history[0];
        self.input_history[0] = input;
        self.ss_history[1] = self.ss_history[0];
        self.ss_history[0] = ss;

        res
    }
}

impl EhlersLoops {
    pub fn new(lp_period: usize, hp_period: usize) -> Self {
        // The default alpha for RMS in the paper is 0.0242
        Self::with_rms_alpha(lp_period, hp_period, 0.0242)
    }

    pub fn with_rms_alpha(lp_period: usize, hp_period: usize, rms_alpha: f64) -> Self {
        Self {
            price_filter: NormalizedRoofing::new(lp_period, hp_period, rms_alpha),
            volume_filter: NormalizedRoofing::new(lp_period, hp_period, rms_alpha),
        }
    }
}

impl Next<(f64, f64)> for EhlersLoops {
    type Output = (f64, f64); // (PriceRMS, VolRMS)

    fn next(&mut self, (price, volume): (f64, f64)) -> Self::Output {
        (
            self.price_filter.next(price),
            self.volume_filter.next(volume),
        )
    }
}

pub const EHLERS_LOOPS_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ehlers Loops",
    description: "Converts price and volume into normalized standard deviation units for scatter plot analysis.",
    params: &[
        ParamDef {
            name: "lp_period",
            default: "20",
            description: "Low-pass filter period (SuperSmoother)",
        },
        ParamDef {
            name: "hp_period",
            default: "125",
            description: "High-pass filter period (Butterworth)",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS’%20TIPS%20-%20JUNE%202022.html",
    formula_latex: r#"
\[
HP = c_1 (Price - 2 Price_{t-1} + Price_{t-2}) + c_2 HP_{t-1} + c_3 HP_{t-2}
\]
\[
SS = s_1 \frac{HP + HP_{t-1}}{2} + s_2 SS_{t-1} + s_3 SS_{t-2}
\]
\[
MS = \alpha SS^2 + (1 - \alpha) MS_{t-1}
\]
\[
RMS = \frac{SS}{\sqrt{MS}}
\]
"#,
    gold_standard_file: "ehlers_loops.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use crate::test_utils::{load_gold_standard_loops, assert_indicator_parity_loops};
    use proptest::prelude::*;

    #[test]
    fn test_ehlers_loops_gold_standard() {
        let case = load_gold_standard_loops("ehlers_loops");
        let el = EhlersLoops::new(20, 125);
        assert_indicator_parity_loops(el, &case.input, &case.expected);
    }

    #[test]
    fn test_ehlers_loops_basic() {
        let mut el = EhlersLoops::new(20, 125);
        let inputs = vec![(100.0, 1000.0), (101.0, 1100.0), (102.0, 1200.0)];
        for input in inputs {
            let (p_rms, v_rms) = el.next(input);
            assert!(!p_rms.is_nan());
            assert!(!v_rms.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_ehlers_loops_parity(
            prices in prop::collection::vec(1.0..100.0, 100..200),
            volumes in prop::collection::vec(100.0..1000.0, 100..200),
        ) {
            let lp_period = 20;
            let hp_period = 125;
            let rms_alpha = 0.0242;
            let mut el = EhlersLoops::with_rms_alpha(lp_period, hp_period, rms_alpha);
            
            let min_len = prices.len().min(volumes.len());
            let inputs: Vec<(f64, f64)> = prices[..min_len].iter().cloned().zip(volumes[..min_len].iter().cloned()).collect();
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| el.next(x)).collect();

            // Batch implementation for Price
            let mut price_results = Vec::with_capacity(inputs.len());
            let hp_period_f = hp_period as f64;
            let lp_period_f = lp_period as f64;

            let hpa1 = (-1.414 * PI / hp_period_f).exp();
            let hpb1 = 2.0 * hpa1 * (1.414 * PI / hp_period_f).cos();
            let hpc2 = hpb1;
            let hpc3 = -hpa1 * hpa1;
            let hpc1 = (1.0 + hpc2 - hpc3) / 4.0;

            let ssa1 = (-1.414 * PI / lp_period_f).exp();
            let ssb1 = 2.0 * ssa1 * (1.414 * PI / lp_period_f).cos();
            let ssc2 = ssb1;
            let ssc3 = -ssa1 * ssa1;
            let ssc1 = 1.0 - ssc2 - ssc3;

            let mut p_input_hist = [0.0; 2];
            let mut p_hp_hist = [0.0; 2];
            let mut p_ss_hist = [0.0; 2];
            let mut p_ms = 0.0;

            for (i, &(p_input, _)) in inputs.iter().enumerate() {
                let bar = i + 1;
                let hp = if bar < 3 { 0.0 } else {
                    hpc1 * (p_input - 2.0 * p_input_hist[0] + p_input_hist[1]) + hpc2 * p_hp_hist[0] + hpc3 * p_hp_hist[1]
                };
                let ss = if bar < 3 { 0.0 } else {
                    ssc1 * (hp + p_hp_hist[0]) / 2.0 + ssc2 * p_ss_hist[0] + ssc3 * p_ss_hist[1]
                };
                if bar == 1 { p_ms = ss * ss; } else { p_ms = rms_alpha * ss * ss + (1.0 - rms_alpha) * p_ms; }
                let res = if p_ms > 0.0 { ss / p_ms.sqrt() } else { 0.0 };
                
                p_hp_hist[1] = p_hp_hist[0]; p_hp_hist[0] = hp;
                p_input_hist[1] = p_input_hist[0]; p_input_hist[0] = p_input;
                p_ss_hist[1] = p_ss_hist[0]; p_ss_hist[0] = ss;
                price_results.push(res);
            }

            // Batch implementation for Volume
            let mut vol_results = Vec::with_capacity(inputs.len());
            let mut v_input_hist = [0.0; 2];
            let mut v_hp_hist = [0.0; 2];
            let mut v_ss_hist = [0.0; 2];
            let mut v_ms = 0.0;

            for (i, &(_, v_input)) in inputs.iter().enumerate() {
                let bar = i + 1;
                let hp = if bar < 3 { 0.0 } else {
                    hpc1 * (v_input - 2.0 * v_input_hist[0] + v_input_hist[1]) + hpc2 * v_hp_hist[0] + hpc3 * v_hp_hist[1]
                };
                let ss = if bar < 3 { 0.0 } else {
                    ssc1 * (hp + v_hp_hist[0]) / 2.0 + ssc2 * v_ss_hist[0] + ssc3 * v_ss_hist[1]
                };
                if bar == 1 { v_ms = ss * ss; } else { v_ms = rms_alpha * ss * ss + (1.0 - rms_alpha) * v_ms; }
                let res = if v_ms > 0.0 { ss / v_ms.sqrt() } else { 0.0 };
                
                v_hp_hist[1] = v_hp_hist[0]; v_hp_hist[0] = hp;
                v_input_hist[1] = v_input_hist[0]; v_input_hist[0] = v_input;
                v_ss_hist[1] = v_ss_hist[0]; v_ss_hist[0] = ss;
                vol_results.push(res);
            }

            for (_i, (s, bp, bv)) in streaming_results.iter().zip(price_results.iter().zip(vol_results.iter())).map(|(s, (bp, bv))| (s, bp, bv)).enumerate() {
                approx::assert_relative_eq!(s.0, *bp, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, *bv, epsilon = 1e-10);
            }
        }
    }
}
