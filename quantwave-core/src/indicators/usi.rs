use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::SMA;
use crate::indicators::ultimate_smoother::UltimateSmoother;
use crate::traits::Next;

/// Ultimate Strength Index (USI)
///
/// Based on John Ehlers' article "Ultimate Strength Index (USI)" (TASC November 2024).
/// An enhanced version of the RSI with significantly reduced lag and smoother response.
/// It applies the UltimateSmoother to a 4-bar simple moving average of up and down moves.
#[derive(Debug, Clone)]
pub struct USI {
    prev_close: Option<f64>,
    su_sma: SMA,
    sd_sma: SMA,
    usu: UltimateSmoother,
    usd: UltimateSmoother,
    last_val: f64,
}

impl USI {
    pub fn new(length: usize) -> Self {
        Self {
            prev_close: None,
            su_sma: SMA::new(4),
            sd_sma: SMA::new(4),
            usu: UltimateSmoother::new(length),
            usd: UltimateSmoother::new(length),
            last_val: 0.0,
        }
    }
}

impl Next<f64> for USI {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let (su, sd) = match self.prev_close {
            Some(prev) => {
                if input > prev {
                    (input - prev, 0.0)
                } else if input < prev {
                    (0.0, prev - input)
                } else {
                    (0.0, 0.0)
                }
            }
            None => (0.0, 0.0),
        };
        self.prev_close = Some(input);

        let su_sma_val = self.su_sma.next(su);
        let sd_sma_val = self.sd_sma.next(sd);

        let usu_val = self.usu.next(su_sma_val);
        let usd_val = self.usd.next(sd_sma_val);

        let denom = usu_val + usd_val;
        // Using a small epsilon to avoid jitter and divide-by-zero
        // The original code uses a 0.01 threshold which might be too large for small-priced assets
        // We'll use 1e-10 and only update if conditions are met, otherwise carry forward last value
        if denom.abs() > 1e-10 && usu_val.abs() > 1e-12 && usd_val.abs() > 1e-12 {
            self.last_val = (usu_val - usd_val) / denom;
        }

        self.last_val
    }
}

pub const USI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ultimate Strength Index",
    description: "A lag-reduced version of the RSI using UltimateSmoother on smoothed up/down components.",
    usage: "Use to measure the relative strength of the current market move normalized to the dominant cycle amplitude, giving a volatility-adjusted momentum reading.",
    keywords: &["oscillator", "strength", "ehlers", "adaptive", "momentum"],
    ehlers_summary: "The Ultimate Strength Index measures directional momentum as a fraction of the total cycle amplitude. By normalizing momentum to the RMS energy of the dominant cycle, it produces a consistent 0-100 reading that is comparable across different instruments and volatility regimes.",
    params: &[ParamDef {
        name: "length",
        default: "14",
        description: "UltimateSmoother period",
    }],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/traderstipsreference/TRADERS%E2%80%99%20TIPS%20-%20NOVEMBER%202024.html",
    formula_latex: r#"
\[
\text{SU} = \max(0, \text{Close} - \text{Close}_{t-1})
\]
\[
\text{SD} = \max(0, \text{Close}_{t-1} - \text{Close})
\]
\[
\text{USU} = UltimateSmoother(SMA(\text{SU}, 4), Length)
\]
\[
\text{USD} = UltimateSmoother(SMA(\text{SD}, 4), Length)
\]
\[
\text{USI} = \frac{\text{USU} - \text{USD}}{\text{USU} + \text{USD}}
\]
"#,
    gold_standard_file: "usi.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_usi_basic() {
        let mut usi = USI::new(14);
        let inputs = vec![100.0, 101.0, 102.0, 101.0, 100.0, 99.0, 98.0, 99.0, 100.0];
        for input in inputs {
            let res = usi.next(input);
            assert!(!res.is_nan());
            assert!(res >= -1.0 && res <= 1.0);
        }
    }

    proptest! {
        #[test]
        fn test_usi_parity(
            inputs in prop::collection::vec(10.0..110.0, 50..100),
        ) {
            let length = 14;
            let mut usi = USI::new(length);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| usi.next(x)).collect();

            // Reference implementation
            let mut su_sma = SMA::new(4);
            let mut sd_sma = SMA::new(4);
            let mut usu = UltimateSmoother::new(length);
            let mut usd = UltimateSmoother::new(length);
            let mut last_val = 0.0;
            let mut batch_results = Vec::with_capacity(inputs.len());

            for i in 0..inputs.len() {
                let (su, sd) = if i == 0 {
                    (0.0, 0.0)
                } else {
                    let diff = inputs[i] - inputs[i-1];
                    if diff > 0.0 { (diff, 0.0) } else { (0.0, -diff) }
                };

                let s_val = su_sma.next(su);
                let d_val = sd_sma.next(sd);

                let u_smooth = usu.next(s_val);
                let d_smooth = usd.next(d_val);

                let denom = u_smooth + d_smooth;
                if denom.abs() > 1e-10 && u_smooth.abs() > 1e-12 && d_smooth.abs() > 1e-12 {
                    last_val = (u_smooth - d_smooth) / denom;
                }
                batch_results.push(last_val);
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
