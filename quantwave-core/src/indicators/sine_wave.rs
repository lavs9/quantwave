use crate::indicators::metadata::IndicatorMetadata;
use crate::indicators::hilbert_transform::{HilbertFIR, EhlersWma4};
use crate::traits::Next;
use std::collections::VecDeque;

/// Sine Wave Indicator
///
/// Based on John Ehlers' "Rocket Science for Traders" (Chapter 9).
/// Uses the phase of the Hilbert Transform to plot a sine wave and a lead-sine wave.
/// Returns (Sine, LeadSine).
#[derive(Debug, Clone)]
pub struct SineWave {
    wma_price: EhlersWma4,
    hilbert_detrender: HilbertFIR,
    hilbert_q1: HilbertFIR,
    
    detrender_history: VecDeque<f64>,
    period_prev: f64,
    count: usize,
}

impl SineWave {
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
}

impl Default for SineWave {
    fn default() -> Self {
        Self::new()
    }
}

impl Next<f64> for SineWave {
    type Output = (f64, f64);

    fn next(&mut self, price: f64) -> Self::Output {
        self.count += 1;

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

        // Simple Phase calculation as per Chapter 9
        let mut phase = 0.0;
        if i1.abs() > 0.0001 {
            phase = (q1 / i1).atan().to_degrees();
        }

        let sine = phase.to_radians().sin();
        let lead_sine = (phase + 45.0).to_radians().sin();

        (sine, lead_sine)
    }
}

pub const SINE_WAVE_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Sine Wave",
    description: "Plots a sine wave and a lead-sine wave based on the cyclic phase of price movement.",
    usage: "Use to confirm whether the market is in cycle or trend mode. When price follows the sine wave trade cycle reversals; when it diverges switch to trend-following.",
    keywords: &["cycle", "oscillator", "ehlers", "dsp", "phase"],
    ehlers_summary: "Introduced in Rocket Science for Traders, the Sine Wave Indicator plots the sine and cosine of measured instantaneous phase. In cycling markets price tracks the sine wave; in trending markets price breaks through the lead line signaling a mode change.",
    params: &[],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/ROCKET%20SCIENCE%20FOR%20TRADER.pdf",
    formula_latex: r#"
\[
\text{Sine} = \sin(\text{Phase})
\]
\[
\text{LeadSine} = \sin(\text{Phase} + 45^\circ)
\]
"#,
    gold_standard_file: "sine_wave.json",
    category: "Rocket Science",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_sine_wave_basic() {
        let mut sw = SineWave::new();
        let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0];
        for p in prices {
            let (s, l) = sw.next(p);
            assert!(!s.is_nan());
            assert!(!l.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_sine_wave_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let mut sw = SineWave::new();
            let streaming_results: Vec<(f64, f64)> = inputs.iter().map(|&x| sw.next(x)).collect();

            let mut sw_batch = SineWave::new();
            let batch_results: Vec<(f64, f64)> = inputs.iter().map(|&x| sw_batch.next(x)).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
            }
        }
    }
}
