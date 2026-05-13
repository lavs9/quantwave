use crate::indicators::metadata::IndicatorMetadata;
use crate::traits::Next;

/// System Evaluation Metrics
///
/// Based on John Ehlers and Ric Way's "Evaluating Trading Systems".
/// Provides a robust set of statistical performance descriptors for a trading system.
#[derive(Debug, Clone, Default)]
pub struct SystemEvaluator {
    gross_winnings: f64,
    gross_losses: f64,
    num_wins: usize,
    num_losses: usize,
    count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SystemEvaluationResults {
    pub average_win_loss_ratio: f64,
    pub average_trade: f64,
    pub profit_factor: f64,
    pub percent_winners: f64,
    pub breakeven_profit_factor: f64,
    pub weighted_average_trade: f64,
    pub theoretical_consecutive_losers: f64,
}

impl SystemEvaluator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Next<f64> for SystemEvaluator {
    type Output = SystemEvaluationResults;

    fn next(&mut self, trade_profit: f64) -> Self::Output {
        self.count += 1;
        if trade_profit > 0.0 {
            self.gross_winnings += trade_profit;
            self.num_wins += 1;
        } else if trade_profit < 0.0 {
            self.gross_losses += trade_profit.abs();
            self.num_losses += 1;
        }

        let total_trades = (self.num_wins + self.num_losses) as f64;
        if total_trades == 0.0 {
            return SystemEvaluationResults::default();
        }

        let win_ratio = self.num_wins as f64 / total_trades;
        let loss_ratio = 1.0 - win_ratio;
        let pf = if self.gross_losses > 0.0 {
            self.gross_winnings / self.gross_losses
        } else if self.gross_winnings > 0.0 {
            100.0 // Cap at 100 for no losses
        } else {
            0.0
        };

        let ave_win = if self.num_wins > 0 { self.gross_winnings / self.num_wins as f64 } else { 0.0 };
        let ave_loss = if self.num_losses > 0 { self.gross_losses / self.num_losses as f64 } else { 0.0 };
        
        let ave_win_loss_ratio = if ave_loss > 0.0 { ave_win / ave_loss } else { 0.0 };
        let average_trade = (self.gross_winnings - self.gross_losses) / total_trades;
        
        let breakeven_pf = if win_ratio > 0.0 { loss_ratio / win_ratio } else { 100.0 };
        
        // Weighted Average Trade = AverageTrade * (AveWin / AveLoss)
        // Note: The paper derives it as T * (AveWin / AveLoss)
        let weighted_average_trade = average_trade * ave_win_loss_ratio;
        
        // N = Log(0.0027) / Log(1 - %)
        // Where % is the probability of a win.
        let theoretical_consecutive_losers = if win_ratio < 1.0 {
            (0.0027f64.ln()) / (1.0 - win_ratio).ln()
        } else {
            0.0
        };

        SystemEvaluationResults {
            average_win_loss_ratio: ave_win_loss_ratio,
            average_trade,
            profit_factor: pf,
            percent_winners: win_ratio,
            breakeven_profit_factor: breakeven_pf,
            weighted_average_trade,
            theoretical_consecutive_losers,
        }
    }
}

pub const SYSTEM_EVALUATOR_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "System Evaluator",
    description: "Calculates robust statistical performance metrics for a trading system based on a stream of trade profits.",
    params: &[],
    formula_source: "https://github.com/lavs9/quantwave/blob/main/references/Ehlers%20Papers/SystemEvaluation.pdf",
    formula_latex: r#"
\[
AveTrade = \% \cdot (PF + 1) - 1
\]
\[
PF_{breakeven} = \frac{1 - \%}{\%}
\]
\[
N_{losers} = \frac{\ln(0.0027)}{\ln(1 - \%)}
\]
"#,
    gold_standard_file: "system_evaluation.json",
    category: "Statistics",
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Next;
    use proptest::prelude::*;

    #[test]
    fn test_system_evaluator() {
        let mut evaluator = SystemEvaluator::new();
        // A simple system: 2 wins of 200, 1 loss of 100
        // PF = 400 / 100 = 4.0
        // % Win = 2 / 3 = 0.666
        // AveTrade = (400 - 100) / 3 = 100.0
        evaluator.next(200.0);
        evaluator.next(-100.0);
        let res = evaluator.next(200.0);

        approx::assert_relative_eq!(res.profit_factor, 4.0);
        approx::assert_relative_eq!(res.percent_winners, 0.6666666666666666);
        approx::assert_relative_eq!(res.average_trade, 100.0);
        assert!(res.weighted_average_trade > 0.0);
    }

    proptest! {
        #[test]
        fn test_system_evaluator_parity(
            inputs in prop::collection::vec(-100.0..100.0, 10..100),
        ) {
            let mut evaluator = SystemEvaluator::new();
            let streaming_results: Vec<SystemEvaluationResults> = inputs.iter().map(|&x| evaluator.next(x)).collect();

            let mut evaluator_batch = SystemEvaluator::new();
            let batch_results: Vec<SystemEvaluationResults> = inputs.iter().map(|&x| evaluator_batch.next(x)).collect();

            for (s, b) in streaming_results.iter().zip(batch_results.iter()) {
                approx::assert_relative_eq!(s.profit_factor, b.profit_factor, epsilon = 1e-10);
                approx::assert_relative_eq!(s.average_trade, b.average_trade, epsilon = 1e-10);
                approx::assert_relative_eq!(s.percent_winners, b.percent_winners, epsilon = 1e-10);
            }
        }
    }
}
