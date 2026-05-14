use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;
use std::f64::consts::PI;

/// Correlation Cycle Indicator
///
/// Based on John Ehlers' "Correlation as a Cycle Indicator".
/// Computes the correlation of price against a fixed-period Cosine and negative Sine wave
/// to derive a phase angle. This angle is used to identify market cycles and trends.
#[derive(Debug, Clone)]
pub struct CorrelationCycle {
    period: usize,
    price_window: VecDeque<f64>,
    cosine_wave: Vec<f64>,
    sine_wave: Vec<f64>,
    prev_angle: f64,
    count: usize,
}

impl CorrelationCycle {
    pub fn new(period: usize) -> Self {
        let mut cosine_wave = Vec::with_capacity(period);
        let mut sine_wave = Vec::with_capacity(period);
        for n in 0..period {
            let angle = 2.0 * PI * n as f64 / period as f64;
            cosine_wave.push(angle.cos());
            sine_wave.push(-angle.sin());
        }

        Self {
            period,
            price_window: VecDeque::with_capacity(period),
            cosine_wave,
            sine_wave,
            prev_angle: 0.0,
            count: 0,
        }
    }

    fn pearson_correlation(n: usize, x: &VecDeque<f64>, y: &Vec<f64>) -> f64 {
        let mut sx = 0.0;
        let mut sy = 0.0;
        let mut sxx = 0.0;
        let mut syy = 0.0;
        let mut sxy = 0.0;

        for i in 0..n {
            let xi = x[i];
            let yi = y[i];
            sx += xi;
            sy += yi;
            sxx += xi * xi;
            syy += yi * yi;
            sxy += xi * yi;
        }

        let nf = n as f64;
        let num = nf * sxy - sx * sy;
        let den = ((nf * sxx - sx * sx) * (nf * syy - sy * sy)).sqrt();

        if den > 0.0 {
            num / den
        } else {
            0.0
        }
    }
}

impl Default for CorrelationCycle {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Next<f64> for CorrelationCycle {
    type Output = (f64, f64, f64); // (Real, Imag, Angle)

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        self.price_window.push_front(input);
        if self.price_window.len() > self.period {
            self.price_window.pop_back();
        }

        if self.price_window.len() < self.period {
            return (0.0, 0.0, 0.0);
        }

        let real = Self::pearson_correlation(self.period, &self.price_window, &self.cosine_wave);
        let imag = Self::pearson_correlation(self.period, &self.price_window, &self.sine_wave);

        // Angle in degrees: 90 + atan(Real / Imag)
        // Ehlers resolve ambiguity: if Imag > 0 then Angle = Angle - 180
        let mut angle = if imag != 0.0 {
            (real / imag).atan().to_degrees() + 90.0
        } else {
            90.0
        };

        if imag > 0.0 {
            angle -= 180.0;
        }

        // Do not allow rate change of angle to go negative
        // If Angle[1] - Angle < 270 and Angle < Angle[1] Then Angle = Angle[1];
        if self.count > self.period + 1 {
            if self.prev_angle - angle < 270.0 && angle < self.prev_angle {
                angle = self.prev_angle;
            }
        }

        self.prev_angle = angle;

        (real, imag, angle)
    }
}

pub const CORRELATION_CYCLE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "CorrelationCycle",
    description: "Determines cycle phase angle by correlating price with orthogonal sinusoids.",
    usage: "Use to measure the dominant cycle period via autocorrelation in an amplitude-independent way. Prefer over DFT methods when price amplitude varies significantly across the measurement window.",
    keywords: &["cycle", "dominant-cycle", "ehlers", "dsp", "spectral"],
    ehlers_summary: "Ehlers introduces Correlation Cycle measurement in Cycle Analytics for Traders (2013) as an improvement on DFT. By normalizing autocorrelation coefficients to unity variance, the resulting periodogram is independent of price amplitude variations, producing more consistent cycle period estimates.",
    params: &[
        ParamDef {
            name: "period",
            default: "20",
            description: "Correlation wavelength",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/CORRELATION%20AS%20A%20CYCLE%20INDICATOR.pdf",
    formula_latex: r#"
\[
R = \text{Corr}(Price, \cos(2\pi n/P)), I = \text{Corr}(Price, -\sin(2\pi n/P))
\]
\[
\text{Angle} = 90 + \arctan(R/I) \text{ (with quadrant resolution)}
\]
"#,
    gold_standard_file: "correlation_cycle.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_correlation_cycle_basic() {
        let mut cc = CorrelationCycle::new(20);
        for i in 0..100 {
            let (r, im, a) = cc.next((2.0 * PI * i as f64 / 20.0).sin());
            assert!(!r.is_nan());
            assert!(!im.is_nan());
            assert!(!a.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_correlation_cycle_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..150),
        ) {
            let period = 20;
            let mut cc = CorrelationCycle::new(period);
            let streaming_results: Vec<(f64, f64, f64)> = inputs.iter().map(|&x| cc.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut cos_w = Vec::new();
            let mut sin_w = Vec::new();
            for n in 0..period {
                let ang = 2.0 * PI * n as f64 / period as f64;
                cos_w.push(ang.cos());
                sin_w.push(-ang.sin());
            }

            let mut prev_a = 0.0;
            for i in 0..inputs.len() {
                if i < period - 1 {
                    batch_results.push((0.0, 0.0, 0.0));
                    continue;
                }
                
                let mut sx = 0.0;
                let mut sy_c = 0.0;
                let mut sy_s = 0.0;
                let mut sxx = 0.0;
                let mut syy_c = 0.0;
                let mut syy_s = 0.0;
                let mut sxy_c = 0.0;
                let mut sxy_s = 0.0;

                for j in 0..period {
                    let xi = inputs[i - j];
                    let yc = cos_w[j];
                    let ys = sin_w[j];
                    sx += xi;
                    sy_c += yc;
                    sy_s += ys;
                    sxx += xi * xi;
                    syy_c += yc * yc;
                    syy_s += ys * ys;
                    sxy_c += xi * yc;
                    sxy_s += xi * ys;
                }

                let nf = period as f64;
                let den_c = ((nf * sxx - sx * sx) * (nf * syy_c - sy_c * sy_c)).sqrt();
                let real = if den_c > 0.0 { (nf * sxy_c - sx * sy_c) / den_c } else { 0.0 };
                
                let den_s = ((nf * sxx - sx * sx) * (nf * syy_s - sy_s * sy_s)).sqrt();
                let imag = if den_s > 0.0 { (nf * sxy_s - sx * sy_s) / den_s } else { 0.0 };

                let mut angle = if imag != 0.0 {
                    (real / imag).atan().to_degrees() + 90.0
                } else {
                    90.0
                };
                if imag > 0.0 { angle -= 180.0; }

                if i > period {
                    if prev_a - angle < 270.0 && angle < prev_a {
                        angle = prev_a;
                    }
                }
                
                prev_a = angle;
                batch_results.push((real, imag, angle));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-10);
            }
        }
    }
}
