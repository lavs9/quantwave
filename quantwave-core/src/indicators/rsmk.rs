use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::EMA;
use crate::traits::Next;
use std::collections::VecDeque;

pub const METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Relative Strength Markos Katsanos",
    description: "An improved relative strength indicator that compares a security to a benchmark, separating periods of strong and weak relative performance.",
    usage: "Use as a momentum-based relative strength indicator. Values above zero indicate the security is outperforming the benchmark over the specified period.",
    keywords: &["relative strength", "momentum", "benchmark", "katsanos"],
    ehlers_summary: "RSMK calculates the log-ratio momentum of a security relative to a benchmark (e.g., SPY). It measures the difference between current log-relative strength and its value N bars ago, then smooths it with an EMA. This approach identifies trends in relative performance with less lag than traditional methods.",
    params: &[
        ParamDef {
            name: "length",
            default: "90",
            description: "Momentum lookback period",
        },
        ParamDef {
            name: "ema_length",
            default: "3",
            description: "EMA smoothing period",
        },
    ],
    formula_source: "TASC March 2020",
    formula_latex: r#"
\[
RSMK = EMA(\ln(\frac{P_t}{B_t}) - \ln(\frac{P_{t-n}}{B_{t-n}}), m) \times 100
\]
"#,
    gold_standard_file: "rsmk_90_3.json",
    category: "Momentum",
};

/// Relative Strength Markos Katsanos (RSMK)
///
/// Compares a security to a benchmark using log-momentum and EMA smoothing.
#[derive(Debug, Clone)]
pub struct RSMK {
    length: usize,
    ema: EMA,
    log_val_window: VecDeque<f64>,
}

impl RSMK {
    pub fn new(length: usize, ema_length: usize) -> Self {
        Self {
            length,
            ema: EMA::new(ema_length),
            log_val_window: VecDeque::with_capacity(length + 1),
        }
    }
}

impl Next<(f64, f64)> for RSMK {
    type Output = f64;

    fn next(&mut self, (price, benchmark): (f64, f64)) -> Self::Output {
        if price <= 0.0 || benchmark <= 0.0 {
            // In case of invalid inputs, we might want to push a default or handle it.
            // But log(ratio) will fail.
            return 0.0;
        }

        let log_val = (price / benchmark).ln();
        self.log_val_window.push_back(log_val);

        if self.log_val_window.len() <= self.length {
            // Not enough data for momentum calculation.
            // We still feed the EMA to initialize it, though the momentum is effectively 0 or based on the first available.
            // Katsanos code: RSMK = XAverage( LogVal - LogVal[Length], EMALength ) * 100 ;
            // If LogVal[Length] is not available, LogVal - LogVal[Length] is undefined.
            return 0.0;
        }

        let old_log_val = self.log_val_window.pop_front().unwrap();
        let momentum = log_val - old_log_val;
        
        self.ema.next(momentum) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsmk_basic() {
        let mut rsmk = RSMK::new(2, 3);
        
        // Data: (Price, Benchmark)
        // LogVal = ln(Price/Benchmark)
        
        // Bar 1: (10, 100) -> LogVal = ln(0.1) = -2.302585
        assert_eq!(rsmk.next((10.0, 100.0)), 0.0);
        
        // Bar 2: (11, 100) -> LogVal = ln(0.11) = -2.207275
        assert_eq!(rsmk.next((11.0, 100.0)), 0.0);
        
        // Bar 3: (12, 100) -> LogVal = ln(0.12) = -2.120264
        // Momentum = ln(0.12) - ln(0.1) = -2.120264 - (-2.302585) = 0.182321
        // EMA(1, 0.182321) = 0.182321
        // RSMK = 0.182321 * 100 = 18.2321
        let val = rsmk.next((12.0, 100.0));
        approx::assert_relative_eq!(val, 18.232155, epsilon = 1e-6);
        
        // Bar 4: (12, 110) -> LogVal = ln(12/110) = -2.215574
        // Momentum = ln(12/110) - ln(0.11) = -2.215574 - (-2.207275) = -0.008299
        // EMA(prev=0.182321, curr=-0.008299, length=3)
        // alpha = 2/(3+1) = 0.5
        // EMA = 0.5 * (-0.008299) + 0.5 * (0.182321) = 0.087011
        // RSMK = 8.7011
        let val = rsmk.next((12.0, 110.0));
        approx::assert_relative_eq!(val, 8.7011, epsilon = 1e-4);
    }
}
