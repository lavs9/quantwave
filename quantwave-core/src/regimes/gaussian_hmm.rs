//! Gaussian / lambda-emission Hidden Markov Model — batch fit, decode, and streaming filter.
//!
//! Generic HMM for univariate observation sequences (e.g. log-returns). Gaussian mode
//! aligns with ldhmm at λ=1; lambda (ecld) emissions match ldhmm for leptokurtic returns.
//!
//! Sources:
//! - Hamilton (1989) — regime switching
//! - Zucchini et al. (2016) — forward-backward, EM, Viterbi
//! - references/ldhmm/ldhmm-cran-reference.pdf — mllk, log_forward, viterbi
//! - references/ldhmm/ssrn-2979516.pdf — lambda emissions
//! - quantwave-core/tests/gold_standard/hmm_gaussian_2state.json — generic Gaussian fixture
//! - quantwave-core/tests/gold_standard/hmm_lambda_2state.json — generic lambda fixture

use super::ecld::ecld_pdf;
use crate::traits::Next;
use serde::{Deserialize, Serialize};

const LOG_FLOOR: f64 = 1e-300;

/// Stationary Gaussian HMM parameters for `m` latent states.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GaussianHmmParams {
    pub n_states: usize,
    /// Initial / stationary distribution δ (sums to 1).
    pub delta: Vec<f64>,
    /// Transition matrix Γ[from][to].
    pub gamma: Vec<Vec<f64>>,
    /// Emission mean μ per state.
    pub means: Vec<f64>,
    /// Emission standard deviation σ per state (must be > 0).
    pub stds: Vec<f64>,
    /// Tail parameter λ per state (λ=1 → Gaussian; λ>1 → heavier tails).
    #[serde(default = "default_lambdas_for_states")]
    pub lambdas: Vec<f64>,
}

fn default_lambdas_for_states() -> Vec<f64> {
    Vec::new()
}

/// Emission density family for HMM fitting and decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum EmissionFamily {
    #[default]
    Gaussian,
    /// Symmetric lambda (ecld / generalized normal) per state.
    Lambda,
}

/// Configuration for EM fitting.
#[derive(Debug, Clone)]
pub struct GaussianHmmFitConfig {
    pub n_states: usize,
    pub max_iter: usize,
    pub tol: f64,
    pub emission_family: EmissionFamily,
    /// When true and `emission_family == Lambda`, per-state λ is updated in the M-step.
    pub fit_lambdas: bool,
}

impl Default for GaussianHmmFitConfig {
    fn default() -> Self {
        Self {
            n_states: 2,
            max_iter: 100,
            tol: 1e-6,
            emission_family: EmissionFamily::Gaussian,
            fit_lambdas: false,
        }
    }
}

/// Result of EM parameter estimation.
#[derive(Debug, Clone)]
pub struct GaussianHmmFitResult {
    pub params: GaussianHmmParams,
    pub log_likelihood: f64,
    pub aic: f64,
    pub bic: f64,
    pub iterations: usize,
}

/// Batch decode output for a fixed parameter set.
#[derive(Debug, Clone)]
pub struct GaussianHmmDecode {
    /// P(C_t = i | x^T) — local / smoothed state probabilities [state][time].
    pub smooth_probs: Vec<Vec<f64>>,
    /// Causal filter P(C_t = i | x_{1:t}) [state][time].
    pub forward_filter: Vec<Vec<f64>>,
    /// Global Viterbi path (0-indexed states).
    pub viterbi_path: Vec<usize>,
    /// Minus log-likelihood (MLLK).
    pub mllk: f64,
}

