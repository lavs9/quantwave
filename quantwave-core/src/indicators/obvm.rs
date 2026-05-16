use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::indicators::smoothing::EMA;
use crate::traits::Next;

pub const METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "OBVM",
    description: "On-Balance Volume Modified - a smoothed version of OBV with an additional signal line.",
    usage: "Used to identify divergences between price and volume flow, and to generate signals via crossovers with its signal line. Values typically follow the trend of buying and selling pressure.",
    keywords: &["volume", "obv", "momentum", "smoothing", "apirine"],
    ehlers_summary: "While originally developed by Joe Granville, this modified version by Vitali Apirine applies exponential smoothing to the OBV values to filter out noise and adds a signal line for better trend identification and crossover signals. It provides a clearer picture of volume-price relationships by reducing high-frequency fluctuations. — TASC April 2020",
    params: &[
        ParamDef {
            name: "obvm_period",
            default: "7",
            description: "EMA period for smoothing OBV",
        },
        ParamDef {
            name: "signal_period",
            default: "10",
            description: "EMA period for the signal line",
        },
    ],
    formula_source: "https://www.traders.com/Documentation/FEEDbk_docs/2020/04/TradersTips.html",
    formula_latex: r#"
\begin{aligned}
TP &= \frac{High + Low + Close}{3} \\
OBV_t &= OBV_{t-1} + \begin{cases} Volume, & \text{if } TP_t > TP_{t-1} \\ -Volume, & \text{if } TP_t < TP_{t-1} \\ 0, & \text{otherwise} \end{cases} \\
OBVM &= EMA(OBV, Period_1) \\
Signal &= EMA(OBVM, Period_2)
\end{aligned}
"#,
    gold_standard_file: "obvm.json",
    category: "Volume Indicators",
};

/// On-Balance Volume Modified (OBVM)
///
/// Modified OBV using typical price and smoothing.
/// Based on Vitali Apirine's article in TASC April 2020.
#[derive(Debug, Clone)]
pub struct Obvm {
    obv: f64,
    prev_price: Option<f64>,
    ema_obv: EMA,
    ema_signal: EMA,
}

impl Obvm {
    pub fn new(obvm_period: usize, signal_period: usize) -> Self {
        Self {
            obv: 0.0,
            prev_price: None,
            ema_obv: EMA::new(obvm_period),
            ema_signal: EMA::new(signal_period),
        }
    }
}

impl Next<(f64, f64, f64, f64)> for Obvm {
    type Output = (f64, f64);

    fn next(&mut self, (high, low, close, volume): (f64, f64, f64, f64)) -> Self::Output {
        let tp = (high + low + close) / 3.0;

        match self.prev_price {
            Some(prev) => {
                if tp > prev {
                    self.obv += volume;
                } else if tp < prev {
                    self.obv -= volume;
                }
            }
            None => {
                // First bar: some start with 0, others with current volume.
                // Article implementations vary; starting with 0 is common.
            }
        }

        self.prev_price = Some(tp);

        let obvm_val = self.ema_obv.next(self.obv);
        let signal_val = self.ema_signal.next(obvm_val);

        (obvm_val, signal_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_obvm_basic() {
        let mut obvm = Obvm::new(7, 10);
        // (H, L, C, V)
        let inputs = vec![
            (10.0, 10.0, 10.0, 1000.0), // TP = 10, OBV = 0
            (11.0, 11.0, 11.0, 1000.0), // TP = 11, OBV = 1000
            (12.0, 12.0, 12.0, 1000.0), // TP = 12, OBV = 2000
            (11.0, 11.0, 11.0, 1000.0), // TP = 11, OBV = 1000
        ];

        let results: Vec<(f64, f64)> = inputs.into_iter().map(|x| obvm.next(x)).collect();
        
        // Check that values are being computed and are finite
        for (obvm_val, signal_val) in results {
            assert!(!obvm_val.is_nan());
            assert!(!signal_val.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_obvm_parity(
            inputs in prop::collection::vec((1.0..100.0, 1.0..100.0, 1.0..100.0, 1.0..1000.0), 10..100),
        ) {
            let mut obvm = Obvm::new(7, 10);
            
            let mut obv = 0.0;
            let mut prev_tp: Option<f64> = None;
            let mut ema_obv = EMA::new(7);
            let mut ema_signal = EMA::new(10);
            
            for (h, l, c, v) in inputs {
                let h: f64 = h;
                let l: f64 = l;
                let c: f64 = c;
                let high = h.max(l).max(c);
                let low = h.min(l).min(c);
                let tp = (high + low + c) / 3.0;
                
                if let Some(prev) = prev_tp {
                    if tp > prev {
                        obv += v;
                    } else if tp < prev {
                        obv -= v;
                    }
                }
                prev_tp = Some(tp);
                
                let expected_obvm = ema_obv.next(obv);
                let expected_signal = ema_signal.next(expected_obvm);
                
                let (actual_obvm, actual_signal) = obvm.next((high, low, c, v));
                
                approx::assert_relative_eq!(actual_obvm, expected_obvm, epsilon = 1e-10);
                approx::assert_relative_eq!(actual_signal, expected_signal, epsilon = 1e-10);
            }
        }
    }
}
