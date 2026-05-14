use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::bandpass::BandPass;
use std::collections::VecDeque;

/// Voss Predictive Filter
///
/// Based on John Ehlers' "A Peek Into The Future".
/// Uses a two-pole bandpass filter followed by a Voss predictor to achieve
/// negative group delay for band-limited signals.
#[derive(Debug, Clone)]
pub struct VossPredictor {
    bandpass: BandPass,
    order: usize,
    voss_history: VecDeque<f64>,
}

impl VossPredictor {
    pub fn new(period: usize, predict: usize) -> Self {
        let order = 3 * predict;
        Self {
            bandpass: BandPass::new(period, 0.25), // Bandwidth default 0.25 as per paper
            order,
            voss_history: VecDeque::with_capacity(order + 1),
        }
    }
}

impl Default for VossPredictor {
    fn default() -> Self {
        Self::new(20, 3)
    }
}

impl Next<f64> for VossPredictor {
    type Output = (f64, f64); // (Filt, Voss)

    fn next(&mut self, input: f64) -> Self::Output {
        let filt = self.bandpass.next(input);
        
        let mut sum_c = 0.0;
        if self.order > 0 {
            for count in 0..self.order {
                let idx = self.order - count;
                // voss_history[0] is Voss[1] (value 1 bar ago)
                // voss_history[idx - 1] is Voss[idx]
                let val = if idx <= self.voss_history.len() {
                    self.voss_history[idx - 1]
                } else {
                    0.0
                };
                sum_c += ((count + 1) as f64 / self.order as f64) * val;
            }
        }

        let voss = ((3.0 + self.order as f64) / 2.0) * filt - sum_c;
        
        self.voss_history.push_front(voss);
        if self.voss_history.len() > self.order {
            self.voss_history.pop_back();
        }

        (filt, voss)
    }
}

pub const VOSS_PREDICTOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "VossPredictor",
    description: "A predictive filter with negative group delay for band-limited signals.",
    usage: "Use for multi-bar price prediction based on a bandpass-filtered dominant cycle. More accurate than simple linear extrapolation due to its IIR filter pole placement.",
    keywords: &["prediction", "cycle", "ehlers", "dsp", "filter"],
    ehlers_summary: "The Voss Predictor is a predictive filter developed by J.F. Voss and adapted by Ehlers in Cycle Analytics for Traders. Its IIR bandpass design inherently extrapolates the filtered signal several bars into the future by virtue of pole placement inside the unit circle, enabling lookahead without buffer access.",
    params: &[
        ParamDef { name: "period", default: "20", description: "Center period of the BandPass filter" },
        ParamDef { name: "predict", default: "3", description: "Number of bars of prediction" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/A%20PEEK%20INTO%20THE%20FUTURE.pdf",
    formula_latex: r#"
\[
Filt = \text{BandPass}(Price, Period, 0.25)
\]
\[
Order = 3 \cdot Predict
\]
\[
SumC = \sum_{n=0}^{Order-1} \frac{n+1}{Order} Voss_{t-(Order-n)}
\]
\[
Voss = \frac{3 + Order}{2} Filt - SumC
\]
"#,
    gold_standard_file: "voss_predictor.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use crate::test_utils::{load_gold_standard_tuple, assert_indicator_parity_tuple};
    use proptest::prelude::*;

    #[test]
    fn test_voss_gold_standard() {
        let case = load_gold_standard_tuple("voss_predictor");
        let vp = VossPredictor::new(20, 3);
        assert_indicator_parity_tuple(vp, &case.input, &case.expected);
    }

    #[test]
    fn test_voss_basic() {
        let mut vp = VossPredictor::default();
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0];
        for input in inputs {
            let (filt, voss) = vp.next(input);
            assert!(!filt.is_nan());
            assert!(!voss.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_voss_parity(
            inputs in prop::collection::vec(1.0..100.0, 60..120),
        ) {
            let period = 20;
            let predict = 3;
            let mut vp = VossPredictor::new(period, predict);
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| vp.next(x)).collect();
            
            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            let mut bp = BandPass::new(period, 0.25);
            let order = 3 * predict;
            let mut v_hist = VecDeque::new();
            
            for &input in &inputs {
                let filt = bp.next(input);
                let mut sum_c = 0.0;
                for count in 0..order {
                    let idx = order - count;
                    let val = if idx <= v_hist.len() {
                        v_hist[idx - 1]
                    } else {
                        0.0
                    };
                    sum_c += ((count + 1) as f64 / order as f64) * val;
                }
                
                let voss = ((3.0 + order as f64) / 2.0) * filt - sum_c;
                v_hist.push_front(voss);
                if v_hist.len() > order {
                    v_hist.pop_back();
                }
                batch_results.push((filt, voss));
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