/// Online forward-filter decoder (causal).
#[derive(Debug, Clone)]
pub struct GaussianHmmFilter {
    params: GaussianHmmParams,
    forward: Vec<f64>,
    initialized: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum GaussianHmmError {
    #[error("invalid HMM parameters: {0}")]
    InvalidParams(String),
    #[error("need at least {min} observations, got {got}")]
    InsufficientData { min: usize, got: usize },
    #[error("EM did not converge within {max_iter} iterations")]
    EmNotConverged { max_iter: usize },
}

impl GaussianHmmParams {
    pub fn new(
        delta: Vec<f64>,
        gamma: Vec<Vec<f64>>,
        means: Vec<f64>,
        stds: Vec<f64>,
    ) -> Result<Self, GaussianHmmError> {
        let n_states = delta.len();
        let lambdas = vec![1.0; n_states];
        Self::new_with_lambdas(delta, gamma, means, stds, lambdas)
    }

    pub fn new_with_lambdas(
        delta: Vec<f64>,
        gamma: Vec<Vec<f64>>,
        means: Vec<f64>,
        stds: Vec<f64>,
        lambdas: Vec<f64>,
    ) -> Result<Self, GaussianHmmError> {
        let n_states = delta.len();
        let params = Self {
            n_states,
            delta,
            gamma,
            means,
            stds,
            lambdas,
        };
        params.validate()?;
        Ok(params)
    }

    pub fn validate(&self) -> Result<(), GaussianHmmError> {
        let m = self.n_states;
        if m == 0 {
            return Err(GaussianHmmError::InvalidParams("n_states must be > 0".into()));
        }
        if self.gamma.len() != m
            || self.means.len() != m
            || self.stds.len() != m
            || self.delta.len() != m
            || (self.lambdas.len() != m && !self.lambdas.is_empty())
        {
            return Err(GaussianHmmError::InvalidParams(
                "parameter vector lengths must match n_states".into(),
            ));
        }
        if self.lambdas.is_empty() {
            // Backward-compatible deserialization: default λ=1 per state.
        } else {
            for (i, &lam) in self.lambdas.iter().enumerate() {
                if !lam.is_finite() || lam <= 0.0 {
                    return Err(GaussianHmmError::InvalidParams(format!(
                        "lambdas[{i}] must be positive and finite"
                    )));
                }
            }
        }
        let delta_sum: f64 = self.delta.iter().sum();
        if (delta_sum - 1.0).abs() > 1e-8 {
            return Err(GaussianHmmError::InvalidParams(format!(
                "delta must sum to 1, got {delta_sum}"
            )));
        }
        for (i, row) in self.gamma.iter().enumerate() {
            if row.len() != m {
                return Err(GaussianHmmError::InvalidParams(format!(
                    "gamma row {i} length mismatch"
                )));
            }
            let row_sum: f64 = row.iter().sum();
            if (row_sum - 1.0).abs() > 1e-8 {
                return Err(GaussianHmmError::InvalidParams(format!(
                    "gamma row {i} must sum to 1, got {row_sum}"
                )));
            }
        }
        for (i, &s) in self.stds.iter().enumerate() {
            if !s.is_finite() || s <= 0.0 {
                return Err(GaussianHmmError::InvalidParams(format!(
                    "stds[{i}] must be positive and finite"
                )));
            }
        }
        Ok(())
    }

    fn lambda_at(&self, state: usize) -> f64 {
        self.lambdas.get(state).copied().unwrap_or(1.0)
    }

    pub fn emission_pdf(&self, state: usize, x: f64) -> f64 {
        ecld_pdf(x, self.means[state], self.stds[state], self.lambda_at(state))
    }

    pub fn emission_log_pdf(&self, state: usize, x: f64) -> f64 {
        self.emission_pdf(state, x).ln()
    }

    /// Minus log-likelihood with scaling (ldhmm.mllk / Zucchini §3.2).
    pub fn mllk(&self, observations: &[f64]) -> Result<f64, GaussianHmmError> {
        if observations.is_empty() {
            return Err(GaussianHmmError::InsufficientData { min: 1, got: 0 });
        }
        Ok(scaled_forward(self, observations).mllk)
    }

