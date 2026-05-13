/// Robustness Score
///
/// Based on John Ehlers' "A Procedure to Evaluate Trading Strategy Robustness".
/// Robustness is determined by the slope of the net profit as a function of the number of tests run.
/// Score = (NetProfit at midpoint of ranked tests) / (Maximum NetProfit).
/// A score >= 0.75 (75%) indicates a reasonable expectation that out-of-sample performance
/// will be at least 75% of optimized in-sample net profit.
#[derive(Debug, Clone, Default)]
pub struct RobustnessEvaluator {
    profits: Vec<f64>,
}

impl RobustnessEvaluator {
    pub fn new() -> Self {
        Self { profits: Vec::new() }
    }

    pub fn add_test_result(&mut self, net_profit: f64) {
        self.profits.push(net_profit);
    }

    pub fn calculate_score(&self) -> f64 {
        if self.profits.is_empty() {
            return 0.0;
        }

        let mut sorted = self.profits.clone();
        // Sort descending
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

        let max_profit = sorted[0];
        if max_profit <= 0.0 {
            return 0.0;
        }

        let midpoint = sorted.len() / 2;
        sorted[midpoint] / max_profit
    }
}

pub fn calculate_robustness(net_profits: &[f64]) -> f64 {
    let mut evaluator = RobustnessEvaluator::new();
    for &p in net_profits {
        evaluator.add_test_result(p);
    }
    evaluator.calculate_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robustness_basic() {
        let profits = vec![100.0, 90.0, 80.0, 70.0, 60.0];
        // Midpoint index is 2 (80.0)
        // Score = 80 / 100 = 0.8
        assert_eq!(calculate_robustness(&profits), 0.8);
    }

    #[test]
    fn test_robustness_empty() {
        assert_eq!(calculate_robustness(&[]), 0.0);
    }

    #[test]
    fn test_robustness_negative() {
        let profits = vec![-10.0, -20.0];
        assert_eq!(calculate_robustness(&profits), 0.0);
    }
}
