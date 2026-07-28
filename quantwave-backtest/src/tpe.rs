//! Tree-structured Parzen Estimator (TPE) — optional Bayesian in-fold optimizer
//! (quantwave-lzzq, part of epic quantwave-xfdr).
//!
//! Source: Bergstra, Bardenet, Bengio & Kegl (2011), "Algorithms for Hyper-Parameter
//! Optimization", NeurIPS 24. This is a from-scratch, dependency-free reimplementation
//! of the core idea (the same construction used by Hyperopt's/Optuna's default TPE
//! sampler): split observed trials into a "good" set (top `gamma` fraction by
//! objective) and a "bad" set (the rest), fit an independent 1-D Gaussian KDE per
//! parameter dimension for each set (`l(x)` for good, `g(x)` for bad), and prefer
//! candidates that maximize the likelihood ratio `l(x) / g(x)` — a proxy for
//! expected improvement. Pure Rust, no extra crates, so it stays wasm/portability
//! friendly and keeps the dependency graph clean.
//!
//! Two entry points are provided:
//! - [`optimize_tpe`]: continuous, bounded parameter search against an arbitrary
//!   objective closure (used directly by the convergence tests below).
//! - [`tpe_select_from_pool`]: TPE-guided selection over a *fixed, discrete* pool of
//!   candidate points (used by `walk_forward::run_walk_forward_optimize_with` to pick
//!   an efficient subset of pre-built grid variants to actually backtest, rather than
//!   evaluating the full grid).
//!
//! Grid search remains the default optimizer everywhere in this crate; TPE is strictly
//! opt-in.

use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

/// TPE hyperparameters.
#[derive(Debug, Clone, PartialEq)]
pub struct TpeConfig {
    /// Total number of trials (objective evaluations) to run.
    pub n_trials: usize,
    /// Number of initial trials drawn uniformly at random before the KDE surrogate
    /// kicks in (needs a minimum sample to be meaningful).
    pub n_startup_trials: usize,
    /// Fraction of trials (by best objective value) treated as the "good" set `l(x)`.
    pub gamma: f64,
    /// Number of candidate points sampled from `l(x)` per trial in the continuous
    /// optimizer; the one maximizing `l(x)/g(x)` is chosen.
    pub n_candidates: usize,
    /// RNG seed — same seed always produces the same trial sequence.
    pub seed: u64,
}

impl TpeConfig {
    /// Sensible defaults for a given trial budget and seed.
    pub fn new(n_trials: usize, seed: u64) -> Self {
        let n_trials = n_trials.max(1);
        Self {
            n_trials,
            n_startup_trials: (n_trials / 5).max(1),
            gamma: 0.25,
            n_candidates: 24,
            seed,
        }
    }
}

impl Default for TpeConfig {
    fn default() -> Self {
        Self::new(20, 42)
    }
}

/// One evaluated trial: the (possibly normalized) parameter vector and its objective
/// score (assumed to be maximized).
#[derive(Debug, Clone, PartialEq)]
pub struct TpeTrial {
    pub params: Vec<f64>,
    pub score: f64,
}

/// Outcome of a TPE run.
#[derive(Debug, Clone, PartialEq)]
pub struct TpeResult {
    pub best_params: Vec<f64>,
    pub best_score: f64,
    pub trials: Vec<TpeTrial>,
}

/// Box-Muller transform for a standard normal sample, so we don't need an extra
/// `rand_distr` dependency just for Gaussian noise.
fn sample_standard_normal(rng: &mut StdRng) -> f64 {
    let u1: f64 = rng.random_range(1e-12..1.0);
    let u2: f64 = rng.random_range(0.0..1.0);
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
}

/// Silverman's rule-of-thumb bandwidth, floored/capped to stay well-behaved for the
/// small sample sizes typical of hyperparameter search.
fn bandwidth_for(samples: &[f64]) -> f64 {
    let n = samples.len();
    if n < 2 {
        return 0.15;
    }
    let mean = samples.iter().sum::<f64>() / n as f64;
    let var = samples.iter().map(|&s| (s - mean).powi(2)).sum::<f64>() / n as f64;
    let std = var.sqrt();
    let bw = 1.06 * std.max(1e-6) * (n as f64).powf(-0.2);
    bw.clamp(0.03, 0.6)
}

