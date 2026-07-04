# gaussian_hmm

<div class="indicator-meta"><span class="category-badge">Regime</span> <span class="kw-badge">regime</span> <span class="kw-badge">hmm</span> <span class="kw-badge">gaussian</span> <span class="kw-badge">lambda</span> <span class="kw-badge">ecld</span> <span class="kw-badge">em</span></div>

Fittable HMM with Gaussian or lambda (ecld) emissions for generic univariate series (e.g. log-returns).

## Visual Example

![Regime decode schematic — smoothed state probabilities over synthetic returns](../../../assets/indicator-previews/supertrend.png)

*Conceptual regime decode: forward-backward smoothed state weights and Viterbi path from a fitted 2-state HMM on log-returns. See [Regime Detection guide](../regimes/) for ldhmm-style workflows.*

## Description

A **fittable first-order Hidden Markov Model** for univariate series such as daily log-returns. Each latent state emits from a Gaussian density (λ=1) or, when configured, a symmetric **lambda (ecld)** distribution for leptokurtic tails — matching the [ldhmm](https://cran.r-project.org/package=ldhmm) R package at λ=1 and extending to lambda emissions per SSRN 2979516.

**Baum–Welch EM** estimates transition matrix Γ, initial distribution δ, and per-state (μ, σ) — optionally per-state λ. **Forward–backward** decoding yields smoothed state probabilities; **Viterbi** yields a global path. A causal **forward filter** implements `Next<f64>` for live regime labeling with batch parity.

Typical workflow: fit on a return series → decode regimes → optionally chain [hmm_forecast](hmm_forecast/) for vol/state forecasts. Gold-standard vectors in `hmm_gaussian_2state.json` lock MLLK, smooth probs, and Viterbi against reference Python. The preset `HMM::bull_bear()` remains available for zero-config backtests.

## Formula / Specification

**Implementation** (`quantwave-core/src/regimes/gaussian_hmm.rs`):

See the `Next<T>` implementation and `METADATA` in the core module.

Gold-standard parity vectors: `quantwave-core/tests/gold_standard/hmm_gaussian_2state.json`.


## Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `n_states` | 2 | Number of latent states (m ≥ 2). |
| `max_iter` | 100 | Maximum EM iterations. |
| `tol` | 1e-6 | EM convergence tolerance on MLLK improvement. |
| `emission_family` | Gaussian | Emission density: Gaussian (λ=1) or Lambda (ecld per-state λ). |
| `fit_lambdas` | false | When emission_family=Lambda, estimate per-state λ in the M-step. |


## Usage Examples

**Polars (batch EM fit + decode)**

```python
import polars as pl

df = (
    pl.read_csv("ohlcv.csv")
    .lazy()
    .with_columns(pl.col("close").pct_change().alias("returns"))
    .ta()
    .hmm_fit("returns", n_states=2, max_iter=100, fit_lambdas=False)
    .collect()
)
# Column hmm_fit_data: hmm_fit_state, hmm_fit_smooth_probs
```

**Python (fit + streaming filter)**

```python
import quantwave as qw

fit = qw.fit_gaussian_hmm(returns, n_states=2, max_iter=100, fit_lambdas=False)
filt = qw.GaussianHmmFilterPy.from_params(fit.params)
for x in returns:
    probs = filt.next(x)  # causal P(state | data so far)
```

**Rust (core)**

```rust
use quantwave_core::regimes::gaussian_hmm::{fit_em, GaussianHmmFitConfig, GaussianHmmFilter};
use quantwave_core::traits::Next;

let fit = fit_em(&returns, &GaussianHmmFitConfig::default())?;
let decode = fit.params.decode(&returns)?;
let mut filt = fit.params.filter();
for &x in &returns {
    let probs = filt.next(x);
}
```

## Edge Cases & Limitations

- Warm-up: first `2` bars may return NaN or partial state per implementation.
- Parameter sensitivity: smaller periods increase noise; larger periods increase lag.
- Sudden gaps or bad ticks can distort rolling windows — consider pre-filtering.
- Single-series indicators ignore volume unless otherwise documented.
- Validated via proptests against gold-standard vectors where available.
- No look-ahead bias; streaming and Polars batch paths are bit-identical.

## Boundary Behavior

| Condition | Behavior |
|-----------|----------|
| Warm-up | Leading bars return NaN until warmup_bars is satisfied. |
| period > len | When period exceeds series length, output is all NaN. |
| NaN inputs | NaN in input propagates to output (NaN out). |
| Invalid params | Non-positive period or missing required params raise ValueError. |
| Empty data | Empty input returns an empty result series. |

## Related Indicators & See Also

- [Indicator Gallery](../gallery.md)
- [Native Indicators index](index.md)
- [Batch vs Streaming guide](../../../examples/batch-streaming.md)
- [RSI](relative_strength_index_rsi.md)
- [SuperTrend](supertrend/)

## Sources & References

**Primary Source**: references/ldhmm/ldhmm-cran-reference.pdf; references/ldhmm/ssrn-2979516.pdf; Hamilton (1989); Zucchini et al. (2016)

**Implementation**: `quantwave-core/src/indicators/gaussian_hmm.rs` (`GAUSSIAN_HMM` / `GAUSSIAN_HMM_METADATA`).
**Parity**: `quantwave-core/tests/gold_standard/hmm_gaussian_2state.json`

**Provenance**: Standards bulk upgrade 2026-07-04 IST — see `docs/DOCUMENTATION_STANDARDS.md`.
