use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::bandpass::BandPass;
use crate::indicators::smoothing::SMA;
use std::f64::consts::PI;

/// Fourier Series Model
///
/// Based on John Ehlers' "Fourier Series Model of the Market".
/// Synthesizes a smoothed market waveform by combining the fundamental,
/// second, and third harmonics of a base cycle, weighted by their
/// relative power.
#[derive(Debug, Clone)]
pub struct FourierSeriesModel {
    fundamental: usize,
    bp1: BandPass,
    bp2: BandPass,
    bp3: BandPass,
    bp1_prev: f64,
    bp2_prev: f64,
    bp3_prev: f64,
    p1_sma: SMA,
    p2_sma: SMA,
    p3_sma: SMA,
    count: usize,
}

impl FourierSeriesModel {
    pub fn new(fundamental: usize) -> Self {
        Self {
            fundamental,
            bp1: BandPass::new(fundamental, 0.1),
            bp2: BandPass::new(fundamental / 2, 0.1),
            bp3: BandPass::new(fundamental / 3, 0.1),
            bp1_prev: 0.0,
            bp2_prev: 0.0,
            bp3_prev: 0.0,
            p1_sma: SMA::new(fundamental),
            p2_sma: SMA::new(fundamental),
            p3_sma: SMA::new(fundamental),
            count: 0,
        }
    }
}

impl Default for FourierSeriesModel {
    fn default() -> Self {
        Self::new(20)
    }
}

impl Next<f64> for FourierSeriesModel {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        
        let bp1 = self.bp1.next(input);
        let bp2 = self.bp2.next(input);
        let bp3 = self.bp3.next(input);

        // Quadrature components (approximate derivatives)
        // Q = (Fundamental / 6.28) * (BP - BP[1])
        let q_scale = self.fundamental as f64 / (2.0 * PI);
        let q1 = q_scale * (bp1 - self.bp1_prev);
        let q2 = q_scale * (bp2 - self.bp2_prev);
        let q3 = q_scale * (bp3 - self.bp3_prev);

        // Power components (summed over fundamental period)
        let p1 = self.p1_sma.next(bp1 * bp1 + q1 * q1) * self.fundamental as f64;
        let p2 = self.p2_sma.next(bp2 * bp2 + q2 * q2) * self.fundamental as f64;
        let p3 = self.p3_sma.next(bp3 * bp3 + q3 * q3) * self.fundamental as f64;

        // Shift history
        self.bp1_prev = bp1;
        self.bp2_prev = bp2;
        self.bp3_prev = bp3;

        // Synthesized wave
        // Wave = BP1 + sqrt(P2/P1)*BP2 + sqrt(P3/P1)*BP3
        let mut wave = bp1;
        if p1 > 0.0 {
            wave += (p2 / p1).sqrt() * bp2;
            wave += (p3 / p1).sqrt() * bp3;
        }

        wave
    }
}

pub const FOURIER_SERIES_MODEL_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "FourierSeriesModel",
    description: "Synthesized market model using fundamental and harmonic frequency components.",
    params: &[
        ParamDef {
            name: "fundamental",
            default: "20",
            description: "Fundamental cycle period",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/FOURIER%20SERIES%20MODEL%20OF%20THE%20MARKET.pdf",
    formula_latex: r#"
\[
BP_k = \text{BandPass}(Price, Fundamental/k)
\]
\[
Q_k = \frac{Fundamental}{2\pi} (BP_{k} - BP_{k,t-1})
\]
\[
P_k = \sum_{n=0}^{F-1} (BP_{k,t-n}^2 + Q_{k,t-n}^2)
\]
\[
Wave = BP_1 + \sqrt{P_2/P_1}BP_2 + \sqrt{P_3/P_1}BP_3
\]
"#,
    gold_standard_file: "fourier_series_model.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_fourier_series_basic() {
        let mut fsm = FourierSeriesModel::new(20);
        for i in 0..100 {
            let val = fsm.next(100.0 + (i as f64 * 0.1).sin());
            assert!(!val.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_fourier_series_parity(
            inputs in prop::collection::vec(1.0..100.0, 100..150),
        ) {
            let fundamental = 20;
            let mut fsm = FourierSeriesModel::new(fundamental);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| fsm.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut bp1_obj = BandPass::new(fundamental, 0.1);
            let mut bp2_obj = BandPass::new(fundamental / 2, 0.1);
            let mut bp3_obj = BandPass::new(fundamental / 3, 0.1);
            
            let mut bp1_vals = Vec::new();
            let mut bp2_vals = Vec::new();
            let mut bp3_vals = Vec::new();
            let mut q1_vals = Vec::new();
            let mut q2_vals = Vec::new();
            let mut q3_vals = Vec::new();

            let q_scale = fundamental as f64 / (2.0 * PI);

            for (i, &input) in inputs.iter().enumerate() {
                let bp1 = bp1_obj.next(input);
                let bp2 = bp2_obj.next(input);
                let bp3 = bp3_obj.next(input);

                let q1 = q_scale * (bp1 - (if i > 0 { bp1_vals[i-1] } else { 0.0 }));
                let q2 = q_scale * (bp2 - (if i > 0 { bp2_vals[i-1] } else { 0.0 }));
                let q3 = q_scale * (bp3 - (if i > 0 { bp3_vals[i-1] } else { 0.0 }));

                bp1_vals.push(bp1);
                bp2_vals.push(bp2);
                bp3_vals.push(bp3);
                q1_vals.push(q1);
                q2_vals.push(q2);
                q3_vals.push(q3);

                let mut p1 = 0.0;
                let mut p2 = 0.0;
                let mut p3 = 0.0;
                let start = if i >= fundamental - 1 { i + 1 - fundamental } else { 0 };
                let count = i + 1 - start;
                for j in start..=i {
                    p1 += bp1_vals[j] * bp1_vals[j] + q1_vals[j] * q1_vals[j];
                    p2 += bp2_vals[j] * bp2_vals[j] + q2_vals[j] * q2_vals[j];
                    p3 += bp3_vals[j] * bp3_vals[j] + q3_vals[j] * q3_vals[j];
                }
                
                // Normalizing to moving average-like behavior
                // (Wait, SMA * period is just the sum)
                
                let mut wave = bp1;
                if p1 > 0.0 {
                    wave += (p2 / p1).sqrt() * bp2;
                    wave += (p3 / p1).sqrt() * bp3;
                }
                batch_results.push(wave);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
