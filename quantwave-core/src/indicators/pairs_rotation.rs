use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::high_pass::HighPass;
use crate::indicators::super_smoother::SuperSmoother;

/// Pairs Rotation (Ehlers Loops)
///
/// Based on John Ehlers' "Pairs Rotation".
/// Filters two price streams using HighPass and SuperSmoother filters,
/// then normalizes them by their RMS (calculated via EMA of squares).
/// This allows visualizing the relative performance and rotation of two securities.
#[derive(Debug, Clone)]
pub struct PairsRotation {
    hp1: HighPass,
    ss1: SuperSmoother,
    ms1: f64,
    hp2: HighPass,
    ss2: SuperSmoother,
    ms2: f64,
    count: usize,
}

impl PairsRotation {
    pub fn new(hp_len: usize, lp_len: usize) -> Self {
        Self {
            hp1: HighPass::new(hp_len),
            ss1: SuperSmoother::new(lp_len),
            ms1: 0.0,
            hp2: HighPass::new(hp_len),
            ss2: SuperSmoother::new(lp_len),
            ms2: 0.0,
            count: 0,
        }
    }
}

impl Default for PairsRotation {
    fn default() -> Self {
        Self::new(125, 20)
    }
}

impl Next<(f64, f64)> for PairsRotation {
    type Output = (f64, f64); // (Normalized1, Normalized2)

    fn next(&mut self, input: (f64, f64)) -> Self::Output {
        self.count += 1;
        let (p1, p2) = input;

        let filt1 = self.ss1.next(self.hp1.next(p1));
        let filt2 = self.ss2.next(self.hp2.next(p2));

        // RMS update using EMA (alpha = 0.0242 for ~1 year period)
        let alpha = 0.0242;
        if self.count == 1 {
            self.ms1 = filt1 * filt1;
            self.ms2 = filt2 * filt2;
        } else {
            self.ms1 = alpha * filt1 * filt1 + (1.0 - alpha) * self.ms1;
            self.ms2 = alpha * filt2 * filt2 + (1.0 - alpha) * self.ms2;
        }

        let norm1 = if self.ms1 > 0.0 {
            filt1 / self.ms1.sqrt()
        } else {
            0.0
        };

        let norm2 = if self.ms2 > 0.0 {
            filt2 / self.ms2.sqrt()
        } else {
            0.0
        };

        (norm1, norm2)
    }
}

pub const PAIRS_ROTATION_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "PairsRotation",
    description: "Relative rotation of two securities using normalized roofing filters.",
    usage: "Use to detect and trade rotation between two correlated assets. When one asset leads and the other lags, the indicator signals a rotation trade opportunity.",
    keywords: &["pairs-trading", "rotation", "relative-strength", "ehlers"],
    ehlers_summary: "Pairs Rotation analysis measures the relative cycle phase between two correlated assets. When one asset is at a cycle peak while its correlated partner is at a trough, a statistical rotation trade can be placed — long the laggard, short the leader — anticipating mean reversion of the spread.",
    params: &[
        ParamDef {
            name: "hp_len",
            default: "125",
            description: "HighPass filter length",
        },
        ParamDef {
            name: "lp_len",
            default: "20",
            description: "LowPass (SuperSmoother) length",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/PAIRS%20ROTATION.pdf",
    formula_latex: r#"
\[
Filt = SuperSmoother(HighPass(Price, HPLen), LPLen)
\]
\[
MS = 0.0242 \cdot Filt^2 + 0.9758 \cdot MS_{t-1}
\]
\[
Normalized = \frac{Filt}{\sqrt{MS}}
\]
"#,
    gold_standard_file: "pairs_rotation.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_pairs_rotation_basic() {
        let mut pr = PairsRotation::new(125, 20);
        for i in 0..100 {
            let (n1, n2) = pr.next((100.0 + i as f64, 100.0 - i as f64));
            assert!(!n1.is_nan());
            assert!(!n2.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_pairs_rotation_parity(
            inputs1 in prop::collection::vec(1.0..100.0, 100..200),
            inputs2 in prop::collection::vec(1.0..100.0, 100..200),
        ) {
            let hp_len = 125;
            let lp_len = 20;
            let mut pr = PairsRotation::new(hp_len, lp_len);
            
            let min_len = inputs1.len().min(inputs2.len());
            let inputs: Vec<(f64, f64)> = inputs1[..min_len].iter().cloned().zip(inputs2[..min_len].iter().cloned()).collect();
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| pr.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(min_len);
            let mut hp1 = HighPass::new(hp_len);
            let mut ss1 = SuperSmoother::new(lp_len);
            let mut hp2 = HighPass::new(hp_len);
            let mut ss2 = SuperSmoother::new(lp_len);
            let mut ms1 = 0.0;
            let mut ms2 = 0.0;
            let alpha = 0.0242;

            for (i, &(p1, p2)) in inputs.iter().enumerate() {
                let f1 = ss1.next(hp1.next(p1));
                let f2 = ss2.next(hp2.next(p2));
                
                if i == 0 {
                    ms1 = f1 * f1;
                    ms2 = f2 * f2;
                } else {
                    ms1 = alpha * f1 * f1 + (1.0 - alpha) * ms1;
                    ms2 = alpha * f2 * f2 + (1.0 - alpha) * ms2;
                }
                
                let n1 = if ms1 > 0.0 { f1 / ms1.sqrt() } else { 0.0 };
                let n2 = if ms2 > 0.0 { f2 / ms2.sqrt() } else { 0.0 };
                batch_results.push((n1, n2));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
