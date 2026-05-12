use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::ultimate_smoother::UltimateSmoother;

/// Ultimate Channel
/// 
/// Based on John Ehlers' "Ultimate Channel and Ultimate Bands" (S&C 2024).
/// Replaces the EMA in Keltner Channels with UltimateSmoothers to mitigate lag.
#[derive(Debug, Clone)]
pub struct UltimateChannel {
    center_smoother: UltimateSmoother,
    str_smoother: UltimateSmoother,
    num_strs: f64,
    prev_close: Option<f64>,
}

impl UltimateChannel {
    pub fn new(length: usize, str_length: usize, num_strs: f64) -> Self {
        Self {
            center_smoother: UltimateSmoother::new(length),
            str_smoother: UltimateSmoother::new(str_length),
            num_strs,
            prev_close: None,
        }
    }
}

impl Next<(f64, f64, f64)> for UltimateChannel {
    type Output = (f64, f64, f64); // (Upper, Center, Lower)

    fn next(&mut self, (high, low, close): (f64, f64, f64)) -> Self::Output {
        let th = match self.prev_close {
            Some(pc) => high.max(pc),
            None => high,
        };
        let tl = match self.prev_close {
            Some(pc) => low.min(pc),
            None => low,
        };
        self.prev_close = Some(close);

        let str_val = self.str_smoother.next(th - tl);
        let center = self.center_smoother.next(close);

        let upper = center + self.num_strs * str_val;
        let lower = center - self.num_strs * str_val;

        (upper, center, lower)
    }
}

pub const ULTIMATE_CHANNEL_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Ultimate Channel",
    description: "A Keltner-style channel using UltimateSmoothers for both the center line and the volatility range to minimize lag.",
    params: &[
        ParamDef { name: "length", default: "20", description: "Center line smoothing period" },
        ParamDef { name: "str_length", default: "20", description: "Smooth True Range (STR) period" },
        ParamDef { name: "num_strs", default: "1.0", description: "Channel width multiplier" },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/UltimateChannel.pdf",
    formula_latex: r#"
\[
TH = \max(High, Close_{t-1})
\]
\[
TL = \min(Low, Close_{t-1})
\]
\[
STR = UltimateSmoother(TH - TL, STRLength)
\]
\[
Center = UltimateSmoother(Close, Length)
\]
\[
Upper = Center + NumSTRs \times STR
\]
\[
Lower = Center - NumSTRs \times STR
\]
"#,
    gold_standard_file: "ultimate_channel.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_ultimate_channel_basic() {
        let mut uc = UltimateChannel::new(20, 20, 1.0);
        let inputs = vec![
            (10.0, 9.0, 9.5),
            (11.0, 10.0, 10.5),
            (12.0, 11.0, 11.5),
        ];
        for input in inputs {
            let (u, c, l) = uc.next(input);
            assert!(!u.is_nan());
            assert!(!c.is_nan());
            assert!(!l.is_nan());
            assert!(u >= c);
            assert!(c >= l);
        }
    }

    proptest! {
        #[test]
        fn test_ultimate_channel_parity(
            inputs in prop::collection::vec((10.0..20.0, 5.0..10.0, 7.0..15.0), 30..100),
        ) {
            let length = 20;
            let str_length = 20;
            let num_strs = 1.0;
            let mut uc = UltimateChannel::new(length, str_length, num_strs);
            
            let mut streaming_results = Vec::with_capacity(inputs.len());
            for &val in &inputs {
                streaming_results.push(uc.next(val));
            }
            
            // Reference implementation
            let mut center_sm = UltimateSmoother::new(length);
            let mut str_sm = UltimateSmoother::new(str_length);
            let mut prev_close = None;
            let mut batch_results = Vec::with_capacity(inputs.len());
            
            for &(h, l, c) in &inputs {
                let th = prev_close.map(|pc: f64| h.max(pc)).unwrap_or(h);
                let tl = prev_close.map(|pc: f64| l.min(pc)).unwrap_or(l);
                prev_close = Some(c);
                
                let str_val = str_sm.next(th - tl);
                let center = center_sm.next(c);
                batch_results.push((center + num_strs * str_val, center, center - num_strs * str_val));
            }
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-10);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-10);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-10);
            }
        }
    }
}
