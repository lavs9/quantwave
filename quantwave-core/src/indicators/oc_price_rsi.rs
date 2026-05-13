use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::momentum::RSI;

/// OCPrice RSI
///
/// Based on John Ehlers' "Every Little Bit Helps".
/// Uses the average of Open and Close as the input to a standard RSI
/// to reduce Nyquist frequency noise and provide a half-bar lead.
#[derive(Debug, Clone)]
pub struct OCPriceRSI {
    rsi: RSI,
}

impl OCPriceRSI {
    pub fn new(period: usize) -> Self {
        Self {
            rsi: RSI::new(period),
        }
    }
}

impl Default for OCPriceRSI {
    fn default() -> Self {
        Self::new(14)
    }
}

impl Next<(f64, f64)> for OCPriceRSI {
    type Output = f64;

    fn next(&mut self, input: (f64, f64)) -> Self::Output {
        let (open, close) = input;
        let oc_avg = (open + close) / 2.0;
        self.rsi.next(oc_avg)
    }
}

pub const OC_PRICE_RSI_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "OCPriceRSI",
    description: "RSI calculated using the average of Open and Close prices to reduce noise.",
    params: &[
        ParamDef {
            name: "period",
            default: "14",
            description: "RSI period",
        },
    ],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/EveryLittleBitHelps.pdf",
    formula_latex: r#"
\[
Input = \frac{Open + Close}{2}
\]
\[
RSI = \text{Wilder's RSI}(Input, Period)
\]
"#,
    gold_standard_file: "oc_price_rsi.json",
    category: "Ehlers DSP",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_oc_price_rsi_basic() {
        let mut ocrsi = OCPriceRSI::new(14);
        for i in 0..50 {
            let val = ocrsi.next((100.0 + i as f64, 101.0 + i as f64));
            if i >= 14 {
                assert!(!val.is_nan());
            }
        }
    }

    proptest! {
        #[test]
        fn test_oc_price_rsi_parity(
            opens in prop::collection::vec(1.0..100.0, 50..100),
            closes in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let period = 14;
            let mut ocrsi = OCPriceRSI::new(period);
            
            let min_len = opens.len().min(closes.len());
            let inputs: Vec<(f64, f64)> = opens[..min_len].iter().cloned().zip(closes[..min_len].iter().cloned()).collect();
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| ocrsi.next(x)).collect();

            // Batch implementation
            let mut batch_results = Vec::with_capacity(min_len);
            let mut rsi = RSI::new(period);
            for &(o, c) in &inputs {
                batch_results.push(rsi.next((o + c) / 2.0));
            }

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                if s.is_nan() {
                    assert!(b.is_nan());
                } else {
                    approx::assert_relative_eq!(s, b, epsilon = 1e-10);
                }
            }
        }
    }
}
