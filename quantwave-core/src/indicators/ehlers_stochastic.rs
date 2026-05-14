use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::roofing_filter::RoofingFilter;
use crate::traits::Next;
use std::collections::VecDeque;

/// Ehlers Stochastic (MESA Stochastic)
/// 
/// Based on John Ehlers' "Anticipating Turning Points".
/// It is a standard Stochastic calculation applied to the output of a Roofing Filter.
/// This removes the distortion caused by Spectral Dilation in standard Stochastics.
#[derive(Debug, Clone)]
pub struct EhlersStochastic {
    roof: RoofingFilter,
    stoch_period: usize,
    roof_window: VecDeque<f64>,
}

impl EhlersStochastic {
    pub fn new(hp_period: usize, ss_period: usize, stoch_period: usize) -> Self {
        Self {
            roof: RoofingFilter::new(hp_period, ss_period),
            stoch_period,
            roof_window: VecDeque::with_capacity(stoch_period),
        }
    }
}

impl Next<f64> for EhlersStochastic {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let roof_val = self.roof.next(input);
        self.roof_window.push_front(roof_val);
        if self.roof_window.len() > self.stoch_period {
            self.roof_window.pop_back();
        }
        
        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for &v in &self.roof_window {
            if v < min { min = v; }
            if v > max { max = v; }
        }
        
        if max == min {
            50.0
        } else {
            100.0 * (roof_val - min) / (max - min)
        }
    }
}

pub const EHLERS_STOCHASTIC_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ehlers Stochastic",
    description: "A Stochastic oscillator applied to the output of a Roofing Filter to eliminate Spectral Dilation.",
    usage: "Use as a cycle-aware stochastic oscillator that adapts its lookback window to the current dominant cycle period rather than using a fixed period.",
    keywords: &["oscillator", "stochastic", "ehlers", "cycle", "adaptive"],
    ehlers_summary: "Ehlers computes the stochastic oscillator using the measured dominant cycle period as the lookback window. This adaptive approach ensures the stochastic spans exactly one full market cycle, making overbought and oversold conditions consistently meaningful.",
    params: &[
        ParamDef { name: "hp_period", default: "48", description: "HighPass critical period" },
        ParamDef { name: "ss_period", default: "10", description: "SuperSmoother critical period" },
        ParamDef { name: "stoch_period", default: "20", description: "Stochastic lookback period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Anticipating Turning Points.pdf",
    formula_latex: r#"
\[
Roof = RoofingFilter(HP, SS)
\]
\[
Stoch = 100 \times \frac{Roof - \min(Roof, L)}{\max(Roof, L) - \min(Roof, L)}
\]
"#,
    gold_standard_file: "ehlers_stochastic.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use crate::test_utils::{load_gold_standard, assert_indicator_parity};
    use proptest::prelude::*;

    #[test]
    fn test_ehlers_stochastic_gold_standard() {
        let case = load_gold_standard("ehlers_stochastic");
        let es = EhlersStochastic::new(48, 10, 20);
        assert_indicator_parity(es, &case.input, &case.expected);
    }

    #[test]
    fn test_ehlers_stochastic_basic() {
        let mut es = EhlersStochastic::new(48, 10, 20);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = es.next(input);
            assert!(res >= 0.0 && res <= 100.0);
        }
    }

    proptest! {
        #[test]
        fn test_ehlers_stochastic_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let hp = 48;
            let ss = 10;
            let stoch = 20;
            let mut es = EhlersStochastic::new(hp, ss, stoch);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| es.next(x)).collect();
            
            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut roof = RoofingFilter::new(hp, ss);
            let mut roof_vals = Vec::new();
            
            for &input in &inputs {
                let r_val = roof.next(input);
                roof_vals.push(r_val);
                
                let start = if roof_vals.len() > stoch { roof_vals.len() - stoch } else { 0 };
                let window = &roof_vals[start..];
                
                let mut min = f64::MAX;
                let mut max = f64::MIN;
                for &v in window {
                    if v < min { min = v; }
                    if v > max { max = v; }
                }
                
                let res = if max == min {
                    50.0
                } else {
                    100.0 * (r_val - min) / (max - min)
                };
                batch_results.push(res);
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
