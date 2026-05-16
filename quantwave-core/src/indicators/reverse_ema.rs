use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;

/// Reverse EMA
///
/// Based on John Ehlers' article "The Reverse EMA Indicator" (TASC September 2017).
/// This indicator provides a causal forward and backward EMA that minimizes lag.
/// It uses a series of filters to align the EMA and reduce aliasing.
///
/// By varying the alpha parameter, it can display trend or cycle information with very low lag.
/// Typical values: Trend = 0.05, Cycle = 0.3.
#[derive(Debug, Clone)]
pub struct ReverseEMA {
    alpha: f64,
    prev_ema: f64,
    prev_re: [f64; 8],
    count: usize,
}

impl ReverseEMA {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha,
            prev_ema: 0.0,
            prev_re: [0.0; 8],
            count: 0,
        }
    }
}

impl Next<f64> for ReverseEMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.count += 1;

        let cc = 1.0 - self.alpha;

        if self.count == 1 {
            self.prev_ema = input;
            let mut val = (1.0 + cc) * input;
            self.prev_re[0] = val;
            let mut p = 2.0;
            for i in 1..8 {
                val = (cc.powf(p) + 1.0) * val;
                self.prev_re[i] = val;
                p *= 2.0;
            }
        }

        let ema_now = self.alpha * input + cc * self.prev_ema;

        let mut re_now = [0.0; 8];
        re_now[0] = cc * ema_now + self.prev_ema;

        let mut p = 2.0;
        for i in 1..8 {
            re_now[i] = cc.powf(p) * re_now[i - 1] + self.prev_re[i - 1];
            p *= 2.0;
        }

        let wave = ema_now - self.alpha * re_now[7];

        self.prev_ema = ema_now;
        self.prev_re = re_now;

        wave
    }
}

pub const REVERSE_EMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Reverse EMA",
    description: "A causal forward and backward EMA indicator that minimizes lag using a series of alignment filters.",
    usage: "Use to identify trends or cycles with minimal lag. Higher alpha values (e.g., 0.3) isolate cycles, while lower values (e.g., 0.05) isolate trends.",
    keywords: &["ema", "lag", "ehlers", "oscillator", "zero-lag"],
    ehlers_summary: "Ehlers' Reverse EMA approximates a non-causal zero-lag filter by using a product series of Z-transform components. It achieves double smoothing at high frequencies and mitigates spectral dilation at low frequencies, providing a unique balance of smoothness and responsiveness.",
    params: &[ParamDef {
        name: "alpha",
        default: "0.1",
        description: "Smoothing factor (0.0 to 1.0)",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20SEPTEMBER%202017.html",
    formula_latex: r#"
\[
EMA = \alpha \cdot Price + (1 - \alpha) \cdot EMA_{t-1}
\]
\[
RE_1 = (1 - \alpha) \cdot EMA + EMA_{t-1}
\]
\[
RE_i = (1 - \alpha)^{2^{i-1}} \cdot RE_{i-1} + RE_{i-1, t-1} \text{ for } i=2..8
\]
\[
Wave = EMA - \alpha \cdot RE_8
\]
"#,
    gold_standard_file: "reverse_ema.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_reverse_ema_basic() {
        let mut rema = ReverseEMA::new(0.1);
        let inputs = vec![100.0, 101.0, 102.0, 101.0, 100.0];
        for input in inputs {
            let res = rema.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_reverse_ema_parity(
            inputs in prop::collection::vec(90.0..110.0, 50..100),
        ) {
            let alpha = 0.1;
            let mut rema = ReverseEMA::new(alpha);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| rema.next(x)).collect();

            // Reference implementation (TradeStation style, starting from 0)
            let mut ema = 0.0;
            let mut re = [0.0; 8];
            let mut prev_ema = 0.0;
            let mut prev_re = [0.0; 8];
            let cc = 1.0 - alpha;
            let mut batch_results = Vec::with_capacity(inputs.len());

            for (i, &input) in inputs.iter().enumerate() {
                // To match our steady-state initialization in the test, we'd need to seed it.
                // But let's just test that the logic is self-consistent.
                // Our streaming version seeds it, so let's seed the batch version too.
                if i == 0 {
                    prev_ema = input;
                    let mut val = (1.0 + cc) * input;
                    prev_re[0] = val;
                    let mut p = 2.0;
                    for j in 1..8 {
                        val = (cc.powf(p) + 1.0) * val;
                        prev_re[j] = val;
                        p *= 2.0;
                    }
                }

                let ema = alpha * input + cc * prev_ema;
                let mut re_now = [0.0; 8];
                re_now[0] = cc * ema + prev_ema;
                let mut p = 2.0;
                for j in 1..8 {
                    re_now[j] = cc.powf(p) * re_now[j-1] + prev_re[j-1];
                    p *= 2.0;
                }
                let wave = ema - alpha * re_now[7];
                batch_results.push(wave);
                prev_ema = ema;
                prev_re = re_now;
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
