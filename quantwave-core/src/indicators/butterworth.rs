use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// 2-Pole Butterworth Filter
///
/// Based on John Ehlers' "Poles, Zeros, and Higher Order Filters".
/// A second-order IIR filter with two poles and two zeros at Z=-1.
#[derive(Debug, Clone)]
pub struct Butterworth2 {
    c1: f64,
    b: f64,
    aa: f64,
    price_history: [f64; 2],
    filt_history: [f64; 2],
    count: usize,
}

impl Butterworth2 {
    pub fn new(period: usize) -> Self {
        let p = period as f64;
        let a = (-1.414 * PI / p).exp();
        let b = 2.0 * a * (1.414 * PI / p).cos();
        let aa = a * a;
        let c1 = (1.0 - b + aa) / 4.0;
        Self {
            c1,
            b,
            aa,
            price_history: [0.0; 2],
            filt_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Next<f64> for Butterworth2 {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let res = if self.count < 3 {
            input
        } else {
            self.b * self.filt_history[0] - self.aa * self.filt_history[1]
                + self.c1 * (input + 2.0 * self.price_history[0] + self.price_history[1])
        };

        self.filt_history[1] = self.filt_history[0];
        self.filt_history[0] = res;
        self.price_history[1] = self.price_history[0];
        self.price_history[0] = input;
        res
    }
}

/// 3-Pole Butterworth Filter
///
/// Based on John Ehlers' "Poles, Zeros, and Higher Order Filters".
/// A third-order IIR filter with three poles and three zeros at Z=-1.
#[derive(Debug, Clone)]
pub struct Butterworth3 {
    c1: f64,
    b: f64,
    c: f64,
    bc: f64,
    cc: f64,
    price_history: [f64; 3],
    filt_history: [f64; 3],
    count: usize,
}

impl Butterworth3 {
    pub fn new(period: usize) -> Self {
        let p = period as f64;
        let a = (-PI / p).exp();
        let b = 2.0 * a * (1.738 * PI / p).cos();
        let c = a * a;
        let bc = b * c;
        let cc = c * c;
        let c1 = (1.0 - b + c) * (1.0 - c) / 8.0;
        Self {
            c1,
            b,
            c,
            bc,
            cc,
            price_history: [0.0; 3],
            filt_history: [0.0; 3],
            count: 0,
        }
    }
}

impl Next<f64> for Butterworth3 {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        let res = if self.count < 4 {
            input
        } else {
            (self.b + self.c) * self.filt_history[0]
                - (self.c + self.bc) * self.filt_history[1]
                + self.cc * self.filt_history[2]
                + self.c1 * (input + 3.0 * self.price_history[0] + 3.0 * self.price_history[1] + self.price_history[2])
        };

        self.filt_history[2] = self.filt_history[1];
        self.filt_history[1] = self.filt_history[0];
        self.filt_history[0] = res;
        self.price_history[2] = self.price_history[1];
        self.price_history[1] = self.price_history[0];
        self.price_history[0] = input;
        res
    }
}

pub const BUTTERWORTH2_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Butterworth2",
    description: "2-pole Butterworth low-pass filter.",
    usage: "Use to smooth price or intermediate indicator values with a flat passband and sharp rolloff. The 3-pole version provides steeper attenuation at the cost of marginally more lag.",
    keywords: &["filter", "ehlers", "dsp", "smoothing", "low-pass"],
    ehlers_summary: "Butterworth filters are maximally flat in the passband, introducing no ripple. Ehlers implements 2-pole and 3-pole Butterworth IIR designs in Cycle Analytics for Traders, noting that the SuperSmoother is actually a critically-damped 2-pole Butterworth variant.",
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Critical period",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Poles.pdf",
    formula_latex: r#"
\[
a = \exp(-1.414\pi/P)
\]
\[
b = 2a \cos(1.414\pi/P)
\]
\[
f = bf_{t-1} - a^2f_{t-2} + \frac{1-b+a^2}{4}(g + 2g_{t-1} + g_{t-2})
\]
"#,
    gold_standard_file: "butterworth2.json",
    category: "Ehlers DSP",
};

pub const BUTTERWORTH3_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Butterworth3",
    description: "3-pole Butterworth low-pass filter.",
    usage: "Use to smooth price or intermediate indicator values with a flat passband and sharp rolloff. The 3-pole version provides steeper attenuation at the cost of marginally more lag.",
    keywords: &["filter", "ehlers", "dsp", "smoothing", "low-pass"],
    ehlers_summary: "Butterworth filters are maximally flat in the passband, introducing no ripple. Ehlers implements 2-pole and 3-pole Butterworth IIR designs in Cycle Analytics for Traders, noting that the SuperSmoother is actually a critically-damped 2-pole Butterworth variant.",
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Critical period",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Poles.pdf",
    formula_latex: r#"
\[
a = \exp(-\pi/P)
\]
\[
b = 2a \cos(1.738\pi/P)
\]
\[
c = a^2
\]
\[
f = (b+c)f_{t-1} - (c+bc)f_{t-2} + c^2f_{t-3} + \frac{(1-b+c)(1-c)}{8}(g + 3g_{t-1} + 3g_{t-2} + g_{t-3})
\]
"#,
    gold_standard_file: "butterworth3.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_butterworth_basic() {
        let mut b2 = Butterworth2::new(14);
        let mut b3 = Butterworth3::new(14);
        for i in 0..20 {
            let val = i as f64;
            assert!(!b2.next(val).is_nan());
            assert!(!b3.next(val).is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_butterworth2_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let p = 14;
            let mut b2 = Butterworth2::new(p);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| b2.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let p_f = p as f64;
            let a = (-1.414 * PI / p_f).exp();
            let b = 2.0 * a * (1.414 * PI / p_f).cos();
            let aa = a * a;
            let c1 = (1.0 - b + aa) / 4.0;

            let mut f_hist = [0.0; 2];
            let mut g_hist = [0.0; 2];

            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let res = if bar < 3 {
                    input
                } else {
                    b * f_hist[0] - aa * f_hist[1] + c1 * (input + 2.0 * g_hist[0] + g_hist[1])
                };
                f_hist[1] = f_hist[0];
                f_hist[0] = res;
                g_hist[1] = g_hist[0];
                g_hist[0] = input;
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }

        #[test]
        fn test_butterworth3_parity(
            inputs in prop::collection::vec(1.0..100.0, 10..100),
        ) {
            let p = 14;
            let mut b3 = Butterworth3::new(p);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| b3.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let p_f = p as f64;
            let a = (-PI / p_f).exp();
            let b = 2.0 * a * (1.738 * PI / p_f).cos();
            let c = a * a;
            let bc = b * c;
            let cc = c * c;
            let c1 = (1.0 - b + c) * (1.0 - c) / 8.0;

            let mut f_hist = [0.0; 3];
            let mut g_hist = [0.0; 3];

            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let res = if bar < 4 {
                    input
                } else {
                    (b + c) * f_hist[0] - (c + bc) * f_hist[1] + cc * f_hist[2]
                        + c1 * (input + 3.0 * g_hist[0] + 3.0 * g_hist[1] + g_hist[2])
                };
                f_hist[2] = f_hist[1];
                f_hist[1] = f_hist[0];
                f_hist[0] = res;
                g_hist[2] = g_hist[1];
                g_hist[1] = g_hist[0];
                g_hist[0] = input;
                batch_results.push(res);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
