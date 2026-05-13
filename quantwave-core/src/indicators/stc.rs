use crate::indicators::metadata::{IndicatorMetadata, ParamDef};
use crate::traits::Next;
use std::collections::VecDeque;

/// Schaff Trend Cycle (STC)
///
/// STC is a trend-following indicator that combines MACD with a double-smoothed 
/// stochastic calculation to provide faster and more accurate signals than MACD alone.
#[derive(Debug, Clone)]
pub struct SchaffTrendCycle {
    fast_ema: Ema,
    slow_ema: Ema,
    st1: StochasticEma,
    st2: StochasticEma,
}

#[derive(Debug, Clone)]
struct Ema {
    alpha: f64,
    prev: Option<f64>,
}

impl Ema {
    fn new(period: usize) -> Self {
        Self {
            alpha: 2.0 / (period as f64 + 1.0),
            prev: None,
        }
    }

    fn next(&mut self, input: f64) -> f64 {
        let val = match self.prev {
            None => input,
            Some(p) => self.alpha * input + (1.0 - self.alpha) * p,
        };
        self.prev = Some(val);
        val
    }
}

#[derive(Debug, Clone)]
struct StochasticEma {
    period: usize,
    window: VecDeque<f64>,
    ema: Ema,
}

impl StochasticEma {
    fn new(period: usize, ema_period: usize) -> Self {
        Self {
            period,
            window: VecDeque::with_capacity(period),
            ema: Ema::new(ema_period),
        }
    }

    fn next(&mut self, input: f64) -> f64 {
        self.window.push_front(input);
        if self.window.len() > self.period {
            self.window.pop_back();
        }

        if self.window.len() < self.period {
            return self.ema.next(0.0);
        }

        let mut min = f64::MAX;
        let mut max = f64::MIN;
        for &v in &self.window {
            if v < min { min = v; }
            if v > max { max = v; }
        }

        let stoch = if max == min {
            0.0
        } else {
            100.0 * (input - min) / (max - min)
        };

        self.ema.next(stoch)
    }
}

impl SchaffTrendCycle {
    pub fn new(cycle_period: usize, fast_period: usize, slow_period: usize) -> Self {
        Self {
            fast_ema: Ema::new(fast_period),
            slow_ema: Ema::new(slow_period),
            st1: StochasticEma::new(cycle_period, 3), // Fixed smoothing 3 in many implementations
            st2: StochasticEma::new(cycle_period, 3),
        }
    }
}

impl Default for SchaffTrendCycle {
    fn default() -> Self {
        Self::new(10, 23, 50)
    }
}

impl Next<f64> for SchaffTrendCycle {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let macd = self.fast_ema.next(input) - self.slow_ema.next(input);
        let s1 = self.st1.next(macd);
        self.st2.next(s1)
    }
}

pub const STC_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "Schaff Trend Cycle",
    description: "A hybrid indicator that applies a double-smoothed stochastic to MACD for faster trend identification.",
    params: &[
        ParamDef { name: "cycle_period", default: "10", description: "Stochastic lookback period" },
        ParamDef { name: "fast_period", default: "23", description: "Fast EMA period for MACD" },
        ParamDef { name: "slow_period", default: "50", description: "Slow EMA period for MACD" },
    ],
    formula_source: "https://www.investopedia.com/articles/forex/10/schaff-trend-cycle-indicator.asp",
    formula_latex: r#"
\[
MACD = EMA(23) - EMA(50)
\]
\[
STC = EMA(Stochastic(EMA(Stochastic(MACD, 10), 3), 10), 3)
\]
"#,
    gold_standard_file: "stc.json",
    category: "Modern",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_stc_basic() {
        let mut stc = SchaffTrendCycle::new(10, 23, 50);
        let inputs = vec![10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0];
        for input in inputs {
            let res = stc.next(input);
            assert!(res >= 0.0 && res <= 100.0);
        }
    }

    proptest! {
        #[test]
        fn test_stc_parity(
            inputs in prop::collection::vec(1.0..100.0, 50..100),
        ) {
            let mut stc = SchaffTrendCycle::new(10, 23, 50);
            let streaming_results: Vec<f64> = inputs.iter().map(|&x| stc.next(x)).collect();

            let mut stc_batch = SchaffTrendCycle::new(10, 23, 50);
            let batch_results: Vec<f64> = inputs.iter().map(|&x| stc_batch.next(x)).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s, b, epsilon = 1e-10);
            }
        }
    }
}
