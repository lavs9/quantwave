use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Kaufman's Adaptive Moving Average (KAMA)
///
/// KAMA is an adaptive moving average that adjusts its smoothing based on the 
/// efficiency of price movement (signal-to-noise ratio).
#[derive(Debug, Clone)]
pub struct Kama {
    period: usize,
    fast_sc: f64,
    slow_sc: f64,
    window: VecDeque<f64>,
    prev_kama: Option<f64>,
}

impl Kama {
    pub fn new(period: usize, fast_period: usize, slow_period: usize) -> Self {
        let fast_sc = 2.0 / (fast_period as f64 + 1.0);
        let slow_sc = 2.0 / (slow_period as f64 + 1.0);
        
        Self {
            period,
            fast_sc,
            slow_sc,
            window: VecDeque::with_capacity(period + 1),
            prev_kama: None,
        }
    }
}

impl Default for Kama {
    fn default() -> Self {
        Self::new(10, 2, 30)
    }
}

impl Next<f64> for Kama {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_front(input);
        if self.window.len() > self.period + 1 {
            self.window.pop_back();
        }

        if self.window.len() <= self.period {
            if self.prev_kama.is_none() {
                self.prev_kama = Some(input);
            }
            return input;
        }

        // Efficiency Ratio (ER)
        // Signal = abs(Price - Price[N])
        let signal = (input - self.window.back().unwrap()).abs();
        
        // Noise = sum(abs(Price - Price[1]), N)
        let mut noise = 0.0;
        for i in 0..self.period {
            noise += (self.window[i] - self.window[i+1]).abs();
        }
        
        let er = if noise != 0.0 { signal / noise } else { 0.0 };
        
        // Smoothing Constant (SC)
        let sc = (er * (self.fast_sc - self.slow_sc) + self.slow_sc).powi(2);
        
        // KAMA
        let prev = self.prev_kama.unwrap_or(input);
        let kama = prev + sc * (input - prev);
        self.prev_kama = Some(kama);
        
        kama
    }
}

pub const KAMA_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "KAMA",
    description: "Kaufman's Adaptive Moving Average adjusts its sensitivity based on market volatility.",
    usage: "Use as an adaptive moving average that is fast in trending markets and slow in choppy, sideways conditions. Reduces whipsaws that plague fixed-period moving averages in ranging markets.",
    keywords: &["moving-average", "adaptive", "smoothing", "classic"],
    ehlers_summary: "Perry Kaufman designed KAMA using an Efficiency Ratio that measures how directionally price has moved versus total path length. A high ratio (strong trend) produces a fast-reacting EMA; a low ratio (choppy market) produces a near-flat line, dramatically reducing false signals during consolidation. — New Trading Systems and Methods, 4th ed.",
    params: &[
        ParamDef { name: "period", default: "10", description: "Efficiency Ratio lookback period" },
        ParamDef { name: "fast_period", default: "2", description: "Fastest smoothing period" },
        ParamDef { name: "slow_period", default: "30", description: "Slowest smoothing period" },
    ],
    formula_source: "https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:kaufman_s_adaptive_moving_average",
    formula_latex: r#"
\[
ER = \frac{|Price - Price_{t-n}|}{\sum |Price - Price_{t-1}|}
\]
\[
SC = [ER(FastSC - SlowSC) + SlowSC]^2
\]
\[
KAMA = KAMA_{t-1} + SC(Price - KAMA_{t-1})
\]
"#,
    gold_standard_file: "kama.json",
    category: "Classic",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_kama_basic() {
        let mut kama = Kama::new(10, 2, 30);
        let inputs = vec![10.0, 11.0, 10.5, 12.0, 13.0, 14.0, 13.5, 15.0, 16.0, 17.0, 18.0, 19.0];
        for input in inputs {
            let res = kama.next(input);
            assert!(!res.is_nan());
        }
    }

    proptest! {
        #[test]
        fn test_kama_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let period = 10;
            let mut kama = Kama::new(period, 2, 30);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| kama.next(x)).collect();

            let mut prev_kama = None;
            let fast_sc = 2.0 / (2.0 + 1.0);
            let slow_sc = 2.0 / (30.0 + 1.0);
            
            for (i, &input) in inputs.iter().enumerate() {
                if i < period {
                    if prev_kama.is_none() { prev_kama = Some(input); }
                    approx::assert_relative_eq!(streaming_results[i], input, epsilon = 1e-10);
                    continue;
                }
                
                let signal = (input - inputs[i - period]).abs();
                let mut noise = 0.0;
                for j in 0..period {
                    noise += (inputs[i-j] - inputs[i-j-1]).abs();
                }
                
                let er = if noise != 0.0 { signal / noise } else { 0.0 };
                let sc = (er * (fast_sc - slow_sc) + slow_sc).powi(2);
                let current_kama = prev_kama.unwrap() + sc * (input - prev_kama.unwrap());
                
                approx::assert_relative_eq!(streaming_results[i], current_kama, epsilon = 1e-10);
                prev_kama = Some(current_kama);
            }
        }
    }
}