/// Gaussian KDE density at `x` given a sample set and bandwidth. Empty samples fall
/// back to a flat (uniform) density of 1.0 so a missing "bad" set doesn't zero out
/// the likelihood ratio.
fn gaussian_kde_pdf(samples: &[f64], bandwidth: f64, x: f64) -> f64 {
    if samples.is_empty() {
        return 1.0;
    }
    let n = samples.len() as f64;
    let norm = 1.0 / (n * bandwidth * (2.0 * std::f64::consts::PI).sqrt());
    let sum: f64 = samples
        .iter()
        .map(|&s| {
            let z = (x - s) / bandwidth;
            (-0.5 * z * z).exp()
        })
        .sum();
    norm * sum
}

/// Split trials into (good, bad) by the top `gamma` fraction of scores (descending).
/// Always leaves at least one trial in `good`.
fn split_good_bad(history: &[TpeTrial], gamma: f64) -> (Vec<&TpeTrial>, Vec<&TpeTrial>) {
    let mut sorted: Vec<&TpeTrial> = history.iter().collect();
    sorted.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let n_good = (((sorted.len() as f64) * gamma).ceil() as usize)
        .max(1)
        .min(sorted.len());
    let (good, bad) = sorted.split_at(n_good);
    (good.to_vec(), bad.to_vec())
}

/// Score a candidate point by the product (across dimensions, assumed independent —
/// the standard TPE simplification) of the good/bad KDE likelihood ratio.
fn score_candidate(point: &[f64], good: &[&TpeTrial], bad: &[&TpeTrial]) -> f64 {
    let dims = point.len();
    let mut score = 1.0f64;
    for dim in 0..dims {
        let good_vals: Vec<f64> = good.iter().map(|t| t.params[dim]).collect();
        let bad_vals: Vec<f64> = bad.iter().map(|t| t.params[dim]).collect();
        let l = gaussian_kde_pdf(&good_vals, bandwidth_for(&good_vals), point[dim]).max(1e-9);
        let g = if bad_vals.is_empty() {
            1.0
        } else {
            gaussian_kde_pdf(&bad_vals, bandwidth_for(&bad_vals), point[dim]).max(1e-9)
        };
        score *= l / g;
    }
    score
}

/// Continuous, bounded-box TPE search maximizing `objective`. `bounds[i] = (lo, hi)`
/// for parameter dimension `i`. Deterministic given `config.seed`.
pub fn optimize_tpe<F: FnMut(&[f64]) -> f64>(
    bounds: &[(f64, f64)],
    config: &TpeConfig,
    mut objective: F,
) -> TpeResult {
    let dims = bounds.len();
    let mut rng = StdRng::seed_from_u64(config.seed);
    let mut history: Vec<TpeTrial> = Vec::with_capacity(config.n_trials);
    let mut best_idx = 0usize;

    if dims == 0 {
        return TpeResult {
            best_params: Vec::new(),
            best_score: f64::NEG_INFINITY,
            trials: Vec::new(),
        };
    }

    let random_point = |rng: &mut StdRng| -> Vec<f64> {
        (0..dims)
            .map(|i| {
                let (lo, hi) = bounds[i];
                if hi > lo { rng.random_range(lo..=hi) } else { lo }
            })
            .collect()
    };

    for trial in 0..config.n_trials {
        let params = if trial < config.n_startup_trials || history.len() < 2 {
            random_point(&mut rng)
        } else {
            let (good, bad) = split_good_bad(&history, config.gamma);
            let mut best_cand: Option<Vec<f64>> = None;
            let mut best_cand_score = f64::NEG_INFINITY;
            for _ in 0..config.n_candidates.max(1) {
                let base_idx = rng.random_range(0..good.len());
                let base = &good[base_idx].params;
                let cand: Vec<f64> = (0..dims)
                    .map(|i| {
                        let (lo, hi) = bounds[i];
                        let range = (hi - lo).max(1e-9);
                        let good_vals: Vec<f64> = good.iter().map(|t| t.params[i]).collect();
                        let bw = bandwidth_for(&good_vals) * range;
                        let noise = sample_standard_normal(&mut rng) * bw;
                        (base[i] + noise).clamp(lo.min(hi), lo.max(hi))
                    })
                    .collect();
                let s = score_candidate(&cand, &good, &bad);
                if s > best_cand_score {
                    best_cand_score = s;
                    best_cand = Some(cand);
                }
            }
            best_cand.unwrap_or_else(|| random_point(&mut rng))
        };

        let score = objective(&params);
        history.push(TpeTrial { params, score });
        if history[history.len() - 1].score > history[best_idx].score {
            best_idx = history.len() - 1;
        }
    }

    TpeResult {
        best_params: history[best_idx].params.clone(),
        best_score: history[best_idx].score,
        trials: history,
    }
}