    pub fn decode(&self, observations: &[f64]) -> Result<GaussianHmmDecode, GaussianHmmError> {
        if observations.is_empty() {
            return Err(GaussianHmmError::InsufficientData { min: 1, got: 0 });
        }
        let fwd = scaled_forward(self, observations);
        let smooth = forward_backward_smooth(self, observations);
        let forward_filter = fwd.filter;
        let viterbi_path = viterbi_decode(self, observations);
        Ok(GaussianHmmDecode {
            smooth_probs: smooth,
            forward_filter,
            viterbi_path,
            mllk: fwd.mllk,
        })
    }

    pub fn aic(&self, observations: &[f64]) -> Result<f64, GaussianHmmError> {
        self.aic_with_options(observations, false)
    }

    pub fn bic(&self, observations: &[f64]) -> Result<f64, GaussianHmmError> {
        self.bic_with_options(observations, false)
    }

    pub fn aic_with_options(
        &self,
        observations: &[f64],
        fit_lambdas: bool,
    ) -> Result<f64, GaussianHmmError> {
        let k = self.free_parameter_count(fit_lambdas) as f64;
        let ll = -self.mllk(observations)?;
        Ok(2.0 * k - 2.0 * ll)
    }

    pub fn bic_with_options(
        &self,
        observations: &[f64],
        fit_lambdas: bool,
    ) -> Result<f64, GaussianHmmError> {
        let n = observations.len() as f64;
        let k = self.free_parameter_count(fit_lambdas) as f64;
        let ll = -self.mllk(observations)?;
        Ok(k * n.ln() - 2.0 * ll)
    }

    fn free_parameter_count(&self, fit_lambdas: bool) -> usize {
        let m = self.n_states;
        let mut k = (m * m - 1) + (m - 1) + 2 * m;
        if fit_lambdas {
            k += m;
        }
        k
    }

    pub fn filter(&self) -> GaussianHmmFilter {
        GaussianHmmFilter::new(self.clone())
    }
}

impl GaussianHmmFilter {
    pub fn new(params: GaussianHmmParams) -> Self {
        let m = params.n_states;
        Self {
            params,
            forward: vec![0.0; m],
            initialized: false,
        }
    }

    /// Causal state probabilities P(C_t | x_{1:t}).
    pub fn state_probabilities(&self) -> Vec<f64> {
        if self.initialized {
            self.forward.clone()
        } else {
            self.params.delta.clone()
        }
    }
}

impl Next<f64> for GaussianHmmFilter {
    type Output = Vec<f64>;

