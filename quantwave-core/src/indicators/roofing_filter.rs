use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::f64::consts::PI;

/// Roofing Filter
/// 
/// Based on John Ehlers' "Predictive Indicators for Effective Trading Strategies" (2013).
/// It combines a 2-pole HighPass filter with a SuperSmoother filter to isolate
/// cycle components between a lower and upper period (e.g., 10 and 48 bars).
/// The Roofing Filter removes spectral dilation and aliasing noise.
#[derive(Debug, Clone)]
pub struct RoofingFilter {
    _hp_period: usize,
    _ss_period: usize,
    
    // HighPass coefficients
    hp_c1: f64,
    hp_c2: f64,
    hp_c3: f64,
    
    // SuperSmoother coefficients
    ss_c1: f64,
    ss_c2: f64,
    ss_c3: f64,
    
    price_history: [f64; 2],
    hp_history: [f64; 2],
    filt_history: [f64; 2],
    count: usize,
}

impl RoofingFilter {
    pub fn new(hp_period: usize, ss_period: usize) -> Self {
        let hp_angle = 0.707 * 2.0 * PI / hp_period as f64;
        let alpha1 = (hp_angle.cos() + hp_angle.sin() - 1.0) / hp_angle.cos();
        let hp_c1 = (1.0 - alpha1 / 2.0).powi(2);
        let hp_c2 = 2.0 * (1.0 - alpha1);
        let hp_c3 = -(1.0 - alpha1).powi(2);
        
        let a1 = (-1.414 * PI / ss_period as f64).exp();
        let ss_c2 = 2.0 * a1 * (1.414 * PI / ss_period as f64).cos();
        let ss_c3 = -a1 * a1;
        let ss_c1 = 1.0 - ss_c2 - ss_c3;
        
        Self {
            _hp_period: hp_period,
            _ss_period: ss_period,
            hp_c1,
            hp_c2,
            hp_c3,
            ss_c1,
            ss_c2,
            ss_c3,
            price_history: [0.0; 2],
            hp_history: [0.0; 2],
            filt_history: [0.0; 2],
            count: 0,
        }
    }
}

impl Next<f64> for RoofingFilter {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;
        
        let hp = if self.count < 3 {
            0.0
        } else {
            self.hp_c1 * (input - 2.0 * self.price_history[0] + self.price_history[1])
                + self.hp_c2 * self.hp_history[0]
                + self.hp_c3 * self.hp_history[1]
        };
            
        let res = if self.count < 3 {
            0.0
        } else {
            self.ss_c1 * (hp + self.hp_history[0]) / 2.0
                + self.ss_c2 * self.filt_history[0]
                + self.ss_c3 * self.filt_history[1]
        };
            
        self.hp_history[1] = self.hp_history[0];
        self.hp_history[0] = hp;
        self.price_history[1] = self.price_history[0];
        self.price_history[0] = input;
        self.filt_history[1] = self.filt_history[0];
        self.filt_history[0] = res;
        
        res
    }
}

pub const ROOFING_FILTER_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Roofing Filter",
    description: "Combines a 2-pole HighPass filter and a SuperSmoother to isolate specific cyclic components.",
    usage: "Apply before oscillators to remove both low-frequency trend drift and high-frequency noise, leaving only the tradable cycle band (roughly 10-48 bars).",
    keywords: &["filter", "ehlers", "dsp", "cycle", "high-pass", "low-pass"],
    ehlers_summary: "Introduced in Cycle Analytics for Traders (2013), the Roofing Filter first applies a high-pass filter to remove the dominant trend component, then a SuperSmoother to remove short-term noise. The result is a cycle-only signal with controlled bandwidth, ideal for use as input to oscillators and cycle indicators.",
    params: &[
        ParamDef { name: "hp_period", default: "48", description: "HighPass critical period" },
        ParamDef { name: "ss_period", default: "10", description: "SuperSmoother critical period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/PredictiveIndicators.pdf",
    formula_latex: r#"
\[
\alpha_1 = \frac{\cos(\sqrt{2}\pi/P_{hp}) + \sin(\sqrt{2}\pi/P_{hp}) - 1}{\cos(\sqrt{2}\pi/P_{hp})}
\]
\[
HP = (1 - \alpha_1/2)^2 (Price - 2 Price_{t-1} + Price_{t-2}) + 2(1 - \alpha_1) HP_{t-1} - (1 - \alpha_1)^2 HP_{t-2}
\]
\[
Filt = c_1 \frac{HP + HP_{t-1}}{2} + c_2 Filt_{t-1} + c_3 Filt_{t-2}
\]
"#,
    gold_standard_file: "roofing_filter.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_roofing_filter_basic() {
        let mut rf = RoofingFilter::new(48, 10);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = rf.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_roofing_filter_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let hp_period = 48;
            let ss_period = 10;
            let mut rf = RoofingFilter::new(hp_period, ss_period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| rf.next(x)).collect();
            
            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let hp_angle = 0.707 * 2.0 * PI / hp_period as f64;
            let alpha1 = (hp_angle.cos() + hp_angle.sin() - 1.0) / hp_angle.cos();
            let hp_c1 = (1.0 - alpha1 / 2.0).powi(2);
            let hp_c2 = 2.0 * (1.0 - alpha1);
            let hp_c3 = -(1.0 - alpha1).powi(2);
            
            let a1 = (-1.414 * PI / ss_period as f64).exp();
            let ss_c2 = 2.0 * a1 * (1.414 * PI / ss_period as f64).cos();
            let ss_c3 = -a1 * a1;
            let ss_c1 = 1.0 - ss_c2 - ss_c3;
            
            let mut price_hist = [0.0; 2];
            let mut hp_hist = [0.0; 2];
            let mut filt_hist = [0.0; 2];
            
            for (i, &input) in inputs.iter().enumerate() {
                let bar = i + 1;
                let hp = if bar < 3 {
                    0.0
                } else {
                    hp_c1 * (input - 2.0 * price_hist[0] + price_hist[1]) + hp_c2 * hp_hist[0] + hp_c3 * hp_hist[1]
                };
                let res = if bar < 3 {
                    0.0
                } else {
                    ss_c1 * (hp + hp_hist[0]) / 2.0 + ss_c2 * filt_hist[0] + ss_c3 * filt_hist[1]
                };
                
                hp_hist[1] = hp_hist[0];
                hp_hist[0] = hp;
                price_hist[1] = price_hist[0];
                price_hist[0] = input;
                filt_hist[1] = filt_hist[0];
                filt_hist[0] = res;
                batch_results.push(res);
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
