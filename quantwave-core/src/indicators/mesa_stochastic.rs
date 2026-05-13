use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::roofing_filter::RoofingFilter;
use crate::indicators::super_smoother::SuperSmoother;
use std::collections::VecDeque;

/// MESA Stochastic
/// 
/// Based on John Ehlers' "Predictive and Successful Indicators" (2014) 
/// and "Anticipating Turning Points".
/// It applies a standard Stochastic formula to price data preprocessed by 
/// a Roofing Filter, followed by a SuperSmoother filter.
#[derive(Debug, Clone)]
pub struct MESAStochastic {
    roofing_filter: RoofingFilter,
    stoch_smoother: SuperSmoother,
    length: usize,
    filt_history: VecDeque<f64>,
}

impl MESAStochastic {
    pub fn new(length: usize, hp_period: usize, ss_period: usize) -> Self {
        Self {
            roofing_filter: RoofingFilter::new(hp_period, ss_period),
            stoch_smoother: SuperSmoother::new(ss_period),
            length,
            filt_history: VecDeque::with_capacity(length),
        }
    }
}

impl Default for MESAStochastic {
    fn default() -> Self {
        Self::new(20, 48, 10)
    }
}

impl Next<f64> for MESAStochastic {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let filt = self.roofing_filter.next(input);
        
        self.filt_history.push_front(filt);
        if self.filt_history.len() > self.length {
            self.filt_history.pop_back();
        }

        let mut highest_c = f64::NEG_INFINITY;
        let mut lowest_c = f64::INFINITY;
        
        for &val in &self.filt_history {
            if val > highest_c { highest_c = val; }
            if val < lowest_c { lowest_c = val; }
        }

        let stoch = if (highest_c - lowest_c).abs() > 1e-10 {
            (filt - lowest_c) / (highest_c - lowest_c)
        } else {
            0.0
        };

        // Multiplied by 100 to match the 20/80 levels in the paper
        self.stoch_smoother.next(stoch * 100.0)
    }
}

pub const MESA_STOCHASTIC_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "MESA Stochastic",
    description: "Standard Stochastic calculation applied to Roofing Filtered data, followed by SuperSmoothing.",
    params: &[
        ParamDef { name: "length", default: "20", description: "Stochastic lookback length" },
        ParamDef { name: "hp_period", default: "48", description: "HighPass critical period" },
        ParamDef { name: "ss_period", default: "10", description: "SuperSmoother critical period" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/Anticipating%20Turning%20Points.pdf",
    formula_latex: r#"
\[
Filt = \text{RoofingFilter}(Price, P_{hp}, P_{ss})
\]
\[
Stoc = \frac{Filt - \min(Filt, L)}{\max(Filt, L) - \min(Filt, L)}
\]
\[
MESAStoch = \text{SuperSmoother}(Stoc \times 100, P_{ss})
\]
"#,
    gold_standard_file: "mesa_stochastic.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use crate::test_utils::{load_gold_standard, assert_indicator_parity};
    use proptest::prelude::*;

    #[test]
    fn test_mesa_stochastic_gold_standard() {
        let case = load_gold_standard("mesa_stochastic");
        let ms = MESAStochastic::new(20, 48, 10);
        assert_indicator_parity(ms, &case.input, &case.expected);
    }

    #[test]
    fn test_mesa_stochastic_basic() {
        let mut ms = MESAStochastic::default();
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let res = ms.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_mesa_stochastic_parity(
            inputs in prop::collection::vec(1.0..100.0, 60..120),
        ) {
            let length = 20;
            let hp_period = 48;
            let ss_period = 10;
            let mut ms = MESAStochastic::new(length, hp_period, ss_period);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| ms.next(x)).collect();
            
            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut rf = RoofingFilter::new(hp_period, ss_period);
            let mut ss = SuperSmoother::new(ss_period);
            let mut filt_hist = VecDeque::new();
            
            for &input in &inputs {
                let filt = rf.next(input);
                filt_hist.push_front(filt);
                if filt_hist.len() > length {
                    filt_hist.pop_back();
                }
                
                let mut highest_c = f64::NEG_INFINITY;
                let mut lowest_c = f64::INFINITY;
                for &val in &filt_hist {
                    if val > highest_c { highest_c = val; }
                    if val < lowest_c { lowest_c = val; }
                }
                
                let stoch = if (highest_c - lowest_c).abs() > 1e-10 {
                    (filt - lowest_c) / (highest_c - lowest_c)
                } else {
                    0.0
                };
                
                let res = ss.next(stoch * 100.0);
                batch_results.push(res);
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
