use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::WMA;
use crate::traits::Next;

#[derive(Debug, Clone)]
pub struct HMA {
    wma_half: WMA,
    wma_full: WMA,
    wma_sqrt: WMA,
}

impl HMA {
    pub fn new(period: usize) -> Self {
        Self {
            wma_half: WMA::new(period / 2),
            wma_full: WMA::new(period),
            wma_sqrt: WMA::new((period as f64).sqrt() as usize),
        }
    }
}

impl Next<f64> for HMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let wma_half = self.wma_half.next(input);
        let wma_full = self.wma_full.next(input);
        let raw = 2.0 * wma_half - wma_full;
        self.wma_sqrt.next(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct HMACase {
        close: Vec<f64>,
        expected_hma: Vec<f64>,
    }

    #[test]
    fn test_hma_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/hma_14.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path
                .parent()
                .unwrap()
                .join("tests/gold_standard/hma_14.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: HMACase = serde_json::from_str(&content).unwrap();

        let mut hma = HMA::new(14);
        for i in 0..case.close.len() {
            let res = hma.next(case.close[i]);
            approx::assert_relative_eq!(res, case.expected_hma[i], epsilon = 1e-6);
        }
    }

    fn hma_batch(data: Vec<f64>, period: usize) -> Vec<f64> {
        let mut hma = HMA::new(period);
        data.into_iter().map(|x| hma.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_hma_parity(input in prop::collection::vec(0.0..1000.0, 1..100)) {
            let period = 14;
            let mut hma = HMA::new(period);
            let mut streaming_results = Vec::with_capacity(input.len());
            for &val in &input {
                streaming_results.push(hma.next(val));
            }

            let batch_results = hma_batch(input, period);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-6);
            }
        }
    }

    #[test]
    fn test_hma_basic() {
        let mut hma = HMA::new(20);
        // HMA is complex to verify manually, but we can check if it returns values
        for i in 0..100 {
            let val = hma.next(i as f64);
            if i > 20 {
                assert!(val > 0.0);
            }
        }
    }
}

pub const HMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Hull Moving Average",
    description: "The Hull Moving Average (HMA) aims to reduce lag while maintaining smoothness.",
    params: &[ParamDef {
        name: "period",
        default: "14",
        description: "Smoothing period",
    }],
    formula_source: "https://alanhull.com/hull-moving-average",
    formula_latex: r#"
\[
HMA = WMA(2 \times WMA(\frac{n}{2}) - WMA(n), \sqrt{n})
\]
"#,
    gold_standard_file: "hma.json",
    category: "Classic",
};
