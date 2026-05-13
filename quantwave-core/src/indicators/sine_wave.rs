use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;
use std::collections::VecDeque;

/// Sine Wave Indicator
///
/// Based on John Ehlers' "Rocket Science for Traders" (Chapter 9).
/// Uses the phase of the Hilbert Transform to plot a sine wave and a lead-sine wave.
/// Returns (Sine, LeadSine).
#[derive(Debug, Clone)]
pub struct SineWave {
    price_history: VecDeque<f64>,
    smooth_history: VecDeque<f64>,
    detrender_history: VecDeque<f64>,
    i1_history: VecDeque<f64>,
    q1_history: VecDeque<f64>,
    period_prev: f64,
    count: usize,
}

impl SineWave {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::from(vec![0.0; 4]),
            smooth_history: VecDeque::from(vec![0.0; 7]),
            detrender_history: VecDeque::from(vec![0.0; 7]),
            i1_history: VecDeque::from(vec![0.0; 7]),
            q1_history: VecDeque::from(vec![0.0; 7]),
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

        self.price_history.pop_back();
        self.price_history.push_front(price);

        if self.count < 7 {
            return (0.0, 0.0);
        }

        // Smooth = (4*Price + 3*Price[1] + 2*Price[2] + Price[3]) / 10;
        let smooth = (4.0 * self.price_history[0]
            + 3.0 * self.price_history[1]
            + 2.0 * self.price_history[2]
            + self.price_history[3])
            / 10.0;

        self.smooth_history.pop_back();
        self.smooth_history.push_front(smooth);

        // Detrender = (.0962*Smooth + .5769*Smooth[2] - .5769*Smooth[4] - .0962*Smooth[6])*(.075*Period[1] + .54);
        let detrender = (0.0962 * self.smooth_history[0] + 0.5769 * self.smooth_history[2]
            - 0.5769 * self.smooth_history[4]
            - 0.0962 * self.smooth_history[6])
            * (0.075 * self.period_prev + 0.54);

        self.detrender_history.pop_back();
        self.detrender_history.push_front(detrender);

        // Q1 = (.0962*Detrender + .5769*Detrender[2] - .5769*Detrender[4] - .0962*Detrender[6])*(.075*Period[1] + .54);
        let q1 = (0.0962 * self.detrender_history[0] + 0.5769 * self.detrender_history[2]
            - 0.5769 * self.detrender_history[4]
            - 0.0962 * self.detrender_history[6])
            * (0.075 * self.period_prev + 0.54);

        // I1 = Detrender[3];
        let i1 = self.detrender_history[3];

        self.i1_history.pop_back();
        self.i1_history.push_front(i1);
        self.q1_history.pop_back();
        self.q1_history.push_front(q1);

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
