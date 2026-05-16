use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::{EMA, SMA};
use crate::traits::Next;

pub const METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Exponential Deviation Bands",
    description: "A price band indicator based on exponential deviation that applies more weight to recent data and generates fewer breakouts than standard deviation bands.",
    usage: "Use as a tool to identify trends and potential trend reversals. Prices consistently above the upper band indicate a strong uptrend, while prices below the lower band indicate a strong downtrend.",
    keywords: &["bands", "volatility", "exponential-deviation", "trend"],
    ehlers_summary: "Introduced by Vitali Apirine, Exponential Deviation Bands use an EMA of the absolute deviation from a base moving average (SMA or EMA) to create volatility bands. This approach is more responsive to recent price changes than standard deviation-based Bollinger Bands.",
    params: &[
        ParamDef {
            name: "period",
            default: "20",
            description: "Period for the base moving average and exponential deviation.",
        },
        ParamDef {
            name: "dev_mult",
            default: "2.0",
            description: "Multiplier for the exponential deviation.",
        },
        ParamDef {
            name: "use_sma",
            default: "false",
            description: "Whether to use SMA (true) or EMA (false) as the base moving average.",
        },
    ],
    formula_source: "Technical Analysis of Stocks & Commodities, July 2019",
    formula_latex: r#"
\[
BaseMA = \text{SMA or EMA}(Price, n) \\
Deviation = |BaseMA - Price| \\
ExpDev = EMA(Deviation, n) \\
Upper = BaseMA + ExpDev \times multiplier \\
Lower = BaseMA - ExpDev \times multiplier
\]
"#,
    gold_standard_file: "exp_dev_bands_20_2.json",
    category: "Classic",
};

#[derive(Debug, Clone)]
enum BaseMA {
    SMA(SMA),
    EMA(EMA),
}

impl Next<f64> for BaseMA {
    type Output = f64;
    fn next(&mut self, input: f64) -> Self::Output {
        match self {
            BaseMA::SMA(inner) => inner.next(input),
            BaseMA::EMA(inner) => inner.next(input),
        }
    }
}

/// Exponential Deviation Bands
#[derive(Debug, Clone)]
pub struct ExpDevBands {
    base_ma: BaseMA,
    exp_dev_ema: EMA,
    multiplier: f64,
}

impl ExpDevBands {
    pub fn new(period: usize, multiplier: f64, use_sma: bool) -> Self {
        let base_ma = if use_sma {
            BaseMA::SMA(SMA::new(period))
        } else {
            BaseMA::EMA(EMA::new(period))
        };

        Self {
            base_ma,
            exp_dev_ema: EMA::new(period),
            multiplier,
        }
    }
}

impl Next<f64> for ExpDevBands {
    type Output = (f64, f64, f64); // (Upper, Basis, Lower)

    fn next(&mut self, input: f64) -> Self::Output {
        let basis = self.base_ma.next(input);
        let deviation = (basis - input).abs();
        let exp_dev = self.exp_dev_ema.next(deviation);

        let upper = basis + exp_dev * self.multiplier;
        let lower = basis - exp_dev * self.multiplier;

        (upper, basis, lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_exp_dev_bands_basic() {
        let mut edb = ExpDevBands::new(20, 2.0, true);
        let price = 100.0;
        let (upper, basis, lower) = edb.next(price);
        
        approx::assert_relative_eq!(basis, 100.0);
        approx::assert_relative_eq!(upper, 100.0);
        approx::assert_relative_eq!(lower, 100.0);
        
        let (upper, basis, _lower) = edb.next(110.0);
        // SMA(2) of [100, 110] = 105.0
        // SMA(20) of [100, 110] = 105.0 (window not full)
        assert_eq!(basis, 105.0);
        // Deviation = |105 - 110| = 5.0
        // EMA of Dev: first input is 0.0? 
        // Wait, SMA(100) next is 100. Dev = |100-100| = 0.
        // EMA(Dev) next(5.0) where prev was 0.0? 
        // EMA starts with first input.
        // Turn 1: SMA next(100) -> 100. Dev = 0. EMA next(0) -> 0. (upper, basis, lower) = (100, 100, 100).
        // Turn 2: SMA next(110) -> 105. Dev = |105-110|=5. EMA next(5) -> 0.095 * 5 + 0.905 * 0 = 0.476 (if period 20)
        // Wait, EMA alpha = 2 / (20 + 1) = 2/21 = 0.095238
        // Next value = 0.095238 * 5.0 + (1 - 0.095238) * 0.0 = 0.47619
        // Upper = 105 + 2 * 0.47619 = 105.95238
        
        approx::assert_relative_eq!(basis, 105.0);
        approx::assert_relative_eq!(upper, 105.95238, epsilon = 1e-4);
    }

    fn exp_dev_bands_batch(data: &[f64], period: usize, multiplier: f64, use_sma: bool) -> Vec<(f64, f64, f64)> {
        let mut edb = ExpDevBands::new(period, multiplier, use_sma);
        data.iter().map(|&x| edb.next(x)).collect()
    }

    proptest! {
        #[test]
        fn test_exp_dev_bands_parity(input in prop::collection::vec(0.1..100.0, 1..100)) {
            let period = 20;
            let multiplier = 2.0;
            let use_sma = false;
            
            let mut edb = ExpDevBands::new(period, multiplier, use_sma);
            let streaming_results: Vec<(f64, f64, f64)> = input.iter().map(|&x| edb.next(x)).collect();
            let batch_results = exp_dev_bands_batch(&input, period, multiplier, use_sma);

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.0, b.0, epsilon = 1e-6);
                approx::assert_relative_eq!(s.1, b.1, epsilon = 1e-6);
                approx::assert_relative_eq!(s.2, b.2, epsilon = 1e-6);
            }
        }
    }
}
