use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::my_rsi::MyRSI;
use crate::indicators::super_smoother::SuperSmoother;
use crate::traits::Next;
use std::collections::VecDeque;

/// RocketRSI
///
/// Based on John Ehlers' "RocketRSI" (TASC May 2018).
/// It applies a SuperSmoother filter to momentum, then computes an RSI variant (MyRSI)
/// and finally applies a Fisher Transform.
#[derive(Debug, Clone)]
pub struct RocketRSI {
    rsi_length: usize,
    _smooth_length: usize,
    price_window: VecDeque<f64>,
    smoother: SuperSmoother,
    my_rsi: MyRSI,
}

impl RocketRSI {
    pub fn new(rsi_length: usize, smooth_length: usize) -> Self {
        Self {
            rsi_length,
            _smooth_length: smooth_length,
            price_window: VecDeque::with_capacity(rsi_length),
            smoother: SuperSmoother::new(smooth_length),
            my_rsi: MyRSI::new(rsi_length),
        }
    }
}

impl Default for RocketRSI {
    fn default() -> Self {
        Self::new(8, 10)
    }
}

impl Next<f64> for RocketRSI {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.price_window.push_front(input);
        if self.price_window.len() > self.rsi_length {
            self.price_window.pop_back();
        }

        if self.price_window.len() < self.rsi_length {
            return 0.0;
        }

        // 1. Momentum
        let mom = self.price_window[0] - self.price_window[self.rsi_length - 1];

        // 2. Filtered Momentum
        let filt = self.smoother.next(mom);

        // 3. MyRSI on Filtered Momentum
        let my_rsi_val = self.my_rsi.next(filt);

        // 4. Fisher Transform
        // Fisher = 0.5 * Log((1 + MyRSI) / (1 - MyRSI))
        // We need to clamp MyRSI to avoid log(0) or log(negative)
        let clamped_rsi = my_rsi_val.clamp(-0.999, 0.999);
        0.5 * ((1.0 + clamped_rsi) / (1.0 - clamped_rsi)).ln()
    }
}

pub const ROCKET_RSI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "RocketRSI",
    description: "Highly responsive RSI variant using SuperSmoother and Fisher Transform.",
    usage: "Use for rapid cycle identification and reversal detection. The Fisher Transform converts the RSI distribution into a Gaussian-like distribution with sharp peaks at reversals.",
    keywords: &["oscillator", "rsi", "ehlers", "dsp", "fisher", "momentum"],
    ehlers_summary: "RocketRSI improves upon standard RSI by first smoothing the momentum with a SuperSmoother filter to eliminate high-frequency noise. The resulting RSI is then passed through a Fisher Transform to create clear, actionable signals at cyclical turning points.",
    params: &[
        ParamDef {
            name: "rsi_length",
            default: "8",
            description: "RSI calculation period",
        },
        ParamDef {
            name: "smooth_length",
            default: "10",
            description: "SuperSmoother filter period",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2018/05/TradersTips.html",
    formula_latex: r#"
\[
Mom = Price - Price_{t-(L-1)}
\]
\[
Filt = \text{SuperSmoother}(Mom, SL)
\]
\[
MyRSI = \frac{\sum \max(0, \Delta Filt) - \sum \max(0, -\Delta Filt)}{\sum |\Delta Filt|}
\]
\[
RocketRSI = 0.5 \cdot \ln\left(\frac{1 + MyRSI}{1 - MyRSI}\right)
\]
"#,
    gold_standard_file: "rocket_rsi.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::proptest;
    use proptest::prelude::*;

    #[test]
    fn test_rocket_rsi_basic() {
        let mut rocket = RocketRSI::new(8, 10);
        for i in 0..100 {
            let input = (i as f64 * 0.1).sin() * 100.0;
            let _ = rocket.next(input);
        }
    }

    proptest! {
        #[test]
        fn test_rocket_rsi_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let rsi_length = 8;
            let smooth_length = 10;
            let mut rocket = RocketRSI::new(rsi_length, smooth_length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| rocket.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(inputs.len());
            
            let mut price_window = VecDeque::with_capacity(rsi_length);
            let mut smoother = crate::indicators::super_smoother::SuperSmoother::new(smooth_length);
            let mut my_rsi = crate::indicators::my_rsi::MyRSI::new(rsi_length);

            for i in 0..inputs.len() {
                price_window.push_front(inputs[i]);
                if price_window.len() > rsi_length {
                    price_window.pop_back();
                }

                if price_window.len() < rsi_length {
                    batch_results.push(0.0);
                    continue;
                }

                let mom = price_window[0] - price_window[rsi_length - 1];
                let filt = smoother.next(mom);
                let rsi_val = my_rsi.next(filt);
                let clamped = rsi_val.clamp(-0.999, 0.999);
                batch_results.push(0.5 * ((1.0 + clamped) / (1.0 - clamped)).ln());
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
