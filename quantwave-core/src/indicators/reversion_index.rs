use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::super_smoother::SuperSmoother;
use crate::traits::Next;
use std::collections::VecDeque;

/// The Reversion Index
///
/// Based on John Ehlers' "The Reversion Index" (TASC January 2026).
/// This indicator identifies peaks and valleys in ranging markets by summing
/// bar-to-bar price changes and normalizing them by their absolute values.
/// It uses two SuperSmoother filters (Length 4 and 8) as trigger and signal lines.
#[derive(Debug, Clone)]
pub struct ReversionIndex {
    length: usize,
    prev_close: Option<f64>,
    deltas: VecDeque<f64>,
    abs_deltas: VecDeque<f64>,
    delta_sum: f64,
    abs_delta_sum: f64,
    smooth: SuperSmoother,
    trigger: SuperSmoother,
}

impl ReversionIndex {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            prev_close: None,
            deltas: VecDeque::with_capacity(length),
            abs_deltas: VecDeque::with_capacity(length),
            delta_sum: 0.0,
            abs_delta_sum: 0.0,
            smooth: SuperSmoother::new(8),
            trigger: SuperSmoother::new(4),
        }
    }
}

impl Next<f64> for ReversionIndex {
    type Output = (f64, f64); // (Smooth, Trigger)

    fn next(&mut self, input: f64) -> Self::Output {
        let delta = match self.prev_close {
            Some(prev) => input - prev,
            None => 0.0,
        };
        self.prev_close = Some(input);

        self.deltas.push_back(delta);
        self.abs_deltas.push_back(delta.abs());
        self.delta_sum += delta;
        self.abs_delta_sum += delta.abs();

        if self.deltas.len() > self.length {
            if let Some(old_delta) = self.deltas.pop_front() {
                self.delta_sum -= old_delta;
            }
            if let Some(old_abs_delta) = self.abs_deltas.pop_front() {
                self.abs_delta_sum -= old_abs_delta;
            }
        }

        let ratio = if self.abs_delta_sum != 0.0 {
            self.delta_sum / self.abs_delta_sum
        } else {
            0.0
        };

        let sm_val = self.smooth.next(ratio);
        let tr_val = self.trigger.next(ratio);

        (sm_val, tr_val)
    }
}

pub const REVERSION_INDEX_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Reversion Index",
    description: "A mean-reversion oscillator that normalizes price changes by their absolute magnitude and applies SuperSmoother filtering.",
    usage: "Use to identify mean-reversion opportunities when price has deviated significantly from its cycle trend. High index values signal overextended moves ripe for reversal.",
    keywords: &["mean-reversion", "oscillator", "ehlers", "cycle"],
    ehlers_summary: "Ehlers Reversion Index measures how far price has deviated from its Instantaneous Trendline in units of cycle amplitude. Because it normalizes by the current cycle energy, the index provides consistent overbought/oversold thresholds regardless of the absolute price level or volatility regime.",
    params: &[ParamDef {
        name: "length",
        default: "20",
        description: "Summation period (approx. half dominant cycle)",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20JANUARY%202026.html",
    formula_latex: r#"
\[
\Delta_t = \text{Close}_t - \text{Close}_{t-1}
\]
\[
\text{Ratio} = \frac{\sum_{i=0}^{L-1} \Delta_{t-i}}{\sum_{i=0}^{L-1} |\Delta_{t-i}|}
\]
\[
\text{Smooth} = SuperSmoother(\text{Ratio}, 8)
\]
\[
\text{Trigger} = SuperSmoother(\text{Ratio}, 4)
\]
"#,
    gold_standard_file: "reversion_index.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_reversion_index_basic() {
        let mut ri = ReversionIndex::new(20);
        let inputs = vec![100.0, 101.0, 102.0, 101.0, 100.0];
        for input in inputs {
            let (sm, tr) = ri.next(input);
            assert!(!sm.is_nan());
            assert!(!tr.is_nan());
            assert!(sm >= -1.0 && sm <= 1.0);
            assert!(tr >= -1.0 && tr <= 1.0);
        }
    }

    proptest! {
        #[test]
        fn test_reversion_index_parity(
            inputs in prop::collection::vec(10.0..110.0, 50..100),
        ) {
            let length = 20;
            let mut ri = ReversionIndex::new(length);
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| ri.next(x)).collect();

            // Reference implementation
            let mut deltas = Vec::new();
            let mut smooth = SuperSmoother::new(8);
            let mut trigger = SuperSmoother::new(4);
            let mut batch_results = Vec::with_capacity(inputs.len());

            for i in 0..inputs.len() {
                let d = if i == 0 { 0.0 } else { inputs[i] - inputs[i-1] };
                deltas.push(d);

                let start = if deltas.len() > length { deltas.len() - length } else { 0 };
                let window = &deltas[start..];

                let d_sum: f64 = window.iter().sum();
                let ad_sum: f64 = window.iter().map(|x| x.abs()).sum();

                let ratio = if ad_sum != 0.0 { d_sum / ad_sum } else { 0.0 };

                let sm = smooth.next(ratio);
                let tr = trigger.next(ratio);
                batch_results.push((sm, tr));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