    fn next(&mut self, x: f64) -> Self::Output {
        let m = self.params.n_states;
        if !self.initialized {
            let mut probs = vec![0.0; m];
            let mut sum = 0.0;
            for i in 0..m {
                probs[i] = self.params.delta[i] * self.params.emission_pdf(i, x);
                sum += probs[i];
            }
            if sum > 0.0 {
                for p in &mut probs {
                    *p /= sum;
                }
            }
            self.forward = probs;
            self.initialized = true;
            return self.forward.clone();
        }

        let mut next = vec![0.0; m];
        let mut sum = 0.0;
        for j in 0..m {
            let mut acc = 0.0;
            for i in 0..m {
                acc += self.forward[i] * self.params.gamma[i][j];
            }
            next[j] = acc * self.params.emission_pdf(j, x);
            sum += next[j];
        }
        if sum > 0.0 {
            for p in &mut next {
                *p /= sum;
            }
        }
        self.forward = next;
        self.forward.clone()
    }
}

/// Fit a Gaussian HMM with Baum–Welch EM on a generic observation vector.
pub fn fit_em(
    observations: &[f64],
    config: &GaussianHmmFitConfig,
) -> Result<GaussianHmmFitResult, GaussianHmmError> {
    let n = observations.len();
    if n < config.n_states + 1 {
        return Err(GaussianHmmError::InsufficientData {
            min: config.n_states + 1,
            got: n,
        });
    }
    let m = config.n_states;
    let mut params = init_params_em(observations, m, config)?;
    let mut prev_mllk = f64::INFINITY;
    let mut iterations = 0usize;
    let fit_lambdas = config.emission_family == EmissionFamily::Lambda && config.fit_lambdas;

    for iter in 0..config.max_iter {
        iterations = iter + 1;
        let (gamma_t, xi_t) = e_step(&params, observations)?;
        params = m_step(&params, observations, &gamma_t, &xi_t, config)?;
        let mllk = params.mllk(observations)?;
        if (prev_mllk - mllk).abs() < config.tol {
            let ll = -mllk;
            return Ok(GaussianHmmFitResult {
                aic: params.aic_with_options(observations, fit_lambdas)?,
                bic: params.bic_with_options(observations, fit_lambdas)?,
                iterations,
                log_likelihood: ll,
                params,
            });
        }
        prev_mllk = mllk;
    }

    let mllk = params.mllk(observations)?;
    Ok(GaussianHmmFitResult {
        log_likelihood: -mllk,
        aic: params.aic_with_options(observations, fit_lambdas)?,
        bic: params.bic_with_options(observations, fit_lambdas)?,
        iterations,
        params,
    })
}

struct ScaledForward {
    filter: Vec<Vec<f64>>,
    mllk: f64,
}

fn scaled_forward(params: &GaussianHmmParams, x: &[f64]) -> ScaledForward {
    let m = params.n_states;
    let n = x.len();
    let mut filter = vec![vec![0.0; n]; m];
    let mut phi = vec![0.0; m];

    for i in 0..m {
        phi[i] = params.delta[i] * params.emission_pdf(i, x[0]);
    }
    let sum0: f64 = phi.iter().sum();
    let mut log_scale = sum0.ln();
    for i in 0..m {
        filter[i][0] = phi[i] / sum0;
    }

    for t in 1..n {
        let mut next = vec![0.0; m];
        for j in 0..m {
            let mut acc = 0.0;
            for i in 0..m {
                acc += filter[i][t - 1] * params.gamma[i][j];
            }
            next[j] = acc * params.emission_pdf(j, x[t]);
        }
        let sum_t: f64 = next.iter().sum();
        log_scale += sum_t.ln();
        for j in 0..m {
            filter[j][t] = next[j] / sum_t;
        }
    }

    ScaledForward {
        filter,
        mllk: -log_scale,
    }
}

fn forward_backward_smooth(params: &GaussianHmmParams, x: &[f64]) -> Vec<Vec<f64>> {
    let m = params.n_states;
    let n = x.len();
    let fwd = scaled_forward(params, x);
    let mut beta = vec![vec![0.0; n]; m];
    for i in 0..m {
        beta[i][n - 1] = 1.0;
    }

    for t in (0..n - 1).rev() {
        let mut next = vec![0.0; m];
        for i in 0..m {
            let mut acc = 0.0;
            for j in 0..m {
                acc += params.gamma[i][j] * params.emission_pdf(j, x[t + 1]) * beta[j][t + 1];
            }
            next[i] = acc;
        }
        let sum_t: f64 = next.iter().sum();
        for i in 0..m {
            beta[i][t] = if sum_t > 0.0 { next[i] / sum_t } else { 0.0 };
        }
    }

    let mut smooth = vec![vec![0.0; n]; m];
    for t in 0..n {
        let mut raw = vec![0.0; m];
        let mut sum = 0.0;
        for i in 0..m {
            raw[i] = fwd.filter[i][t] * beta[i][t];
            sum += raw[i];
        }
        for i in 0..m {
            smooth[i][t] = if sum > 0.0 { raw[i] / sum } else { 0.0 };
        }
    }
    smooth
}

fn viterbi_decode(params: &GaussianHmmParams, x: &[f64]) -> Vec<usize> {
    let m = params.n_states;
    let n = x.len();
    let mut delta = vec![vec![0.0; n]; m];
    let mut psi = vec![vec![0usize; n]; m];

    for i in 0..m {
        delta[i][0] = (params.delta[i] * params.emission_pdf(i, x[0]) + LOG_FLOOR).ln();
    }

    for t in 1..n {
        for j in 0..m {
            let mut best_log = f64::NEG_INFINITY;
            let mut best_i = 0usize;
            for i in 0..m {
                let v = delta[i][t - 1]
                    + (params.gamma[i][j] + LOG_FLOOR).ln()
                    + params.emission_log_pdf(j, x[t]);
                if v > best_log {
                    best_log = v;
                    best_i = i;
                }
            }
            delta[j][t] = best_log;
            psi[j][t] = best_i;
        }
    }

    let mut path = vec![0usize; n];
    path[n - 1] = (0..m)
        .max_by(|&a, &b| {
            delta[a][n - 1]
                .partial_cmp(&delta[b][n - 1])
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap_or(0);

    for t in (0..n - 1).rev() {
        path[t] = psi[path[t + 1]][t + 1];
    }
    path
}

fn init_params_em(
    x: &[f64],
    m: usize,
    config: &GaussianHmmFitConfig,
) -> Result<GaussianHmmParams, GaussianHmmError> {
    let mean_all: f64 = x.iter().sum::<f64>() / x.len() as f64;
    let var_all: f64 = x.iter().map(|v| (v - mean_all).powi(2)).sum::<f64>() / x.len() as f64;
    let base_std = var_all.sqrt().max(1e-4);

    let mut means = Vec::with_capacity(m);
    let mut stds = Vec::with_capacity(m);
    for k in 0..m {
        let offset = (k as f64 - (m as f64 - 1.0) / 2.0) * base_std * 0.5;
        means.push(mean_all + offset);
        stds.push(base_std);
    }

    let mut gamma = vec![vec![0.0; m]; m];
    let stay = 0.9;
    let off = (1.0 - stay) / (m as f64 - 1.0).max(1.0);
    for i in 0..m {
        for j in 0..m {
            gamma[i][j] = if i == j { stay } else { off };
        }
    }

    let delta = vec![1.0 / m as f64; m];
    let lambdas = match config.emission_family {
        EmissionFamily::Gaussian => vec![1.0; m],
        EmissionFamily::Lambda => vec![1.0; m],
    };
    GaussianHmmParams::new_with_lambdas(delta, gamma, means, stds, lambdas)
}

fn e_step(
    params: &GaussianHmmParams,
    x: &[f64],
) -> Result<(Vec<Vec<f64>>, Vec<Vec<Vec<f64>>>), GaussianHmmError> {
    let smooth = forward_backward_smooth(params, x);
    let m = params.n_states;
    let n = x.len();
    let mut xi: Vec<Vec<Vec<f64>>> = (0..n - 1)
        .map(|_| (0..m).map(|_| vec![0.0; m]).collect())
        .collect();

    for t in 0..n - 1 {
        let mut denom = 0.0;
        let mut numer = vec![vec![0.0; m]; m];
        for i in 0..m {
            for j in 0..m {
                numer[i][j] = smooth[i][t]
                    * params.gamma[i][j]
                    * params.emission_pdf(j, x[t + 1]);
                denom += numer[i][j];
            }
        }
        for i in 0..m {
            for j in 0..m {
                xi[t][i][j] = if denom > 0.0 { numer[i][j] / denom } else { 0.0 };
            }
        }
    }
    Ok((smooth, xi))
}

fn m_step(
    prev: &GaussianHmmParams,
    x: &[f64],
    gamma_t: &[Vec<f64>],
    xi: &[Vec<Vec<f64>>],
    config: &GaussianHmmFitConfig,
) -> Result<GaussianHmmParams, GaussianHmmError> {
    let m = gamma_t.len();
    let n = x.len();

    let mut delta = vec![0.0; m];
    for i in 0..m {
        delta[i] = gamma_t[i][0];
    }
    let d_sum: f64 = delta.iter().sum();
    if d_sum > 0.0 {
        for d in &mut delta {
            *d /= d_sum;
        }
    }

    let mut gamma = vec![vec![0.0; m]; m];
    for i in 0..m {
        let mut row_sum = 0.0;
        for j in 0..m {
            let mut acc = 0.0;
            for t in 0..xi.len() {
                acc += xi[t][i][j];
            }
            gamma[i][j] = acc;
            row_sum += acc;
        }
        if row_sum > 0.0 {
            for j in 0..m {
                gamma[i][j] /= row_sum;
            }
        } else {
            for j in 0..m {
                gamma[i][j] = if i == j { 1.0 } else { 0.0 };
            }
        }
    }

    let mut means = vec![0.0; m];
    let mut stds = vec![0.0; m];
    let mut lambdas = prev.lambdas.clone();
    if lambdas.len() != m {
        lambdas = vec![1.0; m];
    }

    for j in 0..m {
        let mut w_sum = 0.0;
        let mut mean_acc = 0.0;
        for t in 0..n {
            let w = gamma_t[j][t];
            w_sum += w;
            mean_acc += w * x[t];
        }
        means[j] = if w_sum > 0.0 { mean_acc / w_sum } else { 0.0 };

        let mut var_acc = 0.0;
        for t in 0..n {
            let w = gamma_t[j][t];
            var_acc += w * (x[t] - means[j]).powi(2);
        }
        stds[j] = if w_sum > 0.0 {
            (var_acc / w_sum).sqrt().max(1e-6)
        } else {
            1e-3
        };

        if config.emission_family == EmissionFamily::Lambda && config.fit_lambdas {
            // Alternate λ / σ profile updates (ldhmm-style numerical MLE for ecld scale).
            let mut sigma = stds[j];
            let mut lambda = lambdas[j];
            for _ in 0..3 {
                lambda = profile_lambda_for_state(x, &gamma_t[j], means[j], sigma, lambda);
                sigma = profile_sigma_for_state(x, &gamma_t[j], means[j], lambda, sigma);
            }
            lambdas[j] = lambda;
            stds[j] = sigma;
        }
    }

    GaussianHmmParams::new_with_lambdas(delta, gamma, means, stds, lambdas)
}

fn weighted_neg_log_likelihood(
    x: &[f64],
    weights: &[f64],
    mu: f64,
    sigma: f64,
    lambda: f64,
) -> f64 {
    if sigma <= 1e-8 || lambda < 1.0 || lambda > 8.0 {
        return f64::INFINITY;
    }
    let mut nll = 0.0;
    for (&obs, &w) in x.iter().zip(weights.iter()) {
        if w > 0.0 {
            let p = ecld_pdf(obs, mu, sigma, lambda);
            if p > 0.0 {
                nll -= w * p.ln();
            } else {
                return f64::INFINITY;
            }
        }
    }
    nll
}

/// Weighted 1-D profile likelihood for λ (golden-section search on log λ).
fn profile_lambda_for_state(
    x: &[f64],
    weights: &[f64],
    mu: f64,
    sigma: f64,
    _initial: f64,
) -> f64 {
    let objective = |log_lam: f64| weighted_neg_log_likelihood(x, weights, mu, sigma, log_lam.exp());
    golden_section_minimize_log(objective, 1.0f64.ln(), 5.0f64.ln())
        .exp()
        .clamp(1.0, 5.0)
}

fn profile_sigma_for_state(
    x: &[f64],
    weights: &[f64],
    mu: f64,
    lambda: f64,
    initial: f64,
) -> f64 {
    let lo = 1e-5f64.ln();
    let hi = (initial.max(1e-4) * 4.0).max(0.05).ln();
    let objective =
        |log_sigma: f64| weighted_neg_log_likelihood(x, weights, mu, log_sigma.exp(), lambda);
    golden_section_minimize_log(objective, lo, hi).exp().max(1e-6)
}

fn golden_section_minimize_log<F>(f: F, mut a: f64, mut b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    const PHI: f64 = 1.618_033_988_749_895;
    let tol = 1e-4;
    let mut c = b - (b - a) / PHI;
    let mut d = a + (b - a) / PHI;
    let mut fc = f(c);
    let mut fd = f(d);
    for _ in 0..80 {
        if (b - a).abs() < tol {
            break;
        }
        if fc < fd {
            b = d;
            d = c;
            fd = fc;
            c = b - (b - a) / PHI;
            fc = f(c);
        } else {
            a = c;
            c = d;
            fc = fd;
            d = a + (b - a) / PHI;
            fd = f(d);
        }
    }
    if fc < fd {
        c
    } else {
        d
    }
}

// --- IndicatorMetadata (quantwave-i9dn) ---

use crate::indicators::metadata::{IndicatorMetadata, ParamDef};

pub const GAUSSIAN_HMM_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "gaussian_hmm",
    description:
        "Fittable HMM with Gaussian or lambda (ecld) emissions for generic univariate series (e.g. log-returns).",
    usage: "Fit with EM on an observation vector (Gaussian or lambda emission family), decode smoothed state \
             probabilities and a Viterbi path, or stream causal forward-filter probabilities for live regime labeling.",
    keywords: &[
        "regime",
        "hmm",
        "gaussian",
        "lambda",
        "ecld",
        "em",
        "mle",
        "viterbi",
        "ldhmm",
    ],
    ehlers_summary: "Homogeneous first-order HMM with Gaussian or symmetric lambda (ecld) state emissions and \
                     Baum–Welch EM fitting. Gaussian mode aligns with ldhmm at λ=1; lambda mode matches ldhmm \
                     leptokurtic daily-return modeling (SSRN 2979516).",
    params: &[
        ParamDef {
            name: "n_states",
            default: "2",
            description: "Number of latent states (m ≥ 2).",
        },
        ParamDef {
            name: "max_iter",
            default: "100",
            description: "Maximum EM iterations.",
        },
        ParamDef {
            name: "tol",
            default: "1e-6",
            description: "EM convergence tolerance on MLLK improvement.",
        },
        ParamDef {
            name: "emission_family",
            default: "Gaussian",
            description: "Emission density: Gaussian (λ=1) or Lambda (ecld per-state λ).",
        },
        ParamDef {
            name: "fit_lambdas",
            default: "false",
            description: "When emission_family=Lambda, estimate per-state λ in the M-step.",
        },
    ],
    formula_source: "references/ldhmm/ldhmm-cran-reference.pdf; references/ldhmm/ssrn-2979516.pdf; Hamilton (1989); Zucchini et al. (2016)",
    formula_latex: "",
    gold_standard_file: "hmm_gaussian_2state.json",
    category: "Regime",
};

pub const LAMBDA_HMM_METADATA: IndicatorMetadata = IndicatorMetadata {
    name: "lambda_hmm",
    description:
        "Lambda-distribution (ecld) emission HMM for leptokurtic returns — ldhmm parity mode.",
    usage: "Same API as gaussian_hmm with emission_family=Lambda; per-state (μ, σ, λ) and optional λ MLE.",
    keywords: &["regime", "hmm", "lambda", "ecld", "ldhmm", "leptokurtic"],
    ehlers_summary: "Symmetric exponential-power emissions (β=2/λ) per ldhmm ecld.*; λ=1 reduces to Gaussian.",
    params: &[
        ParamDef {
            name: "n_states",
            default: "2",
            description: "Number of latent states.",
        },
        ParamDef {
            name: "max_iter",
            default: "100",
            description: "Maximum EM iterations.",
        },
        ParamDef {
            name: "fit_lambdas",
            default: "true",
            description: "Estimate per-state λ in the M-step.",
        },
    ],
    formula_source: "references/ldhmm/ssrn-2979516.pdf; references/ldhmm/ldhmm-cran-reference.pdf",
    formula_latex: "",
    gold_standard_file: "hmm_lambda_2state.json",
    category: "Regime",
};