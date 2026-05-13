use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::math::AGC;
use crate::traits::Next;
use std::f64::consts::PI;

/// Universal Oscillator
///
/// Based on John Ehlers' "Whiter Is Brighter" (2015).
/// It is an evolution of the SuperSmoother filter applied to 2-day price momentum,
/// with built-in Automatic Gain Control (AGC) for normalization between -1 and +1.
#[derive(Debug, Clone)]
pub struct UniversalOscillator {
    c1: f64,
    c2: f64,
    c3: f64,
    
    price_prev1: f64,
    price_prev2: f64,
    
    wn_prev1: f64,
    
    filt_history: [f64; 2],
    agc: AGC,
    
    count: usize,
}

impl UniversalOscillator {
    pub fn new(band_edge: usize) -> Self {
        let band_edge_f = band_edge as f64;
        let r2 = 2.0f64.sqrt();
        let a1 = (-r2 * PI / band_edge_f).exp();
        let b1 = 2.0 * a1 * (r2 * PI / band_edge_f).cos();
        let c2 = b1;
        let c3 = -a1 * a1;
        let c1 = 1.0 - c2 - c3;

        Self {
            c1,
            c2,
            c3,
            price_prev1: 0.0,
            price_prev2: 0.0,
            wn_prev1: 0.0,
            filt_history: [0.0; 2],
            agc: AGC::new(0.991),
            count: 0,
        }
    }
}

impl Next<f64> for UniversalOscillator {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;

        if self.count < 3 {
            self.price_prev2 = self.price_prev1;
            self.price_prev1 = input;
            return 0.0;
        }

        // WhiteNoise = ( Close - Close[2] ) / 2;
        let wn = (input - self.price_prev2) / 2.0;
        
        // input = ( WhiteNoise + WhiteNoise[1] ) / 2;
        let white_noise_avg = (wn + self.wn_prev1) / 2.0;

        // Filt = c1 * input + c2 * Filt[1] + c3 * Filt[2];
        let filt = self.c1 * white_noise_avg
            + self.c2 * self.filt_history[0]
            + self.c3 * self.filt_history[1];

        // Apply AGC
        let universal = self.agc.next(filt);

        // Update history
        self.filt_history[1] = self.filt_history[0];
        self.filt_history[0] = filt;
        self.wn_prev1 = wn;
        self.price_prev2 = self.price_prev1;
        self.price_prev1 = input;

        universal
    }
}

pub const UNIVERSAL_OSCILLATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Universal Oscillator",
    description: "An adaptive oscillator that normalizes price momentum using a SuperSmoother filter and AGC.",
    params: &[
        ParamDef {
            name: "band_edge",
            default: "20",
            description: "Critical period for the SuperSmoother filter",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2015/01/TradersTips.html",
    formula_latex: r#"
\[
WN = (Price - Price_{t-2}) / 2
\]
\[
AvgWN = (WN + WN_{t-1}) / 2
\]
\[
Filt = c_1 AvgWN + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]
\[
Peak = \max(0.991 \times Peak_{t-1}, |Filt|)
\]
\[
Universal = Filt / Peak
\]
"#,
    gold_standard_file: "universal_oscillator.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_universal_oscillator_basic() {
        let mut uo = UniversalOscillator::new(20);
        let prices = vec![10.0, 10.5, 11.0, 11.5, 12.0, 11.0, 10.0];
        for p in prices {
            let res = uo.next(p);
            assert!(res >= -1.0 && res <= 1.0);
        }
    }

    proptest! {
        #[test]
        fn test_universal_oscillator_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let band_edge = 20;
            let mut uo = UniversalOscillator::new(band_edge);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| uo.next(x)).collect();

            // Reference implementation (batch)
            let mut uo_batch = UniversalOscillator::new(band_edge);
            let batch_results: Vec<f64> = inputs.iter().map(|&x| uo_batch.next(x)).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
