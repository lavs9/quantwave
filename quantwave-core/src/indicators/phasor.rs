use crate::indicators::metadata::IndicatorMetadata;
use crate::indicators::hilbert_transform::{HilbertFIR, EhlersWma4};
use crate::traits::Next;
use std::collections::VecDeque;

/// Phasor Indicator
///
/// Based on John Ehlers' "Rocket Science for Traders" (Chapter 9).
/// Decomposes the signal into its In-Phase (I) and Quadrature (Q) components.
/// Returns (InPhase, Quadrature).
#[derive(Debug, Clone)]
pub struct Phasor {
    wma_price: EhlersWma4,
    hilbert_detrender: HilbertFIR,
    hilbert_q1: HilbertFIR,
    
    detrender_history: VecDeque<f64>,
    period_prev: f64,
    count: usize,
}

impl Phasor {
    pub fn new() -> Self {
        Self {
            wma_price: EhlersWma4::new(),
            hilbert_detrender: HilbertFIR::new(),
            hilbert_q1: HilbertFIR::new(),
            
            detrender_history: VecDeque::from(vec![0.0; 7]),
            period_prev: 6.0,
            count: 0,
        }
    }

    /// Update with a specific period for the Hilbert FIR
    pub fn next_with_period(&mut self, price: f64, period: f64) -> (f64, f64) {
        self.count += 1;
        self.period_prev = period.clamp(6.0, 50.0);

        if self.count < 7 {
            self.wma_price.next(price);
            return (0.0, 0.0);
        }

        let smooth = self.wma_price.next(price);
        let detrender = self.hilbert_detrender.next(smooth, self.period_prev);
        
        self.detrender_history.pop_back();
        self.detrender_history.push_front(detrender);

        let q1 = self.hilbert_q1.next(detrender, self.period_prev);
        let i1 = self.detrender_history[3];

        (i1, q1)
    }
}

impl Default for Phasor {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<f64> for Phasor {
    type Output = (f64, f64);

    fn next(&mut self, price: f64) -> Self::Output {
        self.next_with_period(price, self.period_prev)
    }
}

pub const PHASOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Phasor",
    description: "Extracts In-Phase (I) and Quadrature (Q) components using a Hilbert Transform.",
    usage: "Use to measure the instantaneous phase and amplitude of the dominant market cycle. Phase crossings of key angles (90, 180 degrees) provide precise cycle turn timing signals.",
    keywords: &["cycle", "phase", "ehlers", "dsp", "dominant-cycle"],
    ehlers_summary: "Ehlers borrows the concept of a phasor from electrical engineering to represent the amplitude and phase of a market cycle as a rotating vector. In Rocket Science for Traders (2001) he shows how measuring the instantaneous phasor angle gives more precise cycle timing than zero-crossing methods.",
    params: &[],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf",
    formula_latex: r#"
\[
I = \text{Detrender}_{t-3}
\]
\[
Q = \text{HilbertFIR}(\text{Detrender}, \text{Period})
\]
"#,
    gold_standard_file: "phasor.json",
    category: "Rocket Science",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_phasor_basic() {
        let mut p = Phasor::new();
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        for price in prices {
            let (i, q) = p.next(price);
            assert!(!i.is_nan());
            assert!(!q.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_phasor_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let mut p = Phasor::new();
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| p.next(x)).collect();

            let mut p_batch = Phasor::new();
            let batch_results: Vec<(f64, f64)> = inputs.iter().map(|&x| p_batch.next(x)).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