/// TPE-guided selection over a *fixed, discrete* pool of candidate points (e.g.
/// pre-built grid variants, each already normalized to comparable scale — typically
/// min-max to `[0, 1]` per dimension by the caller). Evaluates at most
/// `config.n_trials` pool entries (never more than `points.len()`), calling
/// `objective(pool_index)` for each one chosen. Returns the best pool index found,
/// its score, and the full trial history (in normalized-point space).
///
/// This is the piece `walk_forward::run_walk_forward_optimize_with` uses: instead of
/// exhaustively backtesting every grid variant on the training fold (as the default
/// grid optimizer does), it backtests only the `n_trials` variants TPE judges most
/// promising.
pub fn tpe_select_from_pool<F: FnMut(usize) -> f64>(
    points: &[Vec<f64>],
    config: &TpeConfig,
    mut objective: F,
) -> (usize, f64, Vec<TpeTrial>) {
    let n_pool = points.len();
    if n_pool == 0 {
        return (0, f64::NEG_INFINITY, Vec::new());
    }

    let mut rng = StdRng::seed_from_u64(config.seed);
    let n_trials = config.n_trials.min(n_pool).max(1);
    let mut evaluated = vec![false; n_pool];
    let mut history: Vec<TpeTrial> = Vec::with_capacity(n_trials);
    let mut best_pool_idx = 0usize;
    let mut best_score = f64::NEG_INFINITY;

    for trial in 0..n_trials {
        let remaining: Vec<usize> = (0..n_pool).filter(|&i| !evaluated[i]).collect();
        if remaining.is_empty() {
            break;
        }

        let pick = if trial < config.n_startup_trials || history.len() < 2 {
            remaining[rng.random_range(0..remaining.len())]
        } else {
            let (good, bad) = split_good_bad(&history, config.gamma);
            let mut best_i = remaining[0];
            let mut best_s = f64::NEG_INFINITY;
            for &i in &remaining {
                let s = score_candidate(&points[i], &good, &bad);
                if s > best_s {
                    best_s = s;
                    best_i = i;
                }
            }
            best_i
        };

        evaluated[pick] = true;
        let score = objective(pick);
        history.push(TpeTrial {
            params: points[pick].clone(),
            score,
        });
        if score > best_score {
            best_score = score;
            best_pool_idx = pick;
        }
    }

    (best_pool_idx, best_score, history)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (a) TPE runs: produces the requested number of trials and a finite best score.
    #[test]
    fn test_tpe_runs() {
        let config = TpeConfig::new(15, 7);
        let result = optimize_tpe(&[(-5.0, 5.0)], &config, |p| -(p[0] * p[0]));
        assert_eq!(result.trials.len(), 15);
        assert!(result.best_score.is_finite());
        assert!(result.best_params[0].is_finite());
    }

    /// (b) Converges near the known optimum of a simple convex objective within
    /// n_trials: maximize -(x - 3)^2, optimum at x = 3.
    #[test]
    fn test_tpe_converges_near_known_optimum_1d() {
        let config = TpeConfig::new(60, 123);
        let result = optimize_tpe(&[(-10.0, 10.0)], &config, |p| -(p[0] - 3.0).powi(2));
        assert!(
            (result.best_params[0] - 3.0).abs() < 0.5,
            "expected best x close to 3.0, got {}",
            result.best_params[0]
        );
        assert!(result.best_score > -0.25);
    }

    /// (b) Multi-dimensional convex objective: maximize -((x-2)^2 + (y+1)^2).
    #[test]
    fn test_tpe_converges_near_known_optimum_2d() {
        let config = TpeConfig::new(80, 55);
        let result = optimize_tpe(&[(-10.0, 10.0), (-10.0, 10.0)], &config, |p| {
            -((p[0] - 2.0).powi(2) + (p[1] + 1.0).powi(2))
        });
        assert!((result.best_params[0] - 2.0).abs() < 1.0);
        assert!((result.best_params[1] + 1.0).abs() < 1.0);
    }

    /// (c) Deterministic given a seed: identical seed -> identical trial sequence.
    #[test]
    fn test_tpe_deterministic_given_seed() {
        let config = TpeConfig::new(30, 999);
        let objective = |p: &[f64]| -(p[0] - 1.5).powi(2);
        let r1 = optimize_tpe(&[(-5.0, 5.0)], &config, objective);
        let r2 = optimize_tpe(&[(-5.0, 5.0)], &config, objective);
        assert_eq!(r1.trials, r2.trials);
        assert_eq!(r1.best_params, r2.best_params);
        assert_eq!(r1.best_score, r2.best_score);
    }

    /// Different seeds are not required to match (sanity check the seed is actually
    /// wired through, not incidental).
    #[test]
    fn test_tpe_different_seeds_can_differ() {
        let objective = |p: &[f64]| -(p[0] - 1.5).powi(2);
        let r1 = optimize_tpe(&[(-5.0, 5.0)], &TpeConfig::new(10, 1), objective);
        let r2 = optimize_tpe(&[(-5.0, 5.0)], &TpeConfig::new(10, 2), objective);
        assert_ne!(r1.trials, r2.trials);
    }

    #[test]
    fn test_tpe_select_from_pool_runs_and_bounds_evaluations() {
        // Pool of 50 1-D points in [0, 1]; objective favors points near 0.7.
        let points: Vec<Vec<f64>> = (0..50).map(|i| vec![i as f64 / 49.0]).collect();
        let config = TpeConfig::new(12, 3);
        let mut calls = 0usize;
        let (best_idx, best_score, history) = tpe_select_from_pool(&points, &config, |i| {
            calls += 1;
            -(points[i][0] - 0.7).powi(2)
        });
        assert_eq!(calls, 12, "should evaluate exactly n_trials pool points");
        assert_eq!(history.len(), 12);
        assert!(best_score.is_finite());
        assert!((points[best_idx][0] - 0.7).abs() < 0.3);
    }

    #[test]
    fn test_tpe_select_from_pool_deterministic() {
        let points: Vec<Vec<f64>> = (0..30).map(|i| vec![i as f64 / 29.0]).collect();
        let config = TpeConfig::new(10, 42);
        let objective = |i: usize| -(points[i][0] - 0.4).powi(2);
        let (idx1, score1, hist1) = tpe_select_from_pool(&points, &config, objective);
        let (idx2, score2, hist2) = tpe_select_from_pool(&points, &config, objective);
        assert_eq!(idx1, idx2);
        assert_eq!(score1, score2);
        assert_eq!(hist1, hist2);
    }

    #[test]
    fn test_tpe_select_from_pool_never_exceeds_pool_size() {
        let points: Vec<Vec<f64>> = (0..3).map(|i| vec![i as f64]).collect();
        let config = TpeConfig::new(100, 1); // more trials requested than pool entries
        let mut calls = 0usize;
        let _ = tpe_select_from_pool(&points, &config, |_| {
            calls += 1;
            0.0
        });
        assert_eq!(calls, 3);
    }

    #[test]
    fn test_tpe_select_from_pool_empty_pool_is_safe() {
        let points: Vec<Vec<f64>> = Vec::new();
        let config = TpeConfig::new(5, 1);
        let (idx, score, history) = tpe_select_from_pool(&points, &config, |_| 0.0);
        assert_eq!(idx, 0);
        assert_eq!(score, f64::NEG_INFINITY);
        assert!(history.is_empty());
    }
}
