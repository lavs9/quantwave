use crate::traits::Next;
use std::collections::VecDeque;

/// Standard Deviation (Population)
#[derive(Debug, Clone)]
pub struct StandardDeviation {
    period: usize,
    window: VecDeque<f64>,
    sum: f64,
    sum_sq: f64,
}

impl StandardDeviation {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            window: VecDeque::with_capacity(period),
            sum: 0.0,
            sum_sq: 0.0,
        }
    }
}

impl Next<f64> for StandardDeviation {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_back(input);
        self.sum += input;
        self.sum_sq += input * input;

        if self.window.len() > self.period {
            if let Some(oldest) = self.window.pop_front() {
                self.sum -= oldest;
                self.sum_sq -= oldest * oldest;
            }
        }

        let n = self.window.len() as f64;
        let mean = self.sum / n;
        let variance = (self.sum_sq / n) - (mean * mean);
        
        // Handle floating point precision issues
        variance.max(0.0).sqrt()
    }
}

/// Linear Regression
/// Returns the value of the regression line at the current bar.
#[derive(Debug, Clone)]
pub struct LinearRegression {
    period: usize,
    window: VecDeque<f64>,
    // Precomputed x values and their sums
    sum_x: f64,
    sum_x2: f64,
}

impl LinearRegression {
    pub fn new(period: usize) -> Self {
        let _n = period as f64;
        let mut sum_x = 0.0;
        let mut sum_x2 = 0.0;
        for i in 0..period {
            let x = i as f64;
            sum_x += x;
            sum_x2 += x * x;
        }

        Self {
            period,
            window: VecDeque::with_capacity(period),
            sum_x,
            sum_x2,
        }
    }
}

impl Next<f64> for LinearRegression {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.window.push_back(input);
        if self.window.len() > self.period {
            self.window.pop_front();
        }

        if self.window.len() < self.period {
            // For partial windows, we could recalculate x sums, 
            // but for TTM Squeeze, we'll wait for full window or return partial.
            // Standard approach: wait for full window or adjust n.
            let n = self.window.len() as f64;
            let mut sum_x = 0.0;
            let mut sum_x2 = 0.0;
            let mut sum_y = 0.0;
            let mut sum_xy = 0.0;
            for (i, &y) in self.window.iter().enumerate() {
                let x = i as f64;
                sum_x += x;
                sum_x2 += x * x;
                sum_y += y;
                sum_xy += x * y;
            }
            
            let denominator = n * sum_x2 - sum_x * sum_x;
            if denominator == 0.0 {
                return input;
            }
            
            let b = (n * sum_xy - sum_x * sum_y) / denominator;
            let a = (sum_y - b * sum_x) / n;
            return a + b * (n - 1.0);
        }

        let n = self.period as f64;
        let mut sum_y = 0.0;
        let mut sum_xy = 0.0;
        for (i, &y) in self.window.iter().enumerate() {
            let x = i as f64;
            sum_y += y;
            sum_xy += x * y;
        }

        let denominator = n * self.sum_x2 - self.sum_x * self.sum_x;
        if denominator == 0.0 {
            return input;
        }

        let b = (n * sum_xy - self.sum_x * sum_y) / denominator;
        let a = (sum_y - b * self.sum_x) / n;
        
        a + b * (n - 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdev_basic() {
        let mut sd = StandardDeviation::new(3);
        // [10] -> mean 10, var 0
        assert_eq!(sd.next(10.0), 0.0);
        // [10, 20] -> mean 15, var (100+400)/2 - 225 = 250 - 225 = 25 -> std 5
        assert_eq!(sd.next(20.0), 5.0);
        // [10, 20, 30] -> mean 20, var (100+400+900)/3 - 400 = 1400/3 - 400 = 466.66 - 400 = 66.66 -> std 8.1649
        approx::assert_relative_eq!(sd.next(30.0), 8.1649658092, epsilon = 1e-6);
    }

    #[test]
    fn test_linreg_basic() {
        let mut lr = LinearRegression::new(3);
        // Perfect line: 1, 2, 3
        lr.next(1.0);
        lr.next(2.0);
        let res = lr.next(3.0);
        approx::assert_relative_eq!(res, 3.0);
        
        // Line y = 2x + 5. x in [0, 1, 2]. y = [5, 7, 9]
        let mut lr2 = LinearRegression::new(3);
        lr2.next(5.0);
        lr2.next(7.0);
        let res2 = lr2.next(9.0);
        approx::assert_relative_eq!(res2, 9.0);
    }
}
