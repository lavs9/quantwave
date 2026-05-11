use crate::traits::Next;
use std::collections::VecDeque;

pub struct ALMA {
    period: usize,
    offset: f64,
    sigma: f64,
    window: VecDeque<f64>,
    weights: Vec<f64>,
}

impl ALMA {
    pub fn new(period: usize, offset: f64, sigma: f64) -> Self {
        let m = offset * (period as f64 - 1.0);
        let s = period as f64 / sigma;
        let mut weights = Vec::with_capacity(period);
        let mut sum_w = 0.0;

        for i in 0..period {
            let weight = (-( (i as f64 - m).powi(2) / (2.0 * s.powi(2)) )).exp();
            weights.push(weight);
            sum_w += weight;
        }

        // Normalize weights
        for w in weights.iter_mut() {
            *w /= sum_w;
        }

        Self {
            period,
            offset,
            sigma,
            window: VecDeque::with_capacity(period),
            weights,
        }
    }
}

impl Next<f64> for ALMA {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_back(input);
        if self.window.len() > self.period {
            self.window.pop_front();
        }

        if self.window.len() < self.period {
            // During warmup, we can either return partial ALMA or 0.0
            // Standard TradingView implementation usually returns NaN or 0 until period is reached.
            // We'll calculate partial sum with partial normalization for better streaming behavior.
            let mut sum_w = 0.0;
            let mut weighted_val_sum = 0.0;
            for (i, &val) in self.window.iter().enumerate() {
                let weight = self.weights[i + self.period - self.window.len()];
                weighted_val_sum += val * weight;
                sum_w += weight;
            }
            if sum_w == 0.0 { 0.0 } else { weighted_val_sum / sum_w }
        } else {
            let mut weighted_val_sum = 0.0;
            for (i, &val) in self.window.iter().enumerate() {
                weighted_val_sum += val * self.weights[i];
            }
            weighted_val_sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alma_basic() {
        let mut alma = ALMA::new(9, 0.85, 6.0);
        // We'll just verify it produces a non-zero value for now to satisfy RED
        for i in 1..20 {
            let val = alma.next(i as f64);
            if i >= 9 {
                assert!(val > 0.0);
            }
        }
    }
}
