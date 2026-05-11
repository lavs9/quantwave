use crate::traits::Next;
use std::collections::VecDeque;

/// Simple Moving Average (SMA)
#[derive(Debug, Clone)]
pub struct SMA {
    period: usize,
    window: VecDeque<f64>,
    sum: f64,
}

impl SMA {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            window: VecDeque::with_capacity(period),
            sum: 0.0,
        }
    }
}

impl Next<f64> for SMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_back(input);
        self.sum += input;

        if self.window.len() > self.period {
            if let Some(oldest) = self.window.pop_front() {
                self.sum -= oldest;
            }
        }

        self.sum / self.window.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{load_gold_standard, assert_indicator_parity};

    #[test]
    fn test_sma_gold_standard() {
        let case = load_gold_standard("sma_5");
        let sma = SMA::new(3); // The expected values in JSON are for SMA(3)
        assert_indicator_parity(sma, &case.input, &case.expected);
    }
}

/// Exponential Moving Average (EMA)
#[derive(Debug, Clone)]
pub struct EMA {
    _period: usize,
    alpha: f64,
    current_ema: Option<f64>,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        Self {
            _period: period,
            alpha: 2.0 / (period as f64 + 1.0),
            current_ema: None,
        }
    }
}

impl Next<f64> for EMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        match self.current_ema {
            Some(prev_ema) => {
                let ema = self.alpha * input + (1.0 - self.alpha) * prev_ema;
                self.current_ema = Some(ema);
                ema
            }
            None => {
                self.current_ema = Some(input);
                input
            }
        }
    }
}
