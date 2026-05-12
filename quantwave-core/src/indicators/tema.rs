use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use crate::indicators::smoothing::EMA;

/// Triple Exponential Moving Average (TEMA)
/// TEMA = (3 * EMA1) - (3 * EMA2) + EMA3
/// where EMA1 = EMA(Close), EMA2 = EMA(EMA1), EMA3 = EMA(EMA2)
#[derive(Debug, Clone)]
pub struct TEMA {
    ema1: EMA,
    ema2: EMA,
    ema3: EMA,
}

impl TEMA {
    pub fn new(period: usize) -> Self {
        Self {
            ema1: EMA::new(period),
            ema2: EMA::new(period),
            ema3: EMA::new(period),
        }
    }
}

impl Next<f64> for TEMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let e1 = self.ema1.next(input);
        let e2 = self.ema2.next(e1);
        let e3 = self.ema3.next(e2);

        3.0 * e1 - 3.0 * e2 + e3
    }
}

/// Zero-Lag Exponential Moving Average (ZLEMA)
/// Sometimes referred to as DEMA or a variation.
/// ZLEMA = (2 * EMA1) - EMA2
#[derive(Debug, Clone)]
pub struct ZLEMA {
    ema1: EMA,
    ema2: EMA,
}

impl ZLEMA {
    pub fn new(period: usize) -> Self {
        Self {
            ema1: EMA::new(period),
            ema2: EMA::new(period),
        }
    }
}

impl Next<f64> for ZLEMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let e1 = self.ema1.next(input);
        let e2 = self.ema2.next(e1);

        2.0 * e1 - e2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;
    use std::path::Path;
    use proptest::prelude::*;

    #[derive(Debug, Deserialize)]
    struct TemaCase {
        close: Vec<f64>,
        expected_tema: Vec<f64>,
        expected_zlema: Vec<f64>,
    }

    #[test]
    fn test_tema_zlema_gold_standard() {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let manifest_path = Path::new(&manifest_dir);
        let path = manifest_path.join("tests/gold_standard/tema_14.json");
        let path = if path.exists() {
            path
        } else {
            manifest_path.parent().unwrap().join("tests/gold_standard/tema_14.json")
        };
        let content = fs::read_to_string(path).unwrap();
        let case: TemaCase = serde_json::from_str(&content).unwrap();

        let mut tema = TEMA::new(14);
        let mut zlema = ZLEMA::new(14);
        
        for i in 0..case.close.len() {
            let t = tema.next(case.close[i]);
            let z = zlema.next(case.close[i]);
            approx::assert_relative_eq!(t, case.expected_tema[i], epsilon = 1e-6);
            approx::assert_relative_eq!(z, case.expected_zlema[i], epsilon = 1e-6);
        }
    }

    fn tema_batch(data: Vec<f64>, period: usize) -> Vec<f64> {
        let mut tema = TEMA::new(period);
        data.into_iter().map(|x| tema.next(x)).collect()
    }

    fn zlema_batch(data: Vec<f64>, period: usize) -> Vec<f64> {
        let mut zlema = ZLEMA::new(period);
        data.into_iter().map(|x| zlema.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_tema_parity(input in prop::collection::vec(0.0..1000.0, 1..100)) {
            let period = 14;
            let mut tema = TEMA::new(period);
            let mut streaming_results = Vec::with_capacity(input.len());
            for &val in &input {
                streaming_results.push(tema.next(val));
            }

            let batch_results = tema_batch(input, period);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-6);
            }
        }

        #[test]
        fn test_zlema_parity(input in prop::collection::vec(0.0..1000.0, 1..100)) {
            let period = 14;
            let mut zlema = ZLEMA::new(period);
            let mut streaming_results = Vec::with_capacity(input.len());
            for &val in &input {
                streaming_results.push(zlema.next(val));
            }

            let batch_results = zlema_batch(input, period);
            
            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-6);
            }
        }
    }
}


pub const TEMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Triple Exponential Moving Average",
    description: "TEMA reduces the lag of traditional EMAs.",
    params: &[
        ParamDef { name: "period", default: "14", description: "Smoothing period" },
    ],
    formula_source: "https://www.investopedia.com/terms/t/triple-exponential-moving-average.asp",
    formula_latex: r#"
\[
TEMA = (3 \times EMA_1) - (3 \times EMA_2) + EMA_3
\]
"#,
    gold_standard_file: "tema.json",
};


pub const ZLEMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Zero Lag Exponential Moving Average",
    description: "ZLEMA attempts to eliminate the inherent lag associated with moving averages.",
    params: &[
        ParamDef { name: "period", default: "14", description: "Smoothing period" },
    ],
    formula_source: "https://en.wikipedia.org/wiki/Zero_lag_exponential_moving_average",
    formula_latex: r#"
\[
ZLEMA = EMA(Price + (Price - Price_{t - (period - 1)/2}))
\]
"#,
    gold_standard_file: "zlema.json",
};
