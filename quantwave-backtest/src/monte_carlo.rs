//! Trade PnL bootstrap Monte Carlo (quantwave-cr6v.14 / quantwave-xibc).
//!
//! Clean-room resampling of closed-trade `pnl_net` values (RaptorBT MC concepts;
//! v1 uses trade bootstrap rather than GBM forward paths).

use crate::{BacktestError, BacktestResult};
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};

/// Bootstrap simulation settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonteCarloConfig {
    pub n_simulations: usize,
    pub seed: u64,
}

impl Default for MonteCarloConfig {
    fn default() -> Self {
        Self {
            n_simulations: 1_000,
            seed: 42,
        }
    }
}

/// Summary of bootstrap terminal equity distribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonteCarloSummary {
    pub mean_final_equity: f64,
    pub p5_final_equity: f64,
    pub p50_final_equity: f64,
    pub p95_final_equity: f64,
    pub probability_of_loss: f64,
    pub n_simulations: usize,
    pub n_trades_sampled: usize,
}

/// Bootstrap closed-trade PnLs with replacement; return terminal equity stats.
pub fn monte_carlo_trade_bootstrap(
    result: &BacktestResult,
    initial_cash: f64,
    config: &MonteCarloConfig,
) -> Result<MonteCarloSummary, BacktestError> {
    if config.n_simulations == 0 {
        return Err(BacktestError::InvalidInput(
            "n_simulations must be > 0".into(),
        ));
    }

    let pnls = extract_trade_pnls(result);
    if pnls.is_empty() {
        return Ok(MonteCarloSummary {
            mean_final_equity: initial_cash,
            p5_final_equity: initial_cash,
            p50_final_equity: initial_cash,
            p95_final_equity: initial_cash,
            probability_of_loss: 0.0,
            n_simulations: config.n_simulations,
            n_trades_sampled: 0,
        });
    }

    let mut rng = StdRng::seed_from_u64(config.seed);
    let n_trades = pnls.len();
    let mut finals = Vec::with_capacity(config.n_simulations);

    for _ in 0..config.n_simulations {
        let mut equity = initial_cash;
        for _ in 0..n_trades {
            let idx = rng.gen_range(0..n_trades);
            equity += pnls[idx];
        }
        finals.push(equity);
    }

    finals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = finals.len();
    let mean = finals.iter().sum::<f64>() / n as f64;
    let p5 = percentile(&finals, 0.05);
    let p50 = percentile(&finals, 0.50);
    let p95 = percentile(&finals, 0.95);
    let prob_loss = finals.iter().filter(|&&e| e < initial_cash).count() as f64 / n as f64;

    Ok(MonteCarloSummary {
        mean_final_equity: mean,
        p5_final_equity: p5,
        p50_final_equity: p50,
        p95_final_equity: p95,
        probability_of_loss: prob_loss,
        n_simulations: config.n_simulations,
        n_trades_sampled: n_trades,
    })
}

fn extract_trade_pnls(result: &BacktestResult) -> Vec<f64> {
    let Ok(col) = result.trades.column("pnl_net") else {
        return Vec::new();
    };
    let Ok(ca) = col.f64() else {
        return Vec::new();
    };
    ca.into_iter().map(|v| v.unwrap_or(0.0)).collect()
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() - 1) as f64 * p).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars::prelude::*;

    fn result_with_pnls(pnls: &[f64], initial: f64) -> BacktestResult {
        let trades = DataFrame::new(vec![Column::new("pnl_net".into(), pnls.to_vec())]).unwrap();
        let equity = DataFrame::new(vec![
            Column::new("ts".into(), vec![1i64, 2]),
            Column::new("equity".into(), vec![initial, initial + pnls.iter().sum::<f64>()]),
            Column::new("cash".into(), vec![initial, initial]),
            Column::new("position".into(), vec![0.0, 0.0]),
            Column::new("close".into(), vec![100.0, 100.0]),
        ])
        .unwrap();
        BacktestResult {
            trades,
            equity_curve: equity,
            stats: std::collections::HashMap::from([
                ("initial_cash".to_string(), initial),
                ("final_equity".to_string(), initial + pnls.iter().sum::<f64>()),
            ]),
        }
    }

    #[test]
    fn test_monte_carlo_deterministic_with_seed() {
        let result = result_with_pnls(&[100.0, -50.0, 25.0], 100_000.0);
        let cfg = MonteCarloConfig {
            n_simulations: 500,
            seed: 99,
        };
        let a = monte_carlo_trade_bootstrap(&result, 100_000.0, &cfg).unwrap();
        let b = monte_carlo_trade_bootstrap(&result, 100_000.0, &cfg).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.n_trades_sampled, 3);
    }

    #[test]
    fn test_monte_carlo_zero_trades_flat() {
        let result = result_with_pnls(&[], 100_000.0);
        let summary =
            monte_carlo_trade_bootstrap(&result, 100_000.0, &MonteCarloConfig::default()).unwrap();
        assert_eq!(summary.mean_final_equity, 100_000.0);
        assert_eq!(summary.probability_of_loss, 0.0);
    }

    #[test]
    fn test_monte_carlo_all_negative_trades_high_prob_loss() {
        let result = result_with_pnls(&[-10.0, -10.0, -10.0, -10.0], 100_000.0);
        let summary = monte_carlo_trade_bootstrap(
            &result,
            100_000.0,
            &MonteCarloConfig {
                n_simulations: 200,
                seed: 1,
            },
        )
        .unwrap();
        assert_eq!(summary.probability_of_loss, 1.0);
        assert!(summary.mean_final_equity < 100_000.0);
    }
}